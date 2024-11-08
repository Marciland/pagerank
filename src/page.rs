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

    pub fn add_link(&self, outgoing: &mut Page, incoming: &mut Page) {
        outgoing.outgoing.push(incoming.id);
        incoming.incoming.push(outgoing.id);
    }
}

#[derive(Clone)]
pub struct Page {
    id: u16,
    pr: f32,
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

    pub fn update_pr(&mut self, pages: &[Page], damping: f32) {
        let mut summed_incoming_pagerank = 0.0;
        for page_id in &self.incoming {
            if let Some(page) = pages.iter().find(|&page| page.id == *page_id) {
                summed_incoming_pagerank += page.pr / page.outgoing.len() as f32;
            }
        }
        self.pr = (1.0 - damping) + damping * (summed_incoming_pagerank)
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_pr(&self) -> f32 {
        self.pr
    }
}
