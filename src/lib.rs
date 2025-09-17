#![feature(iter_array_chunks)]

mod array_chunks_pad;
use array_chunks_pad::ArrayChunksPad;

pub trait ArrayChunksPadExtension: Iterator
where
    Self: Sized,
    Self::Item: Copy,
{
    fn array_chunks_pad<const N: usize>(self, filler: Self::Item) -> ArrayChunksPad<Self, N>;
}

impl<I> ArrayChunksPadExtension for I
where
    I: Iterator,
    I::Item: Copy,
{
    fn array_chunks_pad<const N: usize>(self, filler: Self::Item) -> ArrayChunksPad<Self, N>
    where
        Self: Sized,
        Self::Item: Copy,
    {
        ArrayChunksPad::new(self, filler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: Vec<u8>, expected: Vec<[u8; 4]>) {
        assert_eq!(
            input
                .into_iter()
                .array_chunks_pad::<4>(0)
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_0() {
        test(vec![1, 2, 3, 4], vec![[1, 2, 3, 4]]);
    }

    #[test]
    fn test_1() {
        test(vec![1, 2, 3, 4, 5], vec![[1, 2, 3, 4], [5, 0, 0, 0]]);
    }

    #[test]
    fn test_2() {
        test(vec![1, 2, 3, 4, 5, 6], vec![[1, 2, 3, 4], [5, 6, 0, 0]]);
    }

    #[test]
    fn test_3() {
        test(vec![1, 2, 3, 4, 5, 6, 7], vec![[1, 2, 3, 4], [5, 6, 7, 0]]);
    }

    #[test]
    fn test_4() {
        test(
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![[1, 2, 3, 4], [5, 6, 7, 8]],
        );
    }

    #[test]
    fn test_5() {
        test(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![[1, 2, 3, 4], [5, 6, 7, 8], [9, 0, 0, 0]],
        );
    }
}
