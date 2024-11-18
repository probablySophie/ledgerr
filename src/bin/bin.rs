//! The testing terminal command for this crate.
//! Not even a TUI :(

use std::env;
use colored::Colorize;
extern crate ledgerr;

pub const APP_NAME: &str = "Ledgerr";

mod engine;
use engine::files;

mod commands;
use commands::time;

// TODO: Make this macro also build us a bash completions tree!
//       And make it crawl the called functions!
#[macro_export]
/// Handles matches and calls for an Option<Slice>
/// Expects the Option<Slice> and then repeating `TO_MATCH` `FUNC_TO_CALL` "Description"
macro_rules! MatchCompletions {
    ($to_match: expr, $($name:tt, $call:expr, $description:tt),+) => {
    	let options = vec![ $( ($name, $description), )+ ];
        match $to_match
        {
        	Some(value) => { 
        		match value.to_lowercase().as_str() {
		        	$( $name => $call, )+
		        	_ => {
		        		println!("Ledgerr: {}{}{}", "Argument '".yellow(), value, "' not recognised".yellow());
		        		println!("Valid options:");
		        		for option in options
		        		{
		        			println!("\t{}\t{}", option.0.bold(), option.1);
		        		}
		        	}
        		}
        	}
        	None => {
        		println!("Ledgerr: {}", "No argument provided".yellow());
        		println!("Valid options:");
        		for option in options
        		{
        			println!("\t{}\t{}", option.0.bold(), option.1);
        		}
        	}
        }
    };
}

mod money {
	pub fn main(args: &mut Vec<String>)
	{
		println!("{}", args[1]);
	}
}

// Run with cargo run --bin ledgerr -- ARGS
fn main()
{
	// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
	let mut args: Vec<String> = env::args().collect();

	args.remove(0); // Remove the executable
	MatchCompletions!(
		args.first(),
		"time", time::go(&mut args), "Ledgerr's time tracking module",
		"money", money::main(&mut args), "Ledgerr's finance tracking module"	
	);
}