// Rust has the unit type, () , a type with a single zero-size value. The value of this unit type is also specified using ()
// rust follows snake case
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn var_guide() -> () {
    let width = 4;
    let height = 7;
    let depth = 10;
    println!("Hello, world!");
    println!("Area = {}", get_area(width, height));
    // primitve values so not moved
    println!("Volume is {}", get_volume(depth, width, height));

    let (missiles, ready): (u8, u8) = (STARTING_MISSILES as u8, READY_AMOUNT as u8);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missles left", missiles - ready);
    // READY_AMOUNT = 3; // error: cannot assign twice to immutable variable `READY_AMOUNT`
    ()
}
fn get_volume(depth: u32, height: u32, width: u32) -> u32 {
    depth * height * width
}

fn get_area(width: u32, height: u32) -> u32 {
    // tail expression\
    width * height
}
