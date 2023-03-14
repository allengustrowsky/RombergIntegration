mod config;
use config::romberg_config;
use std::convert::TryInto;
use std::time::{Duration, Instant};

fn main() {
    let start_time = Instant::now();
    let params: romberg_config::Values = Default::default();

    let h: f64 = params.upper_bound - params.lower_bound;
    let mut fs_coeff: f64; // coefficient for expression in first sweep
    let mut c: f64; // coefficient for second expression in sweeps 2+
    let mut sum: f64 = 0.0; // used in first sweep only
    // c = 1.0 / (((4i64.pow(2))-1) as f64);
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
    // println!("1st sweep: {:?}", r_table);

    // perform remaining sweeps
    for k in 1..params.num_iterations { // references array
        // println!("i: {}", k);
        c = 1.0 / (((4i64.pow(k as u32))-1) as f64);
        // println!("c: {}", c);
        for j in 1..(params.num_iterations-k+1) { // references array index
            // println!("j: {}", j);
            // println!("{} + {}*({} - {})", r_table[k-1][j], c, r_table[k-1][j], r_table[k-1][j-1]);
            sum = r_table[k-1][j] + c*(r_table[k-1][j] - r_table[k-1][j-1]);
            // println!("safe!");
            r_table[k].push(sum);
            // println!("safe2");
            sum = 0.0;
        }
        // println!("sweep {}: {:?}", k+1, r_table);
    }
    // println!("done: {:?}", r_table);
    // println!("result: {}", r_table[params.num_iterations-1][0]);

    // elapsed time adapted from https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html#measure-the-elapsed-time-between-two-code-sections
    let calc_time = start_time.elapsed();

    // print result
    println!("Result: {}", r_table[params.num_iterations-1][0]);
    println!("Sweeps: {}", params.num_iterations);
    println!("Time to calculate: {:?}", calc_time);
    // print Romberg table 
    if params.print_table {
        println!();
        let wer: String = String::from("hello");
        for row in 0..r_table.len() {
            print!("{:^29}", format!("{} {}", String::from("sweep"), row + 1));
        }
        println!("\n");
        for row in 0..r_table.len() {
            for j in 0..(row+1) { // first index above (DELETE ME AFTER)
                if r_table[j][row-j].abs() < 1.0 { // prevent 0.00000000001957293791
                    print!("|  {:<23}  |", format!("{:+e}", r_table[j][row-j]));
                } else {
                    print!("|  {:<23}  |", r_table[j][row-j]);
                }
            }
            println!();
        }
    }
}
