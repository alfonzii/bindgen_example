use bindgen_example::{add, subtract};

fn main() {
    unsafe {
        let sum = add(5, 3);
        let difference = subtract(5, 3);

        println!("Sum: {}", sum);
        println!("Difference: {}", difference);
    }
}
