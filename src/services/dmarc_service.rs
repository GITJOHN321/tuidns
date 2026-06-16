use crate::infrastructure::resolve_nslookup;

pub fn resolve_dmarc(domain: &str) -> String {
    let host = format!("_dmarc.{domain}");
    let response = resolve_nslookup::query_txt(&host);

    for line in response.lines() {
        if line.contains("text =") {
            return line
                .split('"')
                .nth(1)
                .unwrap_or("")
                .to_string();
        }
    }

    "No DMARC Record Found".to_string()
}
