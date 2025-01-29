type ExceptionType = cty::uint32_t;

#[derive(Debug)]
#[repr(C)]
pub enum RoundingMode {
    ToNearest = 0,
    TowardZero = 1,
    Upward = 2,
    Downward = 3,
}

#[derive(Debug)]
#[repr(C)]
pub enum ExceptionFlags {
    None = 0,
    DivByZero = 1 << 0,
    Invalid = 1 << 1,
    Overflow = 1 << 2,
    Underflow = 1 << 3,
    Inexact = 1 << 4,
}

impl Into<ExceptionType> for ExceptionFlags {
    fn into(self) -> ExceptionType {
        self as ExceptionType
    }
}

impl std::ops::BitAnd<ExceptionFlags> for ExceptionType {
    type Output = ExceptionType;

    fn bitand(self, rhs: ExceptionFlags) -> ExceptionType {
        self & (rhs as ExceptionType)
    }
}

pub struct Exception(ExceptionType);

impl Exception {
    pub fn is_none(&self) -> bool {
        self.0 == ExceptionFlags::None.into()
    }

    pub fn has(&self, exception: ExceptionFlags) -> bool {
        (self.0 & exception) != 0
    }

    pub fn only(&self, exception: ExceptionFlags) -> bool {
        self.0 == exception.into()
    }
}

#[repr(C)]
struct Result32 {
    value: u32,
    exception: ExceptionType,
}

impl Result32 {
    fn new() -> Self {
        Self {
            value: 0,
            exception: 0,
        }
    }
}

mod ffi {
    use super::Result32;

    macro_rules! export_ff_binary {
        ($name:tt) => {
            pub fn $name(
                a: cty::c_float,
                b: cty::c_float,
                rm: cty::c_uint,
                out: *mut Result32,
            ) -> cty::c_void;
        };
    }

    macro_rules! export_ff_unary {
        ($name:tt, $in_ty:ty) => {
            pub fn $name(val: $in_ty, rm: cty::c_uint, out: *mut Result32) -> cty::c_void;
        };
    }

    extern "C" {
        export_ff_binary!(add_f32);
        export_ff_binary!(div_f32);
        export_ff_binary!(mul_f32);
        export_ff_unary!(cvt_u32_f32, cty::c_uint);
        export_ff_unary!(cvt_f32_u32, cty::c_float);
    }
}

macro_rules! impl_binary {
    ($name:tt) => {
        pub fn $name(a: f32, b: f32, rm: RoundingMode) -> (f32, Exception) {
            let mut result = Result32::new();
            unsafe {
                ffi::$name(a, b, rm as cty::c_uint, &mut result);
                (std::mem::transmute::<u32, f32>(result.value), Exception(result.exception))
            }
        }
    };
}

macro_rules! impl_unary {
    ($name:tt, $in_ty:ty, $out_ty:ty) => {
        pub fn $name(val: $in_ty, rm: RoundingMode) -> ($out_ty, Exception) {
            let mut result = Result32::new();
            unsafe {
                ffi::$name(val, rm as cty::c_uint, &mut result);
                (std::mem::transmute::<u32, $out_ty>(result.value), Exception(result.exception))
            }
        }
    };
}

impl_binary!(add_f32);
impl_binary!(div_f32);
impl_binary!(mul_f32);
impl_unary!(cvt_u32_f32, u32, f32);
impl_unary!(cvt_f32_u32, f32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_f32() {
        let (r_1, exc_1) = add_f32(1.0, 2.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 3.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_add_f32_rounding() {
        assert_eq!(add_f32(1.0, 1e-10, RoundingMode::ToNearest).0, 1.0);
        assert_eq!(add_f32(1.0, 1e-10, RoundingMode::TowardZero).0, 1.0);
        assert_eq!(add_f32(1.0, 1e-10, RoundingMode::Upward).0, 1.0000001);
        assert_eq!(add_f32(1.0, 1e-10, RoundingMode::Downward).0, 1.0);
    }

    #[test]
    fn test_add_f32_fpe() {
        let (r_1, exc_1) = add_f32(1.0, 1e-10, RoundingMode::ToNearest);
        assert_eq!(r_1, 1.0);
        assert!(exc_1.only(ExceptionFlags::Inexact));

        let (_, exc_1) = add_f32(f32::MAX, f32::MAX, RoundingMode::ToNearest);
        assert!(exc_1.has(ExceptionFlags::Overflow));
    }

    #[test]
    fn test_div_f32() {
        let (r_1, exc_1) = div_f32(10.0, 2.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 5.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_div_f32_rounding() {
        assert_eq!(div_f32(1.0, 2.1, RoundingMode::ToNearest).0, 0.4761905);
        assert_eq!(div_f32(1.0, 2.1, RoundingMode::TowardZero).0, 0.47619048);
        assert_eq!(div_f32(1.0, 2.1, RoundingMode::Upward).0, 0.4761905);
        assert_eq!(div_f32(1.0, 2.1, RoundingMode::Downward).0, 0.47619048);
    }

    #[test]
    fn test_div_f32_fpe() {
        let (r_1, exc_1) = div_f32(1.0, 2.1, RoundingMode::ToNearest);
        assert_eq!(r_1, 0.4761905);
        assert!(exc_1.only(ExceptionFlags::Inexact));

        let (r_1, exc_1) = div_f32(1.0, 0.0, RoundingMode::ToNearest);
        assert_eq!(r_1.is_infinite(), true);
        assert!(exc_1.only(ExceptionFlags::DivByZero));
    }

    #[test]
    fn test_mul_f32() {
        let (r_1, exc_1) = mul_f32(10.0, 2.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 20.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_mul_f32_rounding() {
        assert_eq!(mul_f32(1.999999, 2.1, RoundingMode::ToNearest).0, 4.199998);
        assert_eq!(mul_f32(1.999999, 2.1, RoundingMode::TowardZero).0, 4.1999974);
        assert_eq!(mul_f32(1.999999, 2.1, RoundingMode::Upward).0, 4.199998);
        assert_eq!(mul_f32(1.999999, 2.1, RoundingMode::Downward).0, 4.1999974);
    }

    #[test]
    fn test_mul_f32_fpe() {
        let (r_1, exc_1) = mul_f32(2.1, 2.1, RoundingMode::ToNearest);
        assert_eq!(r_1, 4.4099994);
        assert!(exc_1.only(ExceptionFlags::Inexact));

        let (r_1, exc_1) = mul_f32(f32::MAX, 1.1, RoundingMode::ToNearest);
        assert_eq!(r_1.is_infinite(), true);
        println!("{:?}", exc_1.0);
        assert!(exc_1.has(ExceptionFlags::Overflow));
    }

    #[test]
    fn test_cvt_u32_f32() {
        let (r_1, exc_1) = cvt_u32_f32(10, RoundingMode::ToNearest);
        assert_eq!(r_1, 10.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_cvt_u32_f32_exc() {
        let (_, exc) = cvt_u32_f32(u32::MAX, RoundingMode::ToNearest);
        assert!(exc.has(ExceptionFlags::Inexact));
    }

    #[test]
    fn test_cvt_f32_u32() {
        let (r_1, exc_1) = cvt_f32_u32(6.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 6);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_cvt_f32_u32_rounding() {
        assert_eq!(cvt_f32_u32(1.5, RoundingMode::ToNearest).0, 2);
        assert_eq!(cvt_f32_u32(1.5, RoundingMode::TowardZero).0, 1);
        assert_eq!(cvt_f32_u32(1.5, RoundingMode::Upward).0, 2);
        assert_eq!(cvt_f32_u32(1.5, RoundingMode::Downward).0, 1);
    }

    #[test]
    fn test_cvt_f32_u32_exc() {
        let (_, exc) = cvt_f32_u32(f32::NAN, RoundingMode::ToNearest);
        assert!(exc.has(ExceptionFlags::Invalid));
    }
}
