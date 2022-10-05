fn main() {
    let s1 = String::from("hello");

    takes_ownership(s1);

    //doesn't work because ownership was passed to the function, and the value on heap was deallocated when the function ended
    //println!("outside function: {}", s);

    let s2 = String::from("bruh");

    //takes ownership from s2 and gives it back to s3
    let s3 = takes_and_gives_back(s2);

}

fn takes_ownership(some_string: String) {
    println!("inside function: {}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
