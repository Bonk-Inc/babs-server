use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use jsonwebtoken::jwk::{Jwk, JwkSet};

use crate::JWK_FILE_PATH;

/// Reads the contents of the JWKs file.
/// 
/// # Errors
/// 
/// This function fails if:
/// - The JWKS file cannot be opened
/// - Content of the JWKS file cannot be read
/// - Content of the JWKS file cannot be serialized to a [`Jwk`] struct
/// 
pub fn get_jwk_tokens() -> Result<Option<Jwk>, String> {
    let file = File::open(JWK_FILE_PATH).map_err(|_| "Cannot open JWK file".to_string())?;
    let mut buffer_reader = BufReader::new(file);
    let mut contents = String::new();
    buffer_reader.read_to_string(&mut contents).map_err(|_| "Cannot read content of JWK file".to_string())?;

    let tokens: JwkSet = serde_json::from_str(&contents).map_err(|_| "Cannot create structs from JWK content".to_string())?;
    Ok(tokens.keys.get(0).cloned())
}
