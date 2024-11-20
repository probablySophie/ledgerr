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

pub fn prompt_all() -> Result<ledgerr::timesheet::Entry, String>
{
	let mut input: String = String::new();
	println!("Making a new timesheet entry");

	let mut new_entry = ledgerr::timesheet::Entry::default();
	
	println!("Date [{}-{}-{}]: ", new_entry.date.year, new_entry.date.month, new_entry.date.day);
	read_line_into(&mut input);
	if input.is_empty()
	{
		// Use the default
	}
	else
	{
		match ledgerr::date_from_string(&input)
		{
		    Ok(date)   => new_entry.date = date,
		    Err(error) => return Err(error),
		}
	}
	
	println!("Client: ");
	read_line_into(&mut input);
	new_entry.client.clone_from(&input);
	
	println!("Project: ");
	read_line_into(&mut input);
	new_entry.project.clone_from(&input);
	
	println!("Task (or none): ");
	read_line_into(&mut input);
	if !input.is_empty()
	{
		new_entry.task = Some(input.clone());
	}
	
	println!("Description (or none): ");
	read_line_into(&mut input);
	if ! input.is_empty()
	{
		new_entry.description = Some(input.clone());
	}
	
	println!("Start Time (24 hours): ");
	read_line_into(&mut input);
	match ledgerr::time_from_string(&input)
	{
	    Ok(time)   => new_entry.start = time,
	    Err(error) => return Err(error),
	};
	
	println!("End Time (24 hours): ");
	read_line_into(&mut input);
	match ledgerr::time_from_string(&input)
	{
	    Ok(time)   => new_entry.end = time,
	    Err(error) => return Err(error),
	};

	// Make sure we started before we ended
	if new_entry.start > new_entry.end 
	{
		return Err(String::from("Start time is greated than end time"));
	}

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
