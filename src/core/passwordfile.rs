use crate::CredentialFailure;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Describes an entry in the [PasswordFile]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Entry {
    username: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordFile {
    entries: Vec<Entry>,
}

impl Entry {
    pub fn new(username: &str, password_hash: &str) -> Self {
        Self {
            username: username.into(),
            password_hash: password_hash.into(),
        }
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password_hash(&self) -> String {
        self.password_hash.clone()
    }
}

impl Drop for Entry {
    fn drop(&mut self) {
        // reset password_hash to zeros to reduce risk of password leak from memory
        self.password_hash().replace_range(0.., "0")
    }
}

impl PasswordFile {
    /// Creates a new, empty [PasswordFile]
    pub fn new() -> Self {
        PasswordFile {
            entries: Vec::new(),
        }
    }

    pub fn enumerate(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for elem in &self.entries {
            result.push((elem.username(), elem.password_hash()));
        }
        result
    }

    pub fn add_user(&mut self, user: &str, password_hash: &str) {
        let entry = Entry::new(user, password_hash);
        self.entries.push(entry);
    }

    pub fn remove_user(&mut self, user: &str) {
        self.entries.retain(|x| x.username().ne(user));
    }

    pub fn verify_credentials(
        &self,
        user: &str,
        password_hash: &str,
    ) -> Result<(), CredentialFailure> {
        match self.get_user(user) {
            Some(user) => {
                if user.password_hash() == password_hash {
                    Ok(())
                } else {
                    Err(CredentialFailure::InvalidPassword)
                }
            }
            None => Err(CredentialFailure::UnknownUser),
        }
    }

    fn get_user(&self, user: &str) -> Option<&Entry> {
        self.entries.iter().find(|&x| x.username() == user)
    }

    /// Saves the [PasswordFile] to the disk.
    pub fn save(&self, path: &Path) -> Result<(), String> {
        match serde_json::to_string(self) {
            Ok(json) => match fs::write(path, json) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!(
                    "could not save content of PasswordFile to {:?}: {}",
                    path, err
                )),
            },
            Err(err) => Err(format!("could not serialize PasswordFile to JSON: {}", err)),
        }
    }

    /// Loads a [PasswordFile] from a file.
    pub fn load(path: &Path) -> Result<PasswordFile, String> {
        let content = fs::read_to_string(path);
        match content {
            Ok(json) => match serde_json::from_str(json.as_str()) {
                Ok(data) => Ok(data),
                Err(err) => Err(format!("could not parse PasswordFile: {}", err)),
            },
            Err(err) => Err(format!(
                "could not load PasswordFile from {:?}: {}",
                path, err
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CredentialFailure;
    use super::{Entry, PasswordFile};

    #[test]
    fn entry_new_assigns_correct_values() {
        let entry = Entry::new("foo", "bar");
        assert_eq!("foo", entry.username());
        assert_eq!("bar", entry.password_hash());
    }

    #[test]
    pub fn passwordfile_new_returns_a_file_with_zero_entries() {
        let pf = PasswordFile::new();

        assert_eq!(0, pf.entries.len());
    }

    #[test]
    fn remove_user() {
        let mut pw = PasswordFile::new();
        pw.add_user("foo", "bar");
        pw.add_user("baz", "quux");

        pw.remove_user("foo");

        assert_eq!(1, pw.enumerate().len());
        assert_eq!("baz", pw.enumerate()[0].0);
        assert_eq!("quux", pw.enumerate()[0].1);
    }

    #[test]
    fn verify_credentials() {
        let mut pw = PasswordFile::new();
        pw.add_user("foo", "bar");
        pw.add_user("baz", "quux");

        assert_eq!(Ok(()), pw.verify_credentials("foo", "bar"));
        assert_eq!(Ok(()), pw.verify_credentials("baz", "quux"));
        assert_eq!(
            Err(CredentialFailure::InvalidPassword),
            pw.verify_credentials("baz", "WRONG")
        );
        assert_eq!(
            Err(CredentialFailure::UnknownUser),
            pw.verify_credentials("UNKNOWN", "whatever")
        );
    }

    #[test]
    fn serialization_then_deserialization() {
        let path = std::path::Path::new("pwd.json");
        std::fs::remove_file(path).unwrap_or(());

        let mut p = PasswordFile::new();
        let john = Entry::new("John", "dkazodkazdôkazd");
        let irvin = Entry::new("Irvin", "adjadaz08519");
        p.entries.push(john);
        p.entries.push(irvin);

        assert!(p.save(path).is_ok());

        let load = PasswordFile::load(path);
        match load {
            Err(err) => println!("{}", err),
            Ok(ok) => {
                assert_eq!("John", ok.entries[0].username());
                assert_eq!("dkazodkazdôkazd", ok.entries[0].password_hash());
                assert_eq!("Irvin", ok.entries[1].username());
                assert_eq!("adjadaz08519", ok.entries[1].password_hash());
            }
        }

        std::fs::remove_file(path).unwrap();
    }
}
