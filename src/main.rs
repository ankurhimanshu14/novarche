mod apis;
mod backend;
mod frontend;

use crate::backend::read_inputs::read_inputs::read_inputs;
use crate::frontend::home::home::home;

fn main() -> Result<(), std::io::Error> {
    home()?;
    // read_buffer().unwrap();
    Ok(())
}
