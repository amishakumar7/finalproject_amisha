Final Project - Amisha Kumar (Write up also attached as pdf)

Dataset: https://snap.stanford.edu/data/ego-Facebook.html The purpose of the project is to analyze the clustering properties of the dataset using triangle counts to measure the tendency of nodes to form tightly-knit groups/ the local clustering coefficient. I’m interested in seeing how closely connected groups of people (triangles) form in different social networks, hence analyzing the Facebook data set.

This data set is formatted specifically for graph analysis, with nodes representing users and edges representing relationships (friendships), making them directly applicable to triangle counting and clustering coefficient computation in Rust. The dataset is also large enough to work for a meaningful analysis of real-world social networks. I’m particularly interested in Facebook because I read about small-world phenomena on large platforms (people are only a few links away from each other despite social and geographic backgrounds).

Graph.rs

I used HashMap and HashSet to store the graph and perform efficient lookups for neighbors and edges. For reading the file to load graph data I used BufReader and File. The struct graph is the main data structure representing the graph, and adjacency_list is a hashmap where the key is a node, and the value is a vector of the node's neighbor.

For graph implementation, I created the function 'new' to initialize an empty graph with an empty HashMap. The function add_edge adds an undirected edge between nodes u and v. I used entry to ensure a vector exists for each node (or_insert_with(Vec::new) inserts a new empty vector if the node is not already in the HashMap.) '.push' appends v to the neighbors of u and appends u to the neighbors of v. The function load_graph_from_file reads a graph description from a file. It splits the line into u and v (edges) and calls add_edge to store the edge in the graph.

The function for degree calculation (degree) returns the number of neighbors (degree) of a given node using get, and if the node is not in the graph, it returns 0 (.map_or(0, |neighbors| neighbors.len())) The function neighbors returns a reference to the vector of neighbors for a given node or returns None if the node does not exist. The fmt function iterates through the HashMap and prints each node and its neighbors from adjacency list.

Using HashMap and Vec ensures efficient lookups and traversal.

Triangle_count.rs

The function count_triangles_for_node counts the number of triangles a specific node is part of. It retrieves the node's neighbors from the graph's adjacency list (using match self.neighbors(node)) and then checks every pair of these neighbors to determine if they are also connected to each other using graph.is_connected. The triangle count increments if they are connected (share an edge). The triangle count is divided by 3 to give the correct total number of distinct triangles, correcting the triple counting.

Then, the count_all_triangles function iterates over every node in the graph, summing up their individual triangle counts.

Clustering.rs

The clustering module calculates how tightly each node is connected to its neighbors (local clustering coefficient) and then averages the measure for the entire graph (Average clustering coefficient). The function compute_local_clustering_coefficient function retrieves all of the neighbors of the given node using the neighbors() method of the Graph struct. If the node has fewer than two neighbors, it returns 0.0 because no triangles can form. If not, it counts how many of the node's neighbors are also connected to each other, indicating the presence of triangles. Since each triangle is counted once per node involved, the total is divided by three. This number is then compared to the maximum possible triangles that could form between the node's neighbors, which gives the node's local clustering coefficient (a value between 0 and 1)

Then, the compute_average_clustering_coefficient function computes this local measure for every node by iterating over all nodes in the graph and calling the function compute_local_clustering_coefficient for each node. It initializes a running total and counts how many nodes have been examined/processed. For each node retrieved from self.nodes(), the function adds the node's local clustering coefficient (compute_local_clustering_coefficient(node)) to the running total, and the node count is increment by 1. After each node is visited, the function divides the total local clustering coefficient by the total number of nodes, which gives the mean of the results. The average indicates how clustered the network is overall.

Test.rs

The tests verify that the clustering coefficient values computed by the program are valid.

How the tests work: A new graph is created. The test selects a particular node (node 1) and computes the local clustering coefficient graph.compute_local_clustering_coefficient(node). The test then uses an assertion assert!(coefficient >= 0.0 && coefficient <= 1.0, "Coefficient out of range") to check that the coefficient is between 0 and 1 since as we know a clustering coefficient represents a probability-esque measure of how connected a node's neighbors are. If the value lies outside this range, it means that there is a logical error in the code.

The other test calls graph.compute_average_clustering_coefficient(), and an assertion again checks whether the average coefficient falls within the range of 0 to 1, if it doesn't, the test fails, as something is incorrect in either the averaging logic or the calculations of individual node coefficients. (Test output screenshot in writeup pdf document)

Cargo run output

Total triangles in the graph: 3222693

Average (Global) Clustering Coefficient: 0.3825

(Local clustering screenshots at the end of the document in writeup pdf)

The total triangle count is high, given the network's size (4039 nodes and 88234). This suggests that many triples of people form fully connected triads, as in if you pick a random user and look at their friends, there is a high chance that those friends are also connected to each other. I can infer from this result that Facebook has closely knit groups rather than random links.

The average clustering coefficient of 0.3825 quantifies this conclusion as a value of 1.0 would mean that, on average, every pair of a user's friends are also friends with each other (a perfect cluster.) Hence, 0.38 indicates that while not all the friends of a friend are connected, the network is significantly more clustered than a random graph. This goes with the fact that friends tend to share mutual acquaintances on media, creating small-world effects. The metrics I found from my project reinforce the idea that social networks,, like Facebook's, evolve into groups of closely connected users rather than loosely or randomly connected users. In the future, I would love to see how the clustering and triangles in Facebook compare to Instagram and Twitter, where you can follow someone who doesn't have to follow you back.
