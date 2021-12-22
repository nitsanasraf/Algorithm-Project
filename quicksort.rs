pub fn quick_sort(arr: &mut Vec<isize>) {
    let len: usize = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort(arr: &mut Vec<isize>, low: isize, high: isize) {
    if low < high {
        let p: isize = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition(arr: &mut Vec<isize>, low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index: isize = low - 1;
    let mut last_index: isize = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    return store_index;
}
