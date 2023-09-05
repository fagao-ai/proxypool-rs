use std::net::Ipv4Addr;
use reqwest;
use reqwest::Error as RequestError;
use async_trait::async_trait;


pub enum ProxyType {
    Http,
    Https,
}
pub struct Proxy{
    pub ip: Ipv4Addr,
    pub port: i8,
    pub proxy_type: ProxyType
}

#[async_trait]
pub trait ProxyCrawler{
    fn crawl_proxies(&self) -> Vec<Proxy>;

    async fn request_website(&self, url: &str) -> Result<String, RequestError> {
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(RequestError::from(response.error_for_status().unwrap_err()))
        }
    }
}