use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("candy12t")
        .about("Rust echo")
        .get_matches();
}
