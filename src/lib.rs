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
    value: f32,
    exception: ExceptionType,
}

impl Result32 {
    fn new() -> Self {
        Self {
            value: 0.0,
            exception: 0,
        }
    }
}

mod ffi {
    use super::Result32;

    macro_rules! export_ff {
        ($name:tt) => {
            pub fn $name(
                a: cty::c_float,
                b: cty::c_float,
                rm: cty::c_uint,
                out: *mut Result32,
            ) -> cty::c_void;
        };
    }

    extern "C" {
        export_ff!(f32_add);
        export_ff!(f32_div);
    }
}

macro_rules! impl_ff {
    ($name:tt) => {
        pub fn $name(a: f32, b: f32, rm: RoundingMode) -> (f32, Exception) {
            let mut result = Result32::new();
            unsafe { ffi::$name(a, b, rm as cty::c_uint, &mut result) };
            (result.value, Exception(result.exception))
        }
    };
}

impl_ff!(f32_add);
impl_ff!(f32_div);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_add() {
        let (r_1, exc_1) = f32_add(1.0, 2.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 3.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_f32_div() {
        let (r_1, exc_1) = f32_div(10.0, 2.0, RoundingMode::ToNearest);
        assert_eq!(r_1, 5.0f32);
        assert!(exc_1.is_none());
    }

    #[test]
    fn test_f32_div_rm_tonearest() {
        let (r_1, _) = f32_div(1.0, 2.1, RoundingMode::ToNearest);
        assert_eq!(r_1, 0.4761905);
    }

    #[test]
    fn test_f32_div_rm_towardzero() {
        let (r_1, _) = f32_div(1.0, 2.1, RoundingMode::TowardZero);
        assert_eq!(r_1, 0.47619048);
    }

    #[test]
    fn test_f32_div_rm_upward() {
        let (r_1, _) = f32_div(1.0, 2.1, RoundingMode::Upward);
        assert_eq!(r_1, 0.4761905);
    }

    #[test]
    fn test_f32_div_rm_downward() {
        let (r_1, _) = f32_div(1.0, 2.1, RoundingMode::Downward);
        assert_eq!(r_1, 0.47619048);
    }

    #[test]
    fn test_f32_div_exc_inexact() {
        let (r_1, exc_1) = f32_div(1.0, 2.1, RoundingMode::ToNearest);
        assert_eq!(r_1, 0.4761905);
        assert!(exc_1.only(ExceptionFlags::Inexact));
    }

    #[test]
    fn test_f32_div_exc_byzero() {
        let (r_1, exc_1) = f32_div(1.0, 0.0, RoundingMode::ToNearest);
        assert_eq!(r_1.is_infinite(), true);
        assert!(exc_1.only(ExceptionFlags::DivByZero));
    }
}
