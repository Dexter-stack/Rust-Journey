//  Rust has three kinds of loops: loop, while, and for. Let’s try each one.


fn main(){
let mut x=1;
    loop{
        println!("{}",x);
        if x == 10{
            break;
        }
        x = x+1;

    }




    // counting backward 

    let mut i = 8;

    while i != 0{
        println!("{}", i);

        i = i-1;
    }
    println!("LIFTOFF!!!");


    let my_array = [1,23,45,56,56,78];

     let a  = [1,2,3,4,5];
     let mut index = 0;

     while index < my_array.len(){
        println!("This is element :{}", my_array[index]);
        index = index + 1;
     }


    
     println!("The lenght is :{}", a.len());


     // write a while loop to go through an array
    let mut j =0;

      while j < my_array.len(){
        if my_array[j] % 2 == 0{
         println!("The even number: {}",my_array[j]);
        }
       
        j = j +1;
            }




            let b = [10,20,30,40,50];

            for i in b.iter(){
                println!("the value is : {}", i);
            }

            // using range in for loop

            for number in (1..100).rev(){

                println!("{}", number);
               
            }
             println!("LIFTOFF!!!!!!");

             for n in 1..50{
                println!("{}", n);
             }
}





