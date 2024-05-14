const INFINITY: i32 = i32::MAX;

pub fn tsp_dp(dist: &[Vec<i32>], n: usize) -> (i32, Vec<usize>) {
    let mut memo = vec![vec![i32::MAX; 1 << n]; n];
    let mut path = vec![vec![usize::MAX; 1 << n]; n]; 

    let min_cost = tsp(0, 1, n, dist, &mut memo, &mut path);
    let route = find_path(n, &path, 0);

    (min_cost, route)
}

pub fn tsp(pos: usize, visited: usize, n: usize, dist: &[Vec<i32>], memo: &mut Vec<Vec<i32>>, path: &mut Vec<Vec<usize>>) -> i32 {
    if visited == (1 << n) - 1 {
        return dist[pos][0];
    }

    if memo[pos][visited] != i32::MAX {
        return memo[pos][visited];
    }

    let mut res = i32::MAX;
    for i in 0..n {
        if (visited & (1 << i)) == 0 && dist[pos][i] != INFINITY {
            let next_cost = tsp(i, visited | (1 << i), n, dist, memo, path);
            if next_cost != INFINITY { 
                let cost = dist[pos][i] + next_cost;
                if cost < res {
                    res = cost;
                    path[pos][visited] = i;
                }
            }
        }
    }

    memo[pos][visited] = res;
    res
}

pub fn find_path(n: usize, path: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut route = vec![start];
    let mut current = start;
    let mut visited = 1 << start;

    while visited != (1 << n) - 1 {
        let next = path[current][visited];
        route.push(next);
        visited |= 1 << next;
        current = next;
    }
    route.push(start);
    route
}