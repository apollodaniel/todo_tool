use std::error::Error;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;

use crate::todo::{self, Todo};


pub enum AppState<'a>{
    NewTodo(TextArea<'a>),
    TodoList
}

pub struct App{
    pub todo_list: Vec<Todo>,
    pub todo_list_state: ListState,
    pub should_quit: bool
}

impl App {
    pub fn new()->Result<Self, Box<(dyn Error)>>{
        Ok(Self { todo_list: todo::list()?, todo_list_state: ListState::default().with_selected(Some(0)), should_quit: false})
    }

    pub fn update_list(&mut self) -> Result<(), Box<dyn Error>>{
        self.todo_list = todo::list()?;        
        Ok(())
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }
}
