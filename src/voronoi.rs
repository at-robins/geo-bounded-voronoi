//! This module computes the bounded voronoi diagramm.

use std::borrow::Borrow;

use geo::{BooleanOps, BoundingRect, Contains, CoordsIter, LineString, Polygon};
use serde::{Deserialize, Serialize};
use voronoice::{BoundingBox, VoronoiBuilder};

use crate::input::{BoundedPointSet, Bounds};

pub fn compute_voronoi<T: Borrow<BoundedPointSet>>(
    bounded_point_set: T,
) -> Result<Vec<BoundedVoronoiCell>, &'static str> {
    let bounded_point_set: &BoundedPointSet = bounded_point_set.borrow();

    let bound = bounded_point_set.bounding_polygon()?;
    let bound_bounds = Bounds::from_polygon(&bound).ok_or("The bounding polygon is invalid.")?;
    let bound_point_set = Bounds::from_point_set(&bounded_point_set.point_set())
        .ok_or("The point set does not contain enough valid points.")?;

    let sites = bounded_point_set.voronoi_point_set();
    let voronoi_digramm = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(BoundingBox::new(
            voronoice::Point {
                x: bound_point_set.centre_x(),
                y: bound_point_set.centre_y(),
            },
            bound_point_set.diff_x() + bound_bounds.diff_x(),
            bound_point_set.diff_y() + bound_bounds.diff_y(),
        ))
        // .set_lloyd_relaxation_iterations(5) // This alters the initial sites.
        .build()
        .ok_or("No Voronoi diagramm could be built for the specified point set.")?;

    let cells = voronoi_digramm
        .iter_cells()
        .map(|cell| BoundedVoronoiCell {
            site: voronoi_point_to_array(cell.site_position()),
            cell: cell.iter_vertices().map(voronoi_point_to_array).collect(),
        })
        .try_fold(Vec::new(), |mut acc, cell| {
            acc.push(cell.apply_bound(&bound)?);
            Ok(acc)
        })?;
    Ok(cells)
}

/// Centers the polygon around the specified coordinates.
///
/// # Parameters
///
/// * `polygon` - the input polygon to center
/// * `x` - the x-coordinate of the new center
/// * `y` - the y-coordinate of the new center
fn center_polygon<T: Borrow<Polygon>>(polygon: T, x: f64, y: f64) -> Result<Polygon, &'static str> {
    let polygon: &Polygon = polygon.borrow();
    let centre = polygon
        .bounding_rect()
        .ok_or("Invalid polygon. Cannot calculate bounding rectangle.")?
        .center();
    let dif_x = centre.x - x;
    let dif_y = centre.y - y;
    let points: Vec<(f64, f64)> = polygon
        .exterior()
        .points()
        .map(|point| (point.x() - dif_x, point.y() - dif_y))
        .collect();
    Ok(Polygon::new(LineString::from(points), Vec::new()))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BoundedVoronoiCell {
    site: [f64; 2],
    cell: Vec<[f64; 2]>,
}

fn voronoi_point_to_array(point: &voronoice::Point) -> [f64; 2] {
    [point.x, point.y]
}

impl BoundedVoronoiCell {
    pub fn apply_bound<T: Borrow<Polygon>>(self, bound: T) -> Result<Self, &'static str> {
        let centered_bound = center_polygon(bound, self.site[0], self.site[1])?;
        let cell_polygon = Polygon::new(
            LineString::from(
                self.cell
                    .into_iter()
                    .map(|point| (point[0], point[1]))
                    .collect::<Vec<(f64, f64)>>(),
            ),
            Vec::new(),
        );
        // Creates intersections between bounding polygon and the voronoi cell
        // and selects the intersection that actually contains the original point.
        let mut bounded_cell = None;
        let geo_site = geo::Point::new(self.site[0], self.site[1]);
        for intersection in cell_polygon.intersection(&centered_bound) {
            if intersection.contains(&geo_site) {
                bounded_cell = Some(
                    intersection
                        .coords_iter()
                        .map(|coordinate| [coordinate.x, coordinate.y])
                        .collect(),
                );
                break;
            }
        }

        match bounded_cell {
            Some(cell_points) => Ok(BoundedVoronoiCell {
                site: self.site,
                cell: cell_points,
            }),
            None => Err("No intersection could be found between the bound and the voronoi cell."),
        }
    }
}
