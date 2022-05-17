mod cmd;
mod common;
mod errors;
mod problems;

extern crate clap;
extern crate itertools;

use crate::errors::Result;

fn main() -> Result<()> {
    cmd::run()
}
