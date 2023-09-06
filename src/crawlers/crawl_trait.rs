use async_trait::async_trait;
use std::net::Ipv4Addr;

pub enum ProxyType {
    Http,
    Https,
}

pub struct Proxy {
    pub ip: Ipv4Addr,
    pub port: i8,
    pub proxy_type: ProxyType,
}

pub struct ChunkProxyCrawlSite {
    pub urls: Vec<String>,
    pub chunk_size: usize,
}

impl ChunkProxyCrawlSite {
    fn new(urls: Vec<String>) -> Self {
        Self {
            urls: urls,
            chunk_size: 100,
        }
    }
}

#[async_trait]
pub trait ProxyCrawler {
    fn crawl_proxies(&self) -> Vec<Proxy>;
    async fn fetch_pages(proxy_crawl_site: ChunkProxyCrawlSite) -> Vec<String>;
    fn parse_page(&self, html: &str);

    fn find_crawl_url(html: &str) -> Vec<String>;
}
