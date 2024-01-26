use std::convert::Infallible;
use std::net::SocketAddr;

use diesel::prelude::*;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use link_shortener::establish_connection;
use link_shortener::models::*;
use tokio::net::TcpListener;

async fn link_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let conn = &mut establish_connection();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            // Return html form
            return Ok(Response::new(Full::new(Bytes::from("Hello from hyper"))));
        }
        (&Method::POST, "/") => {
            // Create link in database and return 201
            return Ok(Response::new(Full::new(Bytes::from(
                "Hello from hyper test",
            ))));
        }
        _ => {
            use link_shortener::schema::links::dsl::*;
            let search_code = &String::from(req.uri().path())[1..];
            let results = links
                .filter(short.eq(search_code))
                .limit(1)
                .select(Link::as_select())
                .load(conn)
                .expect("Error loading posts");

            if results.len() == 0 {
                return Ok(Response::new(Full::new(Bytes::from("Not found url 404"))));
            }

            // redirect 301
            return Ok(Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("Location", results[0].original.clone())
                .body(Full::new(Bytes::from("")))
                .unwrap());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    println!("Server running on: http://{:?}", addr);

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(link_service))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
