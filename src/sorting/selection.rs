// Сортировка выбором

pub fn sort<T>(list: &mut [T])
where
    T: PartialOrd + std::fmt::Debug + Ord,
{
    for i in 0..list.len() {
        if let Some((idx, _)) = list.iter().enumerate().skip(i).min_by_key(|x| x.1) {
            list.swap(i, idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::selection::sort;

    #[test]
    fn test_selection_sort() {
        let mut v = vec![11, 8, 4, 6, 13, 1, 3];
        sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
