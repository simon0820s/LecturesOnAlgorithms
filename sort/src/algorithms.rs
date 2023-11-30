pub fn insertion(mut arr: Vec<f32>) -> Vec<f32> {
    for i in 1..arr.len() {
        let mut j = i;
        //Insert list[i] into sorted_list
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
    arr
}

pub fn merge_sort(arr: &Vec<f32>) -> Vec<f32> {
    if arr.len() < 2 {
        arr.to_vec()
    } else {
        let size = arr.len() / 2;
        let left_arr: Vec<f32> = merge_sort(&arr[0..size].to_vec());
        let right_arr: Vec<f32> = merge_sort(&arr[size..].to_vec());
        let merged: Vec<f32> = merge(&left_arr, &right_arr);
        merged
    }
}

fn merge(left: &Vec<f32>, right: &Vec<f32>) -> Vec<f32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<f32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}
