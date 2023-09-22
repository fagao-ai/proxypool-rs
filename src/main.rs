mod crawlers;

use crawlers::providers::FreeProxyListSite;

fn main() {
    let free_proxy = FreeProxyListSite::new();
    print!("{:?}", free_proxy)
}
