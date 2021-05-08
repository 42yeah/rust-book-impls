fn largest<T>(arr: &[T]) -> T where T: PartialOrd + Copy {
    let mut largest = arr[0];
    for num in arr.iter() {
        if *num > largest {
            largest = *num;
        }
    }
    largest
}

fn main() {
    let array = vec![1, 2, 3];
    let barray = vec![1.0, 2.0, 3.3];
    println!("{}", largest(&array));
    println!("{}", largest(&barray));
}
