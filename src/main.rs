// importing a module
// mod guess_2;

use std::io;

fn main() {
    // mut is for mutable variables, variables are immutable by default
    // let var_name = 5;
    // let x = var_name;

    // let x = x + 5;

    // {
    //     // shadowing in scoped
    //     let x = x * 2;
    //     println!("the value of x in the shadowing scoped is: {x}");
    // }

    // println!("the value of x outside is :{x}");

    // &str is used for string literals
    // let string_value = "ninja";

    // println!("the value of above string is  {string_value}");

    // println!("the value of string is {}", string_value);
    // guess_2::guessing_game_2();

    // DECIMAL

    // let x = 22.3;
    // let y: f32 = 53.3;

    // BOOL
    // // if you want to add any expressions in println then you have to do it outside of "",
    // println!("addition of x and y is :{}", x+y);

    // let t = true;

    // println!("the value of t is: {}",t);

    // TUPLE

    // it is just like objects but tuples are bit different
    // let tup = (500,6.4,1);

    // let destructed = tup.1;

    // println!("the value for tup y is {destructed}");

    // Arrays
    // let ninja = ['n','i','n','j','a'];

    //  just like js array, can be accessed using array indexes
    // println!("The value of ninja is : {}", ninja[3]);

    // // this will create an array of 5 elements and each element will be 3
// let a = [1;100]; // using this ; in the array itself we can create a range of any array
// the first part before ; is the item which you want to
    // println!("the value for a is: {}",a[99]);

    // rust prevents you from accessing an element outside of the array
    // let a: [i32; 5] = [1,2,3,4,5];

    // println!("enter the index to access the element");

    // let mut index = String::new();

    // io::stdin().read_line(&mut index).expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    // another_function(5)
    // access_index()
    // check_not_so_important_stuff();
    for_your_i_only()
}
fn access_index() {

    let array: [i32; 5] = [1,2,3,4,5];

    println!("Enter the index of the number you want to acces");

    let mut index  = String::new();

    io::stdin().read_line(&mut index).expect("failed getting the number");

    let index: usize = index.trim().parse().expect("Enter a valid number");

    let element = array[index];

    println!("the value at {index} is {element}");

}
fn array_and_tuple() {
    // basically tuples are like arrays in rust fixed in size can't be increased, can be used to store data of more than one type
    // arrays are also of fixed length, 
    let tup = (1,2,3);
    let arr: [i32;2] = [1;2] ;// creates array of len 2
    let tup = tup.1;
    let arr = arr[1];
    println!("the value for tuple is: {tup} and array : {arr}");
 }

 fn inline_control_flow(){
    let condition = true;
    let value = if condition { 2 } else { 0}; // return type of if and else should be same
    println!(
        "the value is : {value}"
    )
 }


 fn loops_and_shit(){

    let mut counter = 0;
    // labelling loop
   'ninja_loop: loop {
    counter +=1;
        if counter > 10
        {
            break 'ninja_loop counter;
        }
        else {
            println!("Nishant is cool");

        }
    };
    println!("result -> {counter}");
 }



 fn for_your_i_only() {
   
   for number in (1..5).rev() {
        println!("{number}");
   }
   println!("LIFT OFF!");

 }
 fn add_num(value: i32) -> i32 {
    value + 1
 }

fn check_not_so_important_stuff() {
   // basically tuples are like arrays in rust fixed in size can't be increased, can be used to store data of more than one type
   // arrays are also of fixed length, 
   let tup = (1,2,3);
   let arr: [i32;2] = [1;2] ;// creates array of len 2
   let tup = tup.1;
   let arr = arr[1];
   println!("the value for tuple is: {tup} and array : {arr}");
}

// fn another_function(x: i32) {

//     let new_value = add_two(x, 2);
//     println!("the value of new value is: {}",new_value);

//     let y = {
//         let x = 3;
//         new_value + x
//     };

//     if y >= 10 {
//         println!("the value of y is greater than 10");
//     }
//     else if y == 5 {
//         println!("the value of y is equal to 5");
//     }
//     else {
//         println!("the value of y is less than 10");
//     }

//     let condition = true;

//     // conditional assignment
//     let assigned_new = if condition { "new"} else { "old"};

//     println!("assigned new : {assigned_new}");

//     // loop
//     let mut counter = 0;
//     loop {
//         counter += 1;
//         println!("the value of counter was: {}", counter);

//     }
//     println!("the final value is : {}", counter);

// }
// /**
//  * @param x
//  * @return i32
//  */
// fn add_two (x:i32,other_value: i32) -> i32{
//      x+other_value
// }
