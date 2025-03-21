use std::iter;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut num = 1;
    let mut layer = 0usize;
    while num <= size * size {
        for i in layer..(size as usize - layer) {
            matrix[layer][i] = num;
            num += 1;
        }

        // 用索引 i 会被 Clippy 警告。但是又没一致性了，很操蛋的！
        #[allow(clippy::needless_range_loop)]
        for i in (layer + 1)..(size as usize - layer) {
            matrix[i][size as usize - layer - 1] = num;
            num += 1;
        }

        for i in (layer..(size as usize - layer - 1)).rev() {
            matrix[size as usize - layer - 1][i] = num;
            num += 1;
        }

        for i in ((layer + 1)..(size as usize - layer - 1)).rev() {
            matrix[i][layer] = num;
            num += 1;
        }

        layer += 1;
    }

    matrix
}

// 大哥的代码：
// use std::iter;
// const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
// pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
//     let mut matrix = vec![vec![0; size]; size];
//     let mut movement = VECTORS.iter().cycle();
//     let (mut x, mut y, mut n) = (-1, 0, 1..);
//     for (move_x, move_y) in iter::once(size)
//         .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
//         .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
//     {
//         x += move_x;
//         y += move_y;
//         matrix[y as usize][x as usize] = n.next().unwrap();
//     }
//     matrix
// }

#[test]
fn test_spiral_matrix() {
    let x: Vec<_> = iter::once(5)
        .chain(
            (1..5).rev()          
                .flat_map(|n| iter::repeat(n).take(2)) 
        ).collect();
    println!("x: {:?}", x);
    
    const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
        let mut matrix = vec![vec![0; size]; size];
        let mut movement = VECTORS.iter().cycle();
        let (mut x, mut y, mut n) = (-1, 0, 1..);
        for (move_x, move_y) in iter::once(size)
            .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
            .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
        {
            println!("(move_x: {}, move_y: {})", move_x, move_y);
            x += move_x;
            y += move_y;
            matrix[y as usize][x as usize] = n.next().unwrap();
        }
        matrix
    }
    
    spiral_matrix(5 );
}