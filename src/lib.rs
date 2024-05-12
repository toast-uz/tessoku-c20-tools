use proconio::input;
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

const N: usize = 50;
const K: usize = 400;
const L: usize = 20;

#[derive(Clone, Debug)]
pub struct Input {
    pub ab: Vec<(usize, usize)>,
    pub c: Vec<Vec<usize>>,
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize, k: usize, _l: usize,
        ab: [(usize, usize); k],
        c: [[usize; n]; n],
    }
    Input { ab, c, }
}

#[derive(Clone, Debug)]
pub struct Output {
    pub area: Vec<usize>,
}

pub fn parse_output(f: &str) -> Result<Output, String> {
    let ans = match f.split('\n')
            .filter(|&x| !x.trim().is_empty()).map(|x|
            x.parse::<usize>()).collect::<Result<Vec<usize>, std::num::ParseIntError>>() {
        Ok(ans) => ans,
        Err(err) => { return Err(err.to_string()); },
    };

    if ans.len() != K {
        return Err("output length must be K".to_string());
    }

    if ans.iter().unique().sorted().cloned().collect_vec() != (1..=L).collect_vec() {
        return Err("output must include all areas".to_string());
    }

    Ok(Output { area: ans.iter().map(|&x| x - 1).collect_vec(), })
}

pub fn compute_score(input: &Input, output: &Output) -> (isize, String) {
    // compute connectivity
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::default(); K];
    for (i, j) in iproduct!(0..N, 0..N) {
        let mut neighbor = Vec::new();
        if i < N - 1 { neighbor.push((i + 1, j)); }
        if j < N - 1 { neighbor.push((i, j + 1)); }
        for &(i2, j2) in &neighbor {
            let x = input.c[i][j];
            let y = input.c[i2][j2];
            if x > 0 && y > 0 && x != y {
                adj[x - 1].insert(y - 1);
                adj[y - 1].insert(x - 1);
            }
        }
    }
    let mut area: Vec<HashSet<usize>> = vec![HashSet::default(); L];
    for k in 0..K { area[output.area[k]].insert(k); }
    let mut connected = true;
    for l in 0..L {
        let mut todo = vec![*area[l].iter().next().unwrap()];
        while let Some(k) = todo.pop() {
            area[l].remove(&k);
            for &k2 in &adj[k] {
                if area[l].contains(&k2) { todo.push(k2); }
            }
        }
        if !area[l].is_empty() {
            connected = false;
            break;
        }
    }
    let base_score = if connected { 1e6 } else { 1e3 };

    // compute disparity
    let mut p = vec![0; L];
    let mut q = vec![0; L];
    for k in 0..K {
        p[output.area[k]] += input.ab[k].0;
        q[output.area[k]] += input.ab[k].1;
    }
    let pmax = *p.iter().max().unwrap() as f64;
    let pmin = *p.iter().min().unwrap() as f64;
    let qmax = *q.iter().max().unwrap() as f64;
    let qmin = *q.iter().min().unwrap() as f64;
    let res = base_score * (pmin / pmax).min(qmin / qmax);

    (res.round() as isize, "".to_string())
}
