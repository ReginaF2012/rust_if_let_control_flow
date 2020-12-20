// The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!{"not three"}
    }

}
