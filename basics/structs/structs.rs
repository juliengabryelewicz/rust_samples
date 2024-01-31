fn main() {
    const MAXIMUM_POWER: u16 = 600;
    #[allow(dead_code)]
    struct Vehicle {
        name: String,
        power: u16,
    }
    let vehicle = Vehicle {
        name: String::from("Car 1"),
        power: 820,
    };
    if vehicle.power > MAXIMUM_POWER {
        println!("Too powerful");
    }
}
