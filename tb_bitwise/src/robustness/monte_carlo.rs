use rand::seq::SliceRandom;
use rand::thread_rng;

/// Runs Monte Carlo Block Bootstrapping on a baseline equity curve.
/// Returns a list of randomized equity curves representing the "spaghetti cloud".
pub fn run_block_bootstrap(baseline_curve: &[f64], iterations: usize) -> Vec<Vec<f64>> {
    let mut cloud = Vec::with_capacity(iterations);
    let n = baseline_curve.len();
    if n == 0 {
        return vec![vec![]; iterations];
    }

    // Step 1: Convert cumulative equity curve back to a series of bar-by-bar returns
    let mut returns = Vec::with_capacity(n);
    returns.push(baseline_curve[0]);
    for i in 1..n {
        returns.push(baseline_curve[i] - baseline_curve[i - 1]);
    }

    // Step 2: Define block size (e.g., 20 bars)
    let block_size = 20.min(n);
    let num_blocks = n / block_size;
    
    // Step 3: Run the simulations
    let mut rng = thread_rng();
    for _ in 0..iterations {
        let mut sim_curve = Vec::with_capacity(n);
        let mut current_equity = 0.0;
        
        // We shuffle the indexes of the blocks to preserve internal clustering
        let mut block_indexes: Vec<usize> = (0..num_blocks).collect();
        block_indexes.shuffle(&mut rng);
        
        for &block_idx in &block_indexes {
            let start = block_idx * block_size;
            let end = start + block_size;
            for r in &returns[start..end] {
                current_equity += r;
                sim_curve.push(current_equity);
            }
        }
        
        // Handle any remainder bars if n isn't perfectly divisible by block_size
        let remainder_start = num_blocks * block_size;
        for r in &returns[remainder_start..n] {
            current_equity += r;
            sim_curve.push(current_equity);
        }
        
        cloud.push(sim_curve);
    }
    
    cloud
}
