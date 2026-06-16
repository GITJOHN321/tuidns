use crate::infrastructure::resolve_dkim;

pub fn resolve_dkim(domain: &str) -> String {
    let selectors = [
        "default",
        "selector1",
        "selector2",
        "google",
    ];

    for selector in selectors {
        let host = format!("{selector}._domainkey.{domain}");
        let response = resolve_dkim::query_txt(&host);

        if response.contains("v=DKIM1") {
            return response.trim().to_string();
        }
    }

    "Not Found".to_string()
}
