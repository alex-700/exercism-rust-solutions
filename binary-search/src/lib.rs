pub fn find<T: Copy + std::cmp::Ord>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    let size = (&array).as_ref().len();
    if size == 0 {
        return None;
    }

    let elt = |x| unsafe { (&array).as_ref().get_unchecked(x) };
    let mut l = 0;
    let mut r = size;
    while l + 1 < r {
        let m = l + (r - l) / 2;
        *(if elt(m) <= &key { &mut l } else { &mut r }) = m;
    }
    Some(l).filter(|&x| elt(x) == &key)
}
