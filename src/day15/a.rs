use super::common;

pub fn solve(input: &Vec<((i32, i32), u32)>, line: i32) -> u32 {
    let segments = input
        .iter()
        .filter_map(|&x| common::crossings(x, line))
        .collect::<Vec<_>>();
    let mut edges = segments
        .iter()
        .map(|(x, _)| (x, common::EdgeKind::Opening))
        .collect::<Vec<_>>();
    edges.extend(segments.iter().map(|(_, x)| (x, common::EdgeKind::Closing)));
    edges.sort_by(|(x, _), (y, _)| x.cmp(y));
    let mut prev_edge_pos = edges[0].0;
    let mut current_openings: i32 = 0;
    let mut result = 0;
    for (pos, kind) in edges {
        if current_openings != 0 {
            result += (pos - prev_edge_pos).abs();
        }
        prev_edge_pos = pos;
        match kind {
            common::EdgeKind::Opening => current_openings += 1,
            common::EdgeKind::Closing => current_openings -= 1,
        }
    }
    result as u32 - 1
}
