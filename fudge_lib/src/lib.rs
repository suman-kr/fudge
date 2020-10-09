use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum Loggers {
    Warning,
    Error,
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

/// Function to log the input to terminal with foreground color
///  
/// **Colors:** 
/// `Warning` -> *Yellow*
/// `Error` -> *Red*
/// `Info` -> *Blue*
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
