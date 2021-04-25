pub mod home {

    use std::io;
    use tui::backend::{ CrosstermBackend, Backend };
    use tui::layout::{Alignment, Constraint, Rect, Direction, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::text::{Span, Spans};
    use tui::widgets::{Block, Borders, Paragraph, Tabs, Wrap, Table, Row, Cell};
    use tui::Terminal;
    use crate::backend::read_inputs::read_inputs::read_inputs;
    use crate::apis::human_resources::employee::employee::Employee;

    pub fn home() -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let data = Employee::get().unwrap();

        let mut v_outer: Vec<Row> = Vec::new();

        for row in data {
            let mut v_inner: Vec<Cell> = Vec::new();
            for cell in row {
                v_inner.push(Cell::from(cell));
            }
            v_outer.push(Row::new(v_inner));
        }

        loop {
            terminal.draw( |f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .vertical_margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10)
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

                let blocks = Block::default()
                    .title("Input Field")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White));
                f.render_widget(blocks, chunks[2]);

                let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(5)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(70)
                    ].as_ref()
                )
                .split(f.size());
    
                let read = vec![
                    Spans::from(vec![
                        Span::raw("Enter First Name: "),
                        Span::from(read_inputs().unwrap())
                    ])
                ];
    
                let inputs = Paragraph::new(read)
                    .block(Block::default().title("Data").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });
                f.render_widget(inputs, chunks[0]);


                let table  = Table::new(v_outer.clone())
                .style(Style::default().fg(Color::White))
                .header(
                    Row::new(vec!["employee_id", "first_name", "middle_name", "last_name", "dept_code", "uan", "designation", "reporting_to"])
                        .style(Style::default().fg(Color::Cyan))
                        .bottom_margin(1)
                )
                .block(Block::default().title("Table").borders(Borders::ALL))
                .widths(&[Constraint::Length(15), Constraint::Length(15), Constraint::Length(15),Constraint::Length(15), Constraint::Length(15), Constraint::Length(15),Constraint::Length(15), Constraint::Length(15)])
                .column_spacing(1)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");
                f.render_widget(table, chunks[1]);
            })?;
            terminal.set_cursor(2, 41)?
        }
    }
}
