use crate::models::{ReadingId, AlvariumAnnotation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Annotation {
    pub reading_id: ReadingId,
    pub annotation: AlvariumAnnotation,
}

impl Annotation {
    pub fn new() -> Self {
        Annotation {
            reading_id: ReadingId::default(),
            annotation: AlvariumAnnotation::default(),
        }
    }

    pub fn with_reading_id(mut self, id: ReadingId) -> Self {
        self.reading_id = id;
        self
    }

    pub fn with_annotation(mut self, annotation: AlvariumAnnotation) -> Self {
        self.annotation = annotation;
        self
    }

    pub fn get_reading_id(&self) -> &ReadingId {
        &self.reading_id
    }

    pub fn get_annotation(&self) -> &AlvariumAnnotation {
        &self.annotation
    }

    pub fn get_confidence_score(&self) -> f64 {
        self.annotation.payload.avl
    }
}