use std::{error::Error, io::Stdout, panic};

use crossterm::{execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::events::EventHandler;

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;



pub struct Tui{
    pub terminal: CrosstermTerminal,
    pub events: EventHandler
}

impl Tui {
    
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self{
        Self { terminal: terminal, events: events }
    }

    pub fn enter()->Result<(), Box<(dyn Error)>>{
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen)?;
        
        // reset console if panic
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move|panic|{
            Self::reset().expect("unable to reset console or leave alternate mode");
            panic_hook(panic);
        }));
        
        Ok(())
    }

    pub fn reset()-> Result<(), Box<(dyn Error)>>{
        terminal::disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen)?;

        Ok(())
    }

}