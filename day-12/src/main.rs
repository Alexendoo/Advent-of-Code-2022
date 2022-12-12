use pathfinding::directed::bfs::bfs;

fn main() {
    let input = include_str!("input");
    let board: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let start = board
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&ch| ch == b'S').map(|x| (x, y)))
        .unwrap();

    let path = bfs(
        &start,
        |&(x, y)| {
            let elevation = |x: usize, y: usize| {
                board.get(y)?.get(x).map(|&ch| match ch {
                    b'S' => b'a',
                    b'E' => b'z',
                    _ => ch,
                })
            };

            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .filter_map(move |(dx, dy)| {
                    let x_next = x.checked_add_signed(dx)?;
                    let y_next = y.checked_add_signed(dy)?;

                    let curr = elevation(x, y)?;
                    let next = elevation(x_next, y_next)?;

                    (curr + 1 >= next).then_some((x_next, y_next))
                })
        },
        |&(x, y)| board[y][x] == b'E',
    )
    .unwrap();

    println!("Part 1: {}", path.len() - 1);
}
