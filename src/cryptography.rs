use argon2::{Algorithm, Argon2, password_hash::SaltString};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE;
use fernet::Fernet;

const LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;

pub fn hash(string: &[u8], salt: &[u8]) -> Result<String, argon2::Error> {
    let mut out = [0u8; 2048];
    let argon = Argon2::new(
        Algorithm::Argon2id,
        argon2::Version::V0x10,
        argon2::Params::new(2_u32.pow(16), 1, 3, Some(2048))?,
    );
    argon.hash_password_into(&string, &salt, &mut out)?;
    Ok(URL_SAFE.encode(out))
}

pub fn check_hash(string: &[u8], salt: &[u8], hash_check: String) -> bool {
    hash(string, salt).unwrap() == hash_check
}

pub fn generate_salt(
    rand: &mut impl rand_core::CryptoRngCore,
) -> Result<SaltString, argon2::password_hash::Error> {
    let mut buf = [0u8; LENGTH];
    rand.fill_bytes(&mut buf);
    SaltString::encode_b64(&buf)
}

pub fn encrypt(pwd: &[u8], master_pwd: &[u8], salt: SaltString) -> String {
    let mut key = [0u8; KEY_LENGTH];
    let mut buffer = pwd.clone();
    let argon = Argon2::new_with_secret(
        master_pwd,
        Algorithm::Argon2id,
        argon2::Version::V0x10,
        argon2::Params::new(2_u32.pow(16), 2, 3, Some(KEY_LENGTH)).unwrap(),
    )
    .unwrap();
    argon.hash_password_into(pwd, &salt.as_str().as_bytes(), &mut key);
    let key_b64 = URL_SAFE.encode(key);
    let f = Fernet::new(&key_b64.as_str()).unwrap();
    f.encrypt(&buffer)
}
