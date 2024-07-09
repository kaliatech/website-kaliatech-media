use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FfprobeMeta {
    pub streams: Vec<FfprobeStream>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FfprobeStream {
    pub codec_name: String,
    pub width: u32,
    pub height: u32,
    pub duration: String,
    pub duration_ts: u32,
}

impl FfprobeStream {
    pub fn get_duration(&self) -> Result<f64, std::num::ParseFloatError> {
        self.duration.parse::<f64>()
    }
}
