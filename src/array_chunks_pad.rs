use std::iter::ArrayChunks;

pub struct ArrayChunksPad<I, const N: usize>
where
    I: Iterator,
    I::Item: Copy,
{
    iter: Option<ArrayChunks<I, N>>,
    filler: I::Item,
}

impl<I, const N: usize> ArrayChunksPad<I, N>
where
    I: Iterator,
    I::Item: Copy,
{
    pub fn new(iter: I, filler: I::Item) -> Self {
        Self {
            iter: Some(iter.array_chunks::<N>()),
            filler,
        }
    }
}

impl<I, const N: usize> Iterator for ArrayChunksPad<I, N>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            match iter.next() {
                None => {
                    let mut remainder = self
                        .iter
                        .take()
                        .unwrap()
                        .into_remainder()
                        .unwrap()
                        .peekable();

                    if remainder.peek().is_some() {
                        let mut result = [self.filler; N];

                        for (i, remains) in remainder.enumerate() {
                            result[i] = remains;
                        }

                        Some(result)
                    } else {
                        None
                    }
                }
                el => el,
            }
        } else {
            None
        }
    }
}
