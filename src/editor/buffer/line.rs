use std::{cmp::min, ops::Range};
use unicode_segmentation::UnicodeSegmentation;

pub struct Line {
    inner: String,
}

impl Line {
    pub fn len(&self) -> usize {
        self.inner.graphemes(true).count()
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = min(range.end, self.len());

        self.inner
            .graphemes(true)
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }

    pub fn prefix_whitespace_count(&self) -> usize {
        self.inner.chars().take_while(|c| c.is_whitespace()).count()
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
