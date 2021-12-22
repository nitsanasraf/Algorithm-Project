mod heapsort;
mod is_sorted;
mod mergesort;
mod quicksort;
mod timing;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut step = 100;
    let times = 100;
    let max = 10000;
    while step <= max {
        let mut qs_sum = 0;
        let mut ms_sum = 0;
        let mut hs_sum = 0;
        let mut qs_unsorted_sum = 0;
        let mut ms_unsorted_sum = 0;
        let mut hs_unsorted_sum = 0;
        let mut sorted_arr: Vec<isize> = vec![];
        for x in 0..step {
            sorted_arr.push(x);
        }
        for _ in 0..times {
            let mut unsorted_arr: Vec<isize> = sorted_arr.to_vec();
            unsorted_arr.shuffle(&mut thread_rng());
            qs_sum += timing::quick_timing(&mut sorted_arr, quicksort::quick_sort);
            ms_sum += timing::merge_timing(&mut sorted_arr, mergesort::merge_sort);
            hs_sum += timing::heap_timing(&mut sorted_arr, heapsort::heap_sort);

            qs_unsorted_sum += timing::quick_timing(&mut unsorted_arr, quicksort::quick_sort);
            ms_unsorted_sum += timing::merge_timing(&mut unsorted_arr, mergesort::merge_sort);
            hs_unsorted_sum += timing::heap_timing(&mut unsorted_arr, heapsort::heap_sort);
        }
        println!(
            "Average for {} elements: {}ns - QUICKSORT: SORTED ARR",
            step,
            qs_sum / times
        );
        println!(
            "Average for {} elements: {}ns - MERGESORT: SORTED ARR ",
            step,
            ms_sum / times
        );
        println!(
            "Average for {} elements: {}ns - HEAPSORT: SORTED ARR ",
            step,
            hs_sum / times
        );

        println!(
            "Average for {} elements: {}ns - QUICKSORT: UNSORTED ARR",
            step,
            qs_unsorted_sum / times
        );
        println!(
            "Average for {} elements: {}ns - MERGESORT: UNSORTED ARR",
            step,
            ms_unsorted_sum / times
        );
        println!(
            "Average for {} elements: {}ns - HEAPSORT: UNSORTED ARR",
            step,
            hs_unsorted_sum / times
        );
        step += 100;
    }
}
