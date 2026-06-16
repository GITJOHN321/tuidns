use std::thread;

use crate::domain::dns_model::Host;
use crate::infrastructure::resolve_ip::resolve_ip;
use crate::infrastructure::resolve_ping::resolve_ping;
use crate::infrastructure::resolve_ssl::resolve_ssl;

pub fn query_domain(domain: &str) -> Vec<Host> {

    let subdomains = [
        "",
        "www.",
        "mail.",
        "ftp.",
        "webmail.",
    ];
    let handles: Vec<_> = subdomains
        .into_iter()
        .map(|subdomain| {
            let domain = domain.to_string();
            thread::spawn(move || {

                let hostname =
                    format!("{subdomain}{domain}");

                let ip_info = resolve_ip(&hostname);

                Host {
                    name: if subdomain.is_empty() {
                        "@".to_string()
                    } else {
                        subdomain.to_string()
                    },

                    ip: ip_info.ip,

                    ptr: ip_info.ptr,
                    ping: resolve_ping(&hostname),
                    ssl: resolve_ssl(&hostname)
                }
            })
        })
        .collect();
    let hosts = handles
    .into_iter()
    .map(|h| h.join().unwrap())
    .collect();

    hosts
}
 


