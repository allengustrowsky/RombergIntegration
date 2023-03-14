mod config;
use config::romberg_config;

fn main() {
    let params: romberg_config::Values = Default::default();

    let h: f64 = params.upper_bound - params.lower_bound;
    let mut fs_coeff: f64; // coefficient for expression in first sweep
    let mut c: f64; // coefficient for second expression in sweeps 2+
    let mut sum: f64 = 0.0; // used in first sweep only
    c = 1.0 / (((4i64.pow(2))-1) as f64);
    // initialize Romberg table
    let mut r_table: Vec<Vec<f64>> = vec![vec![]; params.num_iterations as usize];
    
    // 1st sweep
    for i in 0..params.num_iterations {
        fs_coeff = h / (2i64.pow((i as u32)+1) as f64); // h/2, h/4, ...
        let fa: f64 = (params.equation)(params.lower_bound);
        let fb: f64 = (params.equation)(params.upper_bound);
        for n_coeff in 1..2i64.pow(i as u32) {
            sum += (params.equation)(params.lower_bound + ((h*(n_coeff as f64)) / (2i64.pow(i as u32)) as f64));
            // println!("f({} + {}h/{})", params.lower_bound, n_coeff as f64, (2i64.pow(i as u32)) as f64);
        }
        // println!("fa: {}", fa);
        // println!("final: {}[{} + 2({}) + {}]", fs_coeff, fa, sum, fb);
        r_table[0].push(fs_coeff * (fa + 2.0*sum + fb));
        sum = 0.0;
    }
    println!("1st sweep: {:?}", r_table);

    // perform remaining sweeps
    for k in 1..params.num_iterations {
        println!("i: {}", k);
        // 2nd sweep
        for j in k..params.num_iterations {
            r_table[k].push(j as f64);
        }
    }
    println!("done: {:?}", r_table);
    println!("result: {}", r_table[params.num_iterations-1][0]);


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
