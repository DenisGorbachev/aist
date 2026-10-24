pub fn iter_eq_by<L, R>(left: impl IntoIterator<Item = L>, right: impl IntoIterator<Item = R>, mut equal: impl FnMut(L, R) -> bool) -> bool {
    let mut right = right.into_iter();
    left.into_iter()
        .all(|left| right.next().is_some_and(|right| equal(left, right)))
        && right.next().is_none()
}
