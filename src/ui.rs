use std::{error::Error, io::Stdout};

use ratatui::{backend::CrosstermBackend, layout::{Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, widgets::{Block, Borders, List, ListDirection, Paragraph, Wrap}, Terminal};
use tui_textarea::TextArea;

use crate::{app::{App, AppState}, tui::CrosstermTerminal};



pub fn draw_new_todo(terminal: &mut CrosstermTerminal, text_area: &mut TextArea)-> Result<(), Box<(dyn Error)>>{

    terminal.draw(|f|{
        let area = f.size();
        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        ).split(area);

        
        f.render_widget(text_area.widget(), layout[1]);
        f.render_widget(Paragraph::new("Ctrl+x - Exit | Esc - Cancel | Enter - Save").alignment(ratatui::layout::Alignment::Center).bold().wrap(Wrap{trim:true}), layout[3]);
    })?;

    Ok(())
}

pub fn draw_todo_list(terminal: &mut CrosstermTerminal, app: &mut App)-> Result<(), Box<(dyn Error)>>{
    terminal.draw(|f|{
        let area = f.size();

        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Length(1),
            ]
        ).split(area);

        let block = Block::default().title("Todo").borders(Borders::ALL);


        let list = List::new(app.todo_list.iter().map(|f|format!("{}", f)))
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);


        f.render_stateful_widget(list, layout[0], &mut app.todo_list_state);
        f.render_widget(Paragraph::new("Ctrl+x - Exit | Ctrl+d - Delete selection | Ctrl+a - add | Enter - Mark/Unmark").alignment(ratatui::layout::Alignment::Center).bold().wrap(Wrap{trim:true}), layout[1]);
    })?;
    Ok(())
}

pub fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App, app_state: &mut AppState)->Result<(),Box<(dyn Error)>>{
    match app_state{
        crate::app::AppState::NewTodo(e) => draw_new_todo(terminal, e)?,
        crate::app::AppState::TodoList => draw_todo_list(terminal, app)?
    }
    Ok(())
}