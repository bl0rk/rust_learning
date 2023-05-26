#![allow(dead_code)]

pub fn insertion_sort_vec<T>(mut to_order: Vec<T>) -> Vec<T> where T: PartialOrd { 
    // Loop through every elements (starting at index 1) and then go back every element with a for loop in every iteration and compare the elements with 
    // the one before it. 
    for mut cur_i in 1..to_order.len() { 
        // I do not know if having this variable is better or if just doing cur_i - 1 in the three required space would be better. 
        let mut las_i = cur_i - 1; 
        loop { 
            if to_order[las_i] > to_order[cur_i] { 
                to_order.swap(cur_i, las_i); 
            } 

            if las_i == 0 { 
                break; 
            } 

            cur_i -= 1; 
            las_i -= 1; 
        } 
    } 

    to_order 
} 

pub fn bubble_sort_vec<T>(mut to_order: Vec<T>) -> Vec<T> where T: PartialOrd { 
    // Loop through every element (starting at index 1) of the vector and compare it to the one before it, if the one before is greater then swap them. 
    // Do this inside another Loop which will end when no element has been swapped in the for Loop. 
    let mut swapped = true; 
    while swapped { 
        swapped = false; 
        for cur_i in 1..to_order.len() { 
            if to_order[cur_i-1] > to_order[cur_i] { 
                to_order.swap(cur_i-1, cur_i); 
                swapped = true; 
            } 
        } 
    } 

    to_order 
} 

pub fn selection_sort_vec<T>(mut to_order: Vec<T>) -> Vec<T> where T: PartialOrd { 
    // Loop through each index of the list and swap it with the lowest value in the rest of the list. 
    for cur_i in 0..to_order.len() { 
        let mut lower_i = cur_i; 

        for j in cur_i..to_order.len() { 

            if to_order[lower_i] > to_order[j] { 
                lower_i = j; 
            } 
        } 

        to_order.swap(cur_i, lower_i); 
    } 

    to_order 
} 

// Welcome to the probably worst implementation of a merge sort algorithm. 
pub fn merge_sort_vec<T>(mut to_order: Vec<T>) -> Vec<T> where T: PartialOrd + Clone { 
    // Recursively divide the vector into smaller pieces until it can't be divided anymore and then work back up sorting and combining the pieces. 
    if to_order.len() <= 1 { 
        return to_order; 
    } 

    merge_sort(&mut to_order[..]); 

    to_order 
} 

fn merge_sort<T>(spl: &mut [T]) where T: PartialOrd + Clone { 
    if spl.len() <= 1 { 
        return; 
    }

    let middle = (spl.len()-1)/2; 
    let len = spl.len(); 

    // Left 
    merge_sort(&mut spl[0..=middle]); 

    // Right 
    merge_sort(&mut spl[middle+1..len]); 

    let temp_vec = merge(&spl[0..=middle], &spl[middle+1..len]); 
    for i in 0..spl.len() { 
        spl[i] = temp_vec[i].clone(); 
    } 
} 

fn merge<T>(sub1: &[T], sub2: &[T]) -> Vec<T> where T: PartialOrd + Clone { 
    let mut return_vec: Vec<T> = Vec::with_capacity(sub1.len()+sub2.len()); 
    // Horrible Horrible Horrible 
    for _ in 0..return_vec.capacity() { 
        return_vec.push(sub1[0].clone()); 
    } 

    let (size1, size2) = (sub1.len(), sub2.len()); 
    let (mut i1, mut i2) = (0, 0); 
    let mut inisub = 0; 

    // The hell even is this. 
    while i1 < size1 && i2 < size2 { 
        if sub1[i1] <= sub2[i2] { 
            return_vec[inisub] = sub1[i1].clone(); 
            i1 += 1; 
        } else { 
            return_vec[inisub] = sub2[i2].clone(); 
            i2 += 1; 
        } 
        inisub += 1; 
    } 

    // Dislike this. 
    while i1 < size1 { 
        return_vec[inisub] = sub1[i1].clone(); 
        i1 += 1; 
        inisub += 1; 
    } 

    // Same as above. 
    while i2 < size2 { 
        return_vec[inisub] = sub2[i2].clone(); 
        i2 += 1; 
        inisub += 1; 
    } 

    return_vec 
} 

pub fn quick_sort_vec<T>(mut to_order: Vec<T>) -> Vec<T> where T: PartialOrd { 
    // Order a Vector by a pivot (in this implentation the pivot is the last entry in the vector) by looping through it and arranging the entries so that 
    // every entry smaller than the pivot is before it and every entry larger than it is after it. 
    quick_sort(&mut to_order[..]); 

    to_order 
} 

fn quick_sort<T>(to_order: &mut [T]) where T: PartialOrd { 
    if to_order.len() <= 1 { 
        return; 
    } 

    let mut j: isize = -1; 
    let len = to_order.len() - 1; 
    let mut pivot = len; 

    for i in 0..=len { 
        if to_order[i] <= to_order[len] { 
            j += 1; 
            // .try_into().unwrap() needed here becuase the compiler doesn't know that the value needs to start at -1 but won't be less than 0 here 
            to_order.swap(i, j.try_into().unwrap()); 
            pivot = j.try_into().unwrap(); 
        } 
    } 

    // Left 
    quick_sort(&mut to_order[..pivot]); 

    // Right 
    quick_sort(&mut to_order[pivot+1..=len]); 
} 