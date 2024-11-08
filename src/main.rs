use pagerank::{GraphManager, PageManager};

fn main() {
    let mut page_manager = PageManager::default();

    let mut g1 = GraphManager::first(&mut page_manager);
    GraphManager::iteratively_update_pr(&mut g1, 5, 0.85);

    let mut g2 = GraphManager::second(&mut page_manager);
    GraphManager::iteratively_update_pr(&mut g2, 5, 0.85);

    let mut g3 = GraphManager::third(&mut page_manager);
    GraphManager::iteratively_update_pr(&mut g3, 5, 0.85);

    let mut g4 = GraphManager::fourth(&mut page_manager);
    GraphManager::iteratively_update_pr(&mut g4, 5, 0.85);
}
