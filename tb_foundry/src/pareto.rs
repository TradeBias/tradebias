use crate::engine::EvaluatedStrategy;

pub fn pareto_sort(strategies: &mut Vec<EvaluatedStrategy>) {
    let n = strategies.len();
    if n == 0 { return; }

    let mut domination_counts = vec![0; n];
    let mut dominates_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut fronts: Vec<Vec<usize>> = vec![Vec::new()];

    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if dominates(&strategies[i], &strategies[j]) {
                dominates_list[i].push(j);
            } else if dominates(&strategies[j], &strategies[i]) {
                domination_counts[i] += 1;
            }
        }
        if domination_counts[i] == 0 {
            fronts[0].push(i);
        }
    }

    let mut current_front = 0;
    while !fronts[current_front].is_empty() {
        let mut next_front = Vec::new();
        for &i in &fronts[current_front] {
            for &j in &dominates_list[i] {
                domination_counts[j] -= 1;
                if domination_counts[j] == 0 {
                    next_front.push(j);
                }
            }
        }
        current_front += 1;
        if next_front.is_empty() {
            break;
        }
        fronts.push(next_front);
    }

    let mut ranked = Vec::with_capacity(n);
    for front in fronts {
        let mut front_strats: Vec<_> = front.into_iter().map(|idx| strategies[idx].clone()).collect();
        // Intra-front sort by fitness descending (proxy for crowding distance)
        front_strats.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        ranked.extend(front_strats);
    }
    
    *strategies = ranked;
}

fn dominates(a: &EvaluatedStrategy, b: &EvaluatedStrategy) -> bool {
    let fit_a = a.fitness;
    let fit_b = b.fitness;
    let risk_a = a.risk;
    let risk_b = b.risk;
    let comp_a = a.indicator_count;
    let comp_b = b.indicator_count;

    let mut strictly_better = false;

    // 1. Maximize Fitness
    if fit_a < fit_b { return false; }
    if fit_a > fit_b { strictly_better = true; }

    // 2. Minimize Risk (CPCV Variance)
    if risk_a > risk_b { return false; }
    if risk_a < risk_b { strictly_better = true; }

    // 3. Minimize Complexity (Indicator Count)
    if comp_a > comp_b { return false; }
    if comp_a < comp_b { strictly_better = true; }

    strictly_better
}
