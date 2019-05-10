#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::XO32M_STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `XO_READY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO_READYR {
    #[doc = "XO output frequency is not yet stable."]
    NOT_STABLE,
    #[doc = "XO output frequency is stable."]
    STABLE,
}
impl XO_READYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            XO_READYR::NOT_STABLE => false,
            XO_READYR::STABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XO_READYR {
        match value {
            false => XO_READYR::NOT_STABLE,
            true => XO_READYR::STABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STABLE`"]
    #[inline]
    pub fn is_not_stable(&self) -> bool {
        *self == XO_READYR::NOT_STABLE
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline]
    pub fn is_stable(&self) -> bool {
        *self == XO_READYR::STABLE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates XO out frequency statibilty."]
    #[inline]
    pub fn xo_ready(&self) -> XO_READYR {
        XO_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
