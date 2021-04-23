pub mod read_buffer {

    use std::io::{ stdout, Write };
    use crossterm::{
        cursor::{ MoveLeft },
        event::{ read, Event, KeyEvent, KeyCode, EnableMouseCapture, DisableMouseCapture },
        execute, queue,
        style::{ Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor },
        ExecutableCommand, QueueableCommand, Result,
        terminal,
    };

    pub fn read_buffer() -> Result<()> {
        let mut stdout = stdout();
        let mut buffer = String::new();
        stdout
            .execute(SetForegroundColor(Color::Blue))?
            .execute(Print("> "))?
            .execute(ResetColor)?;
    
        terminal::enable_raw_mode()?;
        
        loop{
            match read()? {
                Event::Key(KeyEvent { code, modifiers: _}) => {
                    match code {
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
                            break;
                        },
                        _ => {}
                    }
                },
                Event::Mouse(event) => {
                    println!("{:?}", event)
                },
                Event::Resize(width, height) => {
                    println!("Width: {}, Height: {}", width, height)
                },
            }
        }
    
        println!("\nOur buffer: {}", buffer);
    
        terminal::disable_raw_mode()?;
    
        Ok(())
    }
}