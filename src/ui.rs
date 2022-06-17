use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};
use crate::app::App;
use ansi_term::Colour;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    //
    let chunks = Layout::default() // 首先获取默认构造
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
        .direction(Direction::Vertical) // 垂直分割
        .split(f.size()); // 分割整块 Terminal 区域
        
    let top_chunks = Layout::default()
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
        .direction(Direction::Vertical)
        .split(chunks[0]);
    
    let bottom_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(chunks[1]);
    
    let paragraph = Paragraph::new(Span::styled(
        "(I)nsert (Q)uit",
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("沙雕密码管理器"))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(paragraph, top_chunks[0]);

    let mut site = String::from("");
    for i in 0..=app.data.len() - 1 {
        let current = &app.data[i].0;
        if i == app.index {
            site.push_str(&format!("[{}]", current))
        } else {
            site.push_str(current);
        }
        site.push_str("\n")
    };
    let paragraph = Paragraph::new(site)
        //.style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("平台"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, bottom_chunks[0]);

    let paragraph = Paragraph::new("当我在扯淡")
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("密码"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, bottom_chunks[1]);
}