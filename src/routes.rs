extern crate serde_json;

use rocket::State;
use rocket::response::{Responder, Response};
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response;
use rocket_contrib::json::{Json, JsonValue};
use crate::store::TinStore;


#[derive(Deserialize)]
pub struct Element {
    pub value: String,
    pub expiration: i64,
}

#[derive(Debug)]
pub struct ApiResponse {
    result: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.result.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}


#[get("/get/<key>", format = "application/json")]
pub fn get(key: String, store: State<TinStore>) -> ApiResponse {
    if let Some(value) = store.get(key) {
        ApiResponse {
            result: json!({"result": value}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Key not found."}),
            status: Status::NotFound
        }
    }
}

#[post("/set/<key>", format = "application/json", data = "<body>")]
pub fn set(key: String, body: Json<Element>, store: State<TinStore>) -> ApiResponse {
    if let Some(_) = store.set(key, body.value.clone()) {
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Error while inserting key/value pair."}),
            status: Status::InternalServerError,
        }
    }
}

#[post("/setexp/<key>", format = "application/json", data = "<body>")]
pub fn set_exp(key: String, body: Json<Element>, store: State<TinStore>) -> ApiResponse {
    if let Some(_) = store.set_exp(key, body.value.clone(), body.expiration.clone()) {
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Error while inserting key/value pair."}),
            status: Status::InternalServerError,
        }
    }
}

#[delete("/delete/<key>")]
pub fn delete(key: String, store: State<TinStore>) -> ApiResponse {
    if let Some(_) = store.get(key.clone()) {
        (*store).delete(key);
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Error while deleting key/value pair."}),
            status: Status::InternalServerError,
        }
    }
}

#[get("/check")]
pub fn check(store: State<TinStore>) -> ApiResponse {
    store.check_expired();
    ApiResponse {
        result: json!({"result": "Done."}),
        status: Status::Ok,
    }
}