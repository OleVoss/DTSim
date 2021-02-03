mod app;
mod ui;
mod keys;
mod controller;
mod models;

use std::{io::{self, Write}, time::{Duration}};
use app::App;
use anyhow::{Result, bail};
use crossbeam_channel::{Receiver, Select, tick};
use crossterm::{ExecutableCommand, event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read}, terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode}};

use simplelog::{Config, LevelFilter, TermLogger};
use tui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use ui::UI;

fn main() -> Result<()> {

    setup_logging()?;
    setup_terminal()?;
    
    let mut terminal = start_terminal(io::stdout())?;
    let mut app = App::new();
    let mut ui = UI::new();       

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
<<<<<<< HEAD
        ui.player_tab.strength_slider.set_value(40.0)?;
=======
        ui.player_tab.strength_slider.set_value(40)?;
>>>>>>> dfcdc2b420d86f3f54ab006f8fe891f0cf040153
        draw(&mut terminal, &app, &ui)?;
        if app.should_quit {
            shutdown_terminal()?;
            break;
        }
    
    }

    Ok(())
}

fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &App,
    ui: &UI
) -> Result<()> {
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
