use crate::domain::dns_model::Panel;

pub fn get_panels() -> Vec<Panel> {
    vec![
        Panel {
            name: "cPanel",
            ports: &[2082, 2083, 2086, 2087],
        },
        Panel {
            name: "Plesk",
            ports: &[8443],
        },
        Panel {
            name: "DirectAdmin",
            ports: &[2222],
        },
    ]
}