
/// Solve equations of the form `ax^2 + bx + c = 0`.
#[derive(PartialEq, Debug)]
pub enum QuadRoots {
    Real(f64, f64),
    Single(f64),
    None,
}

#[derive(PartialEq, Debug)]
pub enum CubicRoots {
    ThreeReal(f64, f64, f64),
    OneRealTwoComplex(f64, (f64, f64), (f64, f64)),
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadRoots {
    assert!(a != 0.0);
    let d = b * b - 4.0 * a * c;
    if d > 0.0 {
        return QuadRoots::Real((-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a));
    } else if d == 0.0 {
        return QuadRoots::Single(-b / (2.0 * a));
    } else {
        return QuadRoots::None;
    }
}

pub const EPSILON: f64 = 1e-10;

pub fn near_zero(x: f64) -> bool {
    return x < EPSILON && x > -EPSILON;
}


pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> CubicRoots {
    // From:
    // http://www.1728.org/cubic2.htm
    assert!(!near_zero(a));

    let f = c / a - (b * b) / (3.0 * a * a);
    let g = ((2.0 * b * b * b / a) - (9.0 * b * c) + (27.0 * d * a)) / (27.0 * a * a);
    let h = (g * g) / 4.0 + (f * f * f) / 27.0;

    let three: f64 = 3.0;
    if h > EPSILON {

        let r = -g / 2.0 + h.sqrt();
        let s = r.cbrt();
        let t = -g / 2.0 - h.sqrt();
        let u = t.cbrt();
        let real = -(s + u) / 2.0 - b / (3.0 * a);
        let img = (s - u) * three.sqrt() / 2.0;

        return CubicRoots::OneRealTwoComplex(s + u - b / (3.0 * a), (real, img), (real, -img));
    } else if near_zero(h) && near_zero(f) && near_zero(g) {
        let r = -(d / a).cbrt();

        return CubicRoots::ThreeReal(r, r, r);
    } else {
        // TODO check h < 0
        let i = (g * g / 4.0 - h).sqrt();
        let j = i.cbrt();
        let gh: f64;
        let gi = -g / (2.0 * i);
        if gi < -1.0 {
            gh = -1.0;
        } else if gi > 1.0 {
            panic!("In cubic solver, numeric stability problem. Replace this panic with gi = 1.0");
        } else {
            gh = gi;
        }
        let k = gh.acos() / 3.0;
        let l = -j;
        let m = k.cos();
        let n = three.sqrt() * k.sin();
        let p = -(b / (3.0 * a));

        return CubicRoots::ThreeReal(2.0 * j * k.cos() + p, l * (m + n) + p, l * (m - n) + p);
    }
}

/// Returns one of the two square roots of a complex number.
fn sqrt_cmplx((a, b): Cmplx) -> Cmplx {
    let r = (a * a + b * b).sqrt();
    let y = ((r - a) / 2.0).sqrt();
    let x = b / (2.0 * y);
    return (x, y);
}

fn round_near_zero(x: f64) -> f64 {
    if near_zero(x) {
        return 0.0;
    } else {
        return x;
    }
}

fn sqrt_real(x: f64) -> Cmplx {
    if x >= 0.0 {
        return (x.sqrt(), 0.0);
    } else {
        return (0.0, (-x).sqrt());
    }
}

pub type Cmplx = (f64, f64);

fn cmplx_add((x, xi): Cmplx, (y, yi): Cmplx) -> Cmplx {
    return (x + y, xi + yi);
}

fn cmplx_mult((x, xi): Cmplx, (y, yi): Cmplx) -> Cmplx {
    return (x * y - xi * yi, x * yi + y * xi);
}

fn cmplx_neg((x, xi): Cmplx) -> Cmplx {
    return (-x, -xi);
}

fn real_or_img_div(y: f64, (x, xi): Cmplx) -> Cmplx {
    if x == 0.0 {
        return (0.0, -y / xi);
    } else if xi == 0.0 {
        return (y / x, 0.0);
    } else {
        panic!("real_or_img_div got complex number.");
    }
}

pub fn solve_quartic(aa: f64, bb: f64, cc: f64, dd: f64, ee: f64) -> (Cmplx, Cmplx, Cmplx, Cmplx) {

    assert!(!near_zero(aa));
    let a = 1.0;
    let b = bb / aa;
    let c = cc / aa;
    let d = dd / aa;
    let e = ee / aa;
    let f = c - 3.0 * b * b / 8.0;
    let g = d + b * b * b / 8.0 - b * c / 2.0;
    let h = e - (3.0 * b * b * b * b / 256.0) + (b * b * c / 16.0) - (b * d / 4.0);
    let cb = solve_cubic(1.0, f / 2.0, (f * f - 4.0 * h) / 16.0, -g * g / 64.0);

    let p: Cmplx;
    let q: Cmplx;

    match cb {
        CubicRoots::ThreeReal(mut y1, mut y2, mut y3) => {
            y1 = round_near_zero(y1);
            y2 = round_near_zero(y2);
            y3 = round_near_zero(y3);
            let psq: f64;
            let qsq: f64;
            if y1 == 0.0 {
                psq = y2;
                qsq = y3;
            } else if y2 == 0.0 {
                psq = y1;
                qsq = y3;
            } else {
                psq = y1;
                qsq = y2;
            }
            p = sqrt_real(psq);
            q = sqrt_real(qsq);
        }
        CubicRoots::OneRealTwoComplex(_, (y1, y1i), (y2, y2i)) => {
            assert!(y1 == y2);
            assert!(y1i == -y2i);
            let (pr, pi) = sqrt_cmplx((y1, y1i));
            p = (pr, pi);
            q = (pr, -pi);
        }
    }
    let (pqr, pqi) = cmplx_mult(p, q);
    let r: Cmplx;
    if (pqr, pqi) == (0.0, 0.0) {
        r = (0.0, 0.0);
    } else {
        r = real_or_img_div(-g, (8.0 * pqr, 8.0 * pqi));
    }
    let s = b / (4.0 * a);
    return ((cmplx_add(cmplx_add(cmplx_add(p, q), r), (-s, 0.0))),
            (cmplx_add(cmplx_add(cmplx_add(p, cmplx_neg(q)), cmplx_neg(r)),
                       (-s, 0.0))),
            (cmplx_add(cmplx_add(cmplx_add(cmplx_neg(p), q), cmplx_neg(r)),
                       (-s, 0.0))),
            (cmplx_add(cmplx_add(cmplx_add(cmplx_neg(p), cmplx_neg(q)), r),
                       (-s, 0.0))));
}

fn flt_cmp(x: f64, y: f64) -> bool {
    return near_zero(x - y);
}

pub fn solve_quartic_smallest_positive_real(a: f64, b: f64, c: f64, d: f64, e: f64) -> Option<f64> {
    let mut smallest_real = 1.0 / 0.0;
    let ((r1, i1), (r2, i2), (r3, i3), (r4, i4)) = solve_quartic(a, b, c, d, e);
    if r1 < smallest_real && r1 > EPSILON && i1.abs() < EPSILON {
        smallest_real = r1;
    }
    if r2 < smallest_real && r2 > EPSILON && i2.abs() < EPSILON {
        smallest_real = r2;
    }
    if r3 < smallest_real && r3 > EPSILON && i3.abs() < EPSILON {
        smallest_real = r3;
    }
    if r4 < smallest_real && r4 > EPSILON && i4.abs() < EPSILON {
        smallest_real = r4;
    }
    if smallest_real.is_infinite() {
        return None;
    } else {
        return Some(smallest_real);
    }
}

fn cmplx_cmp((x, y): (f64, f64), (x1, y1): (f64, f64)) -> bool {
    return flt_cmp(x, x1) && flt_cmp(y, y1);
}

fn cmplx_cmp_4((x, y, z, u): (Cmplx, Cmplx, Cmplx, Cmplx),
               (x1, y1, z1, u1): (Cmplx, Cmplx, Cmplx, Cmplx))
               -> bool {
    return cmplx_cmp(x, x1) && cmplx_cmp(y, y1) && cmplx_cmp(z, z1) && cmplx_cmp(u, u1);
}

fn cmplx_pow(c: Cmplx, n: i32) -> Cmplx {
    let mut r = (1.0, 0.0);
    for _ in 0..n {
        r = cmplx_mult(r, c);
    }
    return r;
}

fn check_solution(solution: Cmplx, (a, b, c, d, e): (f64, f64, f64, f64, f64)) -> () {
    let mut t = cmplx_mult((a, 0.0), cmplx_pow(solution, 4));
    t = cmplx_add(t, cmplx_mult((b, 0.0), cmplx_pow(solution, 3)));
    t = cmplx_add(t, cmplx_mult((c, 0.0), cmplx_pow(solution, 2)));
    t = cmplx_add(t, cmplx_mult((d, 0.0), cmplx_pow(solution, 1)));
    let (r, i) = cmplx_add(t, (e, 0.0));
    assert!(r.abs() < EPSILON && i.abs() < EPSILON);
}

fn check_quartic(a: f64, b: f64, c: f64, d: f64, e: f64) -> () {
    let (s1, s2, s3, s4) = solve_quartic(a, b, c, d, e);
    check_solution(s1, (a, b, c, d, e));
    check_solution(s2, (a, b, c, d, e));
    check_solution(s3, (a, b, c, d, e));
    check_solution(s4, (a, b, c, d, e));
}

#[test]
fn test_quartic() {
    assert!(cmplx_cmp_4(solve_quartic(3.0, 6.0, -123.0, -126.0, 1080.0),
                        ((5.0, 0.0), (3.0, 0.0), (-4.0, 0.0), (-6.0, 0.0))));
    assert!(cmplx_cmp_4(solve_quartic(1.0, -5.0 / 20.0, -17.0 / 20.0, 29.0 / 20.0, -87.0 / 20.0),
                        ((1.48758311033, 0.0),
                         (0.222210408124, 1.29967219908),
                         (0.222210408124, -1.29967219908),
                         (-1.68200392658, 0.0))));
    check_quartic(1.0, 0.0, 0.0, 0.0, 0.0);
    check_quartic(1.0, 0.0, 0.0, 0.0, -1.0);
    check_quartic(1.0, 0.0, 0.0, 0.0, 1.0);
    check_quartic(3671.068438605304,
                  -1746.3860099225747,
                  -2300.726389983724,
                  3933.893547694215,
                  -310.96134916444583);
    check_quartic(4837.747294590245,
                  3850.56239982792,
                  1071.5065628786324,
                  4246.753768541942,
                  -4916.426421195232);
    check_quartic(4045.668098223205,
                  3521.431975103304,
                  1345.5650293366095,
                  -658.7986571357129,
                  -2023.230493246667);
    check_quartic(-4532.303951662107,
                  3348.9546185524464,
                  -1337.624783475272,
                  -545.8240198774666,
                  -2151.2666752465716);
    check_quartic(-3036.420315870136,
                  -9139.208975460038,
                  -4672.021580422994,
                  -715.6670967716772,
                  -9720.333591987857);
    check_quartic(0.0010147759073859076,
                  8.15079496605753e-06,
                  -0.0019196993484370938,
                  -0.004747046679606099,
                  -0.0019003462488357658);
    check_quartic(1.0,
                  -20.0,
                  150.01999999999998,
                  -500.19999999999993,
                  625.5001);
}

#[test]
fn test_qaudratic() {
    // x^2 + x = 0 => (0, -1)
    assert_eq!(solve_quadratic(1.0, 1.0, 0.0), QuadRoots::Real(0.0, -1.0));
    // x^2 + x + 1 = 0 => ()
    assert_eq!(solve_quadratic(1.0, 1.0, 1.0), QuadRoots::None);
    // x^2 - 2x + 1 = 0 => (1)
    assert_eq!(solve_quadratic(1.0, -2.0, 1.0), QuadRoots::Single(1.0));
}
