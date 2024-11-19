
#[must_use] pub fn time_as_seconds(time: toml::value::Time) -> i32
{
	(((i32::from(time.hour) * 60) + i32::from(time.minute) ) * 60 
	) 
		+ i32::from(time.second)
}
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[must_use] pub fn time_from_seconds(seconds: i32) -> toml::value::Time
{
	toml::value::Time
	{
		hour: (seconds / 3600) as u8,
		minute: ((seconds / 60 ) % 60) as u8,
		second: (seconds % 60) as u8,
		nanosecond: 0
	}
}

#[must_use] pub fn time_as_minutes(time: toml::value::Time) -> i32
{
	(i32::from(time.hour) * 60) + i32::from(time.minute)
}
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[must_use] pub fn time_from_minutes(minutes: i32) -> toml::value::Time
{
	toml::value::Time
	{
		hour: (minutes / 60) as u8,
		minute: (minutes % 60) as u8,
		second: 0,
		nanosecond: 0
	}
}

#[must_use]
pub fn time_sub(from: toml::value::Time, value: toml::value::Time) -> toml::value::Time
{
	let mut return_time = toml::value::Time { hour: 0, minute: 0, second: 0, nanosecond: 0 };
	if from < value
	{
		// TODO: that's bad
		return return_time;
	}
	// Else
	return_time = time_from_seconds( time_as_seconds(from) - time_as_seconds(value) );
	
	return_time // return return_time;
}

#[must_use]
pub fn pretty_time_from_minutes(minutes: i32) -> String
{
	let mut string = String::new();
	let hours = minutes / 60;
	let minutes = minutes % 60;

	if hours != 0
	{
		string += &(hours.to_string() + " hrs");
	}
	if hours != 0 && minutes != 0 
	{
		string += " ";
	}
	if minutes != 0
	{
		string += &(minutes.to_string() + " mins");
	}

	string
}
