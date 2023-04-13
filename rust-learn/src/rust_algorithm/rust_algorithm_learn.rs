use std::fmt::Debug;

fn poup_sort(arr:&mut Vec<i32>) -> &mut Vec<i32> {
    let mut tmp = arr[0];
    for (index,value) in arr.clone().iter().enumerate() {
        for k in 1..arr.len()-index {
            if arr[k-1] > arr[k] {
                arr.swap(k-1,k);
            }
        }
    }
    arr
}

fn select_sort(arr:&mut Vec<i32>)->Vec<i32>{
    let len = arr.len();
    for i in 0..len {
        let mut small_index = i;
        for j in i..len {
            if arr[i] > arr[j] {
                small_index = j;
            }
        }
        arr.swap(i,small_index);
    }
    arr.clone()
}

fn inser_sort<T>(arr:&Vec<T>)->Vec<T>
where T:PartialOrd+Debug+Copy{
    let mut res:Vec<T> = Vec::with_capacity(arr.len());
    for item in arr {
        for i in 0..arr.len() {
            if i == res.len() || res[i] > *item {
                res.insert(i,*item);
                break;
            }
        }
    }
    res
}

fn merge<T>(left:Vec<T>,right:Vec<T>)->Vec<T>
where T:Ord+Copy{
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut res:Vec<T> = Vec::with_capacity(left.len()+right.len());
    while i< left.len() && j<right.len() {
        if left[i] < right[j]{
            res.push(left[i]);
            i+=1;
        }else{
            res.push(right[j]);
            j+=1;
        }
    }
    while i<left.len() {
        res.push(left[i]);
        i+=1;
    }
    while j<right.len() {
        res.push(right[j]);
        j+=1;
    }
    res
}

fn merge_sort<T>(arr:Vec<T>)->Vec<T>
where T:Ord+Copy{
    if arr.len() == 1 {
        return arr;
    }
    let a = arr[0..arr.len()/2].to_vec();
    let b = arr[arr.len()/2..arr.len()].to_vec();
    merge(merge_sort(a),merge_sort(b))
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test(){
        let arr1 = vec![1,2,3,5];
        let arr2 = vec![4,6,7,8];
        let vec3 = merge_sort(arr1);
        println!("{:?}",vec3);

        let a:Vec<i32> = Vec::with_capacity(9);
        println!("{:?}",a.len());
        let mut arr = vec![6,4,5,8,3];
        // let sort = poup_sort(&mut arr);
        // println!("{:?}",sort);
        let vec1 = select_sort(&mut arr);
        println!("{:?}",vec1);
        let vec2 = inser_sort(&arr);
        println!("{:?}",vec2);
    }
}