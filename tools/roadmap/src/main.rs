fn main() {
    match roadmap::commands::run() {
        Ok(code) => {
            if code != 0 {
                std::process::exit(code);
            }
        }
        Err(error) => {
            eprintln!("{error:#}");
            std::process::exit(2);
        }
    }
}
