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


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    //
    let chunks = Layout::default() // 首先获取默认构造
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
        .direction(Direction::Vertical) // 垂直分割
        .split(f.size()); // 分割整块 Terminal 区域
        
    let paragraph = Paragraph::new(Span::styled(
        "(L)ist (Q)uit",
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("沙雕密码管理器"))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new("当我在扯淡")
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("内容"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[1]);
}