#![feature(proc_macro_hygiene, decl_macro)]
use std::fs::File;
use std::io::Write;
use rocket::{get, ignite, routes,post};
use rocket::request::Form;
use serde_json::{to_writer, from_reader, to_string, from_str, to_string_pretty};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    name: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Teams {
    teams: Vec<Team>,
}




#[get("/greet")]
fn greet() -> &'static str {
    "Hello, world!"
}



#[post("/saveteam", data = "<team>")]
fn charge(team: String)-> std::io::Result<()>{
    let json_string = team.trim_matches('\"').replace("\\", "");
    let teams: Teams = from_str(&json_string).unwrap();
    println!("{:#?}", teams);
    let json = to_string_pretty(&teams).unwrap();
    // let json = to_string(&teams).unwrap();

    // let json_string = r#"{"teams":[{"name":"Gaspard Tame","price":0.0},{"name":"Larry Vière","price":0.0}]}"#;

    let mut file = File::create("/home/jcgouleau/Bureau/WASM/rusty-greeter/team.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();


    // Écrire la chaîne de caractères JSON dans un fichier
    // let mut file = File::create("/home/jcgouleau/Bureau/WASM/rusty-greeter/team.json").unwrap();
    // // to_writer(&mut file,  serde_json::to_string_pretty(&json_string)).unwrap();
    // std::fs::write(
    //     "/home/jcgouleau/Bureau/WASM/rusty-greeter/team.json",
    //     // serde_json::to_string_pretty(&team).unwrap(),
    //     json,
    // ).expect("TODO: panic message");
    Ok(())
}
fn main() {

    ignite().mount("/", routes![greet,charge]).launch();
}
