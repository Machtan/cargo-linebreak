extern crate argonaut;

use argonaut::{Parser, Arg};
use argonaut::ParseStatus::{Parsed, Interrupted};
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
    
    let arg_vec: Vec<_> = env::args().skip(1).collect();
    let mut args: Vec<&str> = Vec::new();
    for arg in arg_vec.iter() {
        args.push(arg);
    }
    
    let a_ignored = Arg::optional_trail();
    let a_text = Arg::named_and_short("text", 't').single();
    let a_length = Arg::named_and_short("length", 'n').single();
    let a_prefix = Arg::named_and_short("prefix", 'p').single();
    let a_suffix = Arg::named_and_short("suffix", 's').single();
    let a_version = Arg::named("version").interrupt();
    let a_help = Arg::named_and_short("help", 'h').interrupt();
    
    let mut parser = Parser::new();
    parser.add(&a_ignored).unwrap();
    parser.add(&a_text).unwrap();
    parser.add(&a_length).unwrap();
    parser.add(&a_prefix).unwrap();
    parser.add(&a_suffix).unwrap();
    parser.add(&a_version).unwrap();
    parser.add(&a_help).unwrap();    
    
    match parser.parse(&args) {
        Ok(Parsed(parsed)) => {
            let fill_text = parsed.named("text").single().unwrap()
                .unwrap_or("=");
            let length_val = parsed.named("length").single().unwrap();
            let length = if let Some(arg) = length_val {
                match usize::from_str(arg) {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid length given: {}", arg);
                        println!("{}", USAGE);
                        return;
                    }
                }
            } else {
                80
            };
            let prefix = parsed.named("prefix").single().unwrap()
                .unwrap_or("\n");
            let suffix = parsed.named("suffix").single().unwrap()
                .unwrap_or("\n");
            
            let mut line = String::new();
            while line.len() + fill_text.len() <= length {
                line.push_str(fill_text);
            }
            
            println!("{}{}{}", prefix, line, suffix);
        },
        Ok(Interrupted(flag)) => {
            match flag {
                "version" => {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                },
                "help" => {
                    println!("{}\n\n{}", USAGE, HELP);
                }
                _ => unreachable!(),
            }
        },
        Err(error) => {
            println!("Parse error: {:?}", error);
        }
    }
}