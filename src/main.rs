mod cmd;
mod common;
mod errors;
mod problems;

extern crate clap;
extern crate itertools;
extern crate num_bigint;
extern crate num_traits;
extern crate reqwest;

use crate::errors::Result;

fn main() -> Result<()> {
    cmd::run()
}
