use std::process::Command;

pub fn resolve_mx(domain: &str) -> String {

    let output = Command::new("dig")
        .arg("+short")
        .arg("MX")
        .arg(domain)
        .output();

    match output {

        Ok(output) => {

            let stdout =
                String::from_utf8_lossy(&output.stdout);

            let mut result = String::new();

            for line in stdout.lines() {

                let ns = line.trim();

                if !ns.is_empty() {

                    result.push_str(ns);
                    result.push('\n');
                }
            }

            if result.is_empty() {

                "No encontrado".to_string()

            } else {

                result.trim_end().to_string()
            }
        }

        Err(error) => {
            format!("Error: {}", error)
        }
    }
}
