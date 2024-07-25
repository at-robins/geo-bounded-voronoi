//! This module handles parsing of input data.

use getset::Getters;
use serde::{Deserialize, Serialize};

/// A set of 2d points bound by a specified polygon.
#[derive(Getters, Clone, Debug, Deserialize, Serialize)]
pub struct BoundedPointSet {
    /// The set of 2d points.
    #[getset(get = "pub")]
    point_set: Vec<[f64; 2]>,
    /// The bounding polygon.
    #[getset(get = "pub")]
    bound: Vec<[f64; 2]>,
}
