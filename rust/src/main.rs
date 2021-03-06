extern crate hyper;
extern crate rustc_serialize;

use std::io::{self, Read, Write};
use std::collections::BTreeMap;

use hyper::client::Client;
use hyper::header::ContentType;

use rustc_serialize::json::{self, ToJson, Json};

#[derive(Debug, RustcDecodable)]
struct DecisionResult {
    outputs: BTreeMap<String, VariableValues>,
}

#[derive(Debug, RustcDecodable)]
struct VariableValues {
    values: Vec<String>,
}

struct Inputs {
    season: String,
    guests: i32,
}

impl Inputs {
    fn new(season: String, guests: String) -> Inputs {
        Inputs {
            season: season.trim().to_string(),
            guests: guests.trim().parse::<i32>().expect("unable to convert int"),
        }
    }
}

impl ToJson for Inputs {
    fn to_json(&self) -> Json {
        let mut season = BTreeMap::new();
        season.insert("type".to_string(), "string".to_json());
        season.insert("value".to_string(), self.season.to_json());
        let mut guests = BTreeMap::new();
        guests.insert("type".to_string(), "integer".to_json());
        guests.insert("value".to_string(), self.guests.to_json());
        let mut inputs = BTreeMap::new();
        inputs.insert("season".to_string(), Json::Object(season));
        inputs.insert("guests".to_string(), Json::Object(guests));
        Json::Object(inputs)
    }
}

fn main() {
    print!("What Season is it? (Spring, Summer, Fall, Winter)  ");
    io::stdout().flush().expect("unable to flush stdout");
    let mut season = String::new();
    io::stdin().read_line(&mut season).expect("unable to read stdin");

    print!("How many Guests?  ");
    io::stdout().flush().expect("unable to flush stdout");
    let mut guests = String::new();
    io::stdin().read_line(&mut guests).expect("unable to read stdin");

    let inputs = Inputs::new(season, guests).to_json();

    let mut res = Client::new()
                      .post("https://dmn.camunda.cloud/api/v1/decision/example-dish")
                      .header(ContentType::json())
                      .body(&inputs.to_string())
                      .send()
                      .expect("unable to read response");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("unable to parse reponse");
    let json: DecisionResult = json::decode(&body).expect("unable to transform json");

    let result = &json.outputs["dish"].values[0];
    println!("You should have {}", result);
}
