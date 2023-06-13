pub(crate) mod ledger;
pub(crate) mod merge;
pub(crate) mod prune_reroot;
pub(crate) mod remove;
pub(crate) mod weighted;

fn main() {
    // ledger::ledger_example();
    // merge::merge_example();
    prune_reroot::main();
    // weighted_tree::insert_weighted_block();
    // weighted_forest::forest_example;
}
