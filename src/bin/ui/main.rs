mod app;
mod events;
mod rendering;
mod ai_controller;

pub use app::App;

use std::io;
use ratatui::DefaultTerminal;
use ratatui::crossterm::{ExecutableCommand, event::{DisableMouseCapture, EnableMouseCapture}};
use std::io::stdout;

pub fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = run(&mut terminal);
    ratatui::restore();
    app_result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::default();
    stdout().execute(EnableMouseCapture).unwrap();
    
    while !app.exit {
        terminal.draw(|frame| app.draw(frame))?;
        app.handle_events()?;
    }
    
    stdout().execute(DisableMouseCapture).unwrap();
    Ok(())
}
