use std::process::Command;

pub fn query_txt(host: &str) -> String {
    let output = Command::new("dig")
        .args(["+short", "TXT", host])
        .output();
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => String::new(),
    }
}
