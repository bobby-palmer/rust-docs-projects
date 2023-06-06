fn main() {
    // making a constant
    const HOURS_IN_DAY: u32 = 24;

    let x = 5;
    println!("The value of x is {x}");
    // shadowing != mutability
    // shadowing is more like transfering the name of a variable
    {
        let x = x * 2;
        println!("the value of x is {x}");
    }
    
}
