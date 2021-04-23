mod backend;

use crate::backend::read_buffer::read_buffer::read_buffer;

fn main() -> Result<(), std::io::Error> {
    read_buffer();
    Ok(())
}