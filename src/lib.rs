use dirs;
use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

struct env_perm {}
trait environment_operations {
    pub fn check_or_set<T, U>(var: T, value: U) -> io::Result<()>;
    pub fn append<T: fmt::Display>(var: T, value: T) -> io::Result<()>;
    pub fn set<T: fmt::Display, U: fmt::Display>(var: T, value: U) -> io::Result<()>;
}
impl environment_operations for env_perm {
    /// Checks if a environment variable is set.
    /// If it is then nothing will happen.
    /// If it's not then it will be added
    /// to your profile.
    fn check_or_set<T, U>(var: T, value: U) -> io::Result<()>
    where
        T: fmt::Display + AsRef<std::ffi::OsStr>,
        U: fmt::Display,
    {
        env::var(&var).map(|_| ()).or_else(|_| set(var, value))
    }

    /// Appends a value to an environment variable
    /// Useful for appending a value to PATH
    fn append<T: fmt::Display>(var: T, value: T) -> io::Result<()> {
        let mut profile = get_profile()?;
        writeln!(profile, "\nexport {}=\"{}:${}\"", var, value, var)?;
        profile.flush()
    }

    /// Sets an environment variable without checking
    /// if it exists.
    /// If it does you will end up with two
    /// assignments in your profile.
    /// It's recommended to use `check_or_set`
    /// unless you are certain it doesn't exist.
    fn set<T: fmt::Display, U: fmt::Display>(var: T, value: U) -> io::Result<()> {
        let mut profile = get_profile()?;
        writeln!(profile, "\nexport {}={}", var, value)?;
        profile.flush()
    }

    fn get_profile() -> io::Result<File> {
        dirs::home_dir()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::Other, "No home directory"))
            .and_then(find_profile)
    }

    #[cfg(target_family = "unix")]
    fn find_profile(mut profile: PathBuf) -> io::Result<File> {
        profile.push(".bash_profile");
        let mut oo = OpenOptions::new();
        oo.append(true).create(false);
        oo.open(profile.clone())
            .or_else(|_| {
                profile.pop();
                profile.push(".bash_login");
                oo.open(profile.clone())
            })
            .or_else(|_| {
                profile.pop();
                profile.push(".profile");
                oo.open(profile.clone())
            })
            .or_else(|_| {
                profile.pop();
                profile.push(".bash_profile");
                oo.create(true);
                oo.open(profile.clone())
            })
    }
}
