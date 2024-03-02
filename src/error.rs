use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CrackerError {
    #[error("Usage: ftp_cracker <logins_list> <passwords_list> <target_ip:port>")]
    WrongArguments,

    #[error("Credentials - Not found")]
    CredentialsNotFound,
}
