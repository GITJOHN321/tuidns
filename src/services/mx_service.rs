use crate::infrastructure::resolve_mx;

pub fn resolve_mx(domain: &str) -> String {
    resolve_mx::resolve_mx(domain)
}
