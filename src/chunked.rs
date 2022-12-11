pub struct ChunkIter<T: Iterator> {
    inner: T,

    chunk_size: usize,
}

impl<T: Iterator> Iterator for ChunkIter<T> {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::new();

        for _ in 0..self.chunk_size {
            match self.inner.next() {
                Some(n) => v.push(n),
                None => break,
            }
        }

        if v.len() == 0 {
            None
        } else {
            Some(v)
        }
    }
}

pub trait Chunked<T: Iterator> {
    fn chunks(self, size: usize) -> ChunkIter<T>;
}

impl<T: Iterator> Chunked<T> for T {
    fn chunks(self, size: usize) -> ChunkIter<T> {
        ChunkIter {
            inner: self,
            chunk_size: size,
        }
    }
}
