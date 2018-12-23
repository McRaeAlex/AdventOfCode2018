fn main() {
    let input = 7403;
    let mut grid: Vec<Vec<i64>> = vec![vec![0; 300]; 300];
    // test: println!("{}", power_level(8, 3, 5));
    for i in 1..301 {
        for j in 1..301 {
            grid[i - 1][j - 1] = power_level(input, i as i64, j as i64);
        }
    }
    let mut max_xy = (0, 0);
    let mut max_val = 0;
    for i in 1..298 {
        for j in 1..298 {
            let sum = get_val_for_xy(&grid, i - 1, j - 1);
            if sum > max_val {
                max_xy = (i, j);
                max_val = sum;
            }
        }
    }

    println!("{:?}", max_xy);
}

fn power_level(serial_number: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let power = rack_id * y + serial_number;
    let power = power * rack_id;
    let power = get_hundreds_digit(power) - 5;
    power
}

fn get_hundreds_digit(num: i64) -> i64 {
    let mut digit = 0;
    let mut number = num;
    for _ in 0..3 {
        digit = number % 10;
        number = number / 10;
    }
    digit
}

fn get_val_for_xy(grid: &Vec<Vec<i64>>, x: usize, y: usize) -> i64 {
    let mut sum = 0;
    for i in 0..3 {
        for j in 0..3 {
            sum += grid[x + i][y + j];
        }
    }
    sum
}
