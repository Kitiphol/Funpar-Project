use rand::Rng;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet,};

type GRAPH = HashMap<String, Vec<String>>;

pub fn luby(graph: &GRAPH) -> HashSet<String> {


    let mut vertexes: HashSet<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();     
    let mut rng = rand::rng();

    while !vertexes.is_empty() {

        let degrees: HashMap<String, usize> = vertexes.iter().map(|vertex| {
            let degree = graph.get(vertex).unwrap().len();

            (vertex.clone(), degree)

        }).collect();

        let chosen_vertexes: HashSet<String> = vertexes.iter()
            .filter(|&vertex| {
                
                let degree = *degrees.get(vertex).unwrap() as f64;
                if (degree == 0.0) {
                    true
                } else {
                    let probability = 1.0 / (2.0 * degree);
                    rng.random::<f64>() < probability 
                }
                
            })
            .cloned()
            .collect();

        let mut checker = chosen_vertexes.clone();

        for v in &chosen_vertexes {
            for nbr in graph.get(v).unwrap_or(&vec![]) {
                if checker.contains(nbr) {
                    if degrees.get(v).unwrap_or(&0) > degrees.get(nbr).unwrap_or(&0)
                        || (degrees.get(v) == degrees.get(nbr) && v > nbr)
                    {
                        checker.remove(v);
                    }
                }
            }
        }


        for vertex in &checker {
            mis.insert(vertex.clone());
            vertexes.remove(vertex);
            for nbr in graph.get(vertex).unwrap_or(&vec![]) {
                vertexes.remove(nbr);
            }
        }



        
    }

    mis
}



fn main() {
    let graph: HashMap<String, Vec<String>> = HashMap::from([
        ("A".to_string(), vec!["B".to_string(), "C".to_string()]),
        ("B".to_string(), vec!["A".to_string(), "C".to_string(), "D".to_string()]),
        ("C".to_string(), vec!["A".to_string(), "B".to_string(), "D".to_string()]),
        ("D".to_string(), vec!["B".to_string(), "C".to_string(), "E".to_string()]),
        ("E".to_string(), vec!["D".to_string()]),
    ]);

    let mis = luby(&graph);
    println!("Maximal Independent Set: {:?}", mis);


}




