// Convert temperatures between Fahrenheit and Celsius.

fn conver_c_to_f(temp :i32)-> i32{
    return (temp*9/5)+32;
}

fn convert_f_to_c(temp :i32)-> i32{
    return (temp -32)*5/9;
}


fn main(){

println!("The value of {}, from degree celcious to F is {}", 32, conver_c_to_f(32));
println!("The value of {}, from degree F to C is {}", 32, convert_f_to_c(32));

println!("The value of {} fibonaci is {}", 9, fib(9));


fn fib(num: i32)-> i32{

    if num ==0{
        return 0;
    }

    if num ==1{
        return 1;
    }

    return fib(num-1)+fib(num-2);
}




}