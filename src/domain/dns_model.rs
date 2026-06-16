#[derive(Debug, Clone, Default)]
pub struct DnsQuery {
    pub domain: String,

    pub hosts: Vec<Host>,
    pub ns: String,
    pub mx: String,
    pub panel: String,
    pub spf: String,
    pub dkim: String,
    pub dmarc: String,
    pub whois: WhoisInfo,
}

#[derive(Debug, Clone, Default)]
pub struct Host {
    pub name: String,
    pub ip: String,
    pub ptr: String,
    pub ping: String,
    pub ssl: Ssl,
}

#[derive(Debug, Clone, Default)]
pub struct Ssl {
    pub date: String,
    pub organization: String,
    pub active: String,
}
#[derive(Debug, Clone, Default)]
pub struct WhoisInfo {
    pub registrar: String,
    pub expire_date: String,
    pub statuses: String,
}
#[derive(Debug, Clone, Default)]
pub struct Panel {
    pub name: &'static str,
    pub ports: &'static [u16],
}