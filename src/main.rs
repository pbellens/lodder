mod app;
mod message;
mod parse;
mod stats;
mod ui;
mod vocab;

use clap::Parser;
use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;
use std::{
    fs,
    io::{self, BufReader},
};
use tui::{backend::CrosstermBackend, Terminal};

fn setup_log() -> Result<log4rs::Handle> {
    //let _level = log::LevelFilter::Info;
    let file_path = "/tmp/lodder.log";
    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)?;
    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )?;

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    log4rs::init_config(config).or_else(|_e| Err(color_eyre::Report::msg("Config init failed!")))
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file with XML
    xml: PathBuf,
    /// Configuration file
    #[arg(short, long, value_name = "config file")]
    cfg: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let cfg: stats::Cfg = match cli.cfg.as_deref() {
        None => stats::Cfg::default(),
        Some(cfgf) => {
            let file = fs::File::open(cfgf).expect("Unable to read config file {cfgf}");
            serde_json::from_reader(BufReader::new(file))?
        }
    };

    // setup logging, maybe better using a log level instead of a bool?
    let _handle = if cfg.log { setup_log().ok() } else { None };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = app::App::new(true);
    app::run_app(
        cli.xml,
        &mut terminal,
        app,
        cfg.stats.as_ref(),
        cfg.header.as_ref(),
    )?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
