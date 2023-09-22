mod crawl_trait;
pub mod providers;


pub use crawl_trait::ProxyCrawler;


struct Provider {
    url: String,

}