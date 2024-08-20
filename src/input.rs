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

/// A 2-dimensional point.
#[derive(Clone, Copy, CopyGetters, Debug, PartialEq, PartialOrd)]
pub struct Point2D {
    /// The x-coordinate.
    #[getset(get_copy = "pub")]
    x: f64,
    /// The y-coordinate.
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
    /// * `x` - the x-coordinate
    /// * `y` - the y-coordinate
    pub fn new(x: f64, y: f64) -> Option<Self> {
        if !x.is_finite() || x.is_subnormal() || !y.is_finite() || y.is_subnormal() {
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

/// The bounds / bounding rectangle of a polygon or point set.
#[derive(Clone, Copy, CopyGetters, Debug, PartialEq, PartialOrd)]
pub struct Bounds {
    /// The minimum x-coordinate of the bounding rectangle.
    #[getset(get_copy = "pub")]
    min_x: f64,
    /// The maximum x-coordinate of the bounding rectangle.
    #[getset(get_copy = "pub")]
    max_x: f64,
    /// The minimum y-coordinate of the bounding rectangle.
    #[getset(get_copy = "pub")]
    min_y: f64,
    /// The maximum y-coordinate of the bounding rectangle.
    #[getset(get_copy = "pub")]
    max_y: f64,
}

impl Bounds {
    /// Returns the bounds of a [`Polygon`] if applicable.
    ///
    /// # Parameters
    ///
    /// * `polygon` - the polygon to get the bounds for
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

    /// Returns the bounds of a [`Point2D`] set if applicable.
    ///
    /// # Parameters
    ///
    /// * `point_set` - the point set to get the bounds for
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

    /// Returns the width of the bounding rectangle.
    pub fn diff_x(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// Returns the height of the bounding rectangle.
    pub fn diff_y(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// Returns the x-coordinate of the centre of the bounding rectangle.
    pub fn centre_x(&self) -> f64 {
        self.min_x() + self.diff_x() / 2.0
    }

    /// Returns the y-coordinate of the centre of the bounding rectangle.
    pub fn centre_y(&self) -> f64 {
        self.min_y() + self.diff_y() / 2.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use geo::CoordsIter;

    use super::*;

    #[test]
    fn test_boundedpointset_bounding_polygon_valid() {
        let poly_points = vec![
            [-10.0, -20.0],
            [0.0, 0.0],
            [1.0, 15.0],
            [234.0, -2.1],
            [23.4, 0.1],
            [-10.0, -20.0],
        ];
        let bps = BoundedPointSet {
            point_set: vec![],
            bound: poly_points.clone(),
        };
        let bounding_poly = bps.bounding_polygon().unwrap();

        assert!(bounding_poly.interiors().is_empty());
        assert_eq!(bounding_poly.exterior().coords_iter().count(), poly_points.len());
        for (i, c) in bounding_poly.exterior_coords_iter().enumerate() {
            assert_eq!(c.x, poly_points[i][0]);
            assert_eq!(c.y, poly_points[i][1]);
        }
    }

    #[test]
    fn test_boundedpointset_bounding_polygon_invalid() {
        // To few points for a proper polygon.
        let poly_points = vec![[0.0, 0.0], [1.0, 1.0]];
        let bps = BoundedPointSet {
            point_set: vec![],
            bound: poly_points,
        };
        assert!(bps.bounding_polygon().is_err())
    }

    #[test]
    fn test_boundedpointset_point_set() {
        // 4 unique values.
        let point_set_duplicates = vec![
            [0.0, 0.0],
            [1.0, 1.0],
            [1.00000000000001, 1.0],
            [15.6, 19930.0],
            [1.0, 1.0],
        ];
        let points: Vec<Point2D> = point_set_duplicates
            .iter()
            .map(|p| Point2D::new(p[0], p[1]).unwrap())
            .collect();
        let bps = BoundedPointSet {
            point_set: point_set_duplicates,
            bound: vec![],
        };
        let point_set_unique = bps.point_set();
        assert_eq!(point_set_unique.len(), 4);
        for point in points {
            assert!(point_set_unique.contains(&point));
        }
    }

    #[test]
    fn test_boundedpointset_voronoi_point_set() {
        // 4 unique values.
        let point_set_duplicates = vec![
            [0.0, 0.0],
            [1.0, 1.0],
            [1.00000000000001, 1.0],
            [15.6, 19930.0],
            [1.0, 1.0],
        ];
        let bps = BoundedPointSet {
            point_set: point_set_duplicates.clone(),
            bound: vec![],
        };
        let point_set_voronoi = bps.voronoi_point_set();
        let point_set_voronoi_converted: Vec<[f64; 2]> =
            point_set_voronoi.iter().map(|p| [p.x, p.y]).collect();
        assert_eq!(point_set_voronoi.len(), 4);
        assert_eq!(point_set_voronoi.len(), point_set_voronoi_converted.len());
        for point in point_set_duplicates {
            assert!(point_set_voronoi_converted.contains(&point));
        }
    }

    #[test]
    fn test_point2d_new_valid() {
        let x = -10.0;
        let y = 28.0;
        let point = Point2D::new(x, y).unwrap();
        assert_eq!(point.x(), x);
        assert_eq!(point.y(), y);
    }

    #[test]
    fn test_point2d_new_invalid() {
        assert!(Point2D::new(f64::INFINITY, -10.0).is_none());
        assert!(Point2D::new(f64::NEG_INFINITY, -10.0).is_none());
        assert!(Point2D::new(f64::NAN, -10.0).is_none());
        assert!(Point2D::new(1.0e-308_f64, -10.0).is_none());
        assert!(Point2D::new(0.0, -10.0).is_some());
        assert!(Point2D::new(10.0, f64::INFINITY).is_none());
        assert!(Point2D::new(10.0, f64::NEG_INFINITY).is_none());
        assert!(Point2D::new(10.0, f64::NAN).is_none());
        assert!(Point2D::new(10.0, 1.0e-308_f64).is_none());
        assert!(Point2D::new(10.0, 0.0).is_some());
    }

    #[test]
    fn test_bounds_from_polygon_valid() {
        let poly: Polygon = Polygon::new(
            LineString::from(vec![
                (-1.0, -2.0),
                (2.0, -5.0),
                (10.0, 20.0),
                (15.0, 55.0),
                (8.0, 4.0),
                (-1.0, -2.0),
            ]),
            Vec::new(),
        );
        let bounds = Bounds::from_polygon(poly).unwrap();
        assert_ulps_eq!(bounds.min_x(), -1.0);
        assert_ulps_eq!(bounds.max_x(), 15.0);
        assert_ulps_eq!(bounds.min_y(), -5.0);
        assert_ulps_eq!(bounds.max_y(), 55.0);
    }

    #[test]
    fn test_bounds_from_polygon_invalid() {
        let poly: Polygon = Polygon::new(LineString::from(Vec::<(f64, f64)>::new()), Vec::new());
        assert!(Bounds::from_polygon(poly).is_none());
    }

    #[test]
    fn test_bounds_from_point_set_valid() {
        let point_set: HashSet<Point2D> = vec![
            (-1.0, -2.0),
            (2.0, -5.0),
            (10.0, 20.0),
            (15.0, 55.0),
            (8.0, 4.0),
            (-1.0, -2.0),
        ]
        .into_iter()
        .map(|(x, y)| Point2D { x, y })
        .collect();
        let bounds = Bounds::from_point_set(point_set).unwrap();
        assert_ulps_eq!(bounds.min_x(), -1.0);
        assert_ulps_eq!(bounds.max_x(), 15.0);
        assert_ulps_eq!(bounds.min_y(), -5.0);
        assert_ulps_eq!(bounds.max_y(), 55.0);
    }

    #[test]
    fn test_bounds_from_point_set_invalid() {
        let bounds = Bounds::from_point_set(HashSet::new());
        assert!(bounds.is_none())
    }

    #[test]
    fn test_bounds_operations() {
        let point_set: HashSet<Point2D> = vec![
            (-1.0, -2.0),
            (2.0, -5.0),
            (10.0, 20.0),
            (15.0, 55.0),
            (8.0, 4.0),
            (-1.0, -2.0),
        ]
        .into_iter()
        .map(|(x, y)| Point2D { x, y })
        .collect();
        let bounds = Bounds::from_point_set(point_set).unwrap();
        assert_ulps_eq!(bounds.diff_x(), 16.0);
        assert_ulps_eq!(bounds.diff_y(), 60.0);
        assert_ulps_eq!(bounds.centre_x(), 7.0);
        assert_ulps_eq!(bounds.centre_y(), 25.0);
    }
}
