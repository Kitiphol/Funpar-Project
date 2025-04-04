use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use dashmap::DashSet;
use std::time::Instant;

type GRAPH = HashMap<String, Vec<String>>;

pub fn luby_algo_par_chunck(graph: &GRAPH) -> HashSet<String> {
    let mut vertexes: Vec<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();
    let mut rng = rand::rng();
    let chunk_size = vertexes.len() / 32; 

    while !vertexes.is_empty() {

       let start = Instant::now();
        let degrees: HashMap<String, usize> = vertexes.par_iter().map( |vertex: &String| {
            let degree = graph.get(vertex).unwrap().len();
            (vertex.clone(), degree)
        }).collect();
        let duration4 = start.elapsed();
        println!(" The degree parallel: {:?}", start.elapsed());

        let chosen_vertexes: HashSet<String> = vertexes.par_chunks(chunk_size).flat_map(|chunk| {
            let mut selected = Vec::new();
            let mut rng = rand::rng();
            for vertex in chunk {
                let degree = *degrees.get(vertex).unwrap() as f64;
                if degree == 0.0 || rng.random::<f64>() < 1.0 / (2.0 * degree) {
                    selected.push(vertex.clone());
                }
            }
            selected
        }).collect();

        let mut checker = chosen_vertexes.clone();

        for v in &chosen_vertexes {
            for nbr in graph.get(v).unwrap_or(&vec![]) {
                if checker.contains(nbr) {
                    if degrees.get(v).unwrap() > degrees.get(nbr).unwrap() {
                        checker.remove(v);
                    } else {
                        checker.remove(nbr);
                    }
                }
            }
        }

        vertexes.retain(|v| !checker.contains(v) && !graph.get(v).unwrap()
        .iter().any(|nbr| checker.contains(nbr)));

        mis.par_extend(checker);
    }

    mis
}

pub fn luby_algo_par_chunck2(graph: &GRAPH) -> HashSet<String> {
    let mut vertexes: Vec<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();
    let chunk_size = vertexes.len() / 4; 

    while !vertexes.is_empty() {

        let start = Instant::now();

        let degrees: HashMap<String, usize> = vertexes.par_chunks(chunk_size).map(|chunk| {
            let mut local_map = HashMap::new();
            for vertex in chunk {
                let degree = graph.get(vertex).map_or(0, |neighbors| neighbors.len());
                local_map.insert(vertex.clone(), degree);
            }
            local_map
        }).reduce(HashMap::new, |mut map1, map2| {
            map1.extend(map2);
            map1
        });

        let duration1 = start.elapsed();
        println!("The duration for making degrees in par chunck : {:?}", duration1);
        
        let start = Instant::now();

        let chosen_vertexes: HashSet<String> = vertexes.par_chunks(chunk_size).flat_map(|chunk| {
            let mut selected = Vec::new();
            let mut rng = rand::rng();
            for vertex in chunk {
                let degree = *degrees.get(vertex).unwrap() as f64;
                if degree == 0.0 || rng.random::<f64>() < 1.0 / (2.0 * degree) {
                    selected.push(vertex.clone());
                }
            }
            selected
        }).collect();

        let duration2 = start.elapsed();
        println!("The duration for selecting the vertex randomly in par chunck : {:?}", duration2);


        let start = Instant::now();

        let checker = DashSet::new();
            for vertex in &chosen_vertexes {
                checker.insert(vertex.clone());
            }
            
            chosen_vertexes.par_iter().for_each(|v| {
                if let Some(neighbors) = graph.get(v) {
                    for nbr in neighbors {
                        if checker.contains(nbr) {
                            let degree_v = *degrees.get(v).unwrap();
                            let degree_nbr = *degrees.get(nbr).unwrap();
                            if degree_v > degree_nbr {
                                checker.remove(v);
                            } else {
                                checker.remove(nbr);
                            }
                        }
                    }
                }
            });
            let duration3 = start.elapsed();
            println!("The duration for removing the adjacent vertex in par chunck : {:?}", duration3);

            
        let start = Instant::now();
        vertexes.retain(|v| !checker.contains(v) && !graph.get(v).unwrap()
        .iter().any(|nbr| checker.contains(nbr)));

        let duration4 = start.elapsed();
        println!("The duration for removing vertexes in the vertexes in par chunck : {:?}", duration4);

        let start = Instant::now();
        mis.extend(checker);
        let duration = start.elapsed();
        println!("Putting the vertexes into the answer in parallel: {:?}", duration);
    }

    mis
}

pub fn luby_algo(graph: &GRAPH) -> HashSet<String> {


    let mut vertexes: HashSet<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();     
    let mut rng: ThreadRng = rand::rng();

    while !vertexes.is_empty() {

        let length = graph.len();

        let degrees: HashMap<String, usize> = vertexes.par_iter().map( |vertex: &String| {
            let degree = graph.get(vertex).unwrap().len();
            (vertex.clone(), degree)
        }).collect();

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

            
            let checker = DashSet::new();
            for vertex in &chosen_vertexes {
                checker.insert(vertex.clone());
            }
            
            chosen_vertexes.par_iter().for_each(|v| {
                if let Some(neighbors) = graph.get(v) {
                    for nbr in neighbors {
                        if checker.contains(nbr) {
                            let degree_v = *degrees.get(v).unwrap();
                            let degree_nbr = *degrees.get(nbr).unwrap();
                            if degree_v > degree_nbr {
                                checker.remove(v);
                            } else {
                                checker.remove(nbr);
                            }
                        }
                    }
                }
            });


        //remove all the MIS vertex and its neighbour from vertexes
        // so that we can stop while loop
        vertexes = vertexes.par_iter()
            .filter(|&v| 
                !checker.contains(v) && !graph.get(v)
                .unwrap_or(&vec![]).iter().any(|nbr| checker.contains(nbr))
            )
            .cloned().collect();

        // add the MIS vertexes into the answer
        mis.par_extend(checker.into_iter().collect::<Vec<String>>());
    }

    mis
}

pub fn luby_seq(graph: &GRAPH) -> HashSet<String> {



    let mut vertexes: HashSet<String> = graph.keys().cloned().collect();
    let mut mis: HashSet<String> = HashSet::new();     
    let mut rng = rand::rng();


    while !vertexes.is_empty() {



        let start = Instant::now();



        let degrees: HashMap<String, usize> = vertexes.iter().map(|vertex| {
            let degree = graph.get(vertex).unwrap().len();


            (vertex.clone(), degree)


        }).collect();

        let duration1 = start.elapsed();
        println!("The duration for making degrees sequentially : {:?}", duration1);
        

        let start = Instant::now();

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

        let duration1 = start.elapsed();
        // println!("The duration for randomly selecting vertex : {:?}", duration1);



        let start = Instant::now();

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
        let duration3 = start.elapsed();
        // println!("The duration for removing the adjacent vertex in par chunck : {:?}", duration3);





        let start = Instant::now();
        for vertex in &checker {
            mis.insert(vertex.clone());
            vertexes.remove(vertex);
            for nbr in graph.get(vertex).unwrap_or(&vec![]) {
                vertexes.remove(nbr);
            }
        }

        let duration4 = start.elapsed();
        // println!("The duration for removing vertexes in the vertexes in sequential and adding the ans : {:?}", duration4);





        
    }

    mis
}

pub fn is_valid_mis(graph: &HashMap<String, Vec<String>>, mis: &HashSet<String>) -> bool {
    
    //ensure that 2 adjacent nodes are not in the MIS
    for node in mis {
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if mis.contains(neighbor) {
                    return false; 
                }
            }
        }
    }
    
    //check that for each not present node make sure that there's at least one neighbour present in MIS
    for (node, neighbors) in graph {
        if !mis.contains(node) && !neighbors.iter().any(|nbr| mis.contains(nbr)) {
            return false; 
        }
    }

    true
}