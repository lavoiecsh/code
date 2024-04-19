use crate::libs::integers::integer::Integer;

pub fn dynamic_programming(values: &[usize], target: usize) -> usize {
    let mut matrix = vec!();
    matrix.resize_with(target + 1, || {
        let mut row = vec!();
        row.resize(values.len(), 0);
        row
    });
    let mut index = 0;
    while index <= target {
        matrix[index][0] = 1;
        for j in 1..matrix[index].len() {
            if values[j] > index {
                matrix[index][j] = matrix[index][j-1];
            } else {
                matrix[index][j] = matrix[index][j-1] + matrix[index - values[j]][j];
            }
        }
        index += 1;
    }
    *matrix.last().unwrap().last().unwrap()
}
