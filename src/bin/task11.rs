fn analyze_slice(s: &[i32]) -> i32 {
    let mut sum = 0;
    for i in s {
        sum += i;
    }
    sum
}

fn main() {
    let array = [1, 5, 5, 3, 4, 2, 4, 3, 5, 4, 3, 2, 4, 3];

    let sum = analyze_slice(&array[2..5]);

    println!("Sum = {sum}");
}