mod graph;
mod triangle_count;
mod clustering;

use graph::Graph;

fn main() {
    // Load the graph from a file
    let graph_file = "facebook_combined.txt"; 
    let graph = match Graph::load_graph_from_file(graph_file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to load graph from file {}: {}", graph_file, e);
            return;
        }
    };

    println!("Graph successfully loaded!");

  
    // Local clustering coefficients
    println!("Local Clustering Coefficients:");
    for &node in graph.nodes() {
        let coeff = graph.compute_local_clustering_coefficient(node);
        println!("Node {}: Local Clustering Coefficient = {:.4}", node, coeff);
    }
    
      // Triangle counting
    let total_triangles = graph.count_all_triangles();
    println!("Total triangles in the graph: {}", total_triangles);


    // Global clustering coefficient
    let avg_clustering_coeff = graph.compute_average_clustering_coefficient();
    println!("Average (Global) Clustering Coefficient: {:.4}", avg_clustering_coeff);
}