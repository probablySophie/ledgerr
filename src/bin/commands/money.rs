use colored::Colorize;
use serde::{Deserialize, Serialize};

pub fn main(args: &mut Vec<String>)
{
	crate::MatchCompletions!{
		args.first(),
		"new", new(args), "Add a new financial transaction"
		// "view", time_view::view(args), "View the current time logs"
	};
}

fn new(args: &mut Vec<String>)
{
	//
}

fn balance(args: &mut Vec<String>)
{
	//
}
