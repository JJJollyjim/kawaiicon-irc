use lambda_http::http::{Response, StatusCode};
use lambda_http::{lambda, Body, Request};
use lambda_runtime::{error::HandlerError, Context};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;

mod cal;
mod parser;
mod parser_chcon;

lazy_static! {
    static ref CACHE: Mutex<Option<(String, Instant)>> = Mutex::new(None);
}

fn get_fresh() -> Result<String, HandlerError> {
    let response = reqwest::get("https://2020.chcon.nz/schedule/")
        .or(Err("Couldn't get schedule"))?
        .text()
        .or(Err("Couldn't read schedule"))?;

    let events = parser_chcon::parse(&response);

    let cal = cal::make_cal(events.into_iter());
    Ok(std::str::from_utf8(&cal)?.to_string())
}

fn get_cached() -> Result<String, HandlerError> {
    let mut cacheref = CACHE.lock().expect("Mutex poisoned");

    match &*cacheref {
        None => {
            println!("Cache is empty, refreshing");
            let succ = get_fresh()?;
            *cacheref = Some((succ.clone(), Instant::now()));
            Ok(succ)
        }
        Some((val, time)) if time.elapsed() > Duration::new(60 * 5, 0) => {
            println!("Cache is stale, refreshing");
            match get_fresh() {
                Ok(succ) => {
                    *cacheref = Some((succ.clone(), Instant::now()));
                    Ok(succ)
                }
                Err(e) => {
                    println!("Refresh failed, serving stale cache: {}", e);
                    Ok(val.clone())
                }
            }
        }
        Some((val, _)) => Ok(val.clone()),
    }
}

fn main() {
    eprintln!("Main");
    lambda!(handler)
}

fn handler(_req: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let cal = get_cached()?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/calendar")
        .body(Body::Text(cal))
        .unwrap())
}
