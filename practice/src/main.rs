fn main() {

    let nums = vec![1, 2, 3, 4, 5];

    let nums_iter = nums.into_iter();

    let result: Vec<i32> = nums_iter.map(|num| num * num).collect();
}