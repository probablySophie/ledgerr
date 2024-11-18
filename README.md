
# Ledgerr

A simple Rust based plain text Finance and Time tracking library.  

Inspired by the general [Plain Text Accounting](https://plaintextaccounting.org) movement, and specifically spurred into action by the blog post [You should track your finances in TOML](https://kmaasrud.com/blog/track-finances-in-toml.html).


> [!WARNING]
> **You should not use this project**.  
> *At this point Ledgerr is still heavily under construction.  It can only vaguely do timesheets and cannot do anything at all with finance.*  

The goal is to eventually create a [Ratatui](https://ratatui.rs/) based TUI app using this library once it is feature complete.

## Simple Binary

The library comes with a simple Ledgerr binary `ledgerr_cmd` that is being used for testing and initial primitive usage.

Running the binary will no arguments (or an invalid argument) will provide a list of arguments and descriptions of what they do :)


## Storage Formats

Everything is stored in TOML

### Time Logging

Stored in a TOML file as:
```TOML
[[time-log]]
date = 2000-01-01
client = "Me"
project = "Dishes"
task = ""
description = "Washed a bunch of dishes"
start = 17:00:00
end = 17:30:00	
```

Loaded into the format:
```rust
pub struct Entry
{
	pub date: toml::value::Date,
	pub client: String,
	pub project: String,
	pub task: Option<String>,
	pub description: Option<String>,
	pub start: toml::value::Time,
	pub end: toml::value::Time,
	pub utc_offset: Option<i8>
}
```

### Finance Logging

```TOML
[[transaction]]
date = 2000-01-01
description = "Grocery shopping"
payee = "My local grocery store"
postings = [ # Changes to accounts
 {account = "Assets:BankName:Savings", amount=-45, currency = "GBP"},
 {account = "Expenses:Food:Groceries", amount=45, currency = "GBP"}
]
logged = 2000-01-02 13:00:02Z # When the transaction was actually logged (in UTC +0)
```

```rust
pub struct Posting
{
	account: String,
	amount: i32,
	currency: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction
{
	date: toml::value::Date,
	description: String,
	payee: String,
	postings: Vec<Posting>,
	logged: toml::value::Datetime,
}
```
