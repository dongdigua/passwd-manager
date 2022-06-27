use aes::cipher::Key;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use crate::ui::{ui, pre_ui};

#[derive(Debug, PartialEq)]
pub enum Cursor {
    Site(i32),
    Passwd(i32)
}

#[derive(Debug)]
pub struct App {
    pub data: Vec<(String, String)>,
    pub index: usize,
    pub show: bool,
    pub insert_mode: bool,
    pub cursor: Cursor,
    pub buffer: String,
}

impl App {
    pub fn new(data: Vec<(String, String)>) -> App {
    App {
        data,
        index: 0,
        show: false,
        insert_mode: false,
        cursor: Cursor::Site(0),
        buffer: String::from(""),
    }
    }
}
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // 处理按键事件
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        if ! app.insert_mode {
                            if 'q' == ch {
                                break;
                            } else if 'i' == ch {
                                app.insert_mode = true;
                            }
                        } else {
                            app.buffer.push(ch);
                        }
                    },

                    KeyCode::Up => {
                        if app.insert_mode {
                            match app.cursor {
                                Cursor::Passwd(_) => app.cursor = Cursor::Site(0),
                                _ => (),
                            };
                            app.buffer = "".to_string();
                        } else if app.index > 0 {
                            app.show = false;
                            app.index = app.index - 1;
                        }
                    },
                    KeyCode::Down => {
                        if app.insert_mode {
                            match app.cursor {
                                Cursor::Site(_) => app.cursor = Cursor::Passwd(0),
                                _ => (),
                            };
                            app.buffer = "".to_string();
                        } else if app.index <= app.data.len() - 2 {
                            app.show = false;
                            app.index = app.index + 1;
                        }
                    },

                    KeyCode::Backspace => {
                        if app.insert_mode {
                            app.buffer.pop();
                        }
                    },
                    KeyCode::Enter => {
                        if app.insert_mode {
                            match app.cursor {
                                // might have bug
                                Cursor::Site(_) => app.data[app.index].0 = app.buffer.to_string(),
                                Cursor::Passwd(_) => app.data[app.index].1 = app.buffer.to_string(),
                            }
                            app.insert_mode = false;
                        } else {
                            app.show = true;
                        }
                    },
                    KeyCode::Esc => {
                        if app.insert_mode {
                            app.insert_mode = false;
                        } else if app.show {
                            app.show = false;
                        }
                    },
                    _ => (),
                }
            }
        }
        // 处理其他逻辑
    }
    Ok(())
}

pub fn run_pre_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<String> {
    let mut buffer = String::from("");
    let mut insert_mode = false;

    loop {
        terminal.draw(|f| pre_ui(f, &mut buffer))?;
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(ch) => {
                        if insert_mode {
                            buffer.push(ch);
                        } else {
                            if 'q' == ch {
                                break;
                            } else if 'i' == ch {
                                insert_mode = true;
                            }
                        }
                    }
                    KeyCode::Esc => insert_mode = false,
                    KeyCode::Backspace => {
                        if insert_mode {buffer.pop();}
                    },
                    KeyCode::Enter => break,
                    _ => (),
                }
            }
        }
    }
    Ok(buffer)
}