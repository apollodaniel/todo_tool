use std::{error::Error, io::Stdout};

use ratatui::{backend::CrosstermBackend, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection}, Terminal};

use crate::{app::App, tui::CrosstermTerminal};



pub fn draw_todo_list(terminal: &mut CrosstermTerminal, app: &mut App)-> Result<(), Box<(dyn Error)>>{
    terminal.draw(|f|{
        let area = f.size();
        let block = Block::default().title("Todo").borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded);


        let list = List::new(app.todo_list.iter().map(|f|format!("{}", f)))
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);


        f.render_stateful_widget(list, area, &mut app.todo_list_state);
    })?;
    Ok(())
}

pub fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App)->Result<(),Box<(dyn Error)>>{
    match app.appstate{
        crate::app::AppState::NewTodo => {},
        crate::app::AppState::TodoList => draw_todo_list(terminal, app)?
    }
    Ok(())
}