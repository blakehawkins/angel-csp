use cassowary::strength::{MEDIUM, REQUIRED, STRONG};
use cassowary::WeightedRelation::*;
use cassowary::{Solver, Variable};

fn main() {
    let allocation = 100;
    let requests = [(100, 95), (2, 1), (1, 4)];
    // let sum_requests: usize = requests.iter().map(|(req, _)| req).sum();
    let sum_weights = requests.iter().map(|(_, weight)| weight).sum::<usize>() as f32;
    let weight_fracs = requests
        .iter()
        .map(|(_, weight)| (*weight as f32) / sum_weights as f32)
        .collect::<Vec<_>>();

    // let requests_weights =
    // println!("{}", sum_requests);
    let a_allocations = Variable::new();
    let b_allocations = Variable::new();
    let c_allocations = Variable::new();

    let mut solver = Solver::new();
    solver
        .add_constraints(&[
            a_allocations | GE(REQUIRED) | 0.0,
            b_allocations | GE(REQUIRED) | 0.0,
            c_allocations | GE(REQUIRED) | 0.0,
            (a_allocations + b_allocations + c_allocations) | LE(REQUIRED) | allocation as f32,
            a_allocations | LE(REQUIRED) | requests[0].0 as f32,
            b_allocations | LE(REQUIRED) | requests[1].0 as f32,
            c_allocations | LE(REQUIRED) | requests[2].0 as f32,
            // Maximise allocations
            (a_allocations + b_allocations + c_allocations) | EQ(STRONG) | allocation as f32,
            // Weights:
            a_allocations | EQ(MEDIUM) | ((requests[0].1 as f32 * allocation as f32) / sum_weights),
            b_allocations | EQ(MEDIUM) | ((requests[1].1 as f32 * allocation as f32) / sum_weights),
            c_allocations | EQ(MEDIUM) | ((requests[2].1 as f32 * allocation as f32) / sum_weights),
            // Fairness:
            // a / weighted_a_req = b / weighted_b_req = c / weighted_c_req
            (a_allocations / weight_fracs[0]) | EQ(MEDIUM) | (b_allocations / weight_fracs[1]),
            (b_allocations / weight_fracs[1]) | EQ(MEDIUM) | (c_allocations / weight_fracs[2]),
        ])
        .unwrap();
    println!("{:?}", solver.fetch_changes());
}
