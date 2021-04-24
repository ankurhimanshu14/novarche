pub mod read_buffer {

    use crossterm::{
        cursor::MoveLeft,
        event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
        execute, queue,
        style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
        terminal, ExecutableCommand, QueueableCommand, Result,
    };
    use std::io::{stdout, Write};

    pub fn read_buffer() -> Result<(String)> {
        let mut stdout = stdout();
        let mut buffer = String::new();
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print(">>> "))?
            .execute(ResetColor)?
            .execute(EnableMouseCapture)?;

        terminal::enable_raw_mode()?;

        loop {
            match read()? {
                Event::Key(KeyEvent { code, modifiers: _ }) => match code {
                    KeyCode::Char(c) => {
                        stdout.queue(Print(c))?;
                        stdout.flush()?;
                        buffer.push(c)
                    }
                    KeyCode::Backspace => {
                        if !buffer.is_empty() {
                            buffer.pop();
                            stdout
                                .queue(MoveLeft(1))?
                                .queue(Print(" "))?
                                .queue(MoveLeft(1))?;
                            stdout.flush()?;
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                },
                Event::Mouse(event) => {
                    // println!("{:?}", event
                }
                Event::Resize(width, height) => {
                    println!("Width: {}, Height: {}", width, height)
                }
            }
        }

        let s = format!("{}", buffer);

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

        terminal::disable_raw_mode()?;

        Ok(s)
    }
}
