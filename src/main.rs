extern crate argonaut;

use argonaut::{Parse, ArgDef};
use std::env;
use std::str::FromStr;

const USAGE: &'static str = "Usage: cargo linebreak <ignored> [options]";
const HELP: &'static str = "\
Prints a line of characters to fill a line with. 
Intended to be used with the cargo-watch tool, 
eg: 'cargo watch linebreak check'.

Optional arguments:
ignored         The first trailing argument is ignored, to be compatible with 
                cargo subcommand invocation.
--text | -t     Set the text or character to fill the line with. Default: '='.
--length | -n   The maximum length of the filled line. Default: 80.
--prefix | -p   Text to print before the line. Default: '\\n'.
--suffix | -s   Text to print after the line. Default: '\\n'.

--version       Show the SemVer version of this tool.
--help | -h     Show this help message.\
";

// Entry point
fn main() {  
    use argonaut::Arg::*;
    
    let a_ignored = ArgDef::optional_trail();
    let a_text = ArgDef::named_and_short("text", 't').option();
    let a_length = ArgDef::named_and_short("length", 'n').option();
    let a_prefix = ArgDef::named_and_short("prefix", 'p').option();
    let a_suffix = ArgDef::named_and_short("suffix", 's').option();
    let a_version = ArgDef::named("version").switch();
    let a_help = ArgDef::named_and_short("help", 'h').switch();
    
    let expected = &[a_ignored, a_text, a_length, a_prefix, a_suffix, a_version,
        a_help];  
    let args: Vec<_> = env::args().skip(1).collect();
    
    let mut fill_text = "=";
    let mut length = 80;
    let mut prefix = "\n";
    let mut suffix = "\n";
    
    let mut parse = Parse::new(expected, &args).expect("Invalid definitions");
    while let Some(item) = parse.next() {
        match item {
            Err(err) => {
                println!("Parse error: {:?}", err);
                println!("{}", USAGE);
            },
            Ok(Option("text", value)) => {
                fill_text = value;
            },
            Ok(Option("length", value)) => {
                length = if let Ok(val) = usize::from_str(value) {
                    val
                } else {
                    println!("Invalid length given: {}", value);
                    println!("{}", USAGE);
                    return;
                };
            },
            Ok(Option("prefix", value)) => {
                prefix = value;
            },
            Ok(Option("suffix", value)) => {
                suffix = value;
            },
            Ok(Switch("version")) => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                return;
            },
            Ok(Switch("help")) => {
                println!("{}\n\n{}", USAGE, HELP);
                return;
            },
            Ok(_) => unreachable!(),
        }
    }
    
    let mut line = String::new();
    while line.len() + fill_text.len() <= length {
        line.push_str(fill_text);
    }
    println!("{}{}{}", prefix, line, suffix);
}