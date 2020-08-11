pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut i: usize = 0; let mut j: usize = 0;
    if i == 0 {
        return -1;
    }
    let width = matrix[0].len();
    let mut max: Option<i32> = None;
    let mut column_sum = vec![0; width];
    let mut row_sum = vec![0; width+1];
    while i < matrix.len() {
        j = i;
        column_sum = vec![0; width];
        while j < matrix.len() {
            let mut index = 0;
            while index < width {
                let cur = matrix[j][index];
                column_sum[index] += cur;
                row_sum[index+1] = row_sum[index] + column_sum[index];
                index += 1;
            }
            match merge_sort(&mut row_sum, 0, width, k) {
                Some(val1) => {
                    match max {
                        Some(val2) => {
                            if val1 > val2 {max = Some(val1);}
                        }
                        _ => {max = Some(val1);}
                    }
                }
                _ => {}
            }
            j += 1;
        }
        i += 1;
    }
    return max.unwrap();
}

fn merge_sort(prefix: &mut Vec<i32>, l: usize, r:usize, k: i32) -> Option<i32> {
    if l == r {
        return if prefix[l] > k {None} else {Some(prefix[l])};
    }

    let mut max= None;

    let mid = (l+r)/2;
    let l_max = merge_sort(prefix, l, mid, k);
    match l_max {
        Some(val1) => {
            match max {
                Some(val2) => {
                    if val1 > val2 {
                        max = Some(val1);
                    }
                },
                _ => {max = Some(val1);}
            }
        },
        _ => {}
    }

    let r_max = merge_sort(prefix, mid+1, r, k);
    match r_max {
        Some(val1) => {
            match max {
                Some(val2) => {
                    if val1 > val2 {
                        max = Some(val1);
                    }
                },
                _ => {max = Some(val1);}
            }
        },
        _ => {}
    }

    let mut l_copy: Vec<i32> = vec![0; mid-l+1];
    l_copy.clone_from_slice(&prefix[l..mid+1]);
    let mut r_copy: Vec<i32> = vec![0; r-mid];
    r_copy.clone_from_slice(&prefix[mid+1..r+1]);

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < l_copy.len() {
        loop {
            let cur = r_copy[j] - l_copy[i];
            match max {
                None => {max = Some(cur)},
                Some(val) => {
                    if cur > val && cur <= k {
                        max = Some(cur)
                    }
                }
            }
            if j+1 >= r_copy.len() || r_copy[j+1] - l_copy[i] > k {
                break;
            }
            j += 1;
        }
        if j+1 >= r_copy.len() {
            break;
        }
        i += 1;
    }

    let mut index: usize = l;
    i = 0; j = 0;
    while index <= r {
        if i >= l_copy.len() {
            prefix[index] = r_copy[j];
            j += 1;
        } else if j >= r_copy.len() {
            prefix[index] = l_copy[i];
            i += 1;
        } else {
            if l_copy[i] < r_copy[j] {
                prefix[index] = l_copy[i];
                i += 1;
            } else {
                prefix[index] = r_copy[j];
                j += 1;
            }
        }
        index += 1;
    }

    return max;
}