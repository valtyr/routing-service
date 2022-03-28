use std::iter;

pub fn pairwise<I>(right: I) -> impl Iterator<Item = (Option<I::Item>, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = iter::once(None).chain(right.clone().into_iter().map(Some));
    left.zip(right)
}
