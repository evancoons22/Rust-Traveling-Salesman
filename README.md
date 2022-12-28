# Rust API for traveling salesman problem
This rust api was built using rocket and a traveling salesman crate. Works with voronoi stippling frontend. 

### Setup
1. Install Rust. 
2. Clone this repository locally -- it is not available on crates.io. `git clone (ssh git@github .....)`
3. `cargo run` in terminal. 
4. If there are problems, make sure that rust is set to nightly with `rustup default nightly` in terminal. (go back to stable with `rustup default stable`)

### Notes
- This api is specifically to parse data from `Voronoi-Stippling-React` frontend. It was added out of interest to see how computationally difficult the TSP problem was. 
- Credit for solution methods to the TSP goes to [travelling_salesman v1.1.22](https://crates.io/crates/travelling_salesman) on crates.io. The hill climbing method appeared to work best for this case. 
- Required crates include **travelling_salesman**, **time**, **rocket**, and **rocket_cors**
