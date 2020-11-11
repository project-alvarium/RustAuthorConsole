use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlvariumHeader {
    alg: String,
    typ: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlvariumAnnotationPayload {
    pub iss: String,
    pub sub: String,
    pub iat: String,
    pub jti: String,
    pub ann: String,
    pub avl: f64
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlvariumSignature(String);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AlvariumAnnotation {
    pub header : AlvariumHeader,
    pub payload : AlvariumAnnotationPayload,
    pub signature : AlvariumSignature
}
