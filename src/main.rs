
fn create_string() {
    let s1: String = String ::from("Hello");
    let s2: String = s1;

    // println!("{}" , s1); Can't be done as we moved the owner 

    println!("{}" , s2); //Will print hello
}
fn main() {
    create_string()
}

//In this code , at first s1 was the owner of "Hello" in the heap , and then we made s2 as the owner of heap