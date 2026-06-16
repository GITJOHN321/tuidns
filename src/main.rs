use std::io;
mod ui;
mod models;
mod services;
mod orchestrators;
mod infrastructure;


use crate::ui::matrix_table::render_matrix_table;
use crate::ui::ns_table::render_basic_table;
use crate::orchestrators::dns_orchestrator::execute_query;
use crate::orchestrators::format_orchestrator::send_clipboard;
use crate::models::dns_model::DnsQuery;

use crossterm::{
    event::{
        KeyCode,
        KeyModifiers,
        EnableMouseCapture,
        DisableMouseCapture,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Color},
    Terminal,
};

fn main() -> io::Result<()> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut input = String::new();
    let mut domain = DnsQuery::default();
    let mut history: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;
    let mut info="Ctrl + C";

    loop {
        terminal.draw(|f| {
            let size = f.area();

            // ==================================================
            // ROOT (70% izquierda, 30% derecha)
            // ==================================================

            let root = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .split(size);

            // ==================================================
            // IZQUIERDA
            // ==================================================

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), //input
                    Constraint::Min(15),    // Panel principal
                    Constraint::Length(14), // Panel secundario
                ])
                .split(root[0]);
            
            
            // --------------------------------------------------
            // PANEL INPUT
            // --------------------------------------------------

            // Cursor visual
            let input_display = format!("{}|", input);
            let input_widget = Paragraph::new(input_display)
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Input")
                        .borders(Borders::ALL),
                );

            f.render_widget(input_widget, left[0]);


            // --------------------------------------------------
            // PANEL PRINCIPAL con render_table
            // --------------------------------------------------


            let data = &domain.hosts;

            render_matrix_table(
                f,
                left[1],
                &data,
                &domain.domain,
            );
            // --------------------------------------------------
            // PANEL Email Info
            // --------------------------------------------------


            render_basic_table(
                f,
                left[2],
                "Email Records".to_string(),
                &format!("- SPF: {}\n- DMARC: {}\n- DKIM: {}",&domain.spf,&domain.dmarc,&domain.dkim),
                Color::White,
            );
            // ==================================================
            // DERECHA
            // ==================================================

            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(4), // Whois
                    Constraint::Length(5), // SSL checker
                    Constraint::Length(8), // NS Records
                    Constraint::Length(8),    // MX Records
                    Constraint::Length(3), // Panel
                ])
                .split(root[1]);



            // --------------------------------------------------
            // PANEL A
            // --------------------------------------------------

            render_basic_table(
                f,
                right[0],
                "WHOIS".to_string(),
                &format!("- Registrant: {}\n- Expire on: {}\n- Estados:\n{}",&domain.whois.registrar,&domain.whois.expire_date,&domain.whois.statuses),
                Color::LightBlue,
            );

            // --------------------------------------------------
            // PANEL MX
            // --------------------------------------------------

            let ssl_text = match domain.hosts.first() {
                Some(host) => format!(
                    "- Provider:\n{}\n- Expire on: {}",
                    host.ssl.organization,
                    host.ssl.date
                ),
                None => "No SSL information found".to_string(),
            };
            render_basic_table(
                f,
                right[1],
                "SSL Checker".to_string(),
                &ssl_text,
                Color::Cyan,
            );

            // --------------------------------------------------
            // PANEL NS
            // --------------------------------------------------

            render_basic_table(
                f,
                right[2],
                "NS Records".to_string(),
                &domain.ns,
                Color::Green,
            );

            // --------------------------------------------------
            // PANEL MX
            // --------------------------------------------------

            render_basic_table(
                f,
                right[3],
                "MX Records".to_string(),
                &domain.mx,
                Color::Yellow,
            );

            // --------------------------------------------------
            // FOOTER
            // --------------------------------------------------

            let footer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50), //panel
                    Constraint::Percentage(50),    // info
                ])
                .split(right[4]);

            render_basic_table(
                f,
                footer[0],
                "Panel".to_string(),
                &format!("{}",&domain.panel),
                Color::Blue,
            );
            render_basic_table(
                f,
                footer[1],
                "Copiar".to_string(),
                &info,
                Color::White,
            );
        })?;

        if crossterm::event::poll(
            std::time::Duration::from_millis(50),
        )? {
            if let crossterm::event::Event::Key(key) =
                crossterm::event::read()?
            {
                match key.code {

                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        send_clipboard(&domain);
                        info = "Copiado!";
                    }

                    KeyCode::Char(c) => {
                        input.push(c);
                    }

                    KeyCode::Backspace => {
                        input.pop();
                    }

                    KeyCode::Enter => {
                        info = "Cntrl + C";
                        if !input.is_empty() {
                            domain = execute_query(&input.trim());
                            if history.last() != Some(&input) {
                                history.push(input.clone());
                                history_index = None;
                            }                        
                        }
                        
                        input.clear();
                        
                    }
                    KeyCode::Up => {
                        if history.is_empty() {
                            return Ok(());
                        }

                        history_index = match history_index {
                            None => Some(history.len() - 1),
                            Some(0) => Some(0),
                            Some(i) => Some(i - 1),
                        };

                        if let Some(i) = history_index {
                            input = history[i].clone();
                        }
                    }
                    KeyCode::Down => {
                        if history.is_empty() {
                            return Ok(());
                        }

                        match history_index {
                            Some(i) if i < history.len() - 1 => {
                                history_index = Some(i + 1);
                                input = history[i + 1].clone();
                            }
                            Some(_) => {
                                history_index = None;
                                input.clear();
                            }
                            None => {}
                        }
                    }

                    KeyCode::Esc => {
                        break;
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
