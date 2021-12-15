mod app;
mod command;
mod ui;
mod widgets;

use crate::app::{App, Mode};
use crate::command::execute_command;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quake_core::QuakeConfig;
use std::error::Error;
use std::fs;
use std::io;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::draw;

pub fn tui_main_loop() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config: QuakeConfig = serde_yaml::from_str(fs::read_to_string(".quake.yaml")?.as_str())?;
    let app = App::new(config);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    // TODO: refactor
    while app.running() {
        terminal.draw(|f| {
            draw(f, &mut app);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.state.mode {
                Mode::Normal => {
                    if let KeyCode::Char(':') = key.code {
                        app.state.mode = Mode::Command;
                    }
                }
                Mode::Command => match key.code {
                    KeyCode::Enter => {
                        let command: String = app.cmd_line.message.drain(..).collect();
                        execute_command(&command, &mut app)?;
                        app.back_to_normal();
                    }
                    KeyCode::Char(c) => {
                        app.message_push(c);
                    }
                    KeyCode::Backspace => {
                        app.message_pop();
                    }
                    KeyCode::Esc => {
                        app.back_to_normal();
                    }
                    _ => {}
                },
                Mode::Insert => match key.code {
                    KeyCode::Esc => {
                        app.back_to_normal();
                    }
                    KeyCode::Char(c) => {
                        app.main_widget.collect_input(c);
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
