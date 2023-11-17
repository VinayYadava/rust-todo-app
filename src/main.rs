#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}



#[launch]

fn rocket() -> _ {
    rocket::build().configure(rocket::Config::figment().merge(("port", 10000))).mount("/",routes![index])
}