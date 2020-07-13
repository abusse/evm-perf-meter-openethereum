use std::time::Instant;

use crate::measurement::Measurement;

pub struct WallTime;
impl Measurement for WallTime {
    type Intermediate = Instant;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }

    fn end(&self, i: Self::Intermediate) -> serde_json::Number {
        serde_json::Number::from(i.elapsed().as_nanos() as u64)
    }

    fn id(&self) -> &'static str {
        "wall_time"
    }

    fn name(&self) -> &'static str {
        "Wall Time"
    }

    fn unit(&self) -> &'static str {
        "ns"
    }
}
