

fn sub_hour(time: &mut toml::value::Time, hours: u8)
{
	time.hour -= hours;
}
fn sub_minute(time: &mut toml::value::Time, minutes: u8)
{
	if minutes == 0 {return}
	if i16::from(time.minute) - i16::from(minutes) < 0
	{
		sub_hour(time, 1);
		time.minute = time.minute + 60 - minutes;
	}
	else
	{
		time.minute -= minutes;
	}
}
fn sub_second(time: &mut toml::value::Time, seconds: u8)
{
	if seconds == 0 {return}
	if i16::from(time.second) - i16::from(seconds) < 0
	{
		sub_minute(time, 1);
		time.second = time.second + 60 - seconds;
	}
	else
	{
		time.second -= seconds;
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
	return_time = from;

	sub_hour(&mut return_time, value.hour);
	sub_minute(&mut return_time, value.minute);
	sub_second(&mut return_time, value.second);
	
	return_time // return return_time;
}
