use ring::digest::{Context, SHA256};

pub fn normalize_password(password: &str) -> [u8; 32] {
    // Creation of the SHA-256 digest
    let mut ctx = Context::new(&SHA256);
    ctx.update(password.as_bytes());
    let digest = ctx.finish();

    // Copy to the array
    let mut result = [0u8; 32];
    result.copy_from_slice(digest.as_ref());

    result

}