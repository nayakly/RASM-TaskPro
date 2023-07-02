#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use mongodb::{
    bson::{doc, Document},
    sync::{Client, Collection},
};

use rocket::{
    http::Status,
    serde::json::Json,
    serde::{Deserialize, Serialize},
    State,
};

#[derive(Debug, Serialize, Deserialize)]
struct UserAddress {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskUpdate {
    address: String,

    #[serde(rename = "taskID")]
    task_id: i32,

    #[serde(rename = "taskName")]
    task_name: Option<String>,

    checked: Option<bool>,
    deleted: Option<bool>,
}

struct AppState {
    col: Collection<Document>,
}

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Todo API written in Rust")))
}

// WARNING: Wipes the entire database to start afresh
#[get("/wipe")]
fn wipe_database(mongo: &State<AppState>) -> Status {
    match mongo.col.delete_many(doc! {}, None) {
        Ok(_) => Status::Ok,
        Err(err) => {
            eprintln!("Failed to wipe the database: {}", err);
            Status::InternalServerError
        }
    }
}

#[post("/read", format = "json", data = "<user_address>")]
async fn retrieve_user(
    user_address: Json<UserAddress>,
    mongo: &State<AppState>,
) -> Result<Json<String>, Status> {
    // Extract the user address from the incoming JSON payload
    let address = user_address.address.clone();

    // Create the query document to find the user
    let query = doc! { "userAddress": address.clone() };

    // Attempt to find the user in the database
    match mongo.col.find_one(query, None) {
        Ok(Some(record)) => {
            // User found, return the user document as a JSON string
            let json_string = record.to_string();
            Ok(Json(json_string))
        }
        Ok(None) => {
            // User not found, create a new user document

            // Create the new user document
            let user_doc = doc! {
                "userAddress": address.clone(),
                "data": Vec::<String>::new(),
            };

            // Insert the new user document into the database
            if mongo.col.insert_one(&user_doc, None).is_err() {
                // Error occurred during the insertion
                return Err(Status::InternalServerError);
            }

            // Return the new user document as a JSON string
            let json_string = user_doc.to_string();
            Ok(Json(json_string))
        }
        Err(_) => {
            // Error occurred during the database operation
            Err(Status::InternalServerError)
        }
    }
}

#[post("/update", format = "json", data = "<update_task>")]
async fn update_task(update_task: Json<TaskUpdate>, mongo: &State<AppState>) -> Status {
    // Extract the fields from the incoming JSON payload
    let address = update_task.address.clone();
    let task_id = update_task.task_id.clone();

    if let Some(task_name) = &update_task.task_name {
        // Update task name

        // Create the filter document to identify the task
        let filter = doc! {
            "userAddress": &address,
            "data.taskID": &task_id
        };

        // Create the update document to set the task name
        let update = doc! {
            "$set": {
                "data.$.taskName": task_name
            }
        };

        // Perform the update operation and check for errors
        if let Err(_) = mongo.col.update_one(filter, update, None) {
            // Error occurred during the update operation
            return Status::InternalServerError;
        }

        // Task name updated successfully
        return Status::Ok;
    }

    if let Some(checked) = update_task.checked {
        // Update checked status

        // Create the filter document to identify the task
        let filter = doc! {
            "userAddress": &address,
            "data.taskID": &task_id
        };

        // Create the update document to set the checked status
        let update = doc! {
            "$set": {
                "data.$.checked": checked
            }
        };

        // Perform the update operation and check for errors
        if let Err(_) = mongo.col.update_one(filter, update, None) {
            // Error occurred during the update operation
            return Status::InternalServerError;
        }

        // Checked status updated successfully
        return Status::Ok;
    }

    if let Some(deleted) = update_task.deleted {
        // Update deleted status

        // Create the filter document to identify the task
        let filter = doc! {
            "userAddress": &address,
            "data.taskID": &task_id
        };

        // Create the update document to set the deleted status
        let update = doc! {
            "$set": {
                "data.$.deleted": deleted
            }
        };

        // Perform the update operation and check for errors
        if let Err(_) = mongo.col.update_one(filter, update, None) {
            // Error occurred during the update operation
            return Status::InternalServerError;
        }

        // Deleted status updated successfully
        return Status::Ok;
    }

    // None of the update fields were present, return BadRequest
    Status::BadRequest
}

#[post("/create", format = "json", data = "<create_task>")]
async fn append_task(create_task: Json<TaskUpdate>, mongo: &State<AppState>) -> Status {
    // Extract the fields from the incoming JSON payload
    let address = create_task.address.clone();
    let task_id = create_task.task_id.clone();
    let task_name = create_task.task_name.clone();
    let checked = create_task.checked.clone();
    let deleted = create_task.deleted.clone();

    // Create the query document to find the user
    let query = doc! { "userAddress": address };

    // Create the new task document
    let new_task = doc! {
        "taskID": task_id,
        "taskName": task_name,
        "checked": checked,
        "deleted": deleted,
        "disabled": false
    };

    // Create the update document to push the new task to the data array
    let update = doc! {
        "$push": {
            "data": new_task
        }
    };

    // Perform the update operation
    if mongo.col.update_one(query.clone(), update, None).is_err() {
        // Return InternalServerError if the update operation fails
        return Status::InternalServerError;
    }

    // Return Ok if the update operation is successful
    Status::Ok
}

#[launch]
async fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve MongoDB URI from environment variable
    let mongodb_uri = std::env::var("MONGODB_URI").expect("Error loading MongoDB URI");

    // Create a MongoDB client and connect to the specified database and collection
    let client = Client::with_uri_str(&mongodb_uri).unwrap();
    let database = client.database("DB1");
    let collection: Collection<Document> = database.collection("todoapp");

    // Create the application state
    let app_state = AppState { col: collection };

    // Build the Rocket application
    rocket::build().manage(app_state).mount(
        "/",
        routes![
            hello,
            wipe_database,
            retrieve_user,
            update_task,
            append_task
        ],
    )
}
