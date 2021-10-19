use my_rust_app::parse_args;

fn main() {
    env_logger::init();
    match parse_args() {
        Err(e) => {println!("operation failed - {}", e); std::process::exit(1); },
        _ => {redis::cmd("SAVE");},
    }

    
}
