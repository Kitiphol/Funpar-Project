use std::collections::HashMap;
use project::luby::{luby_algo, luby_seq, luby_algo_par_chunck, luby_algo_par_chunck2};
use project::luby::is_valid_mis;
use std::time::Instant;
use rand::seq::SliceRandom;

#[test]
fn test_normal_graph() {
    let graph: HashMap<String, Vec<String>> = HashMap::from([
        ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
        ("B".to_string(), vec!["A".to_string(), "C".to_string(), "D".to_string()]),
        ("C".to_string(), vec!["A".to_string(), "B".to_string(), "D".to_string()]),
        ("D".to_string(), vec!["B".to_string(), "C".to_string(), "E".to_string()]),
        ("E".to_string(), vec!["D".to_string()]),
        ("G".to_string(), vec![]),
    ]);

    let start = Instant::now();
    let mis_parallel = luby_algo(&graph);
    let parallel_duration = start.elapsed();
    println!("Parallel Luby algorithm execution time: {:?}", parallel_duration);

    for node in &mis_parallel {
        for neighbor in graph.get(node).unwrap_or(&vec![]) {
            assert!(!mis_parallel.contains(neighbor), "Independent set condition violated");
        }
    }
    println!("Test passed with parallel maximal independent set: {:?}", mis_parallel);

    let start = Instant::now();
    let mis_sequential = luby_seq(&graph);
    let sequential_duration = start.elapsed();
    println!("Sequential Luby algorithm execution time: {:?}", sequential_duration);

    for node in &mis_sequential {
        for neighbor in graph.get(node).unwrap_or(&vec![]) {
            assert!(!mis_sequential.contains(neighbor), "Independent set condition violated");
        }
    }
    println!("Test passed with sequential maximal independent set: {:?}", mis_sequential);
}

#[test]
fn test_cycle_graph() {
    let cycle_graph: HashMap<String, Vec<String>> = (1..=20)
        .map(|i| (format!("{}", i), vec![format!("{}", (i % 20) + 1), format!("{}", (i + 18) % 20 + 1)]))
        .collect();

    let start = Instant::now();
    let mis_parallel = luby_algo(&cycle_graph);
    let parallel_duration = start.elapsed();
    println!("Parallel Luby algorithm execution time: {:?}", parallel_duration);
    assert!(is_valid_mis(&cycle_graph, &mis_parallel), "Cycle Graph MIS is invalid: {:?}", mis_parallel);
    println!("Test passed with parallel maximal independent set: {:?}", mis_parallel);

    let start = Instant::now();
    let mis_sequential = luby_seq(&cycle_graph);
    let sequential_duration = start.elapsed();
    println!("Sequential Luby algorithm execution time: {:?}", sequential_duration);
    assert!(is_valid_mis(&cycle_graph, &mis_sequential), "Cycle Graph MIS is invalid: {:?}", mis_sequential);
    println!("Test passed with sequential maximal independent set: {:?}", mis_sequential);
}

#[test]
fn test_complex_graph() {
    let mut complex_graph: HashMap<String, Vec<String>> = HashMap::new();

    complex_graph.insert("A".to_string(), vec!["B".to_string()]);
    complex_graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string()]);
    complex_graph.insert("C".to_string(), vec!["B".to_string(), "D".to_string()]);
    complex_graph.insert("D".to_string(), vec!["C".to_string()]);

    let clique_nodes = vec!["X1", "X2", "X3", "X4", "X5"];
    for i in 0..clique_nodes.len() {
        complex_graph.insert(
            clique_nodes[i].to_string(),
            clique_nodes.iter().filter(|&&x| x != clique_nodes[i]).map(|x| x.to_string()).collect(),
        );
    }

    let start = Instant::now();
    let mis_parallel = luby_algo(&complex_graph);
    let parallel_duration = start.elapsed();
    println!("Parallel Luby algorithm execution time: {:?}", parallel_duration);
    assert!(is_valid_mis(&complex_graph, &mis_parallel), "Complex Graph MIS is invalid: {:?}", mis_parallel);
    println!("Test passed with parallel maximal independent set: {:?}", mis_parallel);

    let start = Instant::now();
    let mis_sequential = luby_seq(&complex_graph);
    let sequential_duration = start.elapsed();
    println!("Sequential Luby algorithm execution time: {:?}", sequential_duration);
    assert!(is_valid_mis(&complex_graph, &mis_sequential), "Complex Graph MIS is invalid: {:?}", mis_sequential);
    println!("Test passed with sequential maximal independent set: {:?}", mis_sequential);

    let start = Instant::now();
    let mis_new = luby_algo_par_chunck(&complex_graph);
    let duration3 = start.elapsed();
    println!("New Algo Execution Time: {:?}", duration3);
    assert!(is_valid_mis(&complex_graph, &mis_sequential), "Complex Graph MIS is invalid: {:?}", mis_new);
    println!("Test passed with sequential maximal independent set: {:?}", mis_new);
}



#[test]
fn test_star_graph() {
    let mut star_graph: HashMap<String, Vec<String>> = HashMap::new();
    star_graph.insert("Center".to_string(), (1..=15).map(|i| format!("L{}", i)).collect());
    for i in 1..=15 {
        star_graph.insert(format!("L{}", i), vec!["Center".to_string()]);
    }

    let start = Instant::now();
    let mis_parallel = luby_algo(&star_graph);
    let parallel_duration = start.elapsed();
    println!("Parallel Luby algorithm execution time: {:?}", parallel_duration);
    assert!(is_valid_mis(&star_graph, &mis_parallel), "Star Graph MIS is invalid: {:?}", mis_parallel);
    println!("Test passed with parallel maximal independent set: {:?}", mis_parallel);

    let start = Instant::now();
    let mis_sequential = luby_seq(&star_graph);
    let sequential_duration = start.elapsed();
    println!("Sequential Luby algorithm execution time: {:?}", sequential_duration);
    assert!(is_valid_mis(&star_graph, &mis_sequential), "Star Graph MIS is invalid: {:?}", mis_sequential);
    println!("Test passed with sequential maximal independent set: {:?}", mis_sequential);
}

#[test]
fn test_large_cycle_graph() {
    let large_cycle_graph: HashMap<String, Vec<String>> = (1..=10_000)
        .map(|i| (format!("{}", i), vec![format!("{}", (i % 10_000) + 1), format!("{}", (i + 9_998) % 10_000 + 1)]))
        .collect();

    let start = Instant::now();
    let mis_parallel = luby_algo(&large_cycle_graph);
    let parallel_duration = start.elapsed();
    assert!(is_valid_mis(&large_cycle_graph, &mis_parallel), "Large Cycle Graph MIS is invalid: {:?}", mis_parallel);
    println!("Test passed with parallel maximal independent set on large graph: {:?}", mis_parallel);

    let start = Instant::now();
    let mis_seq = luby_seq(&large_cycle_graph);
    let seq_dur = start.elapsed();
    assert!(is_valid_mis(&large_cycle_graph, &mis_parallel), "Large Cycle Graph MIS is invalid: {:?}", mis_seq);
    println!("Test passed with parallel maximal independent set on large graph: {:?}", mis_seq);

    
    

    let start = Instant::now();
    let mis_new = luby_algo_par_chunck(&large_cycle_graph);
    let duration3 = start.elapsed();
    
    let start = Instant::now();
    let mis_new = luby_algo_par_chunck2(&large_cycle_graph);
    let duration4 = start.elapsed();

    println!("\n");
    println!("Par Chunck Algo without DashSet Execution Time: {:?}", duration3);
    println!("Par Chunck with DashSet Execution Time: {:?}", duration4);
    println!("Sequential Luby algorithm execution time on large graph: {:?}", seq_dur);
    println!("Par Iter algorithm execution time on large graph: {:?}", parallel_duration);



}

