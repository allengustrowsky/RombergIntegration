mod config;
use config::romberg_config;
// use integrand;

// main function is special func that'll be executed automatically when you run the code.
fn main() {
    let v3: romberg_config::Values = Default::default();
    
    println!("{}", v3.lower_bound);
    println!("{}", v3.upper_bound);
    println!("{}", v3.num_iterations);
}
