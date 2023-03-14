#[allow(dead_code)]
pub mod romberg_config {
    pub struct Values {
        pub equation: fn(f64) -> f64,
        pub lower_bound: f64,
        pub upper_bound: f64,
        pub num_iterations: usize,
        pub print_table: bool,
    }

    impl Default for Values {
        fn default() -> Self {
            Values {
                equation: integrand,
                lower_bound: 0.0, // EDIT ME! (must be f64)
                upper_bound: std::f64::consts::PI, // EDIT ME! (must be f64)
                num_iterations: 4, // EDIT ME! (must be usize)
                print_table: true // EDIT ME!
            }
        }
    }

    // Edit this function - make it whatever you want it to be as long as it returns f64
    fn integrand(x: f64) -> f64 {
        x.sin() // in radians, omit trailing semi-colon
    }
}