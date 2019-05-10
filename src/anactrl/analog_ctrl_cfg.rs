#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANALOG_CTRL_CFG {
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
#[doc = "Possible values of the field `FRO192M_TRIM_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO192M_TRIM_SRCR {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL,
}
impl FRO192M_TRIM_SRCR {
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
            FRO192M_TRIM_SRCR::EFUSE => false,
            FRO192M_TRIM_SRCR::FRO192MCTRL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO192M_TRIM_SRCR {
        match value {
            false => FRO192M_TRIM_SRCR::EFUSE,
            true => FRO192M_TRIM_SRCR::FRO192MCTRL,
        }
    }
    #[doc = "Checks if the value of the field is `EFUSE`"]
    #[inline]
    pub fn is_efuse(&self) -> bool {
        *self == FRO192M_TRIM_SRCR::EFUSE
    }
    #[doc = "Checks if the value of the field is `FRO192MCTRL`"]
    #[inline]
    pub fn is_fro192mctrl(&self) -> bool {
        *self == FRO192M_TRIM_SRCR::FRO192MCTRL
    }
}
#[doc = "Values that can be written to the field `FRO192M_TRIM_SRC`"]
pub enum FRO192M_TRIM_SRCW {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL,
}
impl FRO192M_TRIM_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO192M_TRIM_SRCW::EFUSE => false,
            FRO192M_TRIM_SRCW::FRO192MCTRL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO192M_TRIM_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO192M_TRIM_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO192M_TRIM_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    #[inline]
    pub fn efuse(self) -> &'a mut W {
        self.variant(FRO192M_TRIM_SRCW::EFUSE)
    }
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    #[inline]
    pub fn fro192mctrl(self) -> &'a mut W {
        self.variant(FRO192M_TRIM_SRCW::FRO192MCTRL)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline]
    pub fn fro192m_trim_src(&self) -> FRO192M_TRIM_SRCR {
        FRO192M_TRIM_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline]
    pub fn fro192m_trim_src(&mut self) -> _FRO192M_TRIM_SRCW {
        _FRO192M_TRIM_SRCW { w: self }
    }
}
