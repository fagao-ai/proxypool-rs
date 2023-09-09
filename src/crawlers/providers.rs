use crate::crawlers::crawl_trait::{ChunkProxyCrawlSite, Proxy, ProxyCrawler};
use async_trait::async_trait;
use futures::future;
use reqwest::Client;

extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

use super::crawl_trait::ProxyType;

struct FreeProxyListSite {
    domain: String,
    proxy_pattern: Regex,
}

impl FreeProxyListSite {
    fn new() -> Self {
        Self {
            domain: "freeproxylists.com".to_string(),
            proxy_pattern: Regex::new(r"(?P<ip>(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?))(?=.*?(?:(?:(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?))|(?P<port>\d{2,5})))").unwrap(),
        }
    }
}

#[async_trait]
impl ProxyCrawler for FreeProxyListSite {
    async fn fetch_pages(&self, proxy_crawl_site: ChunkProxyCrawlSite) -> Vec<String> {
        let client = Client::new();
        let mut html_contents = Vec::new();
        for chunk in proxy_crawl_site.urls.chunks(proxy_crawl_site.chunk_size) {
            let bodies = future::join_all(chunk.into_iter().map(|url| {
                let client = &client;
                async move {
                    let resp = client.get(url).send().await?;
                    resp.text().await
                }
            }))
            .await;

            for b in bodies {
                match b {
                    Ok(b) => html_contents.push(b),
                    Err(e) => eprintln!("Got an error: {}", e),
                }
            }
        }
        html_contents
    }

    async fn crawl_proxies(&self) -> Vec<Proxy> {
        let page_urls = vec![
            "http://www.freeproxylists.com/socks.html".to_string(),
            "http://www.freeproxylists.com/elite.html".to_string(),
            "http://www.freeproxylists.com/anonymous.html".to_string(),
        ];
        let page_sites: ChunkProxyCrawlSite = ChunkProxyCrawlSite::new(page_urls);
        let html_contents = self.fetch_pages(page_sites).await;
        let mut crawl_urls = Vec::new();
        for html in html_contents {
            crawl_urls.extend(self.find_crawl_url(&html));
        }
        let each_page_contents = self.fetch_pages(ChunkProxyCrawlSite::new(crawl_urls)).await;
        let result: String = each_page_contents.join("\n");
        let proxy_ips: Vec<Proxy> = self
            .proxy_pattern
            .captures_iter(&result)
            .map(|m| {
                Proxy::new(
                    m.name("ip").unwrap().as_str(),
                    m.name("port").unwrap().as_str().parse::<i8>().unwrap(),
                    ProxyType::Http,
                )
            })
            .collect();
        proxy_ips
    }

    fn find_crawl_url(&self, html: &str) -> Vec<String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"href\s*=\s*['"](?P<t>[^'"]*)/(?P<uts>\d{10})[^'"]*['"]"#).unwrap();
        }
        let mut crawl_urls = Vec::new();
        for captures in RE.captures_iter(html) {
            let t = captures.name("t").unwrap().as_str();
            let uts = captures.name("uts").unwrap().as_str();
            let new_url = format!("http://www.freeproxylists.com/load_{}_{}.html", t, uts);
            crawl_urls.push(new_url);
        }
        crawl_urls
    }
}
