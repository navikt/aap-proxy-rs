use axum::body;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use hyper::{Body, HeaderMap, StatusCode};
use hyper::upgrade::Upgraded;
use tokio::net::TcpStream;

use crate::utils::is_allowed;

/** Hyper service for fss-proxy */
pub(crate) async fn fss(mut req: Request<Body>) -> Result<Response, hyper::Error> {
    tracing::trace!(?req);

    if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
        if is_allowed(&host_addr) {
            tokio::task::spawn(async move {
                // converts the incomming HTTP request to a HTTP CONNECT request
                // this is to set up a tunnel between the proxy server and the destination server

                *req.headers_mut() = intercept_auth_header(req.headers_mut()).await;

                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, host_addr).await {
                            tracing::warn!("server io error: {}", e);
                        };
                    }
                    Err(e) => tracing::warn!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(body::boxed(body::Empty::new())))
        } else {
            Ok((StatusCode::LOCKED, "URI not allowed.").into_response())
        }
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((StatusCode::BAD_REQUEST, "CONNECT must be to a socket address").into_response())
    }
}

async fn intercept_auth_header(headers: &mut HeaderMap) -> HeaderMap {
    headers.remove("Authorization");
    headers.insert("Authorization", "Bearer <token>".parse().unwrap());
    headers.clone()
}

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;

    // copy the upgraded tcp stream to copy data between the client and the server
    let (from_client, from_server) = tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::info!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    fn not_allowlisted_returns_423() {
        let rt = Runtime::new().unwrap();
        let req = Request::connect("some://shit").method("GET").body(Body::empty()).unwrap();
        let res = fss(req);
        let res = rt.block_on(res).unwrap();
        assert_eq!(res.status(), StatusCode::LOCKED);
    }

    #[test]
    fn allowlisted_returns_200() {
        let rt = Runtime::new().unwrap();
        let req = Request::connect("tokio.rs:443").method("GET").body(Body::empty()).unwrap();
        let res = fss(req);
        let res = rt.block_on(res).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        println!("{:?}", res)
    }

    #[test]
    fn proxy_doesnt_return_auth_header() {
        let rt = Runtime::new().unwrap();
        let req = Request::connect("tokio.rs:443").method("GET").body(Body::empty()).unwrap();
        let res = fss(req);
        let res = rt.block_on(res).unwrap();
        assert!(!res.headers().contains_key("Authorization"));
    }
}