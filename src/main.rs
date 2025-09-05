
fn create_string() {
    let s1: String = String ::from("Hello");
   do_something(&s1); //Basically s1 will remain the owner of hello , but s2 is borrowing the value , this is not a copy , s2 has access to original hello , but it has only borrowed it , not owned it 

   
}

fn do_something(s2 :&String){
    println!("{}" , s2);
}
fn main() {
    create_string()
}

