extern crate die_exit;
use die_exit::*;

#[test]
#[should_panic(expected = "Exited with code 42")]
fn die_with_print_selector() {
    die!("Escape invalid character: '{}'", '_'; 42);
}

#[test]
#[should_panic]
fn die_with_code_first() {
    die!(42; "Escape invalid character");
}

#[test]
#[should_panic]
fn die_empty() {
    die!();
}

#[test]
#[should_panic]
fn die_with_capture() {
    let code = 3;
    Result::<(), &str>::Err("error text").die_with(|error| (format!("Error is: {}", error), code));
}
