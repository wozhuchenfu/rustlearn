
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

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test(){
        let mut arr = vec![6,4,5,8,3];
        let sort = poup_sort(&mut arr);
        println!("{:?}",sort);
    }
}