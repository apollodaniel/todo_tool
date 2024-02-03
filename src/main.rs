use std::error::Error;

use app::{App, AppState};
use crossterm::{event::{self, KeyEvent, KeyModifiers}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use events::EventHandler;
use ratatui::{backend::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph}, Terminal};
use tui::Tui;
use ui::draw;

pub mod todo;
pub mod app;
pub mod ui;
pub mod tui;
pub mod events;

fn main() -> Result<(), Box<(dyn Error)>>{
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let mut app = App::new()?;
    let mut tui = Tui::new(Terminal::new(CrosstermBackend::new(std::io::stdout()))?, EventHandler::new(16));
    
    loop {
        
        match tui.events.next()? {
            tui::Event::Key(e)=>{

                match e.code {
                    event::KeyCode::Esc=>{
                        break;
                    },
                    event::KeyCode::Enter=>{
                        let item = app.todo_list.get(app.todo_list_state.selected().unwrap()).unwrap();
                        todo::execute_command(todo::TodoCommand::Toggle((item.id, item.marked)))?;
                        app.update_list()?;
                    },
                    event::KeyCode::Up=>{
                        let index = app.todo_list_state.selected().unwrap();
                        if let Some(res) = index.checked_sub(1) {
                            app.todo_list_state.select(Some(res));
                        }else{
                            app.todo_list_state.select(Some(app.todo_list.len()-1));
                        }
                    },
                    event::KeyCode::Down=>{
                        let index = app.todo_list_state.selected().unwrap();
                        if index+1 < app.todo_list.len(){
                            app.todo_list_state.select(Some(index+1));
                        }else{
                            app.todo_list_state.select(Some(0));
                        }
                    },
                    event::KeyCode::Char('a') =>{
                        if e.modifiers.contains(KeyModifiers::CONTROL){
                            // creates new todo
                        }
                    }
                    _=>{}
                }

            },
            tui::Event::Tick=>{
                draw(&mut tui.terminal, &mut app)?;
            }
        }
    }
    

    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;


    Ok(())
}
