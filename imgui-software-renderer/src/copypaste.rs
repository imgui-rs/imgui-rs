//! The tiny_skia::Transform::invert method is useful but private in
//! tiny_skia currently, so duplicated here for now.

mod copypaste {
    use tiny_skia::Transform;

    fn dcross(a: f64, b: f64, c: f64, d: f64) -> f64 {
        a * b - c * d
    }

    fn dcross_dscale(a: f32, b: f32, c: f32, d: f32, scale: f64) -> f32 {
        (dcross(a as f64, b as f64, c as f64, d as f64) * scale) as f32
    }

    pub fn compute_inv(ts: &Transform, inv_det: f64) -> Transform {
        Transform::from_row(
            (ts.sy as f64 * inv_det) as f32,
            (-ts.ky as f64 * inv_det) as f32,
            (-ts.kx as f64 * inv_det) as f32,
            (ts.sx as f64 * inv_det) as f32,
            dcross_dscale(ts.kx, ts.ty, ts.sy, ts.tx, inv_det),
            dcross_dscale(ts.ky, ts.tx, ts.sx, ts.ty, inv_det),
        )
    }

    fn is_nearly_zero_within_tolerance(value: f32, tolerance: f32) -> bool {
        debug_assert!(tolerance >= 0.0);
        value.abs() <= tolerance
    }

    fn inv_determinant(ts: &Transform) -> Option<f64> {
        let det = dcross(ts.sx as f64, ts.sy as f64, ts.kx as f64, ts.ky as f64);

        // Since the determinant is on the order of the cube of the matrix members,
        // compare to the cube of the default nearly-zero constant (although an
        // estimate of the condition number would be better if it wasn't so expensive).
        const SCALAR_NEARLY_ZERO: f32 = 1.0 / (1 << 12) as f32;

        let tolerance = SCALAR_NEARLY_ZERO * SCALAR_NEARLY_ZERO * SCALAR_NEARLY_ZERO;
        if is_nearly_zero_within_tolerance(det as f32, tolerance) {
            None
        } else {
            Some(1.0 / det)
        }
    }

    fn is_finite(x: &Transform) -> bool {
        x.sx.is_finite()
            && x.ky.is_finite()
            && x.kx.is_finite()
            && x.sy.is_finite()
            && x.tx.is_finite()
            && x.ty.is_finite()
    }

    pub fn invert(ts: &Transform) -> Option<Transform> {
        debug_assert!(!ts.is_identity());

        if ts.is_scale_translate() {
            if ts.has_scale() {
                let inv_x = 1.0 / ts.sx;
                let inv_y = 1.0 / ts.sy;
                Some(Transform::from_row(
                    inv_x,
                    0.0,
                    0.0,
                    inv_y,
                    -ts.tx * inv_x,
                    -ts.ty * inv_y,
                ))
            } else {
                // translate only
                Some(Transform::from_translate(-ts.tx, -ts.ty))
            }
        } else {
            let inv_det = inv_determinant(ts)?;
            let inv_ts = compute_inv(ts, inv_det);

            if is_finite(&inv_ts) {
                Some(inv_ts)
            } else {
                None
            }
        }
    }
}

pub(crate) use copypaste::invert;
