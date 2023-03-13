# Romberg Intergration

Romberg Integration is a numerical technique used to find an approximation for mathematical functions that cannot be easily integrated. This project allows the integral of a function to be calculated using Romberg Integration.

## Setup
- Must have [Rust](https://www.rust-lang.org/tools/install) setup on your machine (_very_ simple)

## Usage
You can configure the following values:
1. function to integrate
2. lower limit of integration
3. upper limit of integration
4. number of iterations to be performed by the Romberg Algorithm

The only file that needs to be modified for this to work is `config.rs`. This file contains the `Values` struct that will hold the above config variables. NOTE: the struct declaration should not be touched. The `main.rs` file uses the default values of this struct to perform the algorithm, so to configure the algorithm you just have to change the default values to whatever you want. For example, if you wanted to change the lower bound to be 2.0 and the upper bound to be 4.0, your default values would look like
```
impl Default for Values {
    fn default() -> Self {
        Values {
            equation: integrand,
            lower_bound: 2.0, // EDIT ME! (must be f64)
            upper_bound: 4.0, // EDIT ME! (must be f64)
            num_iterations: 4 // EDIT ME! (must be i32)
        }
    }
}
```
You could also change what function you want to integrate by modifying the `integrand` function. For example, to make it the linear function 3x+2, it would look like
```
// Edit this function - make it whatever you want it to be as long as it returns f64
fn integrand(x: f64) -> f64 {
    3*x + 2
}
```
That's it!  Now just compile and run:
- To run, cd to `src` and run `rustc main.rs`
- Run the binary that was just generated. May vary by operating system, but on mac run `./main`