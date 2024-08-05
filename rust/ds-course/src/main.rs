extern crate core;

use std::error::Error;

mod structs {
    pub mod arrays;
    pub mod expressions;
    pub mod linked_lists;
    pub mod matrices;
    pub mod polynomials;
    pub mod queues;
    pub mod smart_ptrs;
    pub mod stacks;
    pub mod strings;
    pub mod tokens;
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
