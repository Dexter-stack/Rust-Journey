

fn main(){

let _reference_to_nothing  = dangle();





}



fn dangle() -> String{
    let s =  String::from("hello dexter");
    s
}


/*

The Rules of References
 Let’s recap what we’ve discussed about references:
 •	At any given time, you can have either but not both of the following: one 
mutable reference or any number of immutable references.
 •	References must always be valid.
 Next, we’ll look at a different kind of reference: slices.

*/