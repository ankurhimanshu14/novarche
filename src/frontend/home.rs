pub mod home {

    use crossterm::{
        event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, KeyModifiers },
    };

    use std::io;
    use tui::backend::{ CrosstermBackend, Backend };
    use tui::layout::{Alignment, Constraint, Rect, Direction, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::text::{Span, Spans};
    use tui::widgets::{Block, Borders, Paragraph, Tabs, Wrap, Table, Row, Cell, List, ListItem};
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

            let (input_cmd, tab_cmd) = read_inputs().unwrap();

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
                
                let titles = ["Administration", "Human Resources", "Accounts", "Raw Material Store", "Engineering", "Production", "Quality Assurance"].iter().cloned().map(Spans::from).collect();
                let tabs = Tabs::new(titles)
                    .block(Block::default().title("Novarche Inc.").borders(Borders::ALL))
                    .select(tab_cmd.unwrap().clone())
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::White).bg(Color::Magenta).add_modifier(Modifier::UNDERLINED))
                    .divider("|");
                f.render_widget(tabs, chunks[0]);

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

                match tab_cmd.unwrap() {
                    0 => {
                        let items = [ListItem::new("> Create New User"), ListItem::new("  --------------------------------------"), ListItem::new("> Change Password"), ListItem::new("> User Profile"), ListItem::new("> Logout")];
                        let list = List::new(items)
                                .block(Block::default().title("Administration").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White))
                                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                                .highlight_symbol(">>");
                        f.render_widget(list, chunks[0]);
                    },
                    1 => {
                        let items = [ListItem::new("Add New Employee"), ListItem::new("View Employee Details")];
                        let list = List::new(items)
                                .block(Block::default().title("Human Resources").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White))
                                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                                .highlight_symbol(">>");
                        f.render_widget(list, chunks[0]);
                    },
                    2 => {
                        let items = [ListItem::new("Add Bank Details"), ListItem::new("Get Salary Details")];
                        let list = List::new(items)
                                .block(Block::default().title("Accounts").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White))
                                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                                .highlight_symbol(">>");
                        f.render_widget(list, chunks[0]);
                    },
                    _ => {}
                }

                let table  = Table::new(v_outer.clone())
                .style(Style::default().fg(Color::White))
                .header(
                    Row::new(vec!["employee_id", "first_name", "middle_name", "last_name", "dept_code", "uan", "designation", "reporting_to"])
                        .style(Style::default().fg(Color::Cyan))
                        .bottom_margin(1)
                )
                .block(Block::default().title("Table").borders(Borders::ALL))
                .widths(&[Constraint::Length(22), Constraint::Length(22), Constraint::Length(22),Constraint::Length(22), Constraint::Length(22), Constraint::Length(22),Constraint::Length(22), Constraint::Length(22)])
                .column_spacing(1)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");
                f.render_widget(table, chunks[1]);
            })?;
            terminal.set_cursor(2, 61)?
        }
    }
}
