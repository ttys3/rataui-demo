use backtrace::Backtrace;
use std::{io, panic};
use std::io::Write;

use crossbeam_channel::{never, tick, unbounded, Receiver, Select};
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn setup_terminal() -> anyhow::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

pub fn shutdown_terminal() {
    let leave_screen =
        io::stdout().execute(LeaveAlternateScreen).map(|_f| ());

    if let Err(e) = leave_screen {
        eprintln!("leave_screen failed:\n{e}");
    }

    let leave_raw_mode = disable_raw_mode();

    if let Err(e) = leave_raw_mode {
        eprintln!("leave_raw_mode failed:\n{e}");
    }
}

pub fn start_terminal<W: Write>(
    buf: W,
) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}


// do log::error! and eprintln! in one line, pass sting, error and backtrace
macro_rules! log_eprintln {
	($string:expr, $e:expr, $bt:expr) => {
		log::error!($string, $e, $bt);
		eprintln!($string, $e, $bt);
	};
}

pub fn set_panic_handlers() -> anyhow::Result<()> {
    // regular panic handler
    panic::set_hook(Box::new(|e| {
        let backtrace = Backtrace::new();
        log_eprintln!("panic: {:?}\ntrace:\n{:?}", e, backtrace);
        shutdown_terminal();
    }));

    Ok(())
}