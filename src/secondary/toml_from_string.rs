
fn split_please<'a>(string: &'a str, options: &[&'a str]) -> core::str::Split<'a, &'a str>
{
	for option in options
	{
		if string.contains(option)
		{
			return string.split(option)
		}
	}
	string.split(" ")
}

/// Get a `toml:value::Date` from an `&str`!
/// Must be in the form YYYY-MM-DD
/// # Errors
/// * If unable to retrieve a year, month, or day
/// * If unable to convert the year, month, or day to a u16 or u8
pub fn date_from_string(string: &str) -> Result<toml::value::Date, String>
{	
	let mut split = split_please(string, &["/", "-", "\\"]);
	
	let year = match split.next()
	{
	    Some(str) => match str.parse() {
	    	Ok(val) => val,
	    	Err(_) => return Err(String::from("Failed to parse year from {str}")),
	    },
	    None => return Err(String::from("Failed to get a year from {string}")),
	};
	let month = match split.next()
	{
	    Some(str) => match str.parse() {
	    	Ok(val) => val,
	    	Err(_) => return Err(String::from("Failed to parse month from {str}")),
	    },
	    None => return Err(String::from("Failed to get a month from {string}")),
	};
	let day = match split.next()
	{
	    Some(str) => match str.parse() {
	    	Ok(val) => val,
	    	Err(_) => return Err(String::from("Failed to parse day from {str}")),
	    },
	    None => return Err(String::from("Failed to get a day from {string}")),
	};
	
	Ok(toml::value::Date{ year, month, day })
}

/// Get a `toml::value::Time` from an `&str`!
/// Requires that the time provided be seperated by ':' and be in 24 hour time.
/// This function will assume that a failed seconds value retrieval is 0
/// # Errors
/// * If the string did not have any ':' values
/// * If unable to convert from either of hour:minute to u8
pub fn time_from_string(time_string: &str) -> Result<toml::value::Time, String>
{
	let mut string = time_string.to_lowercase();
	
	let mut hour_modifier = 0;
	if string.contains("pm")
	{
		hour_modifier = 12;
		string = string.replace("pm", "");
	}
	if string.contains("am")
	{
		string = string.replace("am", "");
	}

	// If there isn't a divider char
	if ! string.contains(':')
	{
		// Assume they provided just an hour
		let Ok(hour) = string.parse::<u8>()
		else {
			return Err(String::from("Failed to convert provided &str to time"));
		};
		return Ok( toml::value::Time { 
			hour: hour + hour_modifier, 
			minute: 0, 
			second: 0, 
			nanosecond: 0 
		} )
	}
	// Else
	let mut split = string.split(':');
	// Hour
	let Some(hour_str) = split.next()
	else {
		return Err(String::from("Failed to get hour"));
	};
	let Ok(hour) = hour_str.parse::<u8>()
	else {
		return Err(String::from("Failed to convert '{hour_str}' to a u8"));
	};
	// Minute
	let minute_str = split.next().unwrap_or("00");
	let Ok(minute) = minute_str.parse()
	else {
		return Err(String::from("Failed to convert '{minute_str}' to a u8"));
	};
	// Second (0 if not provided)
	let second_str = split.next().unwrap_or("00");
	let Ok(second) = second_str.parse()
	else {
		return Err(String::from("Failed to convert '{second_str}' to a u8"));
	};

	Ok(toml::value::Time{ hour: hour + hour_modifier, minute, second, nanosecond: 0 })
}

