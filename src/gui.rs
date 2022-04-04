use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use crate::prelude::*;

pub trait GuiTableModel: Model {
    fn header() -> Vec<String>;
    fn row(&self) -> Vec<String>;
}

pub struct DataManagmentGui<M: GuiTableModel, S: DataStore<M>> {
    state: TableState,
    store: S,
    repo: Repo<M>,
}

impl <M: GuiTableModel, S: DataStore<M>> DataManagmentGui<M, S> {
    pub fn new(store: S) -> Self {
        let repo = store.clone().into_repo();

        Self {
            state: Default::default(),
            store,
            repo,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|frame| self.ui(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => { break; }
                    _ => {}
                }
            }
        }

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(frame.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);

        let header_cells = M::header()
            .into_iter()
            .map(|h| Cell::from(h).style(Style::default().fg(Color::Red)));

        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

        let rows = self.repo.items().map(|item| {
            let cells = item.row().into_iter().map(|c| Cell::from(c));
            Row::new(cells).height(1)
        });

        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Inventory"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Length(25),
                Constraint::Length(40),
                Constraint::Min(10),
                Constraint::Min(10),
            ]);

        frame.render_stateful_widget(t, rects[0], &mut self.state);
    }
}
