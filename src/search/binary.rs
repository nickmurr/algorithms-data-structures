#![allow(unused_doc_comments)]

/// binary_search - O(log*N)
pub fn search<T>(data: &[T], item: T) -> Option<usize>
where
    T: PartialOrd,
{
    /// low, high - contains limits of list part, we searching on
    // В переменных low и high хранятся границы той части списка,
    // в которой выполняется поиск
    let mut low = 0;
    let mut high = data.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = &data[mid];

        if guess == &item {
            return Some(mid); // element found
        }

        if guess > &item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::search::binary::search;

    #[test]
    fn test_binary_search_i32() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(search(&arr, 1), Some(0));
        drop(arr);
    }

    #[test]
    fn test_binary_search_string() {
        let arr: Vec<&str> = vec!["a", "b", "c", "d", "f"];
        assert_eq!(search(&arr, "f"), Some(4));
    }
}
