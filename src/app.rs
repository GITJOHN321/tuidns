use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    style::Color,
};

use crate::models::dns_model::DnsQuery;
use crate::orchestrators::dns_orchestrator::execute_query;
use crate::orchestrators::format_orchestrator::send_clipboard;
use crate::ui::cursor::TextCursor;
use crate::ui::matrix_table::render_matrix_table;
use crate::ui::ns_table::render_basic_table;

pub enum Action {
    Quit,
    Continue,
}

pub struct App {
    pub input: String,
    pub cursor: TextCursor,
    pub domain: DnsQuery,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    pub info: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: TextCursor::new(),
            domain: DnsQuery::default(),
            history: Vec::new(),
            history_index: None,
            info: "Ctrl + C".to_string(),
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let size = f.area();

        let root = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(size);

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(15),
                Constraint::Length(14),
            ])
            .split(root[0]);

        let input_display = self.cursor.render(&self.input);
        let input_widget = Paragraph::new(input_display)
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Input").borders(Borders::ALL));
        f.render_widget(input_widget, left[0]);

        render_matrix_table(f, left[1], &self.domain.hosts, &self.domain.domain);

        render_basic_table(
            f,
            left[2],
            "Email Records".to_string(),
            &format!(
                "- SPF: {}\n- DMARC: {}\n- DKIM: {}",
                self.domain.spf, self.domain.dmarc, self.domain.dkim
            ),
            Color::White,
        );

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(4),
                Constraint::Length(5),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(3),
            ])
            .split(root[1]);

        render_basic_table(
            f,
            right[0],
            "WHOIS".to_string(),
            &format!(
                "- Registrant: {}\n- Expire on: {}\n- Estados:\n{}",
                self.domain.whois.registrar,
                self.domain.whois.expire_date,
                self.domain.whois.statuses
            ),
            Color::LightBlue,
        );

        let ssl_text = match self.domain.hosts.first() {
            Some(host) => format!(
                "- Provider:\n{}\n- Expire on: {}",
                host.ssl.organization, host.ssl.date
            ),
            None => "No SSL information found".to_string(),
        };
        render_basic_table(f, right[1], "SSL Checker".to_string(), &ssl_text, Color::Cyan);

        render_basic_table(f, right[2], "NS Records".to_string(), &self.domain.ns, Color::Green);
        render_basic_table(f, right[3], "MX Records".to_string(), &self.domain.mx, Color::Yellow);

        let footer = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(right[4]);

        render_basic_table(
            f,
            footer[0],
            "Panel".to_string(),
            &format!("{}", self.domain.panel),
            Color::Blue,
        );
        render_basic_table(f, footer[1], "Copiar".to_string(), &self.info, Color::White);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> io::Result<Action> {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                send_clipboard(&self.domain);
                self.info = "Copiado!".to_string();
                Ok(Action::Continue)
            }

            KeyCode::Char(c) => {
                self.input.insert(self.cursor.pos, c);
                self.cursor.pos += 1;
                Ok(Action::Continue)
            }

            KeyCode::Backspace => {
                if self.cursor.pos > 0 {
                    self.input.remove(self.cursor.pos - 1);
                    self.cursor.pos -= 1;
                }
                Ok(Action::Continue)
            }

            KeyCode::Enter => {
                self.info = "Cntrl + C".to_string();
                if !self.input.is_empty() {
                    self.domain = execute_query(&self.input.trim());
                    if self.history.last() != Some(&self.input) {
                        self.history.push(self.input.clone());
                        self.history_index = None;
                    }
                }
                self.input.clear();
                self.cursor.pos = 0;
                Ok(Action::Continue)
            }

            KeyCode::Left => {
                if self.cursor.pos > 0 {
                    self.cursor.pos -= 1;
                }
                Ok(Action::Continue)
            }

            KeyCode::Right => {
                if self.cursor.pos < self.input.len() {
                    self.cursor.pos += 1;
                }
                Ok(Action::Continue)
            }

            KeyCode::Up => {
                if self.history.is_empty() {
                    return Ok(Action::Quit);
                }
                self.history_index = match self.history_index {
                    None => Some(self.history.len() - 1),
                    Some(0) => Some(0),
                    Some(i) => Some(i - 1),
                };
                if let Some(i) = self.history_index {
                    self.input = self.history[i].clone();
                    self.cursor.pos = self.input.len();
                }
                Ok(Action::Continue)
            }

            KeyCode::Down => {
                if !self.history.is_empty() {
                    match self.history_index {
                        Some(i) if i < self.history.len() - 1 => {
                            self.history_index = Some(i + 1);
                            self.input = self.history[i + 1].clone();
                            self.cursor.pos = self.input.len();
                        }
                        Some(_) => {
                            self.history_index = None;
                            self.input.clear();
                            self.cursor.pos = 0;
                        }
                        None => {}
                    }
                }
                Ok(Action::Continue)
            }

            KeyCode::Esc => Ok(Action::Quit),
            _ => Ok(Action::Continue),
        }
    }
}
