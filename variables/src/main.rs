const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x inner scope  is {x}");
    }

    let spaces = "    ";

    println!("spaces I{spaces}I");

    let spaces = spaces.len();

    println!("spaces:  {spaces}");

    println!("The value of x outer scope is {x}");

    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.")
}
