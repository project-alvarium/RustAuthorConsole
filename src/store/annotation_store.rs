use anyhow::{Result, anyhow};
use crate::models::{ReadingId, Annotation};
use std::collections::{
    hash_map::Iter,
    HashMap
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationStore {
    annotations: HashMap<ReadingId, Vec<Annotation>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationStoreFilter {
    pub iss: Option<String>,
    pub sub: Option<String>,
    pub iat: Option<u64>,
    pub jti: Option<String>,
    pub ann: Option<String>,
}

impl AnnotationStore {
    pub fn new() -> Self {
        AnnotationStore {
            annotations: HashMap::<ReadingId, Vec<Annotation>>::new()
        }
    }

    pub fn insert(&mut self, reading_id: &ReadingId, annotation: Annotation) -> Result<()> {
        match self.annotations.get_mut(reading_id) {
            Some(annotations) => Ok(annotations.push(annotation)),
            None => {
                self.annotations.insert(reading_id.clone(), vec![annotation]);
                Ok(())
            },
        }
    }

    pub fn get(&mut self, reading_id: &ReadingId) -> Result<&Vec<Annotation>> {
        match self.annotations.get(reading_id) {
            Some(a) => Ok(a),
            None => {
                Err(anyhow!("Key not present"))
            }
        }
    }

    pub fn iter(&mut self) -> Result<Iter<ReadingId, Vec<Annotation>>> {
        Ok(self.annotations.iter())
    }
}
