use std::collections::HashMap;
use project::luby::luby_algo;
use project::luby::is_valid_mis;

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

    let mis = luby_algo(&graph);

    for node in &mis {
        for neighbor in graph.get(node).unwrap_or(&vec![]) {
            assert!(!mis.contains(neighbor), "Independent set condition violated");
        }
    }

    println!("Test passed with maximal independent set: {:?}", mis);
}


#[test]
fn test_cycle_graph() {
    let cycle_graph: HashMap<String, Vec<String>> = (1..=20)
        .map(|i| (format!("{}", i), vec![format!("{}", (i % 20) + 1), format!("{}", (i + 18) % 20 + 1)]))
        .collect();

    let mis = luby_algo(&cycle_graph);
    assert!(is_valid_mis(&cycle_graph, &mis), "Cycle Graph MIS is invalid: {:?}", mis);
    println!("Test passed with maximal independent set: {:?}", mis);
}

#[test]
fn test_star_graph() {
    let mut star_graph: HashMap<String, Vec<String>> = HashMap::new();
    star_graph.insert("Center".to_string(), (1..=15).map(|i| format!("L{}", i)).collect());
    for i in 1..=15 {
        star_graph.insert(format!("L{}", i), vec!["Center".to_string()]);
    }

    let mis = luby_algo(&star_graph);
    assert!(is_valid_mis(&star_graph, &mis), "Star Graph MIS is invalid: {:?}", mis);
    println!("Test passed with maximal independent set: {:?}", mis);
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

    let mis = luby_algo(&complex_graph);
    assert!(is_valid_mis(&complex_graph, &mis), "Complex Graph MIS is invalid: {:?}", mis);
    println!("Test passed with maximal independent set: {:?}", mis);
}
