use colored::Colorize;
use serde::{Deserialize, Serialize};

#[path = "time_cmd_input.rs"] mod time_cmd_input;

use crate::{files, APP_NAME};

pub fn go(args: &mut Vec<String>)
{
	if args.len() == 1
	{
		println!("Ledgerr: {}", "No arguments provided for time".yellow());
		return;
	}
	args.remove(0); // Remove the "time" from the args
	crate::MatchCompletions!{
		args[0].to_lowercase().as_str(),
		"new", new(args), "Add a new time log",
		"view", view(args), "View the current time logs"
		//"total", save(args), "See the current time spent on each project"
	};
}

fn read_line_into(string: &mut String) -> bool
{
	string.clear();
	match std::io::stdin().read_line(string) {
    	Ok(_)  => {
    		*string = string.trim().to_string();
    		true
    	},// do whatever you want, line is String
    	Err(_) => {false},// handle error, e is IoError
	}
}

fn new(args: &mut Vec<String>)
{
	match time_cmd_input::prompt_all() {
	    Ok(entry)  => {
	    	match files::save::<EntryList>(
	    		files::Location::Data, 
	    		APP_NAME, 
	    		"timesheet.toml", 
	    		EntryList { time_log: vec![entry] }, 
	    		files::SaveOption::Append, 
	    	Some(vec![("time_sheet", "time-sheet")]))	    	
		    {
	    		Ok(_) => println!("Saved file!"),
	    		Err(error) => println!("Error: {error}"),
	    	}
	    },
	    Err(error) => println!("Failed to create new timesheet entry.  Received error:\n{error}"),
	}
}

#[derive(Serialize, Deserialize)]
struct EntryList {
	time_log: Vec<ledgerr::timesheet::Entry>
}
fn view(args: &mut Vec<String>)
{
	// let timesheet_result = files::load::<Vec<ledgerr::timesheet::Entry>>(files::Location::Data, APP_NAME, "timesheet.toml");
	// let timesheet_result = files::load::<ledgerr::timesheet::Entry>(files::Location::Data, APP_NAME, "timesheet.toml");
	let timesheet_result = files::load::<EntryList>(
		files::Location::Data, 
		APP_NAME, 
		"timesheet.toml",
		Some(vec![("time-log", "time_log")])
	);

	if let Ok(timesheet) = timesheet_result
	{
		for entry in timesheet.time_log 
		{
			println!();
			entry.pretty_print();
		}
	}
	else if let Err(error) = timesheet_result
	{
		println!("Ledgerr: {}\n{}", "Failed to load timesheet".yellow(), error.red());
	}
}


// See 
// TODO: fn total()
