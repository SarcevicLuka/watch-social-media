use zxcvbn::zxcvbn;

/// Helper function to validate password strength
pub fn password_strength(password: &str) -> bool {
    match zxcvbn(password, &[]) {
        Ok(entropy) => {
            entropy.score() < 3
        },
        Err(_) => false,
    }
}