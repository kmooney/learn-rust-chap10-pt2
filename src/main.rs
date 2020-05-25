fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
    let string1 = String::from("abcd");
    let result;
    {
	    
	    result = longest(string1.as_str(), "xyz1234");

	}
	println!("The longest string is {}", result);
}
