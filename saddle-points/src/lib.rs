use itertools::Itertools;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }
    let row_max: Vec<_> = input.iter().map(|v| *v.iter().max().unwrap()).collect();
    let col_min: Vec<_> = (0..input[0].len())
        .map(|x| (0..input.len()).map(|y| input[y][x]).min().unwrap())
        .collect();

    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(i, j)| row_max[i] == col_min[j] && row_max[i] == input[i][j])
        .collect()
}
