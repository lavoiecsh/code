use crate::libs::integers::integer::Integer;

pub fn dynamic_programming<T: Integer>(values: &[T], target: T) -> T {
    let mut matrix = vec!();
    matrix.resize_with(target, || {
        let mut row = vec!();
        row.resize(values.len(), T::zero());
        row
    });
    let mut index = T::one();
    while index < target {
        matrix[index][0] = 1;
        for j in 1..matrix[index].len() {
            if values[j] > index {
                matrix[index][j] = 0;
            } else {
                matrix[index][j] = matrix[index][j-1] + matrix[index - values[j]][j];
            }
        }
        index += T::one();
    }
    matrix.last().unwrap().last().unwrap()
}
