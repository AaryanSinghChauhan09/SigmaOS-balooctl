//! balooctl model — KDE file indexing/search (Baloo).

#[derive(Debug, Clone)]
pub struct IndexedFile {
    pub path: String,
    pub mime: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub files: Vec<IndexedFile>,
    pub enabled: bool,
}

impl Default for Model {
    fn default() -> Self { Self { files: vec![], enabled: true } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn index(&mut self, path: &str, mime: &str) -> bool {
        self.files.push(IndexedFile { path: path.into(), mime: mime.into(), size: 0 });
        true
    }

    pub fn search(&self, query: &str) -> Vec<&IndexedFile> {
        self.files.iter().filter(|f| f.path.contains(query)).collect()
    }

    pub fn suspend(&mut self) { self.enabled = false; }
    pub fn resume(&mut self) { self.enabled = true; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_search_suspend() {
        let mut b = Model::new();
        assert!(b.index("/home/user/doc.txt", "text/plain"));
        assert_eq!(b.search("doc").len(), 1);
        b.suspend();
        assert!(!b.enabled);
    }
}
