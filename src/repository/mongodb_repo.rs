/// This file does the following:
///   - Imports the required dependencies
///   - Creates a `MongoRepo` struct with a `col` field to access MongoDB collection
///   - Creates an implementation block that adds methods to the `MongoRepo` struct
///   - Adds an `init` method to the implementation block to load the environment 
///     variable, creates a connection to the database, and returns an instance of the
///     `MongoRepo` struct.
///   - Adds a `create_user` method that takes in a `self` and `new_user` as parameters
///     and returns the created user or an error. Inside the method, we created a new
///     document using the `User` struct. Then we use the `self` referencing the 
///     `MongoRepo` struct to access the `insert_one` function from the collection to
///     create a new user and handle errors. Finally, we returned the created user information.
/// 
use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    /// Does the following:
    ///   - Adds an `init` method to the implementation block to load the environment 
    ///     variable, creates a connection to the database, and returns an instance of the
    ///     `MongoRepo` struct.
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    /// Does the following:
    ///   - Adds a `create_user` method that takes in a `self` and `new_user` as parameters
    ///     and returns the created user or an error. Inside the method, we created a new
    ///     document using the `User` struct. Then we use the `self` referencing the 
    ///     `MongoRepo` struct to access the `insert_one` function from the collection to
    ///     create a new user and handle errors. Finally, we returned the created user information.
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    /// Does the following:
    ///   - Modifies the dependencies to include `oid::ObjectId` and `doc`
    ///   - Adds a `get_user` method that takes in a `self` and `id` as parameters and returns the user detail or
    ///     an error. Inside the method, we converted the `id` to an `ObjectId` and used it as a filter to get 
    ///     matching documents. Then we use the `self` referencing the `MongoRepo` struct to access the `find_one`
    ///     function from the collection to get the details of the user and handle errors. Fianlly, we returned the
    ///     created user information.
    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let user_detail = self 
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    /// Does the following:
    ///   - Modifies the dependencies to include `UpdateResult`
    ///   - Adds an `update_user` method that takes in a `self`, `id`, and `new_user` parameters and returns the
    ///     updated user detail or an error. Inside the method, we converted the `id` to an `ObjectId`, created a
    ///     `filter` variable to get the matching document we wanted to update and used macro to update the document
    ///     fields. Then we use the `self` referencing the `MongoRepo` struct to access the `update_one` function 
    ///     from the collection to update the user matching the `filter` specified and handle errors. Finally, we
    ///     returned the updated user information. 
    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let new_doc = doc!{
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title":new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    /// Does the following:
    ///   - Modifies the dependencies to include DeleteResult
    ///   - Adds a `delete_user` method that takes in a `self` and `id` as parameters and returns the deleted user
    ///     detail or an error. Inside the method, we converted the `id` to an `ObjectId` and created a `filter` 
    ///     variable to get the matching document we wanted to delete. Then we use the `self` referencing the
    ///     MongoRepo struct to access the `delete_one` funciton from the collection to delete the user matching
    ///     the `filter` specified and handle errors. Finally, we returned the deleted user information.
    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    /// The snippet below adds a `get_all_users` method that takes in a `self` as a parameter and returns the
    /// list of users or an error. Inside the method, we use the `self` referencing the `MongoRepo` struct to
    /// access the `find` function from the collection without any filter so that is can match all the documents
    /// inside the database and handle errors. Finally, we returned the list of users.
    /// 
    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}