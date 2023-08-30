use std::io;
use crate::term_util::{setup_terminal, shutdown_terminal, set_panic_handlers, start_terminal};
use scopeguard::defer;
mod term_util;
mod list;


fn main() -> anyhow::Result<()> {
    env_logger::init();

    setup_terminal()?;
    defer! {
		shutdown_terminal();
	}

    set_panic_handlers()?;

    // terminal: &mut Terminal<CrosstermBackend<io::Stdout>>
    let mut terminal = start_terminal(io::stdout())?;

    list::run_loop(&mut terminal)?;
    
    Ok(())
}
