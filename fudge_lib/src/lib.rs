use chrono::Local;
use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// **Colors:**
/// `Warning` -> *Yellow*
/// `Error` -> *Red*
/// `Info` -> *Blue*
pub enum Loggers {
    Warning,
    Error,
    Info,
}

fn format_date_time() -> String {
    let date_time = Local::now().to_string();
    let formatted_string: Vec<&str> = date_time.split(|i| i == '.').collect();
    formatted_string[0].into()
}

fn colored_text<T>(inp: T, color: Color)
where
    T: std::fmt::Debug,
{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    write!(&mut stdout, "{} ", format_date_time()).ok();
    stdout.set_color(ColorSpec::new().set_fg(Some(color))).ok();
    writeln!(&mut stdout, "{:?}", inp).ok();
    stdout.reset().ok();
}

/// Function to log the input to terminal with foreground color
///  
///  @param
/// ---    
///  logger:
/// ```
/// pub enum Loggers {
///     Warning,
///     Error,
///     Info,
/// }
/// ```
/// inp: `T`
///
pub fn log<T>(loggger: Loggers, inp: T) -> Result<(), io::Error>
where
    T: std::fmt::Debug,
{
    match loggger {
        Loggers::Error => Ok(colored_text(inp, Color::Red)),
        Loggers::Info => Ok(colored_text(inp, Color::Blue)),
        Loggers::Warning => Ok(colored_text(inp, Color::Yellow)),
    }
}
