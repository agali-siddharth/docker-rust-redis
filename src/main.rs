use my_rust_app::parse_args;

fn main() {
    match parse_args() {
        Err(e) => {println!("operation failed - {}", e); std::process::exit(1); },
        _ => {redis::cmd("SAVE");},
    }

    
}
