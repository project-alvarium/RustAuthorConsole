//use anyhow::Result;
use hyper::{Request, Body, Response, StatusCode, header};
use crate::streams::ChannelAuthor;
use crate::models::{SubscriptionRequest, SensorId, ReadingId};
use std::sync::{Mutex, Arc};
use crate::store::{ReadingStore, ReadingStoreFilterId, AnnotationStoreFilterId, AnnotationStore, AnnotationStoreFilter};
use std::error::Error;


type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub async fn subscribe_response(
    req: Request<Body>,
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let json_data: serde_json::Result<SubscriptionRequest> = serde_json::from_slice(&data);
    match json_data {
        Ok(sub_req) => {
            let mut author = author.lock().unwrap();
            match author.subscribe(&sub_req.msgid, &hex::decode(sub_req.pk)?) {
                Ok(keyload_link) => {
                    println!("Processed subscription, returning keyload link...");
                    response = Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from("Subscription processed, keyload link: ".to_owned() + &keyload_link.to_string()))?;
                },
                Err(e) => {
                    println!(
                        "POST /subscribe Error while subscribing {:?}", e.to_string()
                    );
                    response = Response::builder()
                        .status(500)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from("Error while subscribing"))?;
                }
            }
        },
        Err(e) => {
            println!("Error in formatting: {:?}", e);
            response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Malformed json request"))?;
        }
    }

    Ok(response)
}

pub async fn channel_address_response(
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let response;

    let author = author.lock().unwrap();
    match author.get_channel_address() {
        Ok(channel_address) => {
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Channel Address: ".to_owned() + &channel_address))?;
        },
        Err(_e) => {
            println!(
                "GET /get_channel_address Error: Failure to get channel address"
            );
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while getting channel address"))?;
        }
    }

    Ok(response)
}

pub async fn announcement_id_response(
    author: Arc<Mutex<ChannelAuthor>>,
) -> Result<Response<Body>, GenericError> {
    let response;

    let author = author.lock().unwrap();
    match author.get_announcement_id() {
        Ok(announcement_id) => {
            response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Announcement Id: ".to_owned() + &announcement_id.0 +
                    ":" + &announcement_id.1.to_owned()))?;
        },
        Err(_e) => {
            println!(
                "GET /get_channel_address Error: Failure to get announcement id"
            );
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while getting announcement id"))?;
        }
    }

    Ok(response)
}

pub async fn readings_response(
    req: Request<Body>,
    reading_store: Arc<Mutex<ReadingStore>>
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let sensor_id: serde_json::Result<ReadingStoreFilterId> = serde_json::from_slice(&data);
    match sensor_id {
        Ok(sensor_id) => {
            let mut reading_store = reading_store.lock().unwrap();
            let sensor_id = SensorId(sensor_id.get_sensor_id());
            let readings = reading_store.get(&sensor_id);

            if readings.is_ok() {
                response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(readings.unwrap())?))?;
            } else {
                println!(
                    "GET /get_readings Error: Failed to fetch readings."
                );
                response = Response::builder()
                    .status(500)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("Error while fetching readings for ".to_owned() +
                        sensor_id.0.as_str()))?;
            }
        },

        Err(_e) => {
            println!(
                "GET /get_readings Error: Failed to read sensor id. Make sure it is formatted correctly"
            );
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while reading sensor_id field"))?;
        }
}

    Ok(response)
}

pub async fn confidence_score_response(
    req: Request<Body>,
    annotation_store: Arc<Mutex<AnnotationStore>>
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let annotation_id: serde_json::Result<AnnotationStoreFilterId> = serde_json::from_slice(&data);
    match annotation_id {
        Ok(annotation_id) => {
            let mut annotation_store = annotation_store.lock().unwrap();
            let reading_id = ReadingId(annotation_id.get_reading_id());
            let annotations = annotation_store.get(&reading_id);

            if annotations.is_ok() {
                let mut confidence_score = 0 as f64;
                annotations.unwrap()
                    .iter()
                    .map(|ann| {
                        confidence_score += ann.get_confidence_score()
                    })
                    .for_each(drop);

                response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("confidence_score: ".to_owned() +
                        confidence_score.to_string().as_str()))?;
            } else {
                println!(
                    "GET /get_confidence_score Error: Failed to fetch confidence score."
                );
                response = Response::builder()
                    .status(500)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("Error while fetching score for ".to_owned() +
                        reading_id.0.as_str()))?;
            }
        },

        Err(_e) => {
            println!(
                "GET /get_confidence_score Error: Failed to read reading id. Make sure it is formatted correctly"
            );
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while reading reading_id field"))?;
        }
    }

    Ok(response)
}

pub async fn annotations_response(
    req: Request<Body>,
    annotation_store: Arc<Mutex<AnnotationStore>>
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let annotation_id: serde_json::Result<AnnotationStoreFilterId> = serde_json::from_slice(&data);
    match annotation_id {
        Ok(annotation_id) => {
            let mut annotation_store = annotation_store.lock().unwrap();
            let reading_id = ReadingId(annotation_id.get_reading_id());
            let annotations = annotation_store.get(&reading_id);

            if annotations.is_ok() {
                response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_vec(annotations.unwrap())?))?;
            } else {
                println!(
                    "GET /get_annotations Error: Failed to fetch annotations."
                );
                response = Response::builder()
                    .status(500)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("Error while fetching annotations for ".to_owned() +
                        reading_id.0.as_str()))?;
            }
        },

        Err(_e) => {
            println!(
                "GET /get_annotations Error: Failed to read reading id. Make sure it is formatted correctly"
            );
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while reading reading_id field"))?;
        }
    }

    Ok(response)
}

pub async fn filter_annotations_response(
    req: Request<Body>,
    annotation_store: Arc<Mutex<AnnotationStore>>
) -> Result<Response<Body>, GenericError> {
    let data = hyper::body::to_bytes(req.into_body()).await?;

    let response;
    let filters: serde_json::Result<AnnotationStoreFilter> = serde_json::from_slice(&data);

    match filters {
        Ok(filters) => {
            let mut annotation_store = annotation_store.lock().unwrap();
            let mut annotations = Vec::new();

            let mapped = annotation_store.iter().map(|ann_iter| {
                ann_iter.for_each(|(_id, ann_vec)| {
                        annotations.extend(ann_vec.into_iter()
                            .filter(|ann| filters.iss.is_some() &&
                                &ann.annotation.payload.iss == filters.iss.as_ref().unwrap()));
                    annotations.extend(ann_vec.into_iter()
                        .filter(|ann| filters.ann.is_some() &&
                                &ann.annotation.payload.ann == filters.ann.as_ref().unwrap()));
                    annotations.extend(ann_vec.into_iter()
                            .filter(|ann| filters.iat.is_some() &&
                                &ann.annotation.payload.iat == filters.iat.as_ref().unwrap()));
                    annotations.extend(ann_vec.into_iter()
                            .filter(|ann| filters.jti.is_some() &&
                                &ann.annotation.payload.jti == filters.jti.as_ref().unwrap()));
                    annotations.extend(ann_vec.into_iter()
                            .filter(|ann| filters.sub.is_some() &&
                                    &ann.annotation.payload.sub == filters.sub.as_ref().unwrap()));
                })
            });

            if mapped.is_ok() {
                response = Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_vec(&annotations)?))?;
            } else {
                response = Response::builder()
                    .status(500)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("Error while mapping annotations"))?;
            }
        },

        Err(_) => {
            response = Response::builder()
                .status(500)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("Error while reading filters fields"))?;
        }
    }

    Ok(response)
}
