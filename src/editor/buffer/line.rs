use std::{cmp::min, ops::Range};

pub struct Line {
    inner: String,
}

impl Line {
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = min(range.end, self.len());

        self.inner.get(start..end).unwrap_or_default().to_string()
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
