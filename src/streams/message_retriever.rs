use anyhow::Result;
use crate::streams::ChannelAuthor;
use crate::store::{AnnotationStore, ReadingStore};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration
};



pub struct MessageRetriever {
    author: Arc<Mutex<ChannelAuthor>>,
    annotation_store: Arc<Mutex<AnnotationStore>>,
    reading_store: Arc<Mutex<ReadingStore>>,
}

impl MessageRetriever {
    pub fn new(
        author: Arc<Mutex<ChannelAuthor>>,
        annotation_store: Arc<Mutex<AnnotationStore>>,
        reading_store: Arc<Mutex<ReadingStore>>,
    ) -> Self {
        MessageRetriever {
            author,
            annotation_store,
            reading_store
        }
    }

    pub fn start(retriever: Self) -> Result<()> {
        println!("Retrieval thread spawning. Searching for new messages...");
        thread::spawn(move || {
            loop {
                Self::handle_messages(&retriever);
                thread::sleep(Duration::from_millis(100))
            }
        });
        Ok(())
    }

    fn handle_messages(&self) {
        //TODO: Handle all panics here
        let mut author = self.author.lock().unwrap();
        let msgs = author.get_next_msgs().unwrap();

        for (reading, annotation) in msgs {
            println!("Got a new {} message", if reading.is_some() {"reading"} else {"annotation"});

            if reading.is_some() {
                let reading = reading.unwrap();
                let sensor_id = reading.get_sensor_id().clone();
                println!("Storing reading: {}", serde_json::to_string(&reading).unwrap());
                let mut reading_store = self.reading_store.lock().unwrap();
                reading_store.insert(&sensor_id, reading).unwrap();
                println!("Stored\n");
            } else {
                let annotation = annotation.unwrap();
                let reading_id = annotation.get_reading_id().clone();
                println!("Storing annotation: {}", serde_json::to_string(&annotation).unwrap());
                let mut annotation_store = self.annotation_store.lock().unwrap();
                annotation_store.insert(&reading_id, annotation).unwrap();
                println!("Stored\n");
            }
        }
    }

}