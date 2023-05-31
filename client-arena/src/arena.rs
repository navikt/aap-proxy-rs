pub struct Arena {
    pub credential: ArenaCredentials,
    pub host: String,
}

pub struct ArenaCredentials {
    pub id: String,
    pub secret: String,
}

pub fn siste_vedtak(ident: &str) {
    let path = "/v1/aap/sisteVedtak";
}

// soap
pub fn nyeste_aktive_sak(ident: &str) {}

const TOKEN_PATH: &str = "/oauth/token";

pub async fn handle_token() {

}

pub async fn token() {

}
