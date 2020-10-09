use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Loggers {
    Warning,
    Danger,
    Info,
}

fn colored_text<T>(inp: T, color: Color)
where
    T: std::fmt::Debug,
{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(color))).ok();
    writeln!(&mut stdout, "{:#?}", inp).ok();
    stdout.reset().ok();
}

pub fn log<T>(log: Loggers, inp: T) -> Result<(), io::Error>
where
    T: std::fmt::Debug,
{
    match log {
        Loggers::Danger => Ok(colored_text(inp, Color::Red)),
        Loggers::Info => Ok(colored_text(inp, Color::Blue)),
        Loggers::Warning => Ok(colored_text(inp, Color::Yellow)),
    }
    // Err(io::Error::new(io::ErrorKind::NotFound, "Not Found"))
}
