/// `hello_rust` is a function that prints a greeting that is personalized based on
/// the arguments given.
///
/// # Arguments
///
/// * `x` - How many times you want to make greeting.
/// * `y` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let number_of_hello = 5i;
/// hello_rust(number_of_hello);
/// ```

pub fn hello_rust(x: int) -> Result<String, String> {
	let mut result = "Hello Rust!".to_string(); 
	for _ in range(0i, x-1) {
		result.push_str("\nHello Rust!");
	}
	if result!="" {
	   Ok(result.to_string())
	} else {
	   Err("No result!".to_string())
	}
}

#[test]
fn test_test() {
	assert!(true);
    assert_eq!(1i,1i);
}

#[test]
#[should_fail(expected = "assertion failed")]
fn test_test_not() {
	assert!(false);
    assert_eq!(1u, 2u);
}

#[test]
fn test_hello_rust() {
	assert!(true);
	assert!(hello_rust(3).is_ok());
    assert_eq!(hello_rust(1).ok().expect("Error in a test of hello_rust"), "Hello Rust!".to_string());
    assert!(true);
}
