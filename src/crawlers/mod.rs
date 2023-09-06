mod crawl_trait;
mod providers;


pub use crawl_trait::ProxyCrawler;


struct Provider {
    url: String,

}