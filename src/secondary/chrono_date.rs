use chrono::{Datelike, Timelike};

#[must_use]
pub fn default_date() -> toml::value::Date
{
	toml::value::Date { year: 0, month: 0, day: 0 }
}
#[must_use]
pub fn default_time() -> toml::value::Time
{
	toml::value::Time { hour: 0, minute: 0, second: 0, nanosecond: 0 }
}


#[must_use]
pub fn date_from_option(option_date: Option<toml::value::Datetime>) -> toml::value::Date
{
	match option_date
	{
	    Some(datetime) => {
	    	match datetime.date
	    	{
		        Some(date) => date,
		        None => crate::secondary::default_date(),
		    }
	    },
	    None => crate::secondary::default_date(),
	}
}
#[must_use]
pub fn time_from_option(option_date: Option<toml::value::Datetime>) -> toml::value::Time
{
	match option_date
	{
	    Some(datetime) => {
	    	match datetime.time
	    	{
		        Some(time) => time,
		        None => crate::secondary::default_time(),
		    }
	    },
	    None => crate::secondary::default_time(),
	}
}

#[must_use]
pub fn datetime_now() -> Option<toml::value::Datetime>
{
	let now = chrono::offset::Utc::now();

	let Ok(year) = u16::try_from(now.year())
	else { return None };
	let Ok(month) = u8::try_from(now.month())
	else { return None };
	let Ok(day) = u8::try_from(now.day())
	else { return None };

	let Ok(hour) = u8::try_from(now.hour())
	else { return None };
	let Ok(minute) = u8::try_from(now.minute())
	else { return None };
	let Ok(second) = u8::try_from(now.second())
	else { return None };

	Some(toml::value::Datetime
	{
	    date: Some(toml::value::Date
	    {
	    	year,
	    	month,
	    	day
	    }),
	    time: Some(toml::value::Time
	    {
	    	hour,
	    	minute,
	    	second,
	    	nanosecond: 0,
	    }),
	    offset: None,
	})
}
