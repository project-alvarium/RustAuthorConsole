pub mod reading;
pub mod annotation;
pub mod subscription;
pub mod alvarium;

pub use reading::*;
pub use annotation::*;
pub use subscription::*;
pub use alvarium::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SensorId(pub String);

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ReadingId(pub String);

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Bytes(pub String);
