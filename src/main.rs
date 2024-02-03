use std::error::Error;

use crossterm::{event::{self, KeyEvent}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph}, Terminal};
use todo_rs_tui::todo;

fn main() -> Result<(), Box<(dyn Error)>>{
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    let mut items: Vec<todo::Todo> = todo::list().unwrap();
    

    let mut list_state = ListState::default().with_selected(Some(0));

    loop {
        terminal.draw(|f|{
            let area = f.size();
            let block = Block::default().title("Todo").borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded);


            let list = List::new(items.iter().map(|f|format!("{}", f)))
                .block(block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom);


            f.render_stateful_widget(list, area, &mut list_state);
        })?;
        
        if let event::Event::Key(e) = event::read()? {
            match e.code {
                event::KeyCode::Esc=>{
                    break;
                },
                event::KeyCode::Enter=>{
                    let item = items.get(list_state.selected().unwrap()).unwrap();
                    todo::execute_command(todo::TodoCommand::Toggle((item.id, item.marked)))?;
                    items = todo::list().unwrap();
                },
                event::KeyCode::Up=>{
                    let index = list_state.selected().unwrap();
                    if let Some(res) = index.checked_sub(1) {
                        list_state.select(Some(res));
                    }else{
                        list_state.select(Some(items.len()-1));
                    }
                },
                event::KeyCode::Down=>{
                    let index = list_state.selected().unwrap();
                    if index+1 < items.len(){
                        list_state.select(Some(index+1));
                    }else{
                        list_state.select(Some(0));
                    }
                },
                _=>{}
            }
        }

    }
    

    terminal::disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;


    Ok(())
}
