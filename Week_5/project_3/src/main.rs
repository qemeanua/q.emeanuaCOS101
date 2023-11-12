use std::io;

fn main() {
    let mut input_1 = String::new();

     println!("Enter price: ");
     io::stdin().read_line(&mut input_1).expect("Not a valid string");
     let totalprice:f32 = input_1.trim().parse().expect("Not a valid string");

     // P = Poundo Yam/Edinkaiko Soup
     // F = Fried Rice & Chicken
     // A = Amala & Ewedu Soup
     // E = Eba & Egusi Soup
     // W = White Rice & Stew

     
     if totalprice > 10000.0
     { 
        let p:f32 = 3200.0;
        let f:f32 = 3000.0;
        let a:f32 = 2500.0;
        let e:f32 = 2000.0;
        let w:f32 = 2500.0;
        let total_order:f32 = p + f + a + e + w;

        let  five_percent_discount =  total_order * 0.05;
        let discount = totalprice - five_percent_discount;
        println!("The price of your food is {}", discount);
      }

      else {
          println!("The price of your food is {} ", totalprice);
      }

}