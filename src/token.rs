use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DECODING_KEY: DecodingKey<'static> = DecodingKey::from_secret(
        Box::leak(std::env::var("JWT_SECRET").unwrap().into_boxed_str()).as_bytes()
    );
    static ref VALIDATION: Validation = Validation::default();
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

pub fn get_username(token: &str) -> Option<String> {
    decode::<Claims>(&token, &DECODING_KEY, &VALIDATION)
        .map(|it| it.claims.sub)
        .ok()
}
