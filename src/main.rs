use ocean::config;

fn main() {
    let config = config::Config::new();
    println!("{:?}", config);
    println!("Ocean started");
}
