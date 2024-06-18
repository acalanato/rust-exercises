
fn largest_f(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 100, 65];
    let other_list = vec![10, 20, 50, 30];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    let result = largest_f(&other_list);
    println!("Largest number is: {largest}!");
    println!("Largest number is: {result}!");
}
