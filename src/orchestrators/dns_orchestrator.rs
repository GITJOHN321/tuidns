use std::{
    sync::mpsc,
    thread,
    time::Duration,
};

use crate::models::dns_model::{DnsQuery, WhoisInfo};
use crate::services::{
    dkim_service,
    dmarc_service,
    dns_service,
    mx_service,
    ns_service,
    panel_service,
    spf_service,
    whois_service,
};

fn whois_timeout() -> WhoisInfo {
    WhoisInfo {
        registrar: "Timeout".to_string(),
        expire_date: "Timeout".to_string(),
        statuses: "Timeout".to_string(),
    }
}

pub fn execute_query(domain: &str) -> DnsQuery {
    let domain = domain.to_string();

    let hosts_handle = {
        let d = domain.clone();
        thread::spawn(move || dns_service::query_domain(&d))
    };

    let ns_handle = {
        let d = domain.clone();
        thread::spawn(move || ns_service::resolve_ns(&d))
    };

    let mx_handle = {
        let d = domain.clone();
        thread::spawn(move || mx_service::resolve_mx(&d))
    };

    let panel_handle = {
        let d = domain.clone();
        thread::spawn(move || panel_service::detect_panel(&d))
    };

    let spf_handle = {
        let d = domain.clone();
        thread::spawn(move || spf_service::resolve_spf(&d))
    };

    let dkim_handle = {
        let d = domain.clone();
        thread::spawn(move || dkim_service::resolve_dkim(&d))
    };

    let dmarc_handle = {
        let d = domain.clone();
        thread::spawn(move || dmarc_service::resolve_dmarc(&d))
    };

    let whois_result = {
        let d = domain.clone();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let _ = tx.send(
                whois_service::resolve_whois(&d)
            );
        });

        rx.recv_timeout(Duration::from_secs(3))
            .unwrap_or_else(|_| whois_timeout())
    };

    DnsQuery {
        domain,
        hosts: hosts_handle.join().unwrap(),
        ns: ns_handle.join().unwrap(),
        mx: mx_handle.join().unwrap(),
        panel: panel_handle.join().unwrap(),
        spf: spf_handle.join().unwrap(),
        dkim: dkim_handle.join().unwrap(),
        dmarc: dmarc_handle.join().unwrap(),
        whois: whois_result,
    }
}