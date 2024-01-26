struct Laptop{
    brand:String,
    cost_of_product:u32,
    constant:u32
}

//impl to calculate the cost of 3 purchases from each brand
impl Laptop{
    fn price(&self)->u32{
        self.cost_of_product *  self.constant
    }    
}

fn main(){
  //cost for 3 of hp laptops
    let cost1 = Laptop{
        brand:String::from("HP"),
        cost_of_product:650000,
        constant:3
        
    };
    
    let cost2 = Laptop{
        brand:String::from("IBM"),
        cost_of_product:755000,
        constant:3
    };


    let cost3 = Laptop{
        brand:String::from("Toshiba"),
        cost_of_product:550000,
        constant:3
    };    

     let cost4 = Laptop{
        brand:String::from("Dell"),
        cost_of_product:850000,
        constant:3
    }; 

    let total = cost1.price() + cost2.price() + cost3.price() + cost4.price();

    display(cost1);
    display(cost2);
    display(cost3);
    display(cost4);
    println!("The total cost of 3 purchases from each brand is {}", total); 

}
fn display(cost:Laptop){
    println!("The cost for 3 {} laptops is {}", cost.brand, cost.price());
}

    
