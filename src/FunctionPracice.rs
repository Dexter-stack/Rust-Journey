
fn main(){

    another_function();
    my_age(23);
    test_function(40,50);
    println!("The fibonaci of {} is {}",9,fib(9));
    println!("The factorial of {} is {}", 5, fact(5));



    let y = {
        let x=3;
        x+1
    };

    println!("The value of y is: {}", y);


    let myfive =  five();
    println!("The value of myfive is :{}", myfive);

    let my_plus= plus_one(43);
    println!("The value of myPlus: {}", my_plus);
}



fn another_function(){

    println!("Another function.");
}

fn my_age(x:i32){
    println!("The value of x is :{}", x);
}

fn test_function(x: i32, y: i32){
    println!("The value of x is :{}", x);
    println!("The value of y is: {}", y);
}


//  Functions with Return Values

fn five() -> i32{
    5
}

fn plus_one(x:i32) -> i32{

    x+1
}


fn fib(num: i32)->i32{

    if num == 0{
        return 0;
    }

    if num == 1 {
      return num;
    }

    return fib(num-1) + fib(num-2);
    

}

fn fact(num: i32)-> i32{

    let mut result :i32 = 1;
    if num == 0{
        return 1;
    }
    if num == 1{
        return 1;
    }

    for n in (1..num+1).rev(){
      result = result * n;

    }
    return result;

}




