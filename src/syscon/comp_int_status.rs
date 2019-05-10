#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMP_INT_STATUS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "no interrupt pending."]
    NO_INT,
    #[doc = "interrupt pending."]
    PENDING,
}
impl STATUSR {
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
            STATUSR::NO_INT => false,
            STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATUSR {
        match value {
            false => STATUSR::NO_INT,
            true => STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline]
    pub fn is_no_int(&self) -> bool {
        *self == STATUSR::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `INT_STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_STATUSR {
    #[doc = "no interrupt pending."]
    NO_INT,
    #[doc = "interrupt pending."]
    PENDING,
}
impl INT_STATUSR {
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
            INT_STATUSR::NO_INT => false,
            INT_STATUSR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_STATUSR {
        match value {
            false => INT_STATUSR::NO_INT,
            true => INT_STATUSR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline]
    pub fn is_no_int(&self) -> bool {
        *self == INT_STATUSR::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == INT_STATUSR::PENDING
    }
}
#[doc = "Possible values of the field `VAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALR {
    #[doc = "P+ is smaller than P-."]
    SMALLER,
    #[doc = "P+ is greater than P-."]
    GREATER,
}
impl VALR {
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
            VALR::SMALLER => false,
            VALR::GREATER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALR {
        match value {
            false => VALR::SMALLER,
            true => VALR::GREATER,
        }
    }
    #[doc = "Checks if the value of the field is `SMALLER`"]
    #[inline]
    pub fn is_smaller(&self) -> bool {
        *self == VALR::SMALLER
    }
    #[doc = "Checks if the value of the field is `GREATER`"]
    #[inline]
    pub fn is_greater(&self) -> bool {
        *self == VALR::GREATER
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt status BEFORE Interrupt Enable."]
    #[inline]
    pub fn status(&self) -> STATUSR {
        STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt status AFTER Interrupt Enable."]
    #[inline]
    pub fn int_status(&self) -> INT_STATUSR {
        INT_STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - comparator analog output."]
    #[inline]
    pub fn val(&self) -> VALR {
        VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
