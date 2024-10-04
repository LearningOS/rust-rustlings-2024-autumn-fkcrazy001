/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::vec;

fn merge_sort<T>(array: &mut [T], l:usize,r:usize, tmp :&mut[T])
where T:std::cmp::PartialOrd + Copy ,
{
    if l >= r {
        return;
    }
    let mid = (r+l)/ 2;
    // println!("l={},r={},mid={}",l,r,mid);
    merge_sort(array, l, mid, tmp);
    merge_sort(array, mid+1, r, tmp);
    let mut ll = l;
    let mut rr =  mid + 1 ;
    for i in l..=r {
        if ll <= mid {
            if rr <= r {
                if array[ll] < array[rr] {
                    tmp[i]=array[ll];
                    ll+=1;
                } else {
                    tmp[i] = array[rr];
                    rr+=1;
                }
            } else {
                tmp[i]=array[ll];
                ll+=1;
            }
        } else if rr <= r {
                tmp[i]=array[rr];
                rr+=1;
        }
    }
    println!("{}-{}",l,r);
    for i in l..=r {
        array[i] = tmp[i];
        // println!("arr[{}]={}",i,array[i]);
    }
}

fn sort<T>(array: &mut [T]) where T:std::cmp::PartialOrd + Copy, {
    let mut tmp:Vec<T> = vec![array[0]; array.len()];
	merge_sort(array, 0, array.len()-1, &mut tmp);
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        // 37 57 73 75
        // 19 46 64 91
        // 19 37 46 57 64 73 75 91
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}