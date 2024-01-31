fn main() {
    #[allow(dead_code)]
    enum Cardinal { North, South, West, East }
    let direction = Cardinal::South;
    match direction {
        Cardinal::North => print!("NORTH"),
        Cardinal::South => print!("SOUTH"),
        Cardinal::East => print!("EAST"),
        Cardinal::West => print!("WEST"),
    }
}
