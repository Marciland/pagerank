use crate::{Page, PageManager};

pub struct GraphManager;

impl GraphManager {
    pub fn iteratively_update_pr(graph: &mut Graph, iterations: u16, damping: f64) {
        for iteration in 0..iterations {
            println!("Iteration: {iteration}");
            graph.debug();
            graph.update_pr(damping);
        }
        println!("Iteration: {iterations}");
        graph.debug();
    }

    pub fn first(page_manager: &mut PageManager) -> Graph {
        let mut g1 = Graph::default();
        let mut a1 = page_manager.create_page();
        let mut b1 = page_manager.create_page();

        PageManager::add_link(&mut b1, &mut a1);
        PageManager::add_link(&mut a1, &mut b1);
        g1.add_page(a1);
        g1.add_page(b1);

        g1
    }

    pub fn second(page_manager: &mut PageManager) -> Graph {
        let mut g2 = Graph::default();
        let mut a2 = page_manager.create_page();
        let mut b2 = page_manager.create_page();
        let mut c2 = page_manager.create_page();

        PageManager::add_link(&mut a2, &mut c2);
        PageManager::add_link(&mut c2, &mut b2);
        PageManager::add_link(&mut b2, &mut a2);
        g2.add_page(a2);
        g2.add_page(b2);
        g2.add_page(c2);

        g2
    }

    pub fn third(page_manager: &mut PageManager) -> Graph {
        let mut g3 = Graph::default();
        let mut a3 = page_manager.create_page();
        let mut b3 = page_manager.create_page();
        let mut c3 = page_manager.create_page();

        PageManager::add_link(&mut a3, &mut b3);
        PageManager::add_link(&mut b3, &mut c3);
        PageManager::add_link(&mut b3, &mut a3);
        PageManager::add_link(&mut a3, &mut c3);

        g3.add_page(a3);
        g3.add_page(b3);
        g3.add_page(c3);

        g3
    }

    pub fn fourth(page_manager: &mut PageManager) -> Graph {
        let mut g4 = Graph::default();
        let mut a4 = page_manager.create_page();
        let mut b4 = page_manager.create_page();
        let mut c4 = page_manager.create_page();

        PageManager::add_link(&mut a4, &mut b4);
        PageManager::add_link(&mut b4, &mut c4);

        g4.add_page(a4);
        g4.add_page(b4);
        g4.add_page(c4);

        g4
    }
}

#[cfg(test)]
mod test_graph_manager {
    use super::GraphManager;
    use crate::PageManager;

    #[test]
    fn iteratively_update_pr() {
        let mut graph = GraphManager::third(&mut PageManager::default());

        GraphManager::iteratively_update_pr(&mut graph, 10, 0.85);

        // using values from the solution
        assert!(graph.pages[0]
            .get_pr()
            .to_string()
            .starts_with(&0.26101167.to_string()));
        assert!(graph.pages[1]
            .get_pr()
            .to_string()
            .starts_with(&0.26101167.to_string()));
        assert!(graph.pages[2]
            .get_pr()
            .to_string()
            .starts_with(&0.37202334.to_string()));
    }
}

#[derive(Default)]
pub struct Graph {
    pages: Vec<Page>,
}

impl Graph {
    fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    fn update_pr(&mut self, damping: f64) {
        let snapshot = self.pages.clone();

        for page in &mut self.pages {
            page.update_pr(&snapshot, damping)
        }
    }

    fn get_pages(&self) -> &[Page] {
        &self.pages
    }

    fn debug(&self) {
        for page in self.get_pages() {
            let id = page.get_id();
            let pr = page.get_pr();
            println!("{id}: {pr}")
        }
    }
}

#[cfg(test)]
mod test_graph {
    use super::Graph;
    use crate::PageManager;

    #[test]
    fn add_page() {
        let mut graph = Graph::default();
        let page = PageManager::default().create_page();

        graph.add_page(page.clone());

        assert!(graph.pages.contains(&page));
    }

    #[test]
    fn update_pr() {
        let mut graph = Graph::default();
        let page = PageManager::default().create_page();
        graph.add_page(page);

        graph.update_pr(0.5);

        assert_eq!(graph.pages[0].get_pr(), 0.5);
    }

    #[test]
    fn get_pages() {
        let mut graph = Graph::default();
        let p1 = PageManager::default().create_page();
        let p2 = PageManager::default().create_page();

        graph.add_page(p1.clone());
        graph.add_page(p2.clone());

        assert_eq!(graph.get_pages(), [p1, p2]);
    }
}
