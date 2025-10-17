fn main(){



let mut s = String::from("Hello world");

let word  = first_word(&s);

println!("{}", word);
s.clear();




// String slicing 

let text =  String::from("hello world");

let hello = &text[0..5];
let world =  &text[6..11];

println!("{}, {}", hello, world);


}


fn first_word(s: &String) -> usize{

    let bytes  = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        
        if item == b' '{
            return i;
    }
}
s.len()
}