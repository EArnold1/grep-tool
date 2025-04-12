use lexopt::prelude::*;

#[derive(Debug)]
pub struct Args {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    let mut query = None;
    let mut file_path = None;
    let mut case_sensitive = false;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('i') => {
                case_sensitive = true;
            }
            Value(val) if query.is_none() => {
                query = Some(val.string()?);
            }
            Value(val) if file_path.is_none() => {
                file_path = Some(val.string()?);
            }
            Short('h') | Long("help") => {
                println!("Usage: hello [-n|--number=NUM] [--shout] THING");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        query: query.ok_or("missing argument SEARCH QUERY")?,
        file_path: file_path.ok_or("missing argument FILE PATH")?,
        case_sensitive,
    })
}
