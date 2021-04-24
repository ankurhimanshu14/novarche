pub mod home {
    use std::io;
    use tui::Terminal;
    use tui::backend::CrosstermBackend;
    use tui::widgets::{Block, Borders, Tabs, Paragraph, Wrap };
    use tui::text::{ Spans, Span };
    use tui::style::{ Style, Color, Modifier };
    use tui::layout::{Layout, Constraint, Direction, Alignment};

    pub fn home() -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        terminal.draw(|f| {
            let chunks = Layout::default()
                                        .direction(Direction::Vertical)
                                        .constraints([
                                            Constraint::Percentage(100)
                                        ].as_ref())
                                        .split(f.size());
            let block = Block::default()
                                        .borders(Borders::ALL)
                                        .style(Style::default().bg(Color::Gray).fg(Color::Black));
            f.render_widget(block, chunks[0]);

            let chunks = Layout::default()
                                        .direction(Direction::Vertical)
                                        .constraints([
                                            Constraint::Percentage(15),
                                            Constraint::Percentage(85)
                                        ].as_ref())
                                        .split(f.size());
            let titles = ["Administration", "Engineering", "Production", "Quality Assurance", "Raw Material Store", "General Store", "Purchase", "Dispatch", "Gate Entry", "Human Resources", "About", "Help"].iter().cloned().map(Spans::from).collect();
            let tabs = Tabs::new(titles)
                            .block(Block::default().title("Novarche").borders(Borders::ALL))
                            .highlight_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))    
                            .divider("|");
            f.render_widget(tabs, chunks[0]);

            let chunks = Layout::default()
                                        .direction(Direction::Horizontal)
                                        .vertical_margin(3)
                                        .constraints([
                                            Constraint::Percentage(10),
                                            Constraint::Percentage(90)
                                        ].as_ref())
                                        .split(f.size());
            let block = Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().bg(Color::Gray).fg(Color::Black));
            f.render_widget(block, chunks[0]);

            let mut t = String::new();
            std::io::stdin().read_line(&mut t).unwrap();
            let text = vec![
                            Spans::from(vec![
                                Span::raw(t),
                                Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
                                Span::raw("."),
                            ]),
                            Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
                        ];
            let para = Paragraph::new(text)
                            .block(Block::default().title("Paragraph").borders(Borders::ALL))
                            .style(Style::default().bg(Color::Gray).fg(Color::Black))
                            .alignment(Alignment::Left)
                            .wrap(Wrap { trim: true });
            f.render_widget(para, chunks[1]);
        })
    }
}