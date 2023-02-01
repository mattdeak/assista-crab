struct SearchResult<T: Sized + Clone + Serializable> {
    pub document: T,
    pub score: f32,
}

pub trait SearchEngine<T: Sized + Clone + Serializable> {
    fn search(
        &self,
        query: &str,
        num_results: Option<u32>,
    ) -> Result<Vec<SearchResult<T>>, Box<dyn Error>>;
}
