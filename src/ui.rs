use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget, List, ListState, ListItem},
    Frame, Terminal,
};
use crate::app::App;
use ansi_term::Colour;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .direction(Direction::Horizontal)
        .split(chunks[1]);
    
    let paragraph = Paragraph::new(Span::styled(
        "(I)nsert (Q)uit",
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("不安全的密码管理器"))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(paragraph, top_chunks[0]);


    let mut sites = vec![];
    for (i, _) in app.data {
        sites.push(ListItem::new(&**i));
    };
    let mut state = ListState::default();
    state.select(Some(app.index));
    let paragraph = List::new(sites)
        .block(Block::default().borders(Borders::ALL).title(format!("平台({})", app.data.len())))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
            )
        .highlight_symbol(">");
    f.render_stateful_widget(paragraph, bottom_chunks[0], &mut state);

    let passwd =
        if app.show {
            format!("[ {} ]\n[ {} ]", &app.data[app.index].0, &app.data[app.index].1)
        } else {
            "".to_string()
        };
    let paragraph = Paragraph::new(passwd)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("密码"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, bottom_chunks[1]);
}
