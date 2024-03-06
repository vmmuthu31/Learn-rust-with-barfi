// -------------------------------------------------
// 			- Compound Data Types
//	 			- &str and String
// 				- Arrays
// 				- Vectors
// 				- Tuples
// 				- Empty Tuple
// -------------------------------------------------

fn main() {
    // &str and String
    let fixed_str = "Fixed length string";
    let mut flexible_str = String::from("This string will grow");
    flexible_str.push('s');

    // Arrays
    let mut array_1 = [4, 5, 6, 8, 9];
    let num = array_1[3];

    println!("{:?}", array_1);
    let array_2 = [0; 10];

    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9];
    let num = vec_1[3];

    // Tuples
    let my_info = ("Salary", 40000, "Age", 40);
    let salary_value = my_info.1;
    let (salary, salary_value, age, age_value) = my_info;

    let unit = ();
}