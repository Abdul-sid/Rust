use std::io;
fn main() {

    let mut sum = 0 ;

loop{
    println!("Kindly enter corresponding number to order the dish");
    println!("1 => zinger : 200 | 2 => broast : 210 | 3 => fish : 300 | 4 => BBQ : 150");
let mut order = String::new();
io::stdin().read_line(&mut order)
        .expect("Failed to read line");
let order_n : i32 = order.trim().parse().unwrap();

println!("What is the quantity of your desired item ?");
let mut quantity = String::new();
io::stdin().read_line(&mut quantity)
        .expect("Failed to read line");
let quantity_n : i32 = quantity.trim().parse().unwrap();

match order_n {
    1 => { println!("{} zinger burger",quantity_n) ;
           sum = sum + (200*quantity_n) ;
    }
    2 => { println!("{} Broast",quantity_n);
           sum = sum + (210*quantity_n) ;
    }
    3 => { println!("{} Fish",quantity_n);
           sum = sum + (300*quantity_n);
    }
    4 => { println!("{} BBQ",quantity_n);
        sum = sum + (150*quantity_n);
    }
    _ => println!("Sorry no item")
}


println!("Another order Yes (Y) , No (N) ?");
let mut stop = String::new();
io::stdin().read_line(&mut stop)
        .expect("Failed to read line");
let stop_n = String::from(stop.trim());

if stop_n == "Y" || stop_n == "y" {
    continue;
}
else if stop_n == "n" || stop_n == "N" {
    println!("Your bill is => {}", sum);
    break;
}

}


}
