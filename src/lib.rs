// The library side of things!

mod structs;
mod secondary; // Stuff to make other stuff work

#[doc(inline)]
pub use structs::*;
#[doc(inline)]
pub use secondary::*;

// TODO: Look into multithreaded reading from drive & writing to drive
// TODO: Read & write functions (or maybe that should be a library caller problem?)
// TODO: Optional save split into year/month/week files


