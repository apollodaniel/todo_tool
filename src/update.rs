use std::error::Error;

use ratatui::widgets::{Block, Borders, Padding};
use tui_textarea::{Input, Key, TextArea};

use crate::{app::{self, App, AppState}, todo};



pub fn update_list_state(input: &Input,app: &mut App, app_state: &mut AppState)->Result<(), Box<dyn Error>>{
    match input {
        Input{key: Key::Enter, ..}=>{
            let item = app.todo_list.get(app.todo_list_state.selected().unwrap()).unwrap();
            todo::execute_command(todo::TodoCommand::Toggle((item.id, item.marked)))?;
            app.update_list()?;
        },
        Input{key: Key::Up, ..}=>{
            let index = app.todo_list_state.selected().unwrap();
            if let Some(res) = index.checked_sub(1) {
                app.todo_list_state.select(Some(res));
            }else{
                app.todo_list_state.select(Some(app.todo_list.len()-1));
            }
        },
        Input{key: Key::Down, ..}=>{
            let index = app.todo_list_state.selected().unwrap();
            if index+1 < app.todo_list.len(){
                app.todo_list_state.select(Some(index+1));
            }else{
                app.todo_list_state.select(Some(0));
            }
        },
        
        Input { key: Key::Char('a'), ctrl: true, alt:false, shift:false } =>{
            // go to new todo app state

            let block = Block::new().title("New todo").borders(Borders::ALL).padding(Padding::horizontal(1));
            let mut text_area = TextArea::new(Vec::new());
            text_area.set_block(block);

            *app_state = AppState::NewTodo(text_area);
        }
        _=>{}
    }
    Ok(())
}

pub fn update_new_todo_state(input: &Input,app: &mut App, app_state: &mut AppState) -> Result<(), Box<(dyn Error)>>{
    match input {
        Input{key: Key::Enter, ..} | Input{ctrl: true, key: Key::Char('m'),alt:false,shift:false}=>{
            // finish
            if let AppState::NewTodo(text_area) = app_state {
                todo::execute_command(todo::TodoCommand::Add(text_area.lines().first().unwrap()))?;
                app.update_list()?;
                *app_state = AppState::TodoList;
            }
        },
        Input { key: Key::Char('d'), ctrl: true, alt:false, shift:false } | Input { key: Key::Esc, .. } =>{
            // go to new todo app state
            *app_state = AppState::TodoList;
        },
        e=>{
            if let AppState::NewTodo(text_area) = app_state {
                text_area.input(e.clone());
            }
        },
    }

    Ok(())
}

pub fn update(input: &Input, app: &mut App, app_state: &mut AppState)->Result<(), Box<dyn Error>>{

    // global keys
    match input {
        Input{key:Key::Char('x'), ctrl: true, ..}=>{
            app.quit();
        },
        _=>{}
    }

    match app_state {
        AppState::NewTodo(_) => update_new_todo_state(input, app, app_state)?,
        AppState::TodoList=>update_list_state(input, app, app_state)?
    }
    

    Ok(())

}