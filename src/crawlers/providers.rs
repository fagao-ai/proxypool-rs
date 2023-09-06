use crate::crawlers::crawl_trait::{ChunkProxyCrawlSite, Proxy, ProxyCrawler};
use async_trait::async_trait;
use futures::future;
use reqwest::Client;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

struct FreeProxyListSite {
    domain: String,
}

#[async_trait]
impl ProxyCrawler for FreeProxyListSite {
    fn crawl_proxies(&self) -> Vec<Proxy> {
        todo!()
    }

    async fn fetch_pages(proxy_crawl_site: ChunkProxyCrawlSite) -> Vec<String> {
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
    fn find_crawl_url(html: &str) -> Vec<String> {
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
    fn parse_page(&self, html: &str) {
        todo!()
    }
}
