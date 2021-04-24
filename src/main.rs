mod backend;
mod frontend;

use crate::backend::read_buffer::read_buffer::read_buffer;
use crate::frontend::home::home::home;

fn main() -> Result<(), std::io::Error> {
    home();
    read_buffer();
    Ok(())
}