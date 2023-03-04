fn main() {

//Variables and Mutability: mutable and immutable
    // let mut a = 5;
    // println!("The value of a is: {a}");
    // a = 6;
    // println!("The value of a is: {a}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // print!("{THREE_HOURS_IN_SECONDS}");

//Shadowing is different from mut
    let x = 5;
    let x = x + 1;
// Inner scope
    {
        let x = x * 2;
        println!("Current value of x is: {x}");
    }

    println!("The value of x is: {x}");

}
