pub mod annotation_store;
pub mod reading_store;

pub use annotation_store::*;
pub use reading_store::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReadingStoreFilterId {
    sensor_id: String
}

impl ReadingStoreFilterId {
    pub fn get_sensor_id(&self) -> String {
        self.sensor_id.clone()
    }

}

#[derive(Debug, Deserialize)]
pub struct AnnotationStoreFilterId {
    reading_id: String
}

impl AnnotationStoreFilterId {
    pub fn get_reading_id(&self) -> String {
        self.reading_id.clone()
    }

}

