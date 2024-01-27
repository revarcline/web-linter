#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "POST: /to_yaml - translate json to yaml\nPOST: /to_json - translate yaml to json"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
