use std::process::Command;

pub fn query_whois(domain: &str, server: &str) -> String {
    let output = Command::new("whois")
        .args(["-h", server, domain])
        .output();
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => String::new(),
    }
}
