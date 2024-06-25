// This is a simple example of a trait in Rust
// Trait is a way to define a set of methods that a type must implement.
// A trait is similar to an interface in other languages.
trait Animal {
    fn speak(&self);
    fn sleep_time(&self);
    fn speed(&self);
    fn age(&self);
}

// A struct is a way to define a custom data type in Rust.
// You can implement a method for a struct using the impl keyword.
struct Dog {
    name: String,
    age: u8,
}

// The Dog struct implements the Animal trait.
impl Animal for Dog {
    fn speak(&self) {
        println!("{} says Woof!", self.name); // Woof!
    }

    fn sleep_time(&self) {
        println!("{} sleeps at 9:00 pm", self.name); // 9:00 pm
    }

    fn speed(&self) {
        println!("{} runs at 20 km/h", self.name);
    }

    fn age(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}


struct Cat {
    name: String,
    age: u8,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says Meow!", self.name); // Meow!
    }

    fn sleep_time(&self) {
        println!("{} sleeps at 10:00 pm", self.name); // 10:00 pm
    }

    fn speed(&self) {
        println!("{} runs at 15 km/h", self.name);
    }

    fn age(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}


// The function is generic over type T, which implements the Ord trait.
// Ord trait can be used to compare values of type T.
// array is a reference to a value of type T.
fn binary_search<T: Ord>(array: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if &array[mid] == target {
            return Some(mid);
        } else if &array[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

// The recursive_binary_search function
// The function is generic over type T, which implements the Ord trait.
// array is a reference to a value of type T.
fn recursive_binary_search<T: Ord>(array: &[T], target: &T, left: usize, right: usize) -> Option<usize> {
    if left > right {
        return None;
    }

    let mid = left + (right - left) / 2;

    if &array[mid] == target {
        Some(mid)
    } else if &array[mid] < target {
        recursive_binary_search(array, target, mid + 1, right)
    } else {
        recursive_binary_search(array, target, left, mid - 1)
    }
}

// The merge_sort function sorts an array using the merge sort algorithm.
// The function is generic over type T, which implements the Ord and Copy traits.
// array is a mutable reference to a value of type T.
// The function separates the array into two halves, sorts each half, and merges the sorted halves.
fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    let mut temp = array.to_vec();
    merge(&array[..mid], &array[mid..], &mut temp[..]);
    array.copy_from_slice(&temp);
}

// The merge function merges two sorted arrays into a single sorted array.
// The function compares the elements of the two arrays and adds the smaller element to the merged array.
fn merge<T: Ord + Copy>(left: &[T], right: &[T], merged: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut merged_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            merged[merged_index] = left[left_index];
            left_index += 1;
        } else {
            merged[merged_index] = right[right_index];
            right_index += 1;
        }
        merged_index += 1;
    }

    while left_index < left.len() {
        merged[merged_index] = left[left_index];
        left_index += 1;
        merged_index += 1;
    }

    while right_index < right.len() {
        merged[merged_index] = right[right_index];
        right_index += 1;
        merged_index += 1;
    }
}

fn main() {
    let mut a: i8 = 5; //The 8-bit signed integer type.
    println!("a = {}", a);
    let mut b = a; //The copy of value.
    b = 10;
    println!("a = {}", a);
    println!("b = {}", b);
    {
        let b = &mut a; //The copy by reference.
        *b += 10;
        println!("b = {}", b);
    }
    println!("a = {}", a);
    

    println!("");

    let dog = Dog {
        name: String::from("Buddy"),
        age: 5,
    };

    let cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };

    dog.speak();
    dog.sleep_time();
    dog.speed();
    dog.age();

    println!("");

    cat.speak();
    cat.sleep_time();
    cat.speed();
    cat.age();

    println!("");

    let array: [i32; 7] = [1, 2, 3, 4, 8, 10, 15]; //An array of 3 32-bit signed integers (unmutable).
    let mut target = 3;

    // match is similar to the switch statement in other languages
    /*
    match case {
        pattern => expression,
        pattern => expression,
        pattern => expression,
    }
    */

    /*
    if let Some(index) = recursive_binary_search(&array, &target, 0, array.len() - 1) {
        println!("Found target at index: {}", index);
    } else {
        println!("Target not found in the array");
    }
    */

    match binary_search(&array, &target) {
        Some(index) => println!("Found target at index: {}", index),
        None => println!("Target not found in the array"),
    }

    target = 10;
    match recursive_binary_search(&array, &target, 0, array.len() - 1) {
        Some(index) => println!("Found target at index: {}", index),
        None => println!("Target not found in the array"),
    }

    let mut array = [10, 5, 2, 3, 7, 6, 8, 9, 4, 1];
    merge_sort(&mut array);
    println!("{:?}", array); // print the sorted array, :? is a debug formatter

    println!("");

}
