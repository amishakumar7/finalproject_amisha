use crate::graph::Graph;
use std::collections::HashSet;

impl Graph {
    // computes the local clustering coefficient for a given node
    
    pub fn compute_local_clustering_coefficient(&self, node: u32) -> f64 {
        // gets the neighbors of the given node
        let neighbors = match self.neighbors(node) {
            Some(neighbors) => neighbors,
            None => return 0.0, // no clustering coefficient if there are no neighbors 
        };

        let degree = neighbors.len();
        if degree < 2 {
            return 0.0; // since no triangles are possible with fewer than two neighbors
        }

        // counts the number of triangles at the node
        let mut triangle_count = 0;

        // check each pair of neighbors to see if they are connected
        for &neighbor1 in neighbors {
            for &neighbor2 in neighbors {
                if neighbor1 != neighbor2 && self.neighbors(neighbor1).unwrap_or(&HashSet::new()).contains(&neighbor2) {
                    triangle_count += 1;
                }
            }
        }

        // divided by three since each triangle is counted three times (once for each node) 
        triangle_count /= 3;

        // calculates the number of possible triangles at this node
        let possible_triangles = degree * (degree - 1) / 2;

        // calculates the clustering coefficient
        triangle_count as f64 / possible_triangles as f64
    }

    // computes the average local clustering coefficient across all the nodes
    pub fn compute_average_clustering_coefficient(&self) -> f64 {
        let mut total_coefficient = 0.0;
        let mut node_count = 0;

        // iterates over all nodes to compute their local clustering coefficient
        for &node in self.nodes() {
            total_coefficient += self.compute_local_clustering_coefficient(node);
            node_count += 1;
        }

        // returns the average clustering coefficient
        if node_count > 0 {
            total_coefficient / node_count as f64
        } else {
            0.0
        }
    }
}
