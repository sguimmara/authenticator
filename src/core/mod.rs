use sha2::{Digest, Sha256};

pub fn hash(text: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(text);

    let result = hasher.finalize();

    let h = hex::encode(result);

    return h;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn hash_returns_correct_value() {
        let text = "hello world";

        let actual = hash(text);

        assert_eq!(
            actual,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        )
    }
}
