use crate::models::{SensorId, ReadingId, Bytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reading {
    pub sensor_id: SensorId,
    pub reading_id: ReadingId,
    pub data: Bytes,
}

impl Reading {
    pub fn new() -> Self {
        Reading {
            sensor_id: SensorId::default(),
            reading_id: ReadingId::default(),
            data: Bytes::default(),
        }
    }

    pub fn with_sensor_id(mut self, id: SensorId) -> Self {
        self.sensor_id = id;
        self
    }

    pub fn with_reading_id(mut self, id: ReadingId) -> Self {
        self.reading_id = id;
        self
    }

    pub fn with_data(mut self, data: Bytes) -> Self {
        self.data = data;
        self
    }

    pub fn get_sensor_id(&self) -> &SensorId {
        &self.sensor_id
    }

    pub fn get_reading_id(&self) -> &ReadingId {
        &self.reading_id
    }

    pub fn get_data(&self) -> &Bytes {
        &self.data
    }
}