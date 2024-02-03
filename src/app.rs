use std::error::Error;

use ratatui::widgets::ListState;

use crate::todo::{self, Todo};


pub enum AppState{
    NewTodo,
    TodoList
}

pub struct App{
    pub appstate: AppState,
    pub todo_list: Vec<Todo>,
    pub todo_list_state: ListState
}

impl App {
    pub fn new()->Result<Self, Box<(dyn Error)>>{
        Ok(Self { appstate: AppState::TodoList, todo_list: todo::list()?, todo_list_state: ListState::default().with_selected(Some(0)) })
    }

    pub fn update_list(&mut self) -> Result<(), Box<dyn Error>>{
        self.todo_list = todo::list()?;        
        Ok(())
    }
}