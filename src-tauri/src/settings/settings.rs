
pub struct Settings {
    pub path: String, // path to directory to scan
    pub ignore: Vec<String>, // file extensions to ignore
    pub verbose: bool, // verbose output
    pub coverage: bool
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            path: String::from("./"),
            ignore: Vec::new(),
            verbose: false,
            coverage: false
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Default::default()
    }
}
