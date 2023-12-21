use warp::{Filter, Reply};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;



#[derive(Debug, Deserialize, Serialize)]
struct Participant {
    _id: Option<ObjectId>, 
    name: Option<String>,
    roll_no: Option<i32>,
    team_name: Option<String>,
    team_id: Option<String>,
    attendance: Option<bool>,
}

async fn handle_request(data: Participant, collection: Collection<Participant>) -> Result<impl Reply, Infallible> {

    println!("Received request data: {:?}", data);

    let participant = collection.find_one(doc! { "roll_no": data.roll_no }, None).await;
    println!("{:?}",&participant);
    match participant {
        Ok(Some(doc)) => {
            // let participant: Participant = bson::from_document(_doc).unwrap();

            println!("Found participant: {:?}", doc);
            Ok(warp::reply::json(&doc))
        },
        Ok(None) => {
            println!("Participant not found");
            Ok(warp::reply::json(&serde_json::json!({ "error": "Participant not found" })))
        },
        Err(e) => {
            println!("Error fetching participant data: {:?}", e);
            Ok(warp::reply::json(&serde_json::json!({ "error": "Error fetching Participant data", "details": e.to_string() })))
        },
    }
}


#[tokio::main]
async fn main() {

    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = client.database("event_name");
    let collection = db.collection::<Participant>("participants");

    let json_body = warp::body::json();

    let routes = warp::post()
        .and(warp::path("participants"))
        .and(json_body)
        .and(warp::any().map(move || collection.clone()))
        .and_then(handle_request);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
