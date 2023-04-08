use dot_generator::*;
use dot_structures::*;
use graphviz_rust::{
    cmd::{CommandArg, Format},
    printer::PrinterContext,
    *,
};

fn main() {
    let g = graph!(di id!("control_state");
        node!("Send"),
        node!("Commit"),
        node!("Connect"),
        subgraph!("cluster_a";
            attr!("label", esc "requests + shared queues"),
            node!("Send"; attr!("color", "green")),
            edge!(node_id!("Send") => node_id!("TryCull") => node_id!("Send"))
        ),
        subgraph!("cluster_b";
            attr!("label", esc "components"),
            node!("Connect"),
            node!("TryCullC"; attr!("label", esc "TryCull\\nComponent")),
            edge!(node_id!("Connect") => node_id!("Lock") => node_id!("TryCullC") => node_id!("Connect"))
        ),
        subgraph!("c";
            node!("Connect"),
            node!("Commit"),
            edge!(node_id!("TryCullC") => node_id!("Commit")),
            edge!(node_id!("Commit") => node_id!("Send"))
        ),
        edge!(node_id!("TryCull") => node_id!("Connect"))
    );

    assert_eq!(
        g,
        parse(
            r#"
            digraph control_state {
                Send
                Commit
                Connect
                subgraph cluster_a {
                    label="requests + shared queues";
                    Send [color=green]
                    Send -> TryCull -> Send
                }
                subgraph cluster_b {
                    label="components";
                    Connect
                    TryCullC [label="TryCull\nComponent"]
                    Connect -> Lock -> TryCullC -> Connect
                }
                subgraph c {
                    Connect
                    Commit
                    TryCullC -> Commit
                    Commit -> Send
                }
                TryCull -> Connect
            }
        "#
        )
        .unwrap()
    );

    fn node_id_of_str(s: &str) -> NodeId {
        NodeId(Id::Plain(s.to_owned()), None)
    }
    let _ = node_id_of_str("a");

    exec(
        g.clone(),
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output("./graphs/graph.svg".to_string()),
        ],
    )
    .unwrap();

    exec(
        g,
        &mut PrinterContext::default(),
        vec![
            CommandArg::Format(Format::Dot),
            CommandArg::Output("./graphs/graph.dot".to_string()),
        ],
    )
    .unwrap();
}
