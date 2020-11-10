mod model;

use std::error::Error;
use std::convert::Infallible;
use warp::hyper::StatusCode;
use warp::{Filter, Rejection, Reply};

use crate::model::{BodyReq, BodyResp, ErrorMessage};

#[tokio::main]
async fn main() {
    let routes = warp::post()
        .and(warp::path!("math" / u16))
        .and(warp::body::json())
        .and(warp::body::content_length_limit(1024))
        .map(|nom: u16, payload: BodyReq| {
            warp::reply::json(&BodyResp {
                op: format!("{} / {}", nom, payload.denom),
                result: nom / payload.denom.get()
            })
        }).recover(handle_recovery);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_recovery(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found".to_string()
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>(){
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method is not allowed".to_string()
    } else if let Some(e) = err.find::<warp::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = match e.source() {
            Some(er) => {
                er.to_string()
            }
            None => "Bad Request".to_string()
        }
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error".to_string();
    }

    let json = warp::reply::json(&ErrorMessage { code: code.as_u16(), message });

    Ok(
        warp::reply::with_status(json, code)
    )
}
