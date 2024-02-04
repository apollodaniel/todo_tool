use std::{borrow::BorrowMut, error::Error};

use app::{App, AppState};
use crossterm::{event::{self, KeyEvent, KeyModifiers}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use events::EventHandler;
use ratatui::{backend::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph}, Terminal};
use tui::Tui;
use tui_textarea::{Input, Key};
use ui::draw;
use update::update;

pub mod todo;
pub mod app;
pub mod ui;
pub mod tui;
pub mod events;
pub mod update;

fn main() -> Result<(), Box<(dyn Error)>>{
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let mut app = App::new()?;
    let mut app_state = AppState::TodoList;
    let mut tui = Tui::new(Terminal::new(CrosstermBackend::new(std::io::stdout()))?, EventHandler::new(16));
    
    while !app.should_quit {
        
        match tui.events.next()? {
            events::Event::Key(e)=>{
                update(&e, &mut app, &mut app_state)?;
            },
            events::Event::Tick=>{
                draw(&mut tui.terminal, &mut app, &mut app_state)?; 
            }
        }
    }
    

    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;


    Ok(())
}
