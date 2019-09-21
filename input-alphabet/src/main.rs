use std::io;
 fn main() {
         println!("Enter text :");

         let mut inp1 = String::new();
         io::stdin()
             .read_line(&mut inp1)
             .expect("Problem reading data");
         let inptext = String::from(inp1.trim());

         datacheck(&inptext);
        
 }

 fn datacheck(data: &String) {
     

     for b in data.chars() {
         match b {
             'a'...'z' => print!(""),
             ' ' => print!(""),
             'A'...'Z' => print!(""),
             '0'...'9' => panic!("Kindly enter only alphabets"),
             _ => panic!(""),
         }
     }

      println!("{}",data)
 }
