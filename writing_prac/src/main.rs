pub trait Summary{
    fn summarize(&self);
}

struct User{
    name : String,
    age : i32
}

//Now what is impl?

impl Summary for User{
    fn summarize(&self) {
        println!(" User {} is {}yrs" , self.name , self.age);
    }
}

use std :: fs;
fn main(){
    let user = User{
        name : String::from("Krish Garg"),
        age : 17
    };

    user.summarize();
}

// we have a user in the main , now self means passing on everthing , user--> self ownership , &self means passing immutable ref , and &mut self means passung mut ref 