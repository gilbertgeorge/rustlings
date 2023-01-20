// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.



fn main() {
    println!("Hello {world}!", world="world");
    println!("Hello {}!", "world");
    println!("{1} {0}!", "world", "Hello");
    println!("Base 10:               {}",   69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C

    // named formatted
    println!("{type}{number:>20X}",type="Base 16:", number=69420);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=10);
    println!("{name:>10}", name="George");

    // You can pad numbers with extra zeroes,
    //and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=20);
    println!("{name:.<10}:{otherName}", name="George", otherName="test");

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=30, width=5);

    println!("{}", test="Debug");
    println!("{:?}", test="Debug");


}
