mod cmd;
mod common;
mod errors;
mod problems;

extern crate clap;
use crate::errors::Result;

fn main() -> Result<()> {
    cmd::run()
}
