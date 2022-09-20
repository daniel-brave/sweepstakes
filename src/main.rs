use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

mod minesweeper;
mod random;

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Mine Sweeper")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::Yellow)),
            Span::from(" background"),
        ])
        .style(Style::default().bg(Color::Green));
    f.render_widget(block, top_chunks[0]);

    // Top right inner block with styled title aligned to the right
    let block = Block::default()
        .title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Right);
    f.render_widget(block, top_chunks[1]);

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default borders
    let block = Block::default().title("With borders").borders(Borders::ALL);
    f.render_widget(block, bottom_chunks[0]);

    // Bottom right block with styled left and right border
    let block = Block::default()
        .title("With styled borders and doubled borders")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double);
    f.render_widget(block, bottom_chunks[1]);
}

pub fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test_neighbors() {
        let pos1 = (2, 2);
        let pos2 = (3, 0);
        let pos3 = (2, 3);
        let mut ms = Minesweeper::new(10, 10, 5);
        ms.open(pos1);
        ms.open(pos2);
        ms.toggle_flag(pos3);
        ms.toggle_flag(pos3);
        println!("{}", ms);
    }
}
