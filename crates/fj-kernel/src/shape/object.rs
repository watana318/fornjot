use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{validate::Validate, Handle, Shape, ValidationResult};

/// Marker trait for geometric and topological objects
pub trait Object:
    'static + Clone + PartialEq + Validate + private::Sealed
{
    /// Internal function
    ///
    /// Please consider using [`Shape::merge`] instead.
    fn merge_into(
        self,
        handle: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self>;
}

impl private::Sealed for Point<3> {}
impl private::Sealed for Curve<3> {}
impl private::Sealed for Surface {}

impl private::Sealed for Vertex<3> {}
impl private::Sealed for Edge<3> {}
impl private::Sealed for Cycle<3> {}
impl private::Sealed for Face {}

impl Object for Point<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Curve<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Surface {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        shape.get_handle_or_insert(self)
    }
}

impl Object for Vertex<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        let point = self
            .point()
            .merge_into(Some(self.point.canonical()), shape)?;
        shape.get_handle_or_insert(Vertex::new(point))
    }
}

impl Object for Edge<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        let curve = self
            .curve()
            .merge_into(Some(self.curve.canonical()), shape)?;

        // Can be cleaned up using `try_map`, once that is stable:
        // https://doc.rust-lang.org/std/primitive.array.html#method.try_map
        let vertices = self.vertices.map(|vertices| {
            vertices.map(|vertex| {
                let vertex = vertex.canonical();
                vertex.get().merge_into(Some(vertex), shape)
            })
        });
        let vertices = match vertices {
            Some([a, b]) => Some([a?, b?]),
            None => None,
        };

        shape.get_handle_or_insert(Edge::new(curve, vertices))
    }
}

impl Object for Cycle<3> {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        let mut edges = Vec::new();
        for edge in self.edges {
            let edge = edge.canonical();
            let edge = edge.get().merge_into(Some(edge), shape)?;
            edges.push(edge);
        }

        shape.get_handle_or_insert(Cycle::new(edges))
    }
}

impl Object for Face {
    fn merge_into(
        self,
        _: Option<Handle<Self>>,
        shape: &mut Shape,
    ) -> ValidationResult<Self> {
        match self {
            Face::Face(face) => {
                let surface =
                    face.surface.get().merge_into(Some(face.surface), shape)?;

                let mut exts = Vec::new();
                for cycle in face.exteriors.as_handle() {
                    let cycle = cycle.get().merge_into(Some(cycle), shape)?;
                    exts.push(cycle);
                }

                let mut ints = Vec::new();
                for cycle in face.interiors.as_handle() {
                    let cycle = cycle.get().merge_into(Some(cycle), shape)?;
                    ints.push(cycle);
                }

                shape.get_handle_or_insert(Face::new(
                    surface, exts, ints, face.color,
                ))
            }
            Face::Triangles(_) => shape.get_handle_or_insert(self),
        }
    }
}

mod private {
    pub trait Sealed {}
}
