use fj_math::Transform;

use crate::{
    iter::ObjectIters,
    objects::{Cycle, CyclesInFace, Edge, Face, FaceBRep, Vertex},
    shape::{LocalForm, Shape},
};

/// Transform the geometry of the shape
///
/// Since the topological types refer to geometry, and don't contain any
/// geometry themselves, this transforms the whole shape.
pub fn transform(shape: &Shape, transform: &Transform) -> Vec<Face> {
    let mut target = Vec::new();

    for face in shape.face_iter() {
        let face = match face {
            Face::Face(face) => {
                let mut tmp = Shape::new();
                let surface = face.surface.get().transform(transform);
                let surface = tmp.insert(surface);

                let exteriors = transform_cycles(&face.exteriors, transform);
                let interiors = transform_cycles(&face.interiors, transform);

                let color = face.color;

                Face::Face(FaceBRep {
                    surface,
                    exteriors,
                    interiors,
                    color,
                })
            }
            Face::Triangles(triangles) => {
                let mut target = Vec::new();

                for (triangle, color) in triangles {
                    let triangle = transform.transform_triangle(&triangle);
                    target.push((triangle, color));
                }

                Face::Triangles(target)
            }
        };
        target.push(face);
    }

    target
}

pub fn transform_cycles(
    cycles: &CyclesInFace,
    transform: &Transform,
) -> CyclesInFace {
    let mut tmp = Shape::new();

    let cycles = cycles.as_local_form().map(|cycle| {
        let edges_local = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve_local = *edge.local().curve.local();
                let curve_canonical = tmp
                    .merge(edge.canonical().get().curve().transform(transform));

                let vertices = edge.canonical().get().vertices.map(|vertex| {
                    let point = vertex.canonical().get().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = tmp.merge(Vertex { point });

                    LocalForm::new(local, canonical)
                });

                let edge_local = Edge {
                    curve: LocalForm::new(curve_local, curve_canonical.clone()),
                    vertices: vertices.clone(),
                };
                let edge_canonical = tmp.merge(Edge {
                    curve: LocalForm::canonical_only(curve_canonical),
                    vertices,
                });

                LocalForm::new(edge_local, edge_canonical)
            })
            .collect();
        let edges_canonical = cycle
            .canonical()
            .get()
            .edges
            .iter()
            .map(|edge| {
                let edge = edge.canonical().get();

                let curve = {
                    let curve = edge.curve().transform(transform);

                    let curve = tmp.merge(curve);
                    LocalForm::canonical_only(curve)
                };
                let vertices = edge.vertices.map(|vertex| {
                    let point = vertex.canonical().get().point;
                    let point = transform.transform_point(&point);

                    let local = *vertex.local();
                    let canonical = tmp.merge(Vertex { point });

                    LocalForm::new(local, canonical)
                });

                let edge = tmp.merge(Edge { curve, vertices });
                LocalForm::canonical_only(edge)
            })
            .collect();

        let cycle_local = Cycle { edges: edges_local };

        let cycle_canonical = tmp.merge(Cycle {
            edges: edges_canonical,
        });

        LocalForm::new(cycle_local, cycle_canonical)
    });

    CyclesInFace::new(cycles)
}
