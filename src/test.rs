#[cfg(test)]
mod tests {
    use crate::graph::Graph;

    #[test]
    fn test_local_clustering_coefficient() {
        let graph = Graph::new(); 
        let node = 1; // Example node
        let coefficient = graph.compute_local_clustering_coefficient(node);
        assert!(coefficient >= 0.0 && coefficient <= 1.0, "Coefficient out of range");
    }

    #[test]
    fn test_average_clustering_coefficient() {
        let graph = Graph::new(); 
        let avg_coefficient = graph.compute_average_clustering_coefficient();
        assert!(avg_coefficient >= 0.0 && avg_coefficient <= 1.0, "Average coefficient out of range");
    }
}