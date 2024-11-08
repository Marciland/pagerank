#[derive(Default)]
pub struct PageManager {
    next_page: u16,
}

impl PageManager {
    pub fn create_page(&mut self) -> Page {
        let page = Page::new(self.next_page);
        self.next_page += 1;

        page
    }

    pub fn add_link(outgoing: &mut Page, incoming: &mut Page) {
        outgoing.outgoing.push(incoming.id);
        incoming.incoming.push(outgoing.id);
    }
}

#[cfg(test)]
mod test_page_manager {
    use crate::PageManager;

    #[test]
    fn create_page() {
        let mut manager = PageManager::default();
        let id = 10;
        manager.next_page = id;

        let page = manager.create_page();

        assert_eq!(page.id, id);
        assert_eq!(manager.next_page, id + 1);
    }

    #[test]
    fn add_link() {
        let mut manager = PageManager::default();
        let mut outgoing = manager.create_page();
        let mut incoming = manager.create_page();

        PageManager::add_link(&mut outgoing, &mut incoming);

        assert!(outgoing.outgoing.contains(&incoming.id));
        assert!(incoming.incoming.contains(&outgoing.id));
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Page {
    id: u16,
    pr: f64,
    outgoing: Vec<u16>,
    incoming: Vec<u16>,
}

impl Page {
    fn new(id: u16) -> Self {
        Self {
            id,
            pr: 1.0,
            outgoing: Vec::new(),
            incoming: Vec::new(),
        }
    }

    pub fn update_pr(&mut self, pages: &[Page], damping: f64) {
        let mut summed_incoming_pagerank = 0.0;

        for page_id in &self.incoming {
            if let Some(page) = pages.iter().find(|&page| page.id == *page_id) {
                summed_incoming_pagerank += page.pr / page.outgoing.len() as f64;
            }
        }

        self.pr = (1.0 - damping) + damping * (summed_incoming_pagerank)
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_pr(&self) -> f64 {
        self.pr
    }
}

#[cfg(test)]
mod test_page {
    use super::Page;
    use crate::PageManager;

    #[test]
    fn new_page() {
        let id = 777;
        let page = Page::new(id);

        assert_eq!(page.id, id);
        assert_eq!(page.pr, 1.0);
    }

    #[test]
    fn get_id() {
        let id = 131;
        let page = Page::new(id);

        assert_eq!(page.get_id(), id);
    }

    #[test]
    fn get_pr() {
        let pr = 32.23;
        let mut page = Page::new(1);
        page.pr = pr;

        assert_eq!(page.get_pr(), pr);
    }

    #[test]
    fn update_pr_no_incoming() {
        let mut page = Page::new(0);

        page.update_pr(&[], 0.5);

        assert_eq!(page.pr, 0.5);
    }

    #[test]
    fn update_pr_with_incoming() {
        let mut page = Page::new(0);
        let mut incoming = Page::new(1);
        PageManager::add_link(&mut incoming, &mut page);

        page.update_pr(&[incoming], 0.5);

        assert_eq!(page.pr, 1.0);
    }

    #[test]
    fn update_pr_with_multiple_incoming() {
        let mut page = Page::new(0);
        let mut incoming1 = Page::new(1);
        let mut incoming2 = Page::new(2);
        PageManager::add_link(&mut incoming1, &mut page);
        PageManager::add_link(&mut incoming2, &mut page);

        page.update_pr(&[incoming1, incoming2], 0.5);

        assert_eq!(page.pr, 1.5);
    }

    #[test]
    fn update_pr_with_missing_links() {
        let mut page = Page::new(0);
        let incoming1 = Page::new(1);
        let incoming2 = Page::new(2);

        page.update_pr(&[incoming1, incoming2], 0.5);

        assert_eq!(page.pr, 0.5);
    }
}
