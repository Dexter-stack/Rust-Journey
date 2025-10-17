// Ownership Rules

// 1. Each value in Rust has a variable that is called it's owner.
// 2. There can be only one owner at a time
// 3. When the owner goes out of scope , the value will be dropped

fn main(){


    {
    let mut s  = String::from("Hello");

    s.push_str(", world");
    s.push_str(", this is Dexter");

    /*   Instead of trying to copy the allocated memory, Rust 
considers s1 to no longer be valid and, therefore, Rust doesn’t need to free 
anything when s1 goes out of scope.*/
    let s2 = s;

    println!("{}", s2);
    }


    // shallow and deep copy 
    /*  shallow copy : the concept of copying the pointer , lenght and capacity without copying the data probably But because Rust also invalidates the 
f
 irst variable, instead of being called 
a shallow copy, it’s known as a MOVE. */


//Deep copy

{

let s1 = String::from("Hello");
let s2 =  s1.clone();

println!("s1 = {}, s2 = {}", s1,s2);

}

{
// Stack-Only copy

let x = 5;
let y = x;

println!("x = {}, y = {}",x, y );

let my_str = String::from("Testing Hello world");
takes_ownership(my_str);

let x = 7;

makes_copy(x);
}
    


// Ownership and Functions

fn takes_ownership(some_string : String){ // some_string comes into scope

    println!("{}", some_string);
}// some_string goes out of scope and 'drop' is called . the backing 
// memory is cleared

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
} 






}