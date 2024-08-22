fn concatenate_strings(s1: &str, s2: &str)-> String {

    let mut result: String = String::from("");
    result.push_str(s1);
    result.push_str(s2);

    result
}

fn main() {
    
    let string1: String = String::from("hello, ");
    let string2: String = String::from("world");

    println!("{}",concatenate_strings(&string1, &string2));

}
