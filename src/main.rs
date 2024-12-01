use bindgen_example::hello;

fn main() {
    unsafe {
        // let sum = add(5, 3);
        // let difference = subtract(5, 3);

        // println!("Sum: {}", sum);
        // println!("Difference: {}", difference);
        let h = hello();
        println!("Hello: {}", h);
    }
}
