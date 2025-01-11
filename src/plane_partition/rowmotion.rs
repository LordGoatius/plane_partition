use itertools::Itertools;

use super::PlanePartition;

pub fn find_orbit_length(matrix: &PlanePartition) -> usize {
    let mut curr = rowmotion(&matrix);
    let mut count = 1;
    while curr != *matrix {
        count += 1;
        curr = rowmotion(&curr);
    }
    count
}

pub fn find_orbit(matrix: &PlanePartition) -> Vec<Vec<Vec<u8>>> {
    let mut orbit = vec![];
    orbit.push(matrix.clone().data);
    let mut curr = rowmotion(&matrix);
    while curr != *matrix {
        orbit.push(curr.clone().data);
        curr = rowmotion(&curr);
    }
    orbit
}

pub fn rowmotion(matrix: &PlanePartition) -> PlanePartition {
    let len_n = matrix.n;
    let len_m = matrix.m;
    let len_l = matrix.l;

    let mut ret = PlanePartition {
        n: len_n,
        m: len_m,
        l: len_l,
        data: vec![vec![0; len_n]; len_n]
    };

    let poss_min_not_in = matrix
        .clone()
        .into_iter()
        .map(|row| row.into_iter().map(|x| (x + 1).clamp(0, len_l as u8)).collect_vec())
        .collect_vec();

    let min_not_in = poss_min_not_in.into_iter().enumerate().map(|(i, row)| {
        row.into_iter().enumerate().map(move |(j, elem)| {
            let left = if j == 0 { u8::MAX } else { matrix[i][j - 1] };
            let otop = if i == 0 { u8::MAX } else { matrix[i - 1][j] };
            if elem == matrix[i][j] {
                0
            } else if elem <= left && elem <= otop {
                elem
            } else {
                0
            }
        }).collect_vec()
    }).collect_vec();


    for i in (0..len_n).rev() {
        let mut min = 0;
        for j in (0..len_m).rev() {
            min = min.max(min_not_in[i][j]);
            ret[i][j] = min;
        }
    }

    for j in (0..len_n).rev() {
        let mut min = 0;
        for i in (0..len_m).rev() {
            min = min.max(min_not_in[i][j]).max(ret[i][j]);
            ret[i][j] = min;
        }
    }

    ret
}
