use std::io;

fn main() {

     let mut result = 0 ;

     println!("Enter first number");

     let mut a1 = String::new();

     io::stdin().read_line(&mut a1)
     .expect("Failed to read line");

     let mut a : i32 = a1.trim().parse().unwrap();


     println!("Enter second number");

     let mut b1 = String::new();

     io::stdin().read_line(&mut b1)
     .expect("Failed to read line");

     let mut b : i32 = b1.trim().parse().unwrap();

     println!("Enter total numbers of series you want");

     let mut total1 = String::new();

     io::stdin().read_line(&mut total1)
     .expect("Failed to read line");

     let mut total : i32 = total1.trim().parse().unwrap();



for i in 1..(total + 1){
    result = a + b;

    a = b;
    b = result;

    println!("{}",result );
}



}