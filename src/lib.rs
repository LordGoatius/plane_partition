use plane_partition::{
    cardinality as card,
    complement::complement as comp,
    is_plane_partition as is_pp,
    rowmotion::{find_orbit, find_orbit_length, rowmotion as rowmotion_crate},
    strongly_stable_to_totally_stable, PlanePartition,
};
use pyo3::prelude::*;

pub mod plane_partition;

/// Prints the package version
#[pyfunction]
fn version() -> PyResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// Returns a string that represents the tikz diagram of a plane partition
/// ```python
/// import plane_partitions as pp
/// pp.to_tikz_diagram([[2,1],[1,0]])
/// ```
#[pyfunction]
fn to_tikz_diagram(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<String> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };

    Ok(format!(
        "{}",
        part
    ))
}

/// Takes in a strongly stable plane partition and returns a totally symetric plane partition
/// ```py
/// import plane_partitions as pp
/// pp.sspp_tp_tspp([[2,1],[1,0]])
/// # returns [[2,2],[2,2]]
/// ```
#[pyfunction]
fn sspp_tp_tspp(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<Vec<Vec<u8>>> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };
    Ok(
        strongly_stable_to_totally_stable(&part)
            .data,
    )
}

/// Takes in a plane partition and returns the resulting plane partition under the action of rowmotion
/// ```py
/// import plane_partitions as pp
/// pp.rowmotion([[0,0],[0,0]])
/// # returns [[1,0],[0,0]]
/// ```
#[pyfunction]
fn rowmotion(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<Vec<Vec<u8>>> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };

    Ok(rowmotion_crate(&part).data)
}

/// Returns the cardinality of a plane partition
#[pyfunction]
fn cardinality(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<usize> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };
    Ok(card(&part))
}

/// Returns the length of the orbit of a plane partition under rowmotion
/// Should be more efficient than finding the whole orbit instead of just the length
#[pyfunction]
fn rowmotion_orbit_length(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<usize> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };
    Ok(find_orbit_length(&part
    ))
}

/// Returns the list of all partitions in an orbit of a plane partition under rowmotion
#[pyfunction]
fn rowmotion_orbit(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<Vec<Vec<Vec<u8>>>> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };

    Ok(find_orbit(&part))
}

/// Returns whether a list of lists is a valid plane partition
#[pyfunction]
fn is_plane_partition(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<bool> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };
    Ok(is_pp(&part))
}

/// Finds the complement of a plane partition
#[pyfunction]
fn complement(matrix: Vec<Vec<u8>>, height: usize) -> PyResult<Vec<Vec<u8>>> {
    let part = PlanePartition {
        n: matrix.len(),
        m: matrix[0].len(),
        l: height,
        data: matrix,
    };
    Ok(comp(&part).data)
}

///Python module for working with plane plane partitions
///Written by Jimmy Ostler <jtostler1@gmail.com>
#[pymodule]
fn plane_partitions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlanePartition>()?;
    Ok(())
}
