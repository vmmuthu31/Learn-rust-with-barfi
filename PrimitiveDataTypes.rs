// -------------------------------------------------
// 			- Scalar Data Types
//	 			- Integers
// 				- Floats
// 				- Chars
// 				- Boolean
// -------------------------------------------------
fn main() {
    // Unsigned integers
    let unsigned_num: u8 = 5; 

    // Signed integers
    let signed_num: i8 = 5; 

    // Floating point numbers
    let float_num: f32 = 5.0; 

    // Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    // Characters
    let char = 'a';

    // Boolean
    let b: bool = true;

    // Type aliasing
    type Age = u8; 
    let peter_age: Age = 42;

    // Type Conversion
    let a = 10; 
    let b = a as f64;
}