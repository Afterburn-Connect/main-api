pub fn hash_string(input: impl ToString) -> String {
    use sha3::{Digest, Sha3_256};
    let mut hasher = Sha3_256::new();
    hasher.update(input.to_string());
    format!("{:x}", hasher.finalize())
}