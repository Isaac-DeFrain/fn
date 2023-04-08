use petgraph::graph::Graph;
use petgraph::dot::Dot;
use petgraph_evcxr::draw_dot;

fn main() {
    let mut g : Graph<&str, &str> = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    g.add_edge(a, b, "a -> b");
    // draw_graph(&g);

    let d = Dot::new(&g);
    draw_dot(d);
}

// TODO trait bounds are not satisfied...
