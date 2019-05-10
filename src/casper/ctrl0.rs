#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL0 {
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
#[doc = "Possible values of the field `ABBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABBPAIRR {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl ABBPAIRR {
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
            ABBPAIRR::PAIR0 => false,
            ABBPAIRR::PAIR1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABBPAIRR {
        match value {
            false => ABBPAIRR::PAIR0,
            true => ABBPAIRR::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline]
    pub fn is_pair0(&self) -> bool {
        *self == ABBPAIRR::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline]
    pub fn is_pair1(&self) -> bool {
        *self == ABBPAIRR::PAIR1
    }
}
#[doc = r" Value of the field"]
pub struct ABOFFR {
    bits: bool,
}
impl ABOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `CDBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDBPAIRR {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl CDBPAIRR {
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
            CDBPAIRR::PAIR0 => false,
            CDBPAIRR::PAIR1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CDBPAIRR {
        match value {
            false => CDBPAIRR::PAIR0,
            true => CDBPAIRR::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline]
    pub fn is_pair0(&self) -> bool {
        *self == CDBPAIRR::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline]
    pub fn is_pair1(&self) -> bool {
        *self == CDBPAIRR::PAIR1
    }
}
#[doc = r" Value of the field"]
pub struct CDOFFR {
    bits: u16,
}
impl CDOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ABBPAIR`"]
pub enum ABBPAIRW {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl ABBPAIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABBPAIRW::PAIR0 => false,
            ABBPAIRW::PAIR1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABBPAIRW<'a> {
    w: &'a mut W,
}
impl<'a> _ABBPAIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABBPAIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline]
    pub fn pair0(self) -> &'a mut W {
        self.variant(ABBPAIRW::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline]
    pub fn pair1(self) -> &'a mut W {
        self.variant(ABBPAIRW::PAIR1)
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
#[doc = r" Proxy"]
pub struct _ABOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _ABOFFW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDBPAIR`"]
pub enum CDBPAIRW {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl CDBPAIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDBPAIRW::PAIR0 => false,
            CDBPAIRW::PAIR1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDBPAIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CDBPAIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDBPAIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CDBPAIRW::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CDBPAIRW::PAIR1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CDOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CDOFFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline]
    pub fn abbpair(&self) -> ABBPAIRR {
        ABBPAIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline]
    pub fn aboff(&self) -> ABOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABOFFR { bits }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline]
    pub fn cdbpair(&self) -> CDBPAIRR {
        CDBPAIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline]
    pub fn cdoff(&self) -> CDOFFR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CDOFFR { bits }
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
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline]
    pub fn abbpair(&mut self) -> _ABBPAIRW {
        _ABBPAIRW { w: self }
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline]
    pub fn aboff(&mut self) -> _ABOFFW {
        _ABOFFW { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline]
    pub fn cdbpair(&mut self) -> _CDBPAIRW {
        _CDBPAIRW { w: self }
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline]
    pub fn cdoff(&mut self) -> _CDOFFW {
        _CDOFFW { w: self }
    }
}
