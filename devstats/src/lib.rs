pub mod lib {
    #[derive(Debug)]
    pub struct Ctx {
        pub debug: i8, // From GHA2DB_DEBUG Debug level: 0-no, 1-info, 2-verbose, including SQLs, default 0
    }
    impl Ctx {
        pub fn new() -> Self {
            Default::default()
        }
    }
    impl Default for Ctx {
        fn default() -> Self {
            Ctx { debug: 0 }
        }
    }
}
