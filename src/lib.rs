use std::collections::{HashSet, VecDeque};

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn is_valid_cell(x: i32, y: i32) -> bool {
    digit_sum(x) + digit_sum(y) <= 25
}

#[allow(dead_code)]
fn bfs(x: i32, y: i32) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((x, y));
    visited.insert((x, y));

    let mut result = 0;
    let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = queue.pop_front() {
        result += 1;

        for (dx, dy) in moves.iter() {
            let new_x = x + dx;
            let new_y = y + dy;

            let key = (new_x, new_y);

            if !visited.contains(&key) && is_valid_cell(new_x, new_y) {
                visited.insert(key);
                queue.push_back(key);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let start_x = 1000;
        let start_y = 1000;

        assert_eq!(bfs(start_x, start_y), 148848);
    }
}
