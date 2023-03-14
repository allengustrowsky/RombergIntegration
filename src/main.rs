mod config;
use config::romberg_config;

fn main() {
    let params: romberg_config::Values = Default::default();

    let _h: f64= params.upper_bound - params.lower_bound;
    let _c: f64;
    _c = 1.0 / (((4i64.pow(2))-1) as f64);
    // initialize Romberg table
    let mut r_table: Vec<Vec<f64>> = vec![vec![]; params.num_iterations as usize];
    println!("r_table: {:?}", r_table);
    

    // pseudocode
    // for first iteartion count
        // calculate initial values (R11, R21, R31, etc.)
    // for rest of the iteration counts startint at k, j = k:
        // c = 1.0 / ((4i64.pow(j-1))-1 as f64)
        // Rkj = Rk(j-1) + c(Rk(j-1) - R(k-1)(j-1))

        // for c: beware of integer division!! must be 1.0/3.0, not 1/3; could use (my_num as f64)/(other_num as f64)

    // let result: f64 = (params.equation)(std::f64::consts::PI);
    // println!("function evaluation at pi: {}", result);
}
