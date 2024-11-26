use colored::Colorize;

/// Handles the loading, error matching, and error printing
/// Your code in `$code` will only be run if successful
macro_rules! load_match {
    ($ident:ident, $code:block) => {
        match crate::time::load()
        {
        	Ok($ident) =>
        	{
        		$code
        	},
			Err(error) => {
				println!("Ledgerr: {}\n{}", "Failed to load timesheet".yellow(), error.red());
			}
        }
    };
}

// TODO: Filter what we're viewing based on the args!
pub fn view(args: &mut Vec<String>)
{
	args.remove(0); // Remove the "time" from the args
	crate::MatchCompletions!{
		args.first(),
		"all", view_all(), "View all timesheet entries",
		"project", project_total(), "View totals by project",
		"client", client_total(), "View totals by client"
	};
}

pub fn view_all()
{
	load_match!(timesheet, {
		for log in timesheet 
		{
			println!(); 
			log.pretty_print();
		}
	});
}

// TODO: a fancy table print, so that everyone gets the same amount of spacing betwen a AANNDD b
// TODO: Support a since=1w and an after=2024-11-10 and before=2024-11-15
pub fn project_total()
{
	load_match!(timesheet, {
		let project_totals = ledgerr::timesheet::get_project_time(&timesheet);
		println!("{}\t| {}", "Project".bold(), "Time".bold());
		for total in project_totals
		{
			println!("{}\t| {}", total.0, ledgerr::pretty_time_from_minutes(total.1));
		}
	});
}
pub fn client_total()
{
	load_match!(timesheet, {
		let client_totals = ledgerr::timesheet::get_client_time(&timesheet);
		println!("{}\t| {}", "Client".bold(), "Time".bold());
		for total in client_totals
		{
			println!("{}\t| {}", total.0, ledgerr::pretty_time_from_minutes(total.1));
		}
	});
}


pub fn list(args: &mut Vec<String>)
{
	args.remove(0); // Remove the "list" from the args
	crate::MatchCompletions!{
		args.first(),
		"projects", list_projects(), "List all projects",
		"clients", list_clients(), "List all clients"
	};
}

fn list_projects()
{
	load_match!(timesheet, {
		let client_project_pairs = ledgerr::timesheet::get_clients_and_projects(&timesheet);
		let mut last_client = String::new();
		for pair in client_project_pairs
		{
			if pair.0 != last_client
			{
				println!("Client: {}", pair.0.bold());
				last_client = pair.0;
			}
			println!("\t* {}", pair.1);
		}
	});
}

fn list_clients()
{
	load_match!(timesheet, {
		let clients = ledgerr::timesheet::get_clients(&timesheet);
		println!("Clients:");
		for client in clients
		{
			println!("* {client}");
		}
	});
}
