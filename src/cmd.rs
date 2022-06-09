use crate::errors::{Error, ErrorKind, Result};
use crate::problems;

/// Entrypoint for the application
pub fn run() -> Result<()> {
    let matches = clap::Command::new("Rust Rosalind")
        .author("Mimi Wang, mimikwang@gmail.com")
        .version("0.1.0")
        .about("Rosalind problems solver")
        .subcommand(problems::dna::command())
        .subcommand(problems::rna::command())
        .subcommand(problems::revc::command())
        .subcommand(problems::fib::command())
        .subcommand(problems::gc::command())
        .subcommand(problems::hamm::command())
        .subcommand(problems::iprb::command())
        .subcommand(problems::prot::command())
        .subcommand(problems::subs::command())
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        Some((problems::dna::SUBCOMMAND, matches)) => problems::dna::run(matches),
        Some((problems::rna::SUBCOMMAND, matches)) => problems::rna::run(matches),
        Some((problems::revc::SUBCOMMAND, matches)) => problems::revc::run(matches),
        Some((problems::fib::SUBCOMMAND, matches)) => problems::fib::run(matches),
        Some((problems::gc::SUBCOMMAND, matches)) => problems::gc::run(matches),
        Some((problems::hamm::SUBCOMMAND, matches)) => problems::hamm::run(matches),
        Some((problems::iprb::SUBCOMMAND, matches)) => problems::iprb::run(matches),
        Some((problems::prot::SUBCOMMAND, matches)) => problems::prot::run(matches),
        Some((problems::subs::SUBCOMMAND, matches)) => problems::subs::run(matches),
        _ => Err(Error::new(ErrorKind::User, "unknown subcommand")),
    }
}
