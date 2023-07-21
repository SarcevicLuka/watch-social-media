use std::str::FromStr;
use actix_web::HttpRequest;
use error::Error;

/// Parse given parameter easily from the request path string
pub fn part_from_path<T: FromStr>(req: &HttpRequest, name: &str) -> Result<T, Error> {
    let value: T = match req.match_info().get(name) {
        Some(val) => match val.parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::Request(format!(
                    "path_attribute_missing:{}",
                    name
                )))
            }
        },
        None => {
            return Err(Error::Request(format!(
                "path_attribute_missing:{}",
                name
            )))
        }
    };

    Ok(value)
}