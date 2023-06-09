pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_insensitive: args.contains(&String::from("--no-case"))
        })
    }
}