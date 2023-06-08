pub fn get_input() -> std::io::Result<String> {
    use ::std::io;

    let mut buf = String::new();

    io::stdin().read_line(&mut buf)?;

    return Ok(buf.trim().to_owned());
}
