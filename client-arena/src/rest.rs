use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64;

use serde::{Deserialize, Serialize};

use crate::arena::Arena;

pub trait RestClient {
    fn siste_vedtak(&self, ident: &str);
}

const PATH: &str = "/v1/aap/sisteVedtak";
const TOKEN_PATH: &str = "/oauth/token";
// arena, arenaoidc

impl RestClient for Arena {
    fn siste_vedtak(&self, ident: &str) {}
}

/**
arena:
    base-uri: https://arena-ords-q1.nais.preprod.local/arena/api og https://arena.adeo.no/arena_ws/services
    ping-path: v1/test/ping
    name: ARENA
    is-enabled:
    credential:
        id:
        secret:
*/


impl Arena {
    fn basic_auth(&self) -> String {
        let basic = format!("Basic {}:{}", self.credential.id, self.credential.secret);
        base64.encode(basic)
    }

    // async fn get_token(&self) -> anyhow::Result<Token, Error> {
    //     let uri = Uri::from_static(&format!("{}{}", self.host, TOKEN_PATH));
    //     let client = Client::default();
    //     let req = Request::post(uri)
    //         .header("Content-Type", "application/x-www-form-urlencoded")
    //         .body(Body::from("grant_type=client_credentials"))
    //         .with_context(||"OIDC token request for Arena")?;
    //
    //     Ok(token)
    // }
}

#[derive(Deserialize, Serialize)]
struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
}
