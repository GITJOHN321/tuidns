use crate::domain::dns_model::Host;
use ratatui::{
    prelude::*,
    style::{Color,Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
};

pub fn render_matrix_table(
    f: &mut Frame,
    area: Rect,
    data: &[Host],
    domain:&str
) {
    let headers = [
        "Host",
        "IP",
        "PTR",
        "Ping",
        "SSL",
    ];
    let header = Row::new(
        headers
            .iter()
            .map(|h| Cell::from(*h))
            .collect::<Vec<Cell>>(),
    ).bold();

    

    let rows = data.iter().map(|row| {
        let height = row.ip.lines().count() as u16;
        Row::new(vec![
            Cell::from(row.name.clone())
                .style(Style::default().fg(Color::Yellow)),

            Cell::from(row.ip.clone())
                .style(Style::default().fg(Color::Green)),

            Cell::from(row.ptr.clone())
                .style(Style::default().fg(Color::Blue)),

            Cell::from(row.ping.clone())
                .style(Style::default().fg(Color::Magenta)),

            Cell::from(row.ssl.active.clone())
                .style(Style::default().fg(Color::Cyan)),
        ])
        .height(height.max(1))
    });

    let widths = [
        Constraint::Length(8),
        Constraint::Length(15),
        Constraint::Min(20),
        Constraint::Length(12),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(
                    Span::styled(
                        format!("  Dominio Principal: {}. ", domain),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    )
                )
                .borders(Borders::ALL),
        )
        .column_spacing(1);

    f.render_widget(table, area);
}