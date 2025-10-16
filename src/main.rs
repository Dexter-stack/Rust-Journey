fn main() {
    // println!("Hello, world Dexter!");
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x =6;
    // println!("The value of x is: {}", x);





    // shadowing

    // let x  = 5;

    // let x = x+1;

    // let x = x*2;
    // println!("The new value of x is {}",x);

    // let spaces = "   ";
    // let spaces = spaces.len();

    // println!("{}",spaces);


    // Datatype 

    // scalar and compound datatype

    // scalar : integer , floating point numbers, booleans and characters

    let y: f32= 3.0;
    let x  =  2.0;
    let a: u32 = 24;


    println!("{}",x);
    println!("{}", y);
    println!("{}",a );


    let sum = 5+10;
    println!("{}", sum);
    let difference = 95.5 -4.3;

    println!("{}", sum);
    println!("{}", difference);

    // boolean

    let t = true;
    let f:bool = false;

    println!("{}", f);
    println!("{}", t);

    //characters

    let c = 'z';
    println!("{}", c);

    // compound data types
    // 1. tupple
    // 2. array

    // tupple

    let tup = (500, 6.4, 1);

    let (x,y,z) = tup;

    println!("the value of y is : {}", y);
    println!("the value of z is : {}", z);
    println!("the value of x is : {}", x);

    let a = (500,6.4,1);

    let five_hundred = a.0;
    let six_point_four = a.1;
    let one = a.2;

    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);

    // Array

    let a  = [1,2,3,4,5];
    println!("this is :{}",a.len());

    println!("{}",a[0]);
    








}
