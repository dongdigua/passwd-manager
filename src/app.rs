use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use crate::ui::ui;

#[derive(Debug)]
pub struct App<'a> {
  pub data: &'a Vec<(String, String)>,
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // 处理按键事件
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        if 'q' == ch {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}