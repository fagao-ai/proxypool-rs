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

impl Proxy {
    pub fn new(ip: &str, port: i8, proxy_type: ProxyType) -> Self {
        Self {
            ip: ip.parse::<Ipv4Addr>().unwrap(),
            port: port,
            proxy_type: ProxyType::Http,
        }
    }
}

pub struct ChunkProxyCrawlSite {
    pub urls: Vec<String>,
    pub chunk_size: usize,
}

impl ChunkProxyCrawlSite {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            urls: urls,
            chunk_size: 100,
        }
    }
}

#[async_trait]
pub trait ProxyCrawler {
    async fn crawl_proxies(&self) -> Vec<Proxy>;
    async fn fetch_pages(&self, proxy_crawl_site: ChunkProxyCrawlSite) -> Vec<String>;
    fn find_crawl_url(&self, html: &str) -> Vec<String>;
}
