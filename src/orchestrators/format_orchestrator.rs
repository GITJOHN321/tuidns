use crate::domain::dns_model::DnsQuery;
use crate::services::format_service::format_dns_query;
use crate::services::clipboard_service::copy_to_clipboard;

pub fn send_clipboard(dns_query: &DnsQuery)-> String{
    let formatted = format_dns_query(&dns_query);
    copy_to_clipboard(&formatted);
    "Text Copied".to_string()
}

