use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Tabs, ListItem, List };
use tui::text::{ Spans };
use tui::style::{ Style, Color, Modifier };
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
                                    .direction(Direction::Vertical)
                                    .margin(0)
                                    .constraints(
                                        [
                                            Constraint::Percentage(15),
                                            Constraint::Percentage(85)
                                        ].as_ref()
                                    )
                                    .split(f.size());

        let titles = [ "Administration", "Human Resources", "Engineering", "Quality Assurance", "Production", "Maintenance", "RM Store", "General Store" ].iter().cloned().map(Spans::from).collect();
        let tabs = Tabs::new(titles)
                        .block(Block::default().title("Novarche").borders(Borders::ALL))
                        .style(Style::default().fg(Color::LightCyan))
                        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
                        .divider("|");
        f.render_widget(tabs, chunks[0]);

        let chunks = Layout::default()
                                    .direction(Direction::Horizontal)
                                    .vertical_margin(3)
                                    .constraints(
                                        [
                                            Constraint::Percentage(10),
                                            Constraint::Percentage(80)
                                        ].as_ref()
                                    )
                                    .split(f.size());

        let items = [ListItem::new("Add User"), ListItem::new("Forgot Password?"), ListItem::new("Change Password")];
        let lists = List::new(items)
                        .block(Block::default().title("Menu").borders(Borders::ALL))
                        .style(Style::default().fg(Color::LightCyan))
                        .highlight_style(Style::default().bg(Color::Yellow).add_modifier(Modifier::BOLD))
                        .highlight_symbol(">>");
        f.render_widget(lists, chunks[0]);
    })

}
