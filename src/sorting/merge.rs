pub fn sort<T>(mut v: Vec<T>) -> Vec<T>
    where
        T: PartialOrd + std::fmt::Debug,
{
    // sort the left half,
    // sort the right half, O(n*log(n))
    // bring the sorted halfs togother O(n)
    if v.len() <= 1 {
        return v;
    }

    let mut res: Vec<T> = Vec::with_capacity(v.len());
    let half = v.split_off(v.len() / 2);
    let a = sort(v);
    let b = sort(half);

    // Bring them together again
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    let mut a_peek = a_iter.next();
    let mut b_peek = b_iter.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_iter.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_iter.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_iter);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val)
                }
                res.extend(b_iter);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{sorting::merge::sort};

    #[test]
    fn test_merge_sort() {
        let v = vec![11, 8, 4, 6, 13, 1, 3];

        let sorted = sort(v);
        assert_eq!(sorted, [1, 3, 4, 6, 8, 11, 13]);

        let v = vec!["man", "eats", "bread", "now"];

        let sorted = sort(v);
        assert_eq!(sorted, ["bread", "eats", "man", "now"]);
    }
}
