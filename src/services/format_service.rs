use crate::domain::dns_model::DnsQuery;

pub fn format_dns_query(q: &DnsQuery) -> String {
    let mut out = String::new();

    out.push_str(&format!("DOMAIN: {}\n\n", q.domain));

    out.push_str("=== HOSTS ===\n");
    for h in &q.hosts {
        let ips: Vec<&str> = h.ip.lines().collect();
        let ptrs: Vec<&str> = h.ptr.lines().collect();
        out.push_str(&format!("{}{}:\n", h.name, q.domain));

        for (ip, ptr) in ips.iter().zip(ptrs.iter()) {
            out.push_str(&format!("{ip} -> {ptr}\n"));
        }
        out.push_str("\n");
    }

    out.push_str(&format!("Name Servers:\n{}\n\n",&q.ns)); 

    out.push_str(&format!("MX:\n{}\n\n",&q.mx)); 

    out.push_str(&format!("- SPF:{}\n",&q.spf));
    out.push_str(&format!("- DMARC:{}\n",&q.dmarc));
    out.push_str(&format!("- DKIM:{}\n",&q.dkim));

    out
}