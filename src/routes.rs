extern crate serde_json;
extern crate clap;

use rocket::State;
use rocket::response::{Responder, Response};
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response;
use rocket_contrib::json::{Json, JsonValue};
use crate::store::TinStore;
use crate::manager::TinQueueManager;


#[derive(Deserialize)]
pub struct Element {
    pub value: String,
    pub expiration: i64,
}

#[derive(Deserialize)]
pub struct QueueElement {
    pub value: String,
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

#[get("/", format = "application/json")]
pub fn home() -> ApiResponse {
    ApiResponse {
        result: json!({"result": clap::crate_version!()}),
        status: Status::Ok,
    }
}


#[get("/store/get/<key>", format = "application/json")]
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

#[post("/store/set/<key>", format = "application/json", data = "<body>")]
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

#[post("/store/setexp/<key>", format = "application/json", data = "<body>")]
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

#[delete("/store/delete/<key>")]
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

#[get("/queues/<queue_name>")]
pub fn get_queue(queue_name: String, queue_manager: State<TinQueueManager>) -> ApiResponse {
    if let Some(queue) = queue_manager.get_queue(queue_name.clone()) {
        ApiResponse {
            result: json!({"result": {"name": queue_name, "len": queue.get_len(), "capacity": queue.get_capacity(), "empty": queue.is_empty()}}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Queue not found."}),
            status: Status::InternalServerError
        }
    }
}

#[delete("/queues/<queue_name>")]
pub fn delete_queue(queue_name: String, queue_manager: State<TinQueueManager>) -> ApiResponse {
    if let Some(_) = queue_manager.get_queue(queue_name.clone()) {
        queue_manager.delete_queue(queue_name);
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Queue not found."}),
            status: Status::InternalServerError
        }
    }
}

#[post("/queues/<queue_name>/create")]
pub fn create_queue(queue_name: String, queue_manager: State<TinQueueManager>) -> ApiResponse {
    if let Some(_) = queue_manager.add_queue(queue_name, 64) {
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Error while creating new queue."}),
            status: Status::InternalServerError
        }
    }
}

#[post("/queues/<queue_name>/push", format = "application/json", data = "<body>")]
pub fn push_to_queue(queue_name: String, body: Json<QueueElement>, queue_manager: State<TinQueueManager>) -> ApiResponse {
    if let Some(queue) = queue_manager.get_queue(queue_name.clone()) {
        let mut updated_queue = queue.clone();
        updated_queue.push(body.value.clone());
        queue_manager.update_queue(queue_name.clone(), updated_queue);
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Queue not found."}),
            status: Status::InternalServerError
        }
    }
}

#[post("/queues/<queue_name>/clear")]
pub fn clear_queue(queue_name: String, queue_manager: State<TinQueueManager>) -> ApiResponse {
    if let Some(queue) = queue_manager.get_queue(queue_name.clone()) {
        let mut cloned_queue = queue.clone();
        cloned_queue.clear_queue();
        queue_manager.update_queue(queue_name, cloned_queue);
        ApiResponse {
            result: json!({"result": "Success."}),
            status: Status::Ok,
        }
    } else {
        ApiResponse {
            result: json!({"result": "Queue not found."}),
            status: Status::InternalServerError
        }
    }
}