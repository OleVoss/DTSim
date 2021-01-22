mod app;
mod ui;
mod components;
mod tabs;

use std::{io::{self, Write}};
use app::App;
use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    execute,
    event::{
        read, Event, KeyCode, KeyEvent, KeyModifiers
    },
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode}};
use simplelog::{Config, LevelFilter, TermLogger};
use tui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};

fn main() -> Result<()> {

    setup_logging()?;

    setup_terminal()?;
    let mut terminal = start_terminal(io::stdout())?;
    let mut app = App::new("DTSim");
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                shutdown_terminal()?;
                break;
            },
            _ => (),
        }
        draw(&mut terminal, &mut app)?;
    }

    Ok(())
}

fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &App,
) -> Result<()> {
    terminal.draw(|mut f| {
        if let Err(e) = app.draw(&mut f) {
            log::error!("failed drawing");
        }
    })?;

    Ok(())
}

fn setup_terminal() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn start_terminal<W: Write>(
    buf: W
) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

fn shutdown_terminal() -> Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn setup_logging() -> Result<()> {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        simplelog::TerminalMode::Mixed,
    );

    Ok(())
}