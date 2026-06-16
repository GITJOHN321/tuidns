use crate::infrastructure::resolve_port::resolve_port;
use crate::domain::panel_catalog::get_panels;

pub fn detect_panel(domain: &str) -> String {

    for panel in get_panels() {

        for port in panel.ports{
            if resolve_port(&domain, *port) {
                return panel.name.to_string();
            }
        }

    }

    "Unknown".to_string()
}

