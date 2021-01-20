use crate::types::{Frame, ParseError};

pub struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(arg) => Ok(arg),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            None => Ok(()),
            Some(_) => return Err(ParseError::TooManyArgs),
        }
    }
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // Skip the first arg - script name
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;
    args.require_no_args()?;
    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    if width < 5 {
        return Err(ParseError::WidthTooSmall);
    }

    if height < 5 {
        return Err(ParseError::HeightTooSmall);
    }

    Ok(Frame { width, height })
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(ParseError::InvalidInteger(s)),
    }
}
