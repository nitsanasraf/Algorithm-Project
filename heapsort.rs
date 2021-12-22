fn heapify(arr: &mut Vec<isize>, size: usize, i: usize) {
    let mut largest: usize = i;
    let left: usize = 2 * i + 1;
    let right: usize = 2 * i + 2;

    // If left child is larger than root
    if left < size && arr[left] > arr[largest] {
        largest = left;
    }

    // If right child is larger than largest so far
    if right < size && arr[right] > arr[largest] {
        largest = right;
    }

    // If largest is not root
    if largest != i {
        arr.swap(i, largest);
        // Recursively heapify the affected sub-tree
        heapify(arr, size, largest);
    }
}

pub fn heap_sort(arr: &mut Vec<isize>, size: usize) {
    // Build heap (rearrange array)
    for i in (0..size / 2).rev() {
        heapify(arr, size, i);
    }

    // One by one extract an element from heap
    for i in (1..size).rev() {
        // Move current root to end
        arr.swap(0, i);
        // call max heapify on the reduced heap
        heapify(arr, i, 0);
    }
}
