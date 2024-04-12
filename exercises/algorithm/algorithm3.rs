/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:PartialOrd>(array: &mut [T]){
    // 实现的是插入排序,交换元素也需要调用方法--！
    let mut i = 0 ;
    let n = array.len();
    while i < n {
        let mut j = i+1 ;
        let mut cur_index = i;
        let mut cur_min = &array[i];
        while j < n {
            if cur_min > &array[j] {
                cur_min = &array[j];
                cur_index = j;
            }
            j+=1
        };
        array.swap(i, cur_index);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
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