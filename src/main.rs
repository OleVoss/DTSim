mod app;
mod controller;
mod keys;
mod models;
mod ui;

use anyhow::{bail, Result};
use app::App;
use crossbeam_channel::{tick, Receiver, Select};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::{
    borrow::Borrow,
    io::{self, Write},
    time::{Duration, Instant},
};

use simplelog::{Config, LevelFilter, TermLogger};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::UI;

fn main() -> Result<()> {
    setup_logging()?;
    setup_terminal()?;

    let mut terminal = start_terminal(io::stdout())?;
    let mut app = App::new();
    let mut ui = UI::new();

    let mut first_update = true;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(ev) => {
                    controller::key_event(&mut app, ev, &mut ui)?;
                }
                Event::Mouse(ev) => {}
                Event::Resize(width, height) => {}
            }
        }
        ui.player_tab.strength_slider.set_value(40.0)?;
        draw(&mut terminal, &app, &ui)?;
        if app.should_quit {
            shutdown_terminal()?;
            break;
        }
    }

    Ok(())
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App, ui: &UI) -> Result<()> {
    terminal.draw(|mut f| {
        if let Err(e) = ui.draw(&mut f, app) {
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

fn start_terminal<W: Write>(buf: W) -> io::Result<Terminal<CrosstermBackend<W>>> {
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
