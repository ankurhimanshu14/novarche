pub mod read_inputs {

    use crossterm::{
        cursor::MoveLeft,
        event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, KeyModifiers },
        execute, queue,
        style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
        terminal, ExecutableCommand, QueueableCommand, Result,
    };
    use std::io::{stdout, Write};

    pub fn read_inputs() -> Result<(Option<String>, Option<usize>)> {
        let mut stdout = stdout();
        let select = 0;
        let mut buffer = String::new();
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print("$ "))?
            .execute(ResetColor)?
            .execute(EnableMouseCapture)?;

        terminal::enable_raw_mode()?;

        let mut data_set: (Option<String>, Option<usize>) = (Some(buffer.clone()), Some(select.clone()));
        
        'repl: loop {

            'inner: loop {
                
                match read()? {
                    Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                        KeyCode::Char(c) => {
                            stdout.queue(Print(c))?;
                            stdout.flush()?;
                            buffer.push(c)
                        },
                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                stdout
                                    .queue(MoveLeft(1))?
                                    .queue(Print(" "))?
                                    .queue(MoveLeft(1))?;
                                stdout.flush()?;
                            }
                        },
                        KeyCode::Enter => {
                            data_set.0 = Some(buffer.clone());
                            for _ in 0..buffer.len() {
                                if !buffer.is_empty() {
                                    buffer.pop();
                                    stdout
                                        .queue(MoveLeft(1))?
                                        .queue(Print(" "))?
                                        .queue(MoveLeft(1))?;
                                    stdout.flush()?;
                                }
                            }
                            break 'repl;
                        },
                        _ => {}
                    },
                    Event::Mouse(event) => {
                        match event {
                            MouseEvent::Down(MouseButton::Left, (1..=16), 1, KeyModifiers::NONE) => {
                                data_set.1 = Some(0);
                                break 'repl;
                            },
                            MouseEvent::Down(MouseButton::Left, (18..=34), 1, KeyModifiers::NONE) => {
                                data_set.1 = Some(1);
                                break 'repl;
                            },
                            MouseEvent::Down(MouseButton::Left, (36..=45), 1, KeyModifiers::NONE) => {
                                data_set.1 = Some(2);
                                break 'repl;
                            },
                            MouseEvent::Down(MouseButton::Left, _, _, KeyModifiers::NONE) => break 'inner,
                            _ => {}
                        }
                    }
                    Event::Resize(width, height) => {
                        println!("Width: {}, Height: {}", width, height)
                    }
                }
                terminal::disable_raw_mode()?;
            }
        }
        Ok(data_set)
    }
}
