use anyhow::{Result, anyhow};
use std::collections::HashMap;
use crate::models::{SensorId, Reading};

pub struct ReadingStore {
    readings: HashMap<SensorId, Vec<Reading>>
}

impl ReadingStore {
    pub fn new() -> Self {
        ReadingStore {
            readings: HashMap::<SensorId, Vec<Reading>>::new()
        }
    }

    pub fn insert(&mut self, sensor_id: &SensorId, reading: Reading) -> Result<()> {
        match self.readings.get_mut(sensor_id) {
            Some(annotations) => Ok(annotations.push(reading)),
            None => {
                self.readings.insert(sensor_id.clone(), vec![reading]);
                Ok(())
            },
        }
    }

    pub fn get(&mut self, sensor_id: &SensorId) -> Result<&Vec<Reading>> {
        match self.readings.get(sensor_id) {
            Some(r) => Ok(r),
            None => {
                Err(anyhow!("Key not present"))
            }

        }

    }
}
