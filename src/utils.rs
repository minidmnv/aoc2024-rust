pub fn get_middle_number(numbers: &Vec<i32>) -> Option<i32> {
    if numbers.is_empty() {
        return None; // Return None if the Vec is empty
    }
    let middle_index = numbers.len() / 2;
    Some(numbers[middle_index])
}

pub fn split_around_pivot(vec: Vec<i32>, pivot: usize) -> (Vec<i32>, Vec<i32>) {
    if pivot >= vec.len() {
        panic!("Pivot index out of bounds");
    }

    let left = vec[..pivot].to_vec();
    let right = vec[pivot + 1..].to_vec();

    (left, right)
}

pub fn find_element<T, I>(collection: I, target: T) -> Option<T>
    where
        T: PartialEq + Copy,
        I: IntoIterator<Item = T>,
{
    collection.into_iter().find(|&item| item == target)
}
