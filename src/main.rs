extern crate philipshue;
use std::env;
use philipshue::bridge::Bridge;

fn run() -> i32 {
    let bridge = Bridge::new(env::var("huebridge").unwrap(), env::var("hueuser").unwrap());

    match bridge.get_light(8) {
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
