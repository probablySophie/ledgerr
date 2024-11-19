// This heavily relies on the TOML crate
// https://docs.rs/toml/latest/toml/index.html

use std::io::Write;

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Location
{
	Data,
	Config,
	Local
}

/// Returns the platform specific directory for the given Location
fn dir(location: Location) -> Option<std::path::PathBuf>
{
	match location
	{
	    Location::Data => dirs::data_dir(),
	    Location::Config => dirs::config_dir(),
	    Location::Local => Some(std::path::PathBuf::new()),
	}
}

fn get_path(location: Location, folder: &str, file_name: &str) -> Result<String, String>
{
	// Get a path from the location we were given
	let Some(pathbuf) = dir(location)
	else {
		return Err(String::from("Failed to get directory"));
	};
	// Try convert our path into a str
	let Some(path) = pathbuf.to_str()
	else {
		return Err(String::from("Failed to convert path to str"));
	};
	// Convert the path into a String
	let mut path = path.to_string();
	// If there's a folder, then + /folder_name
	if ! folder.is_empty() {
		path += &(String::from("/") + folder);
	}
	path += &(String::from("/") + file_name);

	Ok(path)
}

// Makes in place replacements for save & load
fn make_replacements(string: &mut String, replacements: Option<Vec<(&str, &str)>>)
{
	// If there are any replacements
	let Some(replacement_sets) = replacements
	else{
		return
	};

	// Make the replacements
	for replacement in replacement_sets
	{
		*string = string.replace(replacement.0, replacement.1);
	}
}

/// Loads a file!
/// Takes a `Location` to load from, the folder in that location, and the file name.
/// Takes an optional set of replacements to make e.g. `"time-log"` -> `"time_log"`
/// # Errors
/// * Returns an error if something went wrong loading the file.
pub fn load<T: for<'de> toml::macros::Deserialize<'de>>
	(
		// The `Location` to load the file from
		location: Location, 
		// The folder to load inside of the `Location` (should probably be the app name)
		folder: &str, 
		// The file name to load **should probably end with .toml**
		file_name: &str,
		replacements: Option<Vec<(&str, &str)>>
	) 
	-> Result<T, String>
{	
	let path = match get_path(location, folder, file_name)
	{
		Ok(path) => path,
		Err(error) => return Err(error),
	};
	// Try read from the file
	let mut file_string = match std::fs::read_to_string(&path)
	{
		Ok(string) => string,
		Err(error) => return Err(error.to_string()),	
	};
	// Make the replacements (if there are any)
	make_replacements(&mut file_string, replacements);
	
	let err = toml::from_str::<T>(&file_string);
	if let Err(errr) = err
	{
		return Err(errr.to_string())
	}
	// Try convert the file string into TOML
	let Ok(extracted) = toml::from_str::<T>(&file_string)
	else {
		return Err(String::from("Failed to convert read string to TOML"));
	};

	Ok(extracted)
}

#[allow(unused)]
pub enum SaveOption
{
	Replace,
	Append
}

#[allow(clippy::needless_pass_by_value)]
/// Saves a file!
/// # Errors
/// TODO: 
pub fn save<T: serde::ser::Serialize> (
	location: Location, 
	folder: &str, 
	file_name: &str, 
	contents: T,
	save_option: SaveOption,
	replacements: Option<Vec<(&str, &str)>>
) -> Result<bool, String>
{
	let path = match get_path(location, folder, file_name)
	{
	    Ok(path) => path,
	    Err(error) => return Err(error),
	};
	let mut contents_string = match toml::to_string_pretty(&contents)
	{
	    Ok(string) => string,
	    Err(error) => return Err(error.to_string()),
	};
	
	make_replacements(&mut contents_string, replacements);

	// Make sure the directories we want to save in exist
	let _ = std::fs::create_dir_all(path.clone());
	
	match save_option {
	    SaveOption::Replace => 
	    {
	    	match std::fs::write(path, contents_string)
	    	{
	    		Ok(())     => Ok(true),
	    		Err(error) => Err(error.to_string()),
	    	}
	    },
	    SaveOption::Append => 
	    {
	    	match std::fs::OpenOptions::new().append(true).open(path)
	    	{
		        Ok(file) =>
		        {
		        	let mut writer = std::io::BufWriter::new(file);

		        	match writeln!(writer, "\n{contents_string}")
		        	{
		        		Ok(()) => Ok(true),
		        		Err(error) => Err(error.to_string())
		        	}
		        },
		        Err(error) => Err(error.to_string()),
		    }
	    },
	}
}
// dirs::data_dir() -> $HOME/.local/share
// dirs::config_dir() -> $HOME/.config
