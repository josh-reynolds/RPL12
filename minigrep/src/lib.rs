

fn parse_config(args: &[String]) -> (&str, &str) {
    // does not handle if not enough args are passed
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
