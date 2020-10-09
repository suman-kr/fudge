pub use fudge_lib::{log, Loggers};

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}
#[derive(Debug)]
enum VehiclePrice {
    Car(i32),
    Bike(i32),
}

fn main() {
    let bike_price = VehiclePrice::Bike(30000);
    log(Loggers::Error, "Hello World".to_string()).ok();
    log(Loggers::Warning, "Hello World".to_string()).ok();
    log(Loggers::Info, "Hello World".to_string()).ok();
    log(
        Loggers::Info,
        Student {
            age: 22,
            name: "John".to_string(),
        },
    )
    .ok();
    log(Loggers::Warning, bike_price).ok();
}
