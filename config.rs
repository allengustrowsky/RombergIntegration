#[allow(dead_code)]
pub mod romberg_config {
    pub struct Values {
        pub equation: fn(f64) -> f64,
        pub lower_bound: f64,
        pub upper_bound: f64,
        pub num_iterations: i32,
    }

    impl Default for Values {
        fn default() -> Self {
            Values {
                equation: integrand,
                lower_bound: 0.0,
                upper_bound: 3.14159,
                num_iterations: 4
            }
        }
    }

    fn integrand(x: f64) -> f64 {
        x*x
    }
}