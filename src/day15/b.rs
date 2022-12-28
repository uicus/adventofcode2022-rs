use super::common;

pub fn solve(input: &Vec<((i32, i32), u32)>, limit: i32) -> i64 {
    for line in 0..limit {
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
        for (pos, kind) in edges {
            if current_openings == 0 {
                if pos - prev_edge_pos != 0 {
                    return *prev_edge_pos as i64 * 4000000 + line as i64;
                }
            }
            prev_edge_pos = pos;
            match kind {
                common::EdgeKind::Opening => current_openings += 1,
                common::EdgeKind::Closing => current_openings -= 1,
            }
        }
    }
    0
}
