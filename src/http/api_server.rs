use hyper::{service::{make_service_fn, service_fn}, Body, Method, Request, Response, Server, StatusCode};

use crate::streams::ChannelAuthor;
use crate::store::{AnnotationStore, ReadingStore};

use std::{net::SocketAddr, sync::{Arc, Mutex}};
use crate::http::*;

static NOTFOUND: &[u8] = b"Not Found";
type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub async fn start(
    port: u16,
    author: Arc<Mutex<ChannelAuthor>>,
    annotation_store: Arc<Mutex<AnnotationStore>>,
    reading_store: Arc<Mutex<ReadingStore>>
) -> Result<(), GenericError> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = make_service_fn(move |_| {
        let author = author.clone();
        let annotation_store = annotation_store.clone();
        let reading_store = reading_store.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                responder(
                    req,
                    author.clone(),
                    annotation_store.clone(),
                    reading_store.clone()
                )
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("API listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn responder(
    req: Request<Body>,
    author: Arc<Mutex<ChannelAuthor>>,
    annotation_store: Arc<Mutex<AnnotationStore>>,
    reading_store: Arc<Mutex<ReadingStore>>,

) -> Result<Response<Body>, GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/subscribe") => subscribe_response(req, author).await,
        (&Method::GET, "/get_channel_address") => channel_address_response(author).await,
        (&Method::GET, "/get_announcement_id") => announcement_id_response(author).await,
        (&Method::GET, "/get_readings") => readings_response(req, reading_store).await,
        (&Method::GET, "/get_annotations") => annotations_response(req, annotation_store).await,
        (&Method::GET, "/get_confidence_score") => confidence_score_response(req, annotation_store).await,
        (&Method::GET, "/get_filtered_annotations") => filter_annotations_response(req, annotation_store).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    }
}
