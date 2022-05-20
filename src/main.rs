mod encode;
mod decode;
mod helpers;
mod structs;
mod constants;
mod types;

fn main() {
    // encode::exec();
    println!("{:?}", helpers::calulate_diff(255, 0, 2));
    println!("{:?}", helpers::calulate_diff(255, 1, 2));
    println!("{:?}", helpers::calulate_diff(255, 2, 2));
    println!("{:?}", helpers::calulate_diff(254, 2, 2));
    println!("{:?}", helpers::calulate_diff(0, 0, 2));
    println!("{:?}", helpers::calulate_diff(0, 1, 2));
    println!("{:?}", helpers::calulate_diff(0, 2, 2));
    println!("{:?}", helpers::calulate_diff(1, 2, 2));
    println!("{:?}", helpers::calulate_diff(1, 4, 2));
    println!("{:?}", helpers::calulate_diff(0, 32, 32));
    println!("{:?}", helpers::calulate_diff(0, 33, 32));
    println!("{:?}", helpers::calulate_diff(33, 32, 32));
    println!("{:?}", helpers::calulate_diff(64, 32, 32));
}
