/// This code does the following:
///  - Imports the required dependencies
///  - Uses the routing macro to specify HTTP method, route, and indicates that the handler expects body data
///  - Creates a `create_user` handler that takes in the `db`, a type to the `MongoRepo` and a new_user as parameters. 
///    Inside the handler, we created a `data` variable for creating a user, inserted it into the database using the 
///    `db.create_user` method, and returned the correct response if the insert was successful or error if any.
///  - Creates a `get_user` handler that takes in the `db`, a type to the `MongoRepo` and a `path` for accessing route 
///    path as parameters. Inside the handler, we created an `id` variable to get the user's `id`, get the user's 
///    details from the database using the `db.get_user` method. We returned the correct response if the request was
///    successful or error if any.

use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

/// Does the following:
///   - Uses the routing macro to specify HTTP method, route, and indicates that the handler expects body data
///   - Creates a `create_user` handler that takes in the `db`, a type to the `MongoRepo` and a `new_user` as
///     parameters. Inside the handler, we created a `data` variable for creating a user, inserted it into the
///     database using the `db.create_user` method, and returned the correct response if the insert was successful or
///     error if any.
#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned() ,
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Does the following:
///   - Uses the routing macro to specify HTTP method, corresponding route and route parameter
///   - Creates a `get_user` handler that takes in the `db`, a type to the `MongoRepo` and a `path` for accessing route 
///     path as parameters. Inside the handler, we created an `id` vriable to get the user's id, get the user's details
///     from the database using the `db.get_user` method. We returned the correct response if the request was successful
///     or error if any.
#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Does the following:
///   - Modifies the dependencies to include `ObjectId`
///   - Uses the routing macro to specify HTTP method, corresponding route, route parameter, and body data
///   - Creates an `update_user` handler that takes in the `db`, a type to the `MongoRepo`, `path`, and new_user
///     as parameters. Inside the handler, we created an `id` variable to get the user's id, update the user's details 
///     from the database using the `db.update_user` method by passing in the updated user's information. Finally, we
///     checked if the update was successful and returned the updated user or error if any.
#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let update_user_info = db.get_user(&id);
                return match update_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Does the following:
///   - Uses the routing macro to specify HTTP method, corresponding route and route parameter
///   - Creates a `delete_user` handler that takes in the `db`, a type to the `MongoRepo` and `path` as parameters.
///     Inside the handler, we created an `id` variable to get the user's id and dleete the user from the database
///     using the `db.delete_user` method by passing in the `id`. Finally, we returned the appropriate response or
///     error if any.
#[delete("/user/<path>")]
pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Does the following:
///   - Uses the routing macro to specify HTTP method and corresponding route
///   - Creates a `get_all_users` handler that uses the `db.delete_user` method to get the list of users. Then, we
///     returned the list of users or error if any.
#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}