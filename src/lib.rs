#[derive(Debug)]
#[repr(C)]
pub enum RoundingMode {
    ToNearest = 0,
    TowardZero = 1,
    Upward = 2,
    Downward = 3,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum Exception {
    None = 0,
    DivByZero = 1 << 0,
    Invalid = 1 << 1,
    Overflow = 1 << 2,
    Underflow = 1 << 3,
    Inexact = 1 << 4,
}

#[repr(C)]
struct Result32 {
    value: f32,
    exception: Exception,
}

impl Result32 {
    fn new() -> Self {
        Self {
            value: 0.0,
            exception: Exception::None,
        }
    }
}

mod ffi {
    use super::Result32;

    extern "C" {
        pub fn f32_div(
            a: cty::c_float,
            b: cty::c_float,
            rm: cty::c_uint,
            out: *mut Result32,
        ) -> cty::c_void;
    }
}

pub fn f32_div(a: f32, b: f32, rm: RoundingMode) -> (f32, Exception) {
    let mut result = Result32::new();
    unsafe { ffi::f32_div(a, b, rm as cty::c_uint, &mut result) };
    (result.value, result.exception)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_div() {
        let (r_1, exc_1) = f32_div(10.0f32, 2.0f32, RoundingMode::ToNearest);
        assert_eq!(r_1, 5.0f32);
        assert_eq!(exc_1, Exception::None);
    }

    #[test]
    fn test_f32_div_rm_tonearest() {
        let (r_1, _) = f32_div(1f32, 2.1f32, RoundingMode::ToNearest);
        assert_eq!(r_1, 0.4761905f32);
    }

    #[test]
    fn test_f32_div_rm_towardzero() {
        let (r_1, _) = f32_div(1f32, 2.1f32, RoundingMode::TowardZero);
        assert_eq!(r_1, 0.47619048f32);
    }

    #[test]
    fn test_f32_div_rm_upward() {
        let (r_1, _) = f32_div(1f32, 2.1f32, RoundingMode::Upward);
        assert_eq!(r_1, 0.4761905f32);
    }

    #[test]
    fn test_f32_div_rm_downward() {
        let (r_1, _) = f32_div(1f32, 2.1f32, RoundingMode::Downward);
        assert_eq!(r_1, 0.47619048f32);
    }

    #[test]
    fn test_f32_div_exc_inexact() {
        let (r_1, exc_1) = f32_div(1f32, 2.1f32, RoundingMode::ToNearest);
        assert_eq!(r_1, 0.4761905f32);
        assert_eq!(exc_1, Exception::Inexact);
    }

    #[test]
    fn test_f32_div_exc_byzero() {
        let (r_1, exc_1) = f32_div(1f32, 0.0f32, RoundingMode::ToNearest);
        assert_eq!(r_1.is_infinite(), true);
        assert_eq!(exc_1, Exception::DivByZero);
    }
}
