pub const N: usize = 100000;

pub fn binary_search (arr: &[u32], item: &u32, start: &usize, end:&usize ) -> usize {
    let mid = (start + end) / 2;

    if item == &arr[mid] {
        return mid;
    } else if item < &arr[mid] {
        return binary_search(arr, item, start, &(mid-1));
    } else {
        return binary_search(arr, item, &(mid+1), end);
    }
}