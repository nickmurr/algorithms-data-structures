fn sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    if n > 1 {
        let mut j = 0;
        for i in 0..n {
            if &v[i] < &v[n - 1] {
                v.swap(i, j);
                j += 1;
            }
        }
        v.swap(j, n - 1); // pivot
        sort(&mut v[0..j]);
        sort(&mut v[(j + 1)..n]);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::qsort::sort;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![11, 8, 4, 6, 13, 1, 3];

        sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
