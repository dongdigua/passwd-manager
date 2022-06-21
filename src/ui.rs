use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Widget, List, ListState, ListItem},
    Frame, Terminal,
};
use crate::app::{App, Cursor};
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
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .direction(Direction::Horizontal)
        .split(chunks[1]);

    let bottom_right_chunks = Layout::default()
        .constraints([Constraint::Length(5), Constraint::Min(5)].as_ref())
        .direction(Direction::Vertical)
        .split(bottom_chunks[1]);
    
    let paragraph = Paragraph::new(Span::styled(
        "(I)nsert (Q)uit",
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title("不安全密码管理器"))
    .alignment(tui::layout::Alignment::Left);
    f.render_widget(paragraph, top_chunks[0]);


    let mut sites = vec![];
    for (i, _) in &app.data {
        sites.push(ListItem::new(String::from(i)));
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

    let mut site_buffer = "".to_string();
    let mut passwd_buffer = "".to_string();

    if app.insert_mode {
        match app.cursor {
            Cursor::Site(_) => {
                site_buffer = String::from("") + &app.buffer + "|";
                passwd_buffer = (&app.data[app.index].1).to_string();
            }
            Cursor::Passwd(_) => {
                passwd_buffer = String::from("") + &app.buffer + "|";
                site_buffer = (&app.data[app.index].0).to_string()
            }
        }
    } else {
        site_buffer = (&app.data[app.index].0).to_string();
        if app.show {
            passwd_buffer = (&app.data[app.index].1).to_string();
        }
    }
    let paragraph = Paragraph::new(site_buffer)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("名称"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, bottom_right_chunks[0]);

    let paragraph = Paragraph::new(passwd_buffer)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().borders(Borders::ALL).title("密码"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, bottom_right_chunks[1]);
}

pub fn pre_ui<B: Backend>(f: &mut Frame<B>, buffer: &String) {
    let chunks = Layout::default() // 首先获取默认构造
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref()) // 按照 3 行 和 最小 3 行的规则分割区域
        .direction(Direction::Vertical) // 垂直分割
        .split(centered_rect(50, 40, f.size())); // 分割整块 Terminal 区域

    let mut passwd = String::from("");
    for _i in 0..buffer.len() {
        passwd.push('*')
    }
    let block = Paragraph::new(passwd)
        .block(Block::default().title("enter the main password").borders(Borders::ALL))
        .alignment(Alignment::Left);

    f.render_widget(block, chunks[0]);

}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Min(36),
                Constraint::Percentage((100 - percent_x) / 2)
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
