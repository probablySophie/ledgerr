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
impl Transaction
{
	#[must_use]
	pub fn new(postings: Vec<Posting>, payee: String, description: String) -> Self
	{
		let datetime = crate::secondary::datetime_now().unwrap_or(
			toml::value::Datetime {
				date: Some(crate::secondary::default_date()),
				time: Some(crate::secondary::default_time()),
				offset: None,
			}
		);
		Transaction
		{
			date: datetime.date.unwrap_or( crate::secondary::default_date() ),
			description,
			payee,
			postings,
			logged: datetime,
		}
	}
}

fn insert_into_vec(posting: &Posting, vec: &mut Vec<(String, i32)>)
{
	for account in vec.iter_mut()
	{
		if account.0 == posting.account
		{
			account.1 += posting.amount;
			return
		}
	}
	// Else
	vec.push( (posting.account.clone(), posting.amount) );
}

// TODO: Parallellise
// TODO: Testing for get_accounts
#[must_use]
/// Get a list of accounts and their balances in the form `Vec<(String, i32)>`
pub fn get_accounts(transactions: &[Transaction]) -> Vec<(String, i32)>
{
	let mut accounts = Vec::new();
	// For each transaction
	for transaction in transactions
	{
		// For each posting in that transaction
		for posting in &transaction.postings
		{
			// Either insert it into the vec or append it to the end
			insert_into_vec(posting, &mut accounts);
		}
	}
	accounts
}

// TODO: Parallellise
#[must_use]
pub fn get_payees(transactions: &[Transaction]) -> Vec<String>
{
	let mut payees: Vec<String> = Vec::new();

	for transaction in transactions
	{
		let mut found = false;
		for payee in &payees
		{
			if transaction.payee == *payee
			{
				found = true;
				break;
			}
		}
		if !found 
		{
			payees.push(transaction.payee.clone());
		}
	}
	payees
}

// TODO: Create starting balance for next month/year/whenever
// TODO: And testing for that

// TODO: Filtering, & testing for the filtering
