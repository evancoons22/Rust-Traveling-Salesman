#![feature(proc_macro_hygiene, decl_macro)]
extern crate time;
extern crate travelling_salesman;
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};


#[post("/", format = "json", data = "<points>")]
fn tsp(points: Json<Vec<f64>>) -> Json<Vec<usize>> { 
    
    let newpoints = points.into_inner();
    let mut coords = Vec::new();
    let len = newpoints.len() / 2;

    for i in 0..len { 
      coords.push((newpoints[i * 2], newpoints[i * 2 + 1]));
    }


   let tour = travelling_salesman::hill_climbing::solve(&coords[..], time::Duration::seconds(4));
   let response = tour.route;
   println!("{:?}", response);
  
    Json(response)
}

fn main() {
  let cors = CorsOptions::default()
      .allowed_origins(AllowedOrigins::all())
      .allowed_methods(
          vec![Method::Get, Method::Post]
              .into_iter()
              .map(From::from)
              .collect(),
      )
      .allow_credentials(true);
  rocket::ignite().mount("/tsp", routes![tsp]).attach(cors.to_cors().unwrap()).launch();
}
