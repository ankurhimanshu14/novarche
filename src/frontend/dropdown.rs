pub mod dropdown {

    use std::io;
    use tui::backend::{ CrosstermBackend, Backend };
    use tui::layout::{Alignment, Constraint, Rect, Direction, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::text::{Span, Spans};
    use tui::widgets::{Block, Borders, BorderType, Paragraph, Tabs, Wrap, Table, Row, Cell, List, ListItem};
    use tui::Terminal;
    // use crate::backend::read_inputs::read_inputs::read_inputs;
    // use crate::apis::human_resources::employee::employee::Employee;

    pub fn dropdown(x: u16, y: u16) -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.draw(|f| {
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
            .split(Rect {
                x,
                y,
                width: 15,
                height: 50,
            });
            
            let drop = Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::White))
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::White).fg(Color::White));
            f.render_widget(drop, chunks[0]);
    });
    terminal.set_cursor(2, 41).unwrap();
    Ok(())
    }
}
