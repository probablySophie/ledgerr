use serde::{Serialize, Deserialize};

// Getting the current time
// http://stackoverflow.com/questions/26593387/ddg#44378174

// https://doc.rust-lang.org/std/time/struct.SystemTime.html

// https://docs.rs/time/latest/time/struct.Date.html
// https://docs.rs/time/latest/time/struct.Time.html


/// A single timesheet entry.
#[derive(Serialize, Deserialize, Debug)]
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
		// TODO: Make the date default to today
		// TODO: Make both the times default to now
    	Self { 
    		date: toml::value::Date { year: 2000, month: 1, day: 1 },
    		client: String::new(), 
    		project: String::new(), 
    		task: None,
    		description: None, 
    		start: toml::value::Time { hour: 0, minute: 0, second: 0, nanosecond: 0 },
    		end: toml::value::Time { hour: 0, minute: 0, second: 0, nanosecond: 0 },
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

// TODO: Parallellise all of these?
// TODO: Testing for get_clients
#[must_use]
pub fn get_clients(entries: &[Entry]) -> Vec<String>
{
	get_list_by_value!{entries, client}
}

// TODO: Testing for get_projects
#[must_use]
pub fn get_projects(entries: &[Entry]) -> Vec<String>
{
	get_list_by_value!{entries, project}
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
	get_time_by!{entries, project}
}

/// Returns a list of projects & total minutes in the form `Vec<(String, i32)>`
#[must_use] pub fn get_client_time(entries: &[Entry]) -> Vec<(String, i32)>
{
	get_time_by!{entries, client}
}

// TODO: Get all tasks from a Vec<Entry>
// TODO: And testing for that

// TODO: Filtering & testing for the filtering

// TODO: Multi-day entries? e.g. 8:00 pm Saturday -> 3:00 am Sunday
