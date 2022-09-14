//! Sketch approximation

use std::collections::BTreeSet;

use crate::objects::Sketch;

use super::{face::FaceApprox, Approx, ApproxCache, Tolerance};

impl Approx for &Sketch {
    type Approximation = BTreeSet<FaceApprox>;

    fn approx_with_cache(
        self,
        tolerance: Tolerance,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        self.faces().approx_with_cache(tolerance, cache)
    }
}
