use super::PlanePartition;

pub fn complement(matrix: &PlanePartition) -> PlanePartition {
    let len_n = matrix.n;
    let len_m = matrix.m;
    let len_l = matrix.l;

    let mut complement: PlanePartition = PlanePartition {
        n: len_n,
        m: len_m,
        l: len_l,
        data: vec![vec![len_l as u8; len_m]; len_n],
    };

    for i in (0..len_n).rev() {
        for j in (0..len_m).rev() {
            complement[i][j] = complement[i][j] - matrix[len_n - 1 - i][len_m - 1 - j];
        }
    }

    complement
}
