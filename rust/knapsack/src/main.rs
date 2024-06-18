//! Solves the knapsack problem

use std::{io, io::BufRead, cmp::max};

pub fn main() {
    let mut stdin_iter = io::stdin().lock().lines();

    loop {
        let mut possible_line = stdin_iter.next();
        if possible_line.is_none() {
            break;
        }

        let capacity_and_number_line = possible_line.expect("Missing capacity and number line").unwrap();
        let two_numbers = get_numbers(&capacity_and_number_line);
        let capacity = two_numbers[0];
        let number_of_objects = two_numbers[1];

        let mut values : Vec<usize> = Vec::new();
        let mut weights : Vec<usize> = Vec::new();
        for _ in 0..number_of_objects {
            possible_line = stdin_iter.next();
            let value_and_weight_line = possible_line.expect("Missing value and weight line").unwrap();
            let value_and_weight = get_numbers(&value_and_weight_line);
            values.push(value_and_weight[0]);
            weights.push(value_and_weight[1]);
        }

        // let (_, used_capacity, result) = knapsack(capacity, values, weights);
        let vals = knapsack_01_bottomup_optimized_succinct(&values, &weights, capacity);

        // println!("Used capacity {}", used_capacity);
        // println!("Result: {:?}", result);
        println!("{:?}", vals);
    }

    ()    
}

fn get_numbers(s: &str) -> Vec<usize> {
    return s
     .to_string()
     .split_whitespace()
     .map(|item| item.to_string().parse::<usize>())
     .collect::<Result<Vec<_>, _>>()
     .unwrap_or_else(|err| panic!("Failure during parse: {}", err));
}

fn knapsack_01_bottomup_optimized_succinct(values: &[usize], weights: &[usize], capacity: usize) -> Vec<usize>:
    let vals : Vec<usize> = Vec::new();
    for n in range(len(values)):
        for cap in range(capacity, weights[n] - 1, -1):
            vals[cap] = max(vals[cap], vals[cap - weights[n]] + values[n])
    return vals;
}

/// knapsack_table(w, weights, values) returns the knapsack table (`n`, `m`) with maximum values, where `n` is number of items
///
/// # Arguments:
///   * `w` - knapsack capacity
///   * `weights` - set of weights for each item
///   * `values` - set of values for each item
fn knapsack_table(w: &usize, weights: &[usize], values: &[usize]) -> Vec<Vec<usize>> {
    // Initialize `n` - number of items
    let n: usize = weights.len();
    // Initialize `m`
    // m[i, w] - the maximum value that can be attained with weight less that or equal to `w` using items up to `i`
    let mut m: Vec<Vec<usize>> = vec![vec![0; w + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=*w {
            // m[i, j] compiled according to the following rule:
            if i == 0 || j == 0 {
                m[i][j] = 0;
            } else if weights[i - 1] <= j {
                // If `i` is in the knapsack
                // Then m[i, j] is equal to the maximum value of the knapsack,
                // where the weight `j` is reduced by the weight of the `i-th` item and the set of admissible items plus the value `k`
                m[i][j] = max(values[i - 1] + m[i - 1][j - weights[i - 1]], m[i - 1][j]);
            } else {
                // If the item `i` did not get into the knapsack
                // Then m[i, j] is equal to the maximum cost of a knapsack with the same capacity and a set of admissible items
                m[i][j] = m[i - 1][j]
            }
        }
    }
    m
}

/// knapsack_items(weights, m, i, j) returns the indices of the items of the optimal knapsack (from 1 to `n`)
///
/// # Arguments:
///   * `weights` - set of weights for each item
///   * `m` - knapsack table with maximum values
///   * `i` - include items 1 through `i` in knapsack (for the initial value, use `n`)
///   * `j` - maximum weight of the knapsack
fn knapsack_items(weights: &[usize], m: &[Vec<usize>], i: usize, j: usize) -> Vec<usize> {
    if i == 0 {
        return vec![];
    }
    if m[i][j] > m[i - 1][j] {
        let mut knap: Vec<usize> = knapsack_items(weights, m, i - 1, j - weights[i - 1]);
        knap.push(i);
        knap
    } else {
        knapsack_items(weights, m, i - 1, j)
    }
}

/// knapsack(w, weights, values) returns the tuple where first value is "optimal profit",
/// second value is "knapsack optimal weight" and the last value is "indices of items", that we got (from 1 to `n`)
///
/// # Arguments:
///   * `w` - knapsack capacity
///   * `weights` - set of weights for each item
///   * `values` - set of values for each item
///
/// # Complexity
///   - time complexity: O(nw),
///   - space complexity: O(nw),
///
/// where `n` and `w` are "number of items" and "knapsack capacity"
pub fn knapsack(w: usize, weights: Vec<usize>, values: Vec<usize>) -> (usize, usize, Vec<usize>) {
    // Checks if the number of items in the list of weights is the same as the number of items in the list of values
    assert_eq!(weights.len(), values.len(), "Number of items in the list of weights doesn't match the number of items in the list of values!");
    // Initialize `n` - number of items
    let n: usize = weights.len();
    // Find the knapsack table
    let m: Vec<Vec<usize>> = knapsack_table(&w, &weights, &values);
    // Find the indices of the items
    let items: Vec<usize> = knapsack_items(&weights, &m, n, w);
    // Find the total weight of optimal knapsack
    let mut total_weight: usize = 0;
    for i in items.iter() {
        total_weight += weights[i - 1];
    }
    // Return result
    (m[n][w], total_weight, items)
}

#[cfg(test)]
mod tests;
