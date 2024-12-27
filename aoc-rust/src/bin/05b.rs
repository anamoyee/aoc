#[allow(unused)]
use aoc::{input, input_test};
use colored::*;
use std::collections::{HashMap, VecDeque};

fn is_valid_page(page: &[u32], rules: &[[u32; 2]]) -> bool {
    for &[before, after] in rules {
        let idx_before = page.iter().position(|&item| item == before);
        let idx_after = page.iter().position(|&item| item == after);

        if idx_before.is_none() || idx_after.is_none() {
            continue;
        }

        let idx_before = idx_before.unwrap();
        let idx_after = idx_after.unwrap();

        if idx_before > idx_after {
            return false;
        }
    }

    true
}

fn sort_page_with_rules(page: &Vec<u32>, rules: &[[u32; 2]]) -> Vec<u32> {
    // Build the graph and in-degree map for nodes in `page`
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Initialize nodes from `page`
    for &node in page {
        graph.entry(node).or_default();
        in_degree.entry(node).or_insert(0);
    }

    // Filter and add edges only if both nodes are in `page`
    for &[a, b] in rules {
        if page.contains(&a) && page.contains(&b) {
            graph.entry(a).or_default().push(b);
            *in_degree.entry(b).or_insert(0) += 1;
        }
    }

    // Perform Kahn's algorithm
    let mut queue: VecDeque<u32> = VecDeque::new();
    for (&node, &count) in &in_degree {
        if count == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted_page = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted_page.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(entry) = in_degree.get_mut(&neighbor) {
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check for completeness
    if sorted_page.len() != page.len() {
        panic!("Cycle detected! Cannot sort page with given rules.");
    }

    sorted_page
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let [rules, pages]: [&str; 2] = input
        .splitn(2, "\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            line.split('|')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u32; 2]>>();

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let pages = pages
        .iter()
        .filter(|&page| !is_valid_page(page, &rules))
        .collect::<Vec<&Vec<u32>>>();

    let pages = pages
        .iter()
        .map(|&page| sort_page_with_rules(page, &rules))
        .collect::<Vec<_>>();

    let out = pages
        .iter()
        .map(|x| x[x.len().div_ceil(2) - 1])
        .sum::<u32>();

    // println!("{}", format!("{:#?}", rules).bright_blue().bold());
    println!("{}", format!("{:#?}", out).bright_blue().bold());
}
