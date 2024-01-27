use std::convert::Infallible;
use std::net::SocketAddr;

use diesel::prelude::*;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use link_shortener::establish_connection;
use link_shortener::generate_short;
use link_shortener::models::*;
use link_shortener::run_migrations;
use link_shortener::schema::links;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

static INDEX: &str = "templates/index.html";
static NOTFOUND: &[u8] = b"Not Found";

/// HTTP status code 404
fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(NOTFOUND.into()).map_err(|e| match e {}).boxed())
        .unwrap()
}

async fn simple_file_send(
    filename: &str,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, Infallible> {
    // Open file for reading
    let file = File::open(filename).await;
    if file.is_err() {
        eprintln!("ERROR: Unable to open file.");
        return Ok(not_found());
    }

    let mut file: File = file.unwrap();

    let mut content = Vec::new();

    file.read_to_end(&mut content).await.unwrap();

    let boxed_body = Full::new(content.into()).map_err(|e| match e {}).boxed();

    // Send response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)
        .unwrap();

    Ok(response)
}

async fn link_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, Infallible> {
    let conn = &mut establish_connection();
    run_migrations(conn).unwrap();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => simple_file_send(INDEX).await,
        (&Method::POST, "/") => {
            // Create link in database and return 201

            let body_buff = req.collect().await;

            let mut host = String::from("http://localhost:3000/");
            let mut url = String::from("");

            for (k, e) in form_urlencoded::parse(&body_buff.unwrap().to_bytes()) {
                if k == "url" {
                    url = e.to_string();
                }
                if k == "host" {
                    host = e.to_string();
                }
            }

            if url == "" || !url.starts_with("http") {
                return Ok(Response::new(
                    Full::new("Invalid url".into())
                        .map_err(|e| match e {})
                        .boxed(),
                ));
            }

            let new_link = NewLink {
                original: &url,
                short: &generate_short(&url),
            };

            println!("Inserting new link: {:?}", new_link.short);

            diesel::insert_into(links::table)
                .values(&new_link)
                .returning(Link::as_select())
                .get_result(conn)
                .expect("Error saving new link");

            return Ok(Response::new(
                Full::new(Bytes::from(format!("{}{}", host, new_link.short)).into())
                    .map_err(|e| match e {})
                    .boxed(),
            ));
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
                return Ok(not_found());
            }

            // redirect 301
            return Ok(Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("Location", results[0].original.clone())
                .body(
                    Full::new("Redirect...".into())
                        .map_err(|e| match e {})
                        .boxed(),
                )
                .unwrap());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

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
        let _server = tokio::task::spawn(async move {
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
