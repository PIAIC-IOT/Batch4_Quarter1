mod hr;


mod faisalabad {
    #[derive(Debug)]
    pub struct Food {
        pub name : String,
        pub price : u16,
    }

    #[derive(Debug)]
    pub enum Direction {
        Right,
        Left,
        Up,
        Down,
    }

    pub fn abc (){
        println!("Welcoe in abc");
    }

}

use faisalabad::Food;


fn main() {
    faisalabad::abc();
    let food1 = Food {
         name : "Munna Daal Chawal".to_string(),
         price : 150,
     };
     println!("{:#?}",food1);
    nagiha();
    hr::salary();

    let d1 = faisalabad::Direction::Right;
    println!("Your Direction is : {:?}",d1);
}



fn nagiha() {
    println!("Welcome in IoT Batch 3 by Nagiha");
}