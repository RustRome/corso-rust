fn main() {
    // bool can be true or false
    let bool_t = true;
    let bool_f = false;

    // interger with fixed size in all architectures
    // i8, i16, i32, i64, u8, u16, u32, u64
    // the letter in front of the size ("i" and "u") denotes whether signed (i) or unsigned (u)
    let i_8 = 8_i8;
    // preferred notation
    let i_16: i16 = 32;
    // usize and isize have architecture dependent size;
    let i_size: isize = 8; //signed
    let u_size: usize = 8; //unsigned

    //f32 and f64 are floating point numbers, e.g. decimals
    let f = 0.32;
    let f_32: f32 = 0.32; //force f32 type

    // array is a fixed size collection of same elements, is declared as:
    // let array: [type; N] = [elem, elem, elem, elem];
    let array: [i32; 5] = [0, 1, 2, 3, 4];
    // it could be indexed
    //println!("The first element of the array is {}", array[0]);
    // and you can iterate over it:
    //for e in array.iter() {
    //    println!("{}", e);
    //}

    // slice is dinamically sized, it slice into a collection of elements
    // slice the first three element, second term exclusive
    let slice = &array[0..3];
    //for e in slice {
    //    println!("{}", e);
    //}

    // second term inclusive
    let slice = &array[0..=3];
    //for e in slice {
    //    println!("{}", e);
    //}

    // char supports UTF-32
    let c = 'o';
    let keyboard = '⌨';
    //println!("keyboard: {}", b);

    // str is a "string slice"
    let s = "Hello new Rustcean";
    //println!("s = {}", s);

    // tuple is a etherogeneous sequence, it can contains differets types
    let tuple = ("ciao", 32_i32, [0, 1, 2]);
    //println!("First element of tuple: {}", tuple.0);
    //println!("Second element of tuple: {}", tuple.1);
    //println!("Third element of tuple: {}", tuple.2);

    //for i in &tuple.2 {
    //    println!("i = {}", i);
    //}
}
