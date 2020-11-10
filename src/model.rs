use std::num::NonZeroU16;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BodyReq {
    pub denom: NonZeroU16
}