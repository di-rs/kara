pub struct Line {
    pub inner: String,
}

impl Line {
    pub const fn len(&self) -> usize {
        self.inner.len()
    }
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let string = String::from(value);

        #[cfg(debug_assertions)]
        {
            if string.is_empty() {
                assert_eq!(string.lines().count(), 0);
            } else {
                assert_eq!(string.lines().count(), 1);
            }
        }

        Self { inner: string }
    }
}
