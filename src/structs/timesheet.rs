use serde::{Serialize, Deserialize};

#[path = "../tests/timesheet_test.rs"] mod test;

// Getting the current time
// http://stackoverflow.com/questions/26593387/ddg#44378174

// https://doc.rust-lang.org/std/time/struct.SystemTime.html

// https://docs.rs/time/latest/time/struct.Date.html
// https://docs.rs/time/latest/time/struct.Time.html


/// A single timesheet entry.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry
{
	/// The date that this logged time period occurred on
	pub date: toml::value::Date,
	/// Whom this time was used for.
	/// Specifically useful for billing, or otherwise calculating "I told you so" hours.
	pub client: String,
	/// The specific project that the time was spent on
	pub project: String,
	/// Optional specific project task for more specific/detailed time keeping
	pub task: Option<String>,
	/// Optional description of how the time was spent/what was achieved/anything else.
	pub description: Option<String>,
	/// The time the timesheet entry started (local-time)
	pub start: toml::value::Time,
	/// The time the timesheet entry ended (local-time)
	pub end: toml::value::Time,
	/// Optional UTC offset for the timezone the logged time period occured in.
	pub utc_offset: Option<i8>
}
impl Default for Entry
{	
	fn default() -> Self
	{
		let datetime = crate::secondary::datetime_now();
		let date = crate::secondary::date_from_option(datetime);
		let time = crate::secondary::time_from_option(datetime);
		
    	Self { 
    		date,
    		client: String::new(), 
    		project: String::new(), 
    		task: None,
    		description: None, 
    		start: time,
    		end: time,
    		utc_offset: None
    	}
	}
}
impl Entry
{
	pub fn pretty_print(&self)
	{
		let time_sub = crate::time_sub(self.end, self.start);
		println!("Date: {}-{}-{}", self.date.year, self.date.month, self.date.day);
		println!("From: {}:{} to {}:{}", self.start.hour, self.start.minute, self.end.hour, self.end.minute);
		println!("Thats {} hours and {} minutes", time_sub.hour, time_sub.minute);
		if let Some(val) = self.utc_offset { println!("UTC+{val}") }		
		println!("\tClient      | {}", self.client);
		println!("\tProject     | {}", self.project);

		// Only print these if there's something there!
		if let Some(val) = self.task.clone() { println!("\tTask        | {val}") }
		if let Some(val) = self.description.clone() { println!("\tDescription | {val}") }
	}

	#[must_use] pub fn minutes(&self) -> i32
	{
		crate::time_as_minutes(self.end) - crate::time_as_minutes(self.start)
	}
}

fn already_there(string: &String, strings: &[String]) -> Option<usize>
{
	for (i, check) in strings.iter().enumerate()
	{
		if string == check
		{
			return Some(i)
		}
	}
	// Not there
	None
}

// TODO: Parallellise all of these?
macro_rules! get_list_by_value {
    ($entries: ident, $value:tt) => {
        let mut results: Vec<String> = Vec::new();
        
		for entry in $entries
		{
			if already_there(&entry.$value, &results).is_none() {
				results.push( entry.$value.clone() )
			}
		}		
        return results
    };
}

// TODO: Testing for get_clients
#[must_use]
pub fn get_clients(entries: &[Entry]) -> Vec<String>
{
	get_list_by_value!{entries, client}
}

// TODO: Testing for get_projects
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn get_projects(entries: &[Entry], filter_client: Option<String>) -> Vec<String>
{
	if filter_client.is_none()
	{
		get_list_by_value!{entries, project}
	}
	// Else
	let mut projects: Vec<String> = Vec::new();
	'entries: for entry in entries
	{
		// Skip bad matches
		if Some(&entry.client) != filter_client.as_ref() { continue }

		// Skip if already in the list
		for existing_project in &projects
		{
			if *existing_project == entry.project
			{
				continue 'entries
			}
		}
		// It's not in the list!
		projects.push(entry.project.clone());
	}
	projects
}

#[must_use]
/// Returns a `Vec<(String, String)>` as `Vec<(Client_name, Project_name)>` containing a list of all Clients and their Projects
pub fn get_clients_and_projects(entries: &[Entry]) -> Vec<(String, String)>
{
	let clients = get_clients(entries);
	let mut client_projects: Vec<(String, String)> = Vec::new();

	// For each client
	for client in clients
	{
		// For each entry
		'entries: for entry in entries
		{
			// If they don't match.  Skip
			if entry.client != client { continue }

			// Is the client-project pair already in the list?
			for pair in &client_projects
			{
				// Yes.  Skip to the next entry on the list
				if pair.0 == client && pair.1 == entry.project 
				{
					continue 'entries
				}
			}
			// No, add to the list
			client_projects.push( ( client.clone(), entry.project.clone() ) );
		}
	}
	client_projects
}

fn in_time_list(identifier: &String, list: &[(String, i32)]) -> Option<usize>
{
	for (i, item) in list.iter().enumerate()
	{
		if item.0 == *identifier
		{
			return Some(i)
		}
	}
	None
}

// TODO: Parallellise?
macro_rules! get_time_by {
    ($entries:ident, $value:tt) => {
        let mut results: Vec<(String, i32)> = Vec::new();

		for entry in $entries
		{
			match in_time_list( &entry.$value, &results)
			{
				Some(i) => 
				{
					results[i].1 += entry.minutes();
				},
				None => 
				{
					results.push( ( entry.$value.clone(), entry.minutes() ) );
				}
			}
		}		

        return results
    };
}

/// Returns a list of projects & total minutes in the form `Vec<(String, i32)>`
#[must_use] pub fn get_project_time(entries: &[Entry]) -> Vec<(String, i32)>
{
	// TODO: This should not just filter by project, but client too so projects with the same name don't stack
	get_time_by!{entries, project}
}

/// Returns a list of projects & total minutes in the form `Vec<(String, i32)>`
#[must_use] pub fn get_client_time(entries: &[Entry]) -> Vec<(String, i32)>
{
	get_time_by!{entries, client}
}


#[must_use]
#[allow(dead_code)]
/// Creates a new `Vec<Entry>` based on the given filters
fn filter(entries: &[Entry], before: Option<toml::value::Date>, after: Option<toml::value::Date>) -> Vec<Entry>
{
	let mut filtered = Vec::new();

	let date_after = after.unwrap_or( toml::value::Date { year: 0, month: 0, day: 0 } );
	let date_before = before.unwrap_or( toml::value::Date { year: u16::MAX, month: u8::MAX, day: u8::MAX } );

	for entry in entries
	{
		if date_after < entry.date && entry.date < date_before
		{
			filtered.push ( (*entry).clone() );
		}
	}

	filtered
}

// TODO: Multi-day entries? e.g. 8:00 pm Saturday -> 3:00 am Sunday
