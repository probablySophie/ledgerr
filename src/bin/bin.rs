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
// TODO: Also make this able to print an (expected --a or --b)??
#[macro_export]
macro_rules! MatchCompletions {
    ($to_match: expr, $($name:tt, $call:expr, $description:tt),+) => {
        match $to_match
        {
        	$( $name => $call, )+
        	_ => println!("Ledgerr: {}{}{}", "Argument '".yellow(), $to_match, "' not recognised".yellow())
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

	if args.len() == 1
	{
		println!("Ledgerr: {}", "No arguments provided".yellow());
		return
	}
	args.remove(0); // Remove the executable
	MatchCompletions!(
		args[0].to_lowercase().as_str(),
		"time", time::go(&mut args), "Ledgerr's time tracking module",
		"money", money::main(&mut args), "Ledgerr's finance tracking module"	
	);
}
