use parry3d_f64::shape::Segment;

use crate::{
    kernel::geometry::{Circle, Curve, Line},
    math::Point,
};

/// The edges of a shape
pub struct Edges {
    /// The cycles that the edges of the shape form
    ///
    /// Code reading this field generally assumes that cycles do not overlap.
    /// This precondition is currently not checked, and must be upheld by all
    /// code writing to this field.
    pub cycles: Vec<Cycle>,
}

impl Edges {
    /// Construct a new instance of `Edges`, with a single cycle
    pub fn single_cycle(edges: impl IntoIterator<Item = Edge>) -> Self {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        Self {
            cycles: vec![cycle],
        }
    }

    /// Compute vertices to approximate the edges
    ///
    /// `tolerance` defines how far these vertices are allowed to deviate from
    /// the actual edges of the shape.
    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point>) {
        for cycle in &self.cycles {
            cycle.approx_vertices(tolerance, out);
        }
    }

    /// Compute line segments to approximate the edges
    ///
    /// `tolerance` defines how far these line segments are allowed to deviate
    /// from the actual edges of the shape.
    pub fn approx_segments(&self, tolerance: f64, out: &mut Vec<Segment>) {
        for cycle in &self.cycles {
            cycle.approx_segments(tolerance, out);
        }
    }
}

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
pub struct Cycle {
    pub edges: Vec<Edge>,
}

impl Cycle {
    /// Compute vertices to approximate the edges of this cycle
    ///
    /// `tolerance` defines how far these vertices are allowed to deviate from
    /// the actual edges of the shape.
    ///
    /// No assumptions must be made about already existing contents of `out`, as
    /// this method might modify them.
    pub fn approx_vertices(&self, tolerance: f64, out: &mut Vec<Point>) {
        for edge in &self.edges {
            out.extend(edge.approx_vertices(tolerance));
        }

        // As this is a cycle, the last vertex of an edge could be identical to
        // the first vertex of the next. Let's remove those duplicates.
        out.dedup();
    }

    /// Compute segments to approximate the edges of this cycle
    ///
    /// `tolerance` defines how far these segments are allowed to deviate from
    /// the actual edges of the shape.
    ///
    /// No assumptions must be made about already existing contents of `out`, as
    /// this method might modify them.
    pub fn approx_segments(&self, tolerance: f64, out: &mut Vec<Segment>) {
        let mut vertices = Vec::new();
        self.approx_vertices(tolerance, &mut vertices);

        for segment in vertices.windows(2) {
            let v0 = segment[0];
            let v1 = segment[1];

            out.push([v0, v1].into());
        }
    }
}

/// An edge of a shape
#[derive(Clone, Debug)]
pub struct Edge {
    /// The curve that defines the edge's geometry
    ///
    /// In principle, curves could be reused for multiple edges. However, this
    /// requires a facility, here in `Edge`, to define the boundary of the edge
    /// on the curve.
    ///
    /// While such a facility doesn't exist, edges are assumed to be bounded by
    /// the points with `0` and `1` parameters on the curve. For a line, those
    /// would be the two points that define the line, for example.
    pub curve: Curve,

    /// Indicates whether the curve is reversed
    pub reverse: bool,
}

impl Edge {
    /// Create an arc
    ///
    /// So far, the name of this method is a bit ambitious, as only full circles
    /// are supported.
    pub fn arc(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle { radius }),
            reverse: false,
        }
    }

    /// Create a line segment
    pub fn line_segment(start: Point, end: Point) -> Self {
        Self {
            curve: Curve::Line(Line { a: start, b: end }),
            reverse: false,
        }
    }

    /// Reverse the edge
    pub fn reverse(&mut self) {
        self.reverse = !self.reverse;
    }

    /// Compute vertices to approximate the edge
    ///
    /// `tolerance` defines how far the implicit line segments between those
    /// vertices are allowed to deviate from the actual edge.
    pub fn approx_vertices(&self, tolerance: f64) -> Vec<Point> {
        let mut vertices = Vec::new();
        self.curve.approx_vertices(tolerance, &mut vertices);

        if self.reverse {
            vertices.reverse()
        }

        vertices
    }
}
