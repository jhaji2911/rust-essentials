
// importing a module
// mod guess_game;

// use std::io;

fn main(){
    // mut is for mutable variables, variables are immutable by default
//   let x  = 5;

//   let x = x+ 5;

//   {
//     // shadowing in scoped 
//     let x = x * 2;
//     println!("the value of x is: {x}");
//   }

//   println!("using shadowing the value of x is: {x}");


// &str is used for string literals 
// let string_value = "ninja";

// println!("the value of string is {}", string_value);
// guess_game::guessing_game();

// DECIMAL

// let x = 22.3;
// let y: f32 = 53.3;


// BOOL
// // if you want to add any expressions in println then you have to do it outside of "",
// println!("addition of x and y is :{}", x+y);

// let t = true;

// println!("the value of t is: {}",t);


// TUPLE

// let tup = (500,6.4,1);

// let destructed = tup.0;

// println!("the value for tup y is {destructed}")


// Arrays 
// let ninja = ['n','i','n','j','a'];

// println!("The value of ninja is : {}", ninja[0]);

// // this will create an array of 5 elements and each element will be 3
// let a = [3; 5];

// println!("the value for a is: {}",a[1]);


// rust prevents you from accessing an element outside of the array
// let a: [i32; 5] = [1,2,3,4,5];

// println!("enter the index to access the element");

// let mut index = String::new();

// io::stdin().read_line(&mut index).expect("Failed to read line");

// let index: usize = index.trim().parse().expect("Index entered was not a number");

// let element = a[index];

// println!("The value of the element at index {index} is: {element}");

another_function(5)

}

fn another_function(x: i32) {

    let new_value = add_two(x, 2);
    println!("the value of new value is: {}",new_value);

    let y = {
        let x = 3;
        new_value + x
    };

    if y >= 10 {
        println!("the value of y is greater than 10");
    }
    else if y == 5 {
        println!("the value of y is equal to 5");
    }
    else {
        println!("the value of y is less than 10");
    }

    let condition = true;

    // conditional assignment
    let assigned_new = if condition { "new"} else { "old"};

    println!("assigned new : {assigned_new}");

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("the value of counter was: {}", counter);

    }
    println!("the final value is : {}", counter);

}
/**
 * @param x
 * @return i32
 */
fn add_two (x:i32,other_value: i32) -> i32{
     x+other_value
}