use crate::infrastructure::resolve_nslookup;

pub fn resolve_spf(domain: &str) -> String {
    let response = resolve_nslookup::query_txt(domain);

    for line in response.lines() {
        let line = line.trim();

        if let Some(pos) = line.find("v=spf1") {
            return line[pos..]
                .trim_matches('"')
                .to_string();
        }
    }

    "No SPF Record Found".to_string()
}
