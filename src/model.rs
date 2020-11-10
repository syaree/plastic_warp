use std::num::NonZeroU16;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct BodyReq {
    pub denom: NonZeroU16
}

#[derive(Debug, Serialize)]
pub struct BodyResp {
    pub op: String,
    pub result: u16
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String
}
