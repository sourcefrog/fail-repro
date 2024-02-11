use fail::fail_point;

fn main() {
    println!("Has failpoints: {}", fail::has_failpoints());
    println!(
        "FAILPOINTS is {:?}",
        std::env::var("FAILPOINTS").unwrap_or_default()
    );
    fail_point!("main");
    println!("Failpoint passed");
}
