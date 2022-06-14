use std::cmp::Ordering;

use euclid::{default::Point2D, Trig};
use num_traits::{Float, FromPrimitive};

use crate::{
    core::{Options, _c},
    geometry::{rotate_lines, rotate_points, Line},
};

#[derive(Clone)]
struct EdgeEntry<F: Float + FromPrimitive + Trig> {
    pub ymin: F,
    pub ymax: F,
    pub x: F,
    pub islope: F,
}

struct ActiveEdgeEntry<F: Float + FromPrimitive + Trig> {
    pub s: F,
    pub edge: EdgeEntry<F>,
}

pub fn polygon_hachure_lines<F: Float + FromPrimitive + Trig>(
    polygon_list: &mut Vec<Vec<Point2D<F>>>,
    options: &Options,
) -> Vec<Line<F>> {
    let angle = options.hachure_angle.unwrap_or(0.0) + 90.0;
    let mut gap = options.hachure_gap.unwrap_or(0.0);
    if gap < 0.0 {
        gap = options.stroke_width.unwrap_or(0.0) * 4.0;
    }

    gap = f32::max(gap, 0.1);

    let center = Point2D::new(_c(0.0), _c(0.0));
    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(&polygon, &center, _c(angle)))
    }

    let mut lines = straight_hachure_lines(polygon_list, _c(gap));

    if angle != 0.0 {
        polygon_list
            .iter_mut()
            .for_each(|polygon| *polygon = rotate_points(&polygon, &center, _c(-angle)));
        lines = rotate_lines(&lines, &center, _c(-angle));
    }

    return lines;
}

fn straight_hachure_lines<F: Float + FromPrimitive + Trig>(
    polygon_list: &mut Vec<Vec<Point2D<F>>>,
    gap: F,
) -> Vec<Line<F>> {
    let mut vertex_array: Vec<Vec<Point2D<F>>> = vec![];
    for polygon in polygon_list.iter_mut() {
        if polygon.first() != polygon.last() {
            polygon.push(
                polygon
                    .first()
                    .expect("can not get first element of polygon")
                    .clone(),
            );
        }
        if polygon.len() > 2 {
            vertex_array.push(polygon.clone());
        }
    }

    let mut lines: Vec<Line<F>> = vec![];
    let gap = F::max(gap, _c(0.1));

    // create sorted edges table
    let mut edges: Vec<EdgeEntry<F>> = vec![];

    for vertices in vertex_array.iter() {
        let mut edge_extension = vertices[..]
            .windows(2)
            .filter_map(|w| {
                let p1 = w[0];
                let p2 = w[1];
                if p1.y != p2.y {
                    let ymin = F::min(p1.y, p2.y);
                    Some(EdgeEntry {
                        ymin,
                        ymax: F::max(p1.y, p2.y),
                        x: if ymin == p1.y { p1.x } else { p2.x },
                        islope: (p2.x - p1.x) / (p2.y - p1.y),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<EdgeEntry<F>>>();

        edges.append(&mut edge_extension);
    }

    edges.sort_by(|e1, e2| {
        if e1.ymin < e2.ymin {
            Ordering::Less
        } else if e1.ymin > e2.ymin {
            Ordering::Greater
        } else if e1.x < e2.x {
            Ordering::Less
        } else if e1.x > e2.x {
            Ordering::Greater
        } else if e1.ymax == e2.ymax {
            Ordering::Equal
        } else {
            let ordering = (e1.ymax - e2.ymax) / F::abs(e1.ymax - e2.ymax);
            if ordering > _c(0.0) {
                Ordering::Greater
            } else if ordering < _c(0.0) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    });

    if edges.is_empty() {
        return lines;
    }

    let mut active_edges: Vec<ActiveEdgeEntry<F>> = Vec::new();
    let mut y = edges.iter().next().unwrap().ymin;

    loop {
        if !edges.is_empty() {
            let ix = edges
                .iter()
                .enumerate()
                .find(|(_ind, v)| v.ymin > y)
                .map(|(ind, _v)| ind - 1);

            if let Some(indx) = ix {
                let removed_elements = edges.splice(0..indx + 1, vec![]);

                removed_elements
                    .into_iter()
                    .for_each(|ee| active_edges.push(ActiveEdgeEntry { s: y, edge: ee }));
            }
        }

        active_edges.retain(|ae| ae.edge.ymax > y);
        active_edges.sort_by(|ae1, ae2| {
            if ae1.edge.x == ae2.edge.x {
                Ordering::Equal
            } else {
                let ratio = (ae1.edge.x - ae2.edge.x) / F::abs(ae1.edge.x - ae2.edge.x);
                if ratio > _c(0.0) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        });
        if active_edges.len() > 1 {
            active_edges[..].windows(2).for_each(|ae| {
                let ce = &ae[0];
                let ne = &ae[1];
                lines.push(Line::from(&[
                    euclid::Point2D::new(ce.edge.x, y),
                    euclid::Point2D::new(ne.edge.x, y),
                ]));
            });
        }

        y = y + gap;
        active_edges.iter_mut().for_each(|ae| {
            ae.edge.x = ae.edge.x + (gap * ae.edge.islope);
        });
        if lines.is_empty() || active_edges.is_empty() {
            break;
        }
    }

    return lines;
}