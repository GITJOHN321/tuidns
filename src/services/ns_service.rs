use crate::infrastructure::resolve_ns;

pub fn resolve_ns(domain: &str) -> String {
    resolve_ns::resolve_ns(domain)
}
