pub trait Flattenable<T> {
    fn flatten(self) -> Vec<T>;
}

impl<const N: usize, T: Copy> Flattenable<T> for [T; N] {
    fn flatten(self) -> Vec<T> {
        self.to_vec()
    }
}

pub fn flatten<I, T, U>(iter: I) -> Vec<U>
where
    I: IntoIterator<Item = T>,
    T: Flattenable<U>,
{
    iter.into_iter().flat_map(|item| item.flatten()).collect()
}
