pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let row = grid.len();
    let col = grid[0].len();
    let mut result: Vec<Vec<i32>> = vec![vec![0; col - 2 ]; row - 2];

    for i in 0..=row - 3 {    
        for j in 0..=col - 3 {       
            let mut max_value: i32 = 0;

            for x in 0..3 {
                for y in 0..3 {
                    max_value = max_value.max(grid[i + x][j + y]);
                }
                println!();
            }
            println!("{}", max_value);
            result[i][j] = max_value
        }
    }

    result
}