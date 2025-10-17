//  References and Borrowing

// & this is use for rferencing a value without the ownership


fn main(){


    let s1  = String::from("Hello world");

    let len = calculate_length(&s1);

    println!("The length of '{}' is  {}.", s1, len);

    let mut my_string = String::from("Hello");
    change(&mut my_string);
    {
    let r1 = &mut my_string;
    println!("{}",r1);

    }

    {
 let r2 = &mut my_string;
  println!("{}",r2);
    }

// Rule 1: You can’t have multiple mutable references:
// let r1 = &mut s;
// let r2 = &mut s; // ❌ error

// Rule 2: You can’t mix mutable & immutable borrows:
// let mut s = String::from("hello");

// let r1 = &s; // immutable
// let r2 = &mut s; // ❌ error

// ✅ Either:

// Many immutable references (&T)
// OR

// Exactly one mutable reference (&mut T)

// But NOT both at the same time.

   
    

   




}



fn calculate_length(s: &String) -> usize{

    s.len()
}


// We call having references as function parameters "Borrowing"
// we cannot modify something we have a reference to 


fn change(some_string: &mut String){
    some_string.push_str(", world");
}