use actix_web::{HttpResponse, http::StatusCode};
use serde::ser::Serialize;

pub (in crate) trait Response {
    fn get_status(&self) -> u16;

    fn as_body(self) -> actix_http::body::Body;

    fn as_response(self) -> HttpResponse 
    where
        Self: std::marker::Sized
    {
        HttpResponse::build(StatusCode::from_u16(self.get_status()).unwrap()).body(self.as_body())
    }
}

#[derive(Debug, Default, Clone)]
pub (in crate) struct Data<T> 
where
    T: Serialize + Sized
{
    pub status: StatusCode,
    pub data: T,
}

impl<T> Response for Data<T> 
where
    T: Serialize
{
    fn get_status(&self) -> u16 {
        self.status.as_u16()
    }
    
    fn as_body(self) -> actix_http::body::Body {
        self.into()
    }
}

impl<T> std::convert::Into<actix_http::body::Body> for Data<T> 
where
    T: Serialize
{
    fn into(self) -> actix_http::body::Body {
        actix_http::body::Body::Bytes(
            bytes::Bytes::from(serde_json::to_string::<WiredData<T>>(&(self.into())).unwrap_or(String::from("{}")))
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub (in crate) struct WiredData<T> 
where
    T: Serialize
{
    pub status: u16,
    pub data: T,
}

impl<T> std::convert::From<Data<T>> for WiredData<T> 
where
    T: Serialize
{
    fn from(d: Data<T>) -> Self {
        Self {
            status: d.status.as_u16(),
            data: d.data,
        }
    }
}

#[derive(Debug, Default)]
pub (in crate) struct Error {
    pub status: StatusCode,
    pub title: String,
    pub message: String,
    pub trace: Option<String>,
}

impl Response for Error {
    fn get_status(&self) -> u16 {
        self.status.as_u16()
    }
    
    fn as_body(self) -> actix_http::body::Body {
        self.into()
    }
}

impl std::convert::Into<actix_http::body::Body> for Error {
    fn into(self) -> actix_http::body::Body {
        actix_http::body::Body::Bytes(
            bytes::Bytes::from(serde_json::to_string::<WiredError>(&(self.into())).unwrap_or(String::from("{}")))
        )
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
struct WiredInnerError {
    pub title: String,
    pub message: String,
    pub trace: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
struct WiredError {
    pub status: u16,
    pub error: WiredInnerError,
}

impl std::convert::From<Error> for WiredError {
    fn from(e: Error) -> Self {
        Self {
            status: e.status.as_u16(),
            error: WiredInnerError {
                title: e.title,
                message: e.message,
                trace: e.trace,
            },
        }
    }
}