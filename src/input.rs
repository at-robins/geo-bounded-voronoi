//! This module handles parsing of input data.

use core::f64;
use std::{borrow::Borrow, collections::HashSet};

use geo::{BoundingRect, LineString, Polygon};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

/// A set of 2d points bound by a specified polygon.
#[derive(Getters, Clone, Debug, Deserialize, Serialize)]
pub struct BoundedPointSet {
    /// The set of 2d points.
    point_set: Vec<[f64; 2]>,
    /// The bounding polygon.
    #[getset(get = "pub")]
    bound: Vec<[f64; 2]>,
}

impl BoundedPointSet {
    /// Returns the bounding polygon or an error if less than 3 points have been specified.
    pub fn bounding_polygon(&self) -> Result<Polygon, &'static str> {
        let bound_points: Vec<(f64, f64)> = self
            .bound()
            .into_iter()
            .map(|point| (point[0], point[1]))
            .collect();
        if bound_points.len() < 3 {
            Err("At least 3 points are needed to specify a bounding polygon.")
        } else {
            Ok(Polygon::new(LineString::from(bound_points), Vec::new()))
        }
    }

    /// The set of unique, filtered 2d points.
    pub fn point_set(&self) -> HashSet<Point2D> {
        (&self.point_set)
            .into_iter()
            .filter_map(|point| Point2D::new(point[0], point[1]))
            .collect()
    }

    /// Returns the point set as unique set of [`points`](voronoice::Point).
    pub fn voronoi_point_set(&self) -> Vec<voronoice::Point> {
        self.point_set()
            .into_iter()
            .map(|point| point.into())
            .collect()
    }
}

#[derive(Clone, Copy, CopyGetters, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    #[getset(get_copy = "pub")]
    x: f64,
    #[getset(get_copy = "pub")]
    y: f64,
}

impl Point2D {
    /// Tries to create a new 2 dimensional point from
    /// the specified coordinates.
    /// Fails if one of the specified coordinates is not normal.
    ///
    /// # Parameters
    ///
    /// * `x` - the x coordinate
    /// * `y` - the y coordinate
    pub fn new(x: f64, y: f64) -> Option<Self> {
        if x.is_normal() || y.is_normal() {
            None
        } else {
            Some(Point2D { x, y })
        }
    }
}

// Eq and Ord can be implemented as all non-normal
// coordinates have been filtered out during creation.
impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::hash::Hash for Point2D {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl From<Point2D> for voronoice::Point {
    fn from(value: Point2D) -> Self {
        voronoice::Point {
            x: value.x(),
            y: value.y(),
        }
    }
}

#[derive(Clone, Copy, CopyGetters, Debug, PartialEq, PartialOrd)]
pub struct Bounds {
    #[getset(get_copy = "pub")]
    min_x: f64,
    #[getset(get_copy = "pub")]
    max_x: f64,
    #[getset(get_copy = "pub")]
    min_y: f64,
    #[getset(get_copy = "pub")]
    max_y: f64,
}

impl Bounds {
    pub fn from_polygon<T: Borrow<Polygon>>(polygon: T) -> Option<Self> {
        polygon.borrow().bounding_rect().map(|bound| {
            let (min_x, min_y) = bound.min().x_y();
            let (max_x, max_y) = bound.max().x_y();
            Self {
                min_x,
                max_x,
                min_y,
                max_y,
            }
        })
    }

    pub fn from_point_set<T: Borrow<HashSet<Point2D>>>(point_set: T) -> Option<Self> {
        let point_set: &HashSet<Point2D> = point_set.borrow();
        if point_set.is_empty() {
            None
        } else {
            let mut min_x = f64::MAX;
            let mut max_x = f64::MIN;
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;

            for point in point_set {
                if min_x > point.x() {
                    min_x = point.x();
                }
                if max_x < point.x() {
                    max_x = point.x();
                }
                if min_y > point.y() {
                    min_y = point.y();
                }
                if max_y < point.y() {
                    max_y = point.y();
                }
            }
            Some(Self {
                min_x,
                max_x,
                min_y,
                max_y,
            })
        }
    }

    pub fn diff_x(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn diff_y(&self) -> f64 {
        self.max_y - self.min_y
    }

    pub fn centre_x(&self) -> f64 {
        self.min_x() + self.diff_x() / 2.0
    }
    pub fn centre_y(&self) -> f64 {
        self.min_y() + self.diff_y() / 2.0
    }
}
