#![feature(proc_macro_hygiene, decl_macro)]
extern crate time;
extern crate travelling_salesman;
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use std::collections::HashMap;


#[post("/", format = "json", data = "<map>")]
fn hello(map: Json<HashMap<i32, (f64, f64)>>) -> String { 
    
    // let coords: Vec<(f64, f64)> = (*map.values().cloned().collect::<Vec<(f64, f64)>>()).to_vec();
  
    let tour = travelling_salesman::simulated_annealing::solve(
        &(*map.values().cloned().collect::<Vec<(f64, f64)>>()).to_vec(),
        time::Duration::seconds(5),
      );

    for (key, value) in &*map {
        println!("{}: {:?}", key, value);
    }
    
    format!("Tour distance: {}, route: {:?}", tour.distance, tour.route)
}

// #[get("/<base64>")]
// fn stippling(base64: String) -> String{ 
//     format!("The base 64 string was accepted {}", base64.len())
// }


// struct Point { 
//     x: f64, 
//     y: f64
// }

fn main() {
  rocket::ignite().mount("/", routes![stippling]).launch();
}
