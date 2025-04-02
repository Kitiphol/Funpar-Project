use rand::Rng;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::thread_rng;

type GRAPH = HashMap<String, Vec<String>>;

pub fn luby_algo(graph: &GRAPH) -> HashSet<String> {


    let mut vertexes: HashSet<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();     
    let mut rng = rand::rng();

    while !vertexes.is_empty() {

        let length = graph.len();

        let degrees: HashMap<String, usize> = vertexes.par_iter().fold(
            || HashMap::new(), |mut local_map, vertex| {
                let degree = graph.get(vertex).iter().len();
                local_map.insert(vertex.clone(), degree);
                local_map
            }
        ).reduce(|| HashMap::new(), |mut map1, map2| {
            map1.extend(map2);
            map1
        }  );


        let chosen_vertexes: HashSet<String> = vertexes.par_iter()
            .filter(|&vertex| {

                let mut rng = rand::rng(); 
                let degree = *degrees.get(vertex).unwrap() as f64;

                if degree == 0.0 {
                    true;
                } 
                
                let probability = 1.0 / (2.0 * degree);

                if rng.random::<f64>() < probability {
                    true
                } else {
                    false
                }
            })
            .cloned().collect();

            
        let mut checker = chosen_vertexes.clone();

        for v in &chosen_vertexes {
            for nbr in graph.get(v).unwrap() {
                if checker.contains(nbr) {

                    if degrees.get(v).unwrap() > degrees.get(nbr).unwrap() 
                    {
                        checker.remove(v);
                    } else {
                        checker.remove(nbr);
                    }

                }
            }
        }


        //remove all the MIS vertex and its neighbour from vertexes
        // so that we can stop while loop
        vertexes.retain(|v| 
            !checker.contains(v) && !graph.get(v)
            .unwrap_or(&vec![]).iter().any(|nbr| checker.contains(nbr)));


        // add the MIS vertexes into the answer
        mis.par_extend(checker);
    }

    mis
}



pub fn is_valid_mis(graph: &HashMap<String, Vec<String>>, mis: &HashSet<String>) -> bool {
    
    for node in mis {
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if mis.contains(neighbor) {
                    return false; 
                }
            }
        }
    }
    
    for (node, neighbors) in graph {
        if !mis.contains(node) && !neighbors.iter().any(|nbr| mis.contains(nbr)) {
            return false; 
        }
    }

    true
}