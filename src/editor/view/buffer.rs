pub struct Buffer {
    inner: Vec<String>,
}

impl Buffer {
    pub fn line(&self, idx: usize) -> Option<&String> {
        self.inner.get(idx)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            inner: vec!["Hello, World!".to_string()],
        }
    }
}
