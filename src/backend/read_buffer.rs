pub mod read_buffer {

    use std::io::{ stdout, Write };
    use crossterm::{
        cursor::{ MoveLeft },
        event::{ read, Event, KeyEvent, KeyCode, EnableMouseCapture, DisableMouseCapture },
        execute,
        style::{ Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor },
        ExecutableCommand, Result,
        terminal,
    };

    pub fn read_buffer() -> Result<()> {
        let mut buffer = String::new();
        stdout()
                .execute(SetForegroundColor(Color::Blue))?
                .execute(Print("> "))?
                .execute(ResetColor)?;
    
        terminal::enable_raw_mode()?;
        
        loop{
            match read()? {
                Event::Key(KeyEvent { code, modifiers: _}) => {
                    match code {
                        KeyCode::Char(c) => {
                            let mut char_buffer = [0; 4];
                            let bytes = c.encode_utf8(&mut char_buffer).as_bytes();
                            stdout().write_all(&bytes)?;
                            stdout().flush()?;
                            buffer.push(c)
                        },
                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                stdout().execute(MoveLeft(1))?;
                                stdout().write_all(&[b' '])?;
                                stdout().execute(MoveLeft(1))?;
                                stdout().flush()?;
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