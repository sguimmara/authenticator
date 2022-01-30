use std::path::Path;

/// Describes an entry in the [PasswordFile]
#[derive(Debug)]
struct Entry {
    username: String,
    password_hash: String,
}

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

    /// Saves the [PasswordFile] to the disk.
    pub fn save(&self, path: &Path) {
        unimplemented!()
    }

    /// Loads a [PasswordFile] from a file.
    pub fn load(path: &Path) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
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
}
