#[cfg(test)]
use crate::time_from_string;

#[cfg(test)]
fn time_test(time: &str, hour: u8, minute: u8, second: u8)
{
	assert_eq!(
		time_from_string(time), 
		Ok(toml::value::Time {
			hour,
			minute,
			second,
			nanosecond: 0
		}));
}

#[test]
fn time_string_hour_minute_second()
{ time_test("13:42:56", 13, 42, 56); }

#[test]
fn time_string_hour_minute()
{ time_test("14:50", 14, 50, 0); }

#[test]
fn time_string_hour()
{ time_test("16", 16, 0, 0); }

#[test]
fn time_string_hour_minute_am()
{ time_test("4:23am", 4, 23, 0); }

#[test]
fn time_string_hour_am()
{ time_test("8am", 8, 0, 0); }

#[test]
fn time_string_hour_minute_pm()
{ time_test("6:12pm", 18, 12, 0); }

#[test]
fn time_string_hour_pm()
{ time_test("9pm", 21, 0, 0); }

// TODO: Bad entries, like 28 o'clock or 13pm
