pub mod home {

    use std::io;
    use tui::backend::{ CrosstermBackend, Backend };
    use tui::layout::{Alignment, Constraint, Rect, Direction, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::text::{Span, Spans};
    use tui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};
    use tui::Terminal;
    use crate::backend::read_buffer::read_buffer::read_buffer;

    pub fn home() -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        loop {
            terminal.draw( |f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .vertical_margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(5),
                            Constraint::Percentage(90),
                            Constraint::Percentage(5)
                        ].as_ref()
                    )
                    .split(f.size());
                
                let titles = ["Administration", "Human Resources", "Accounts"].iter().cloned().map(Spans::from).collect();
                let tabs = Tabs::new(titles)
                    .block(Block::default().title("Novarche Inc.").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::White).bg(Color::Magenta).add_modifier(Modifier::UNDERLINED))
                    .divider("|");
                f.render_widget(tabs, chunks[0]);
    
                let read = vec![
                    Spans::from(vec![
                        Span::raw("Enter First Name: "),
                        Span::from(read_buffer().unwrap())
                    ])
                ];
    
                let inputs = Paragraph::new(read)
                    .block(Block::default().title("Data").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });
                f.render_widget(inputs, chunks[1]);
    
                let blocks = Block::default()
                    .title("Input Field")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White));
                f.render_widget(blocks, chunks[2]);
            })?;
            terminal.set_cursor(2, 61)?
        }
    }
}
