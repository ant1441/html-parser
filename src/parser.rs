struct Parser {
    script_nesting_level: usize,
    pause_flag: bool,
}

impl std::default::Default for Parser {
    fn default() {
        Parser {
            script_nesting_level: 0,
            pause_flag: false,
        }
    }
}
