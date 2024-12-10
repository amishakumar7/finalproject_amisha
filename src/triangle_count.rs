use std::collections::HashSet;
use crate::graph::Graph;

impl Graph {
    // counts the triangles for a given node
    
    pub fn count_triangles_for_node(&self, node: u32) -> u32 {
        // gets the neighbors of the given node
        let neighbors = match self.neighbors(node) {
            Some(neighbors) => neighbors,
            None => return 0, // No neighbors, so no triangles
        };

        // counts triangles by looking for common neighbors between pairs of neighbors
        let mut triangle_count = 0;
        
        // checks if they are also connected for each pair of neighbors
        for &neighbor1 in neighbors {
            for &neighbor2 in neighbors {
                if neighbor1 != neighbor2 && self.neighbors(neighbor1).unwrap_or(&HashSet::new()).contains(&neighbor2) {
                    triangle_count += 1;
                }
            }
        }

        // I divided by 3 since each triangle is counted 3 times (once for each node)
        triangle_count / 3
    }

    // counts all triangles in the graph via iterating over each node
    pub fn count_all_triangles(&self) -> u32 {
        let mut total_triangles = 0;

        // to iterate through each node and count triangles
        for &node in self.nodes() {
            total_triangles += self.count_triangles_for_node(node);
        }

        total_triangles
    }
}