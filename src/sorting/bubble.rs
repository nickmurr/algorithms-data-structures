// O(n^2)

pub fn sort<T>(v: &mut [T])
    where
        T: PartialOrd + std::fmt::Debug,
{
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..v.len() - 1 - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{sorting::bubble::sort};

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![11, 8, 4, 6, 13, 1, 3];

        sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
