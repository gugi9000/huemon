extern crate philipshue;
use std::env;
use philipshue::bridge::Bridge;

fn run() -> i32 {
    let bridge = Bridge::new(env::var("huebridge").expect("huebridge env missing"), env::var("hueuser").expect("hueuser env missing"));
    let huebulb_id: usize = env::var("huebulb").expect("huebulb env missing").parse().unwrap();
    match bridge.get_light(huebulb_id) {
        Ok(light) => {
            println!("{} is {}",
                        light.name,
                        if light.state.reachable {"reachable"} else {"unreachable"});
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return 4;
        }
    }
    0
}

fn main() {
    std::process::exit(run())
}
