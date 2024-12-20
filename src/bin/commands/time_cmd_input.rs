
pub fn existing_client_project(entries: &[ledgerr::timesheet::Entry]) -> Result<ledgerr::timesheet::Entry, String>
{
	let mut input: String = String::new();
	println!("Making a new timesheet entry with existing client & project");
	
	let mut new_entry = ledgerr::timesheet::Entry::default();
	read_date(&mut new_entry, &mut input)?;

	// println!("Client: ");
	// read_line_into(&mut input);
	// new_entry.client.clone_from(&input);
	
	// println!("Project: ");
	// read_line_into(&mut input);
	// new_entry.project.clone_from(&input);

	println!("Please select a client from this list: ");
	let clients = ledgerr::timesheet::get_clients(entries);
	for (i, client) in clients.iter().enumerate()
	{
		println!("{i} - {client}");
	}
	read_line_into(&mut input);
	match input.parse::<u16>()
	{
		Ok(u16) =>
		{
			let index = usize::from(u16);
			new_entry.client = match clients.get(index)
			{
				Some(val) => val.to_string(),
				None => return Err(String::from("Invalid index")),
			}
		},
		Err(e) =>
		{
			println!("Error: {e}");
			return Err(String::from("Failed to convert given client num to u16"));
		}
	}
	
	println!("Please select a project from this list: ");
	let projects = ledgerr::timesheet::get_projects(entries, Some(new_entry.client.clone()));
	for (i, project) in projects.iter().enumerate()
	{
		println!("{i} - {project}");
	}
	read_line_into(&mut input);
	match input.parse::<u16>()
	{
		Ok(u16) =>
		{
			let index = usize::from(u16);
			new_entry.project = match projects.get(index)
			{
				Some(val) => val.to_string(),
				None => return Err(String::from("Invalid index")),
			}
		},
		Err(e) =>
		{
			println!("Error: {e}");
			return Err(String::from("Failed to convert given project num to u16"));
		}
	}

	read_task_description(&mut new_entry, &mut input);
	read_time(&mut new_entry, &mut input)?;
	
	println!("Please confirm that this is correct: ");
	println!();
	new_entry.pretty_print();

	println!("y/n (n): ");
	read_line_into(&mut input);

	// Time to save!
	if input.trim().starts_with('y')
	{ 
		return Ok(new_entry);
	}
	// Else
	Err(String::from("User provided input '") + input.trim() + "'.  Not saving.")
}

pub fn prompt_all() -> Result<ledgerr::timesheet::Entry, String>
{
	let mut input: String = String::new();
	println!("Making a new timesheet entry");

	let mut new_entry = ledgerr::timesheet::Entry::default();
	
	read_date(&mut new_entry, &mut input)?;
	
	println!("Client: ");
	read_line_into(&mut input);
	new_entry.client.clone_from(&input);
	
	println!("Project: ");
	read_line_into(&mut input);
	new_entry.project.clone_from(&input);

	read_task_description(&mut new_entry, &mut input);	
	read_time(&mut new_entry, &mut input)?;

	println!("Please confirm that this is correct: ");
	println!();
	new_entry.pretty_print();

	println!("y/n (n): ");
	read_line_into(&mut input);

	// Time to save!
	if input.trim().starts_with('y')
	{ 
		return Ok(new_entry);
	}
	// Else
	Err(String::from("User provided input '") + input.trim() + "'.  Not saving.")
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

fn read_date(new_entry: &mut ledgerr::timesheet::Entry, input: &mut String) -> Result<(), String>
{
	println!("Date [{}-{}-{}]: ", new_entry.date.year, new_entry.date.month, new_entry.date.day);
	read_line_into(input);
	if input.is_empty()
	{
		return Ok(())
	}
	// Else
	match ledgerr::date_from_string(&input)
	{
	    Ok(date)   => 
	    {
	    	new_entry.date = date;
	    	return Ok(())
	    },
	    Err(error) => return Err(error),
	}
}

fn read_task_description(new_entry: &mut ledgerr::timesheet::Entry, input: &mut String)
{
	println!("Task (or none): ");
	read_line_into(input);
	if !input.is_empty()
	{
		new_entry.task = Some(input.clone());
	}
	
	println!("Description (or none): ");
	read_line_into(input);
	if ! input.is_empty()
	{
		new_entry.description = Some(input.clone());
	}
}

fn read_time(new_entry: &mut ledgerr::timesheet::Entry, input: &mut String) -> Result<(), String>
{
	println!("Start Time (24 hours): ");
	read_line_into(input);
	match ledgerr::time_from_string(&input)
	{
	    Ok(time)   => new_entry.start = time,
	    Err(error) => return Err(error),
	};
	
	println!("End Time (24 hours): ");
	read_line_into(input);
	match ledgerr::time_from_string(&input)
	{
	    Ok(time)   => new_entry.end = time,
	    Err(error) => return Err(error),
	};
	
	// Make sure we started before we ended
	if new_entry.start > new_entry.end 
	{
		return Err(String::from("Start time is greater than end time"));
	}

	Ok(())
}
