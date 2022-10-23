use warp::*;
use hyper::Response;
use serde::Serialize;
use serde_json::to_string;

pub struct Json {
  pub body: hyper::Body,
  pub status: u16,
}

impl Reply for Json {
  fn into_response(self) -> reply::Response {
    Response::builder()
      .status(self.status)
      .body(self.body)
      .unwrap()
  }
}

impl Json {
  fn to_body<T: Serialize + Send>(body: T) -> hyper::Body {
    hyper::Body::from(to_string(&body).unwrap())
  }

  pub fn new<T: Serialize + Send>(body: T, status: u16) -> Self {
    Json {
      body: Json::to_body(body),
      status,
    }
  }

  pub fn succ<T: Serialize + Send>(body: T) -> Self {
    Json {
      body: Json::to_body(body),
      status: 200,
    }
  }

  pub fn not_found<T: Serialize + Send>(err: T) -> Self {
    Json {
      body: Json::to_body(err),
      status: 404,
    }
  }
}

pub fn to_json<T: Serialize + Send>(body: Result<T, String>) -> Json {
  body
    .map(|body| Json::succ(body))
    .unwrap_or_else(Json::not_found)
}
