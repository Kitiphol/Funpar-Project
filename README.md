# Introduction

This project implements and evaluates a parallel version of Luby’s Algorithm for computing a Maximal Independent Set (MIS) on large graphs. The MIS problem is a fundamental problem in graph theory with applications in distributed computing and network design. The goal is to compare the performance of the parallel vs. sequential approaches and identify trade-offs.

## Methodology

The graph is represented as a HashMap<String, Vec<String>>, where each vertex maps to its neighbors. The implementation follows the randomized nature of Luby’s algorithm:

Sequential Algorithm: 
  * Implements the traditional Luby’s algorithm using a single thread.
Parallel Approaches:
  * Chunk-based Parallelism (par_chunks): Divides vertices into chunks to parallelize degree computation and random selection.
  * Iterator-based Parallelism (par_iter): Directly parallelizes over all vertices using Rayon’s par_iter().

Additional testing was done using DashSet to evaluate the performance of more parallelism.


## Result

Experiments were conducted on synthetic graphs with up to **50,000 nodes** in a cycle graph. The execution times (in milliseconds) for different versions are summarized below:

| Version                              | Execution Time |
|-------------------------------------|----------------|
| **Sequential Luby**                 | **52.84 ms**   |
| Parallel (Chunk-based)             | 72.30 ms       |
| Parallel (Chunk + DashSet)         | 80.71 ms       |
| Parallel (Iterator-based)          | 91.23 ms       |


## Observations
The sequential version consistently outperformed all parallel variants.
The chunked parallel implementation was faster than the iterator-based one due to reduced overhead and better locality.
Introducing DashSet for removing all unncessary vertexes slightly degraded performance due to synchronization overhead.


## Conclusion
Contrary to expectations, the sequential Luby’s algorithm was faster than both parallel versions for all tested graph sizes (up to 50,000 vertices). While parallelism introduces concurrency, it also brings overhead from synchronization, thread spawning, and cache contention—which can outweigh its benefits for moderately sized graphs.




## Contributing
https://courses.csail.mit.edu/6.852/08/papers/Luby.pdf


## License

[MIT](https://choosealicense.com/licenses/mit/)
