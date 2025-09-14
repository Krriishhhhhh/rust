#[derive(Debug)]
struct User{
    name: String,
    age:i32
}

fn main(){
    let krish = User{
        name : String::from("Krish"),
        age : 17
    };

    println!("{:?}" , krish)
}


