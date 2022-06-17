use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};
use std::io;
use passwd_manager::app;
use passwd_manager::app::App;
use passwd_manager::app::Cursor;


fn main() -> Result<(), io::Error> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data = &vec![
        (String::from("bilibili"), String::from("123456")),
        (String::from("GitHub"), String::from("password"))
    ];
    let app = App {
        data,
        index: 0,
        show: false,
        insert_mode: false,
        cursor: Cursor::Site(0),
        buffer: &mut vec![],
    };
    // 渲染界面
    app::run_app(&mut terminal, app)?;
    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
