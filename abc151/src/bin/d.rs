#![allow(non_snake_case)]
use competitive_hpp::prelude::*;

#[fastout]
fn main() {
    input! {
        H:usize, W:usize,
        maze: [Chars; H],
    }
    let maze: Vec<Vec<char>> = maze;
    let dire: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut ans = 0;

    for (h, w) in iproduct!(0..H, 0..W) {
        let mut dist = vec![vec![0; W]; H];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((h, w));

        while !queue.is_empty() {
            if let Some((i, j)) = queue.pop_front() {
                for (dy, dx) in dire.iter().copied() {
                    let y = i as isize + dy;
                    let x = j as isize + dx;
                    if y >= 0 && y < H as isize && x >= 0 && x < W as isize {
                        let y = y as usize;
                        let x = x as usize;
                        if maze[y as usize][x as usize] == '.' {
                            if dist[y][x] != 0 {
                                continue;
                            }
                            let tmp = dist[i][j] + 1;
                            dist[y][x] = tmp;
                            if ans < tmp {
                                ans = tmp;
                            }
                            queue.push_back((y, x));
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
