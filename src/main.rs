#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use rocket::{get, ignite, routes,post};
use rocket::request::Form;
use serde_json::{to_writer, from_reader, to_string};
#[get("/greet")]
fn greet() -> &'static str {
    "Hello, world!"
}



#[post("/saveteam", data = "<team>")]
fn charge(team: String)-> std::io::Result<()>{
    let json_string = to_string("test").unwrap();
    // Écrire la chaîne de caractères JSON dans un fichier
    let mut file = File::create("/home/jcgouleau/Bureau/WASM/rusty-greeter/team.json").unwrap();
    // to_writer(&mut file,  serde_json::to_string_pretty(&json_string)).unwrap();
    std::fs::write(
        "/home/jcgouleau/Bureau/WASM/rusty-greeter/team.json",
        serde_json::to_string_pretty(&team).unwrap(),
    ).expect("TODO: panic message");
    Ok(())
}
fn main() {

    ignite().mount("/", routes![greet,charge]).launch();
}
