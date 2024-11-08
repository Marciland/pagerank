use crate::{Page, PageManager};

#[derive(Default)]
pub struct Graph {
    pages: Vec<Page>,
}

impl Graph {
    fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    fn update_pr(&mut self, damping: f32) {
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

pub struct GraphManager;

impl GraphManager {
    pub fn iteratively_update_pr(graph: &mut Graph, iterations: u16, damping: f32) {
        for iteration in 0..iterations {
            println!("Iteration: {iteration}");
            graph.debug();
            graph.update_pr(damping);
        }
        graph.debug();
    }

    pub fn first(page_manager: &mut PageManager) -> Graph {
        let mut g1 = Graph::default();
        let mut a1 = page_manager.create_page();
        let mut b1 = page_manager.create_page();

        page_manager.add_link(&mut b1, &mut a1);
        page_manager.add_link(&mut a1, &mut b1);
        g1.add_page(a1);
        g1.add_page(b1);

        g1
    }

    pub fn second(page_manager: &mut PageManager) -> Graph {
        let mut g2 = Graph::default();
        let mut a2 = page_manager.create_page();
        let mut b2 = page_manager.create_page();
        let mut c2 = page_manager.create_page();

        page_manager.add_link(&mut a2, &mut c2);
        page_manager.add_link(&mut c2, &mut b2);
        page_manager.add_link(&mut b2, &mut a2);
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

        page_manager.add_link(&mut a3, &mut b3);
        page_manager.add_link(&mut b3, &mut c3);
        page_manager.add_link(&mut b3, &mut a3);
        page_manager.add_link(&mut a3, &mut c3);

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

        page_manager.add_link(&mut a4, &mut b4);
        page_manager.add_link(&mut b4, &mut c4);

        g4.add_page(a4);
        g4.add_page(b4);
        g4.add_page(c4);

        g4
    }
}
