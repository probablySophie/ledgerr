use colored::Colorize;
use serde::{Deserialize, Serialize};

#[path = "time_cmd_input.rs"] 
mod time_cmd_input;

#[path = "time_view.rs"] 
mod time_view;

use crate::{files, APP_NAME};

pub fn go(args: &mut Vec<String>)
{
	args.remove(0); // Remove the "time" from the args
	crate::MatchCompletions!{
		args.first(),
		"new", new(), "Add a brand new time log",
		"add", add(), "Add a new time log to an existing client & project",
		"view", time_view::view(args), "View the current time logs",
		"list", time_view::list(args), "List the given item from the logs"
	};
}

fn new()
{
	match time_cmd_input::prompt_all() {
	    Ok(entry)  => {
	    	match files::save::<EntryList>(
	    		files::Location::Data, 
	    		APP_NAME, 
	    		"timesheet.toml", 
	    		EntryList { time_log: vec![entry] }, 
	    		files::SaveOption::Append, 
		    	None,//Some(vec![("time_sheet", "time-sheet")])
		    )	    	
		    {
	    		Ok(_) => println!("Saved file!"),
	    		Err(error) => println!("Error: {error}"),
	    	}
	    },
	    Err(error) => println!("Failed to create new timesheet entry.  Received error:\n{error}"),
	}
}

fn add()
{
	let logs = match load()
	{
		Ok(entries) => entries,
		Err(error) => 
		{
			println!("Error: {error}");
			return
		},
	};
	
	match time_cmd_input::existing_client_project(&logs) {
	    Ok(entry)  => {
	    	match files::save::<EntryList>(
	    		files::Location::Data, 
	    		APP_NAME, 
	    		"timesheet.toml", 
	    		EntryList { time_log: vec![entry] }, 
	    		files::SaveOption::Append, 
		    	None,//Some(vec![("time_sheet", "time-sheet")])
		    )	    	
		    {
	    		Ok(_) => println!("Saved file!"),
	    		Err(error) => println!("Error: {error}"),
	    	}
	    },
	    Err(error) => println!("Failed to create new timesheet entry.  Received error:\n{error}"),
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
struct EntryList {
	time_log: Vec<ledgerr::timesheet::Entry>
}

pub fn load() -> Result< Vec<ledgerr::timesheet::Entry>, String >
{
	match files::load::<EntryList>(
		files::Location::Data, 
		APP_NAME, 
		"timesheet.toml",
		None,//Some(vec![("time-log", "time_log")])
	)
	{
		Ok (list)  => Ok(list.time_log),
		Err(error) => Err(error),
	}
}
