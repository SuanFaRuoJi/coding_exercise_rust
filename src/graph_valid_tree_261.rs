use std::convert::TryInto;

pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    if n as usize != edges.len() + 1 {
        return false;
    }

    let mut visited: Vec<i32> = vec![-1;n as usize];
    let mut graph: Vec<Vec<usize>> = vec![Vec::new();n as usize];
    for edge in &edges {
        let from: usize = edge[0] as usize;
        let to: usize = edge[1] as usize;
        graph[from].push(to);
        graph[to].push(from);
    }
    let mut prev = -1;
    let mut q: Vec<usize> = Vec::new();
    let mut index = 0;
    q.push(0);
    while index < q.len() {
        let cur_node = q[index];
        index += 1;
        for child in &graph[cur_node] {
            if *child == visited[cur_node] as usize {
                continue
            }
            if visited[*child] != -1 {
                return false
            }
            visited[*child] = cur_node as i32;
            q.push(*child);
        }
    }
    return index == n as usize;
}