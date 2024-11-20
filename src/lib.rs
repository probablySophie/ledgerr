// The library side of things!

mod structs;
mod secondary; // Stuff to make other stuff work

#[doc(inline)]
pub use structs::*;
#[doc(inline)]
pub use secondary::*;
