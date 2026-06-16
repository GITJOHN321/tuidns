use std::process::Command;

#[derive(Default)]
pub struct IpInfo {
    pub ip: String,
    pub ptr: String,
}

use crate::infrastructure::resolve_ptr::resolve_ptr;

pub fn resolve_ip(hostname: &str) -> IpInfo {
    let output = Command::new("dig")
        .arg("+short")
        .arg(&hostname)
        .output();

    let mut ip_list = String::new();
    let mut ptr_list = String::new();
    

    match output {
        Ok(output) => {

            let stdout =
                String::from_utf8_lossy(&output.stdout);
            if stdout.is_empty(){
                    return IpInfo{
                                ip: "No Resuelve".to_string(),
                                ptr: "----".to_string()
                            }
                    
            }

            for line in stdout.lines() {

                if line.parse::<std::net::IpAddr>().is_ok() {
                    ip_list.push_str(line);
                    ip_list.push('\n');

                    let ptr = resolve_ptr(line);
                    ptr_list.push_str(&ptr);
                    ptr_list.push('\n');
                }

                
            }
        }
        Err(e) => ip_list = format!("{}",e),
    }
    IpInfo{
        ip: ip_list,
        ptr: ptr_list
    }
}