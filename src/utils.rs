pub fn get_input() -> Option<String> {
    use ::std::io;

    let mut buf = String::new();

    if io::stdin().read_line(&mut buf).is_err() {
        println!("Print data again");
    }

    if &buf == "" {
        None
    } else {
        Some(buf.trim().to_owned())
    }
}
