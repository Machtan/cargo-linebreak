extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::str::FromStr;
use std::ops::Deref;

// Wrap chars to make them deserializable from a string
struct Char(char);

impl FromStr for Char {
    type Err = String;
    fn from_str(s: &str) -> Result<Char, Self::Err> {
        match s.chars().count() {
            0 => Err("The string is empty!".to_string()),
            1 => Ok(Char(s.chars().last().unwrap())),
            _ => Err("The string is more than 1 character long!".to_string())
        }
    }
}

impl Deref for Char {
    type Target = char;
    
    fn deref(&self) -> &char {
        let Char(ref c) = *self;
        c
    }
}

// Entry point
fn main() {    
    let mut ch = Char('=');
    let mut prefix = "\n".to_string();
    let mut suffix = "\n".to_string();
    let mut n: usize = 80;
    {
        let mut parser = ArgumentParser::new();
        
        parser.set_description("
        Prints a line of characters based on input.
        Intended to be used with the cargo-watch tool,
        eg: 'cargo watch linebreak check'
        ");
        
        parser.refer(&mut ch).add_option(&["-c", "--char"], Store, 
            "The character to fill the line with. Default '='");
        
        parser.refer(&mut n).add_option(&["-n", "--line-length"], Store, 
            "The length of the filled line. Default 80");
        
        parser.refer(&mut prefix).add_option(&["-p", "--prefix"], Store,
            "The text to print before the line. Default \"\\n\"");
        
        parser.refer(&mut suffix).add_option(&["-s", "--suffix"], Store,
            "The text to print after the line. Default \"\\n\"");
        
        parser.parse_args_or_exit();
    }
    let line: String = (0..n).map(|_| {let Char(c) = ch; c}).collect();
    println!("{}{}{}", prefix, line, suffix);
}