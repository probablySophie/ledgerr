use serde::{Serialize, Deserialize};

/// A single balance change.
#[derive(Serialize, Deserialize)]
pub struct Posting
{
	/// The account that is being changed
	account: String,
	/// The amount that is being added or removed
	amount: i32,
	/// The currency that is being used.
	/// **Currently serves no purpose**
	currency: String,
}

/// A single transaction
#[derive(Serialize, Deserialize)]
pub struct Transaction
{
	/// The `time::Date` the transaction happened
	date: toml::value::Date,
	/// A user written description for the transaction
	description: String,
	/// Who was paid, must be consistant for any filtering or cumulation functionality to work
	payee: String,
	/// All changes in balances that occured as a result of this transaction
	postings: Vec<Posting>,
	/// When this transaction was logged - no relation to the date it occured
	/// Stored in UTC+0 and localised when shown to the user
	logged: toml::value::Datetime,
}

// TODO: Read from string
// TODO: Write to string
// TODO: Test - read & write back same

// TODO: Get all payees from list of Transactions
// TODO: And testing for that

// TODO: Get current balances of accounts by reading all inputs
// TODO: And testing for that
// TODO: Create starting balance for next month/year/whenever
// TODO: And testing for that

// TODO: Filtering, & testing for the filtering
