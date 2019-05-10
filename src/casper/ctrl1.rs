#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1 {
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
#[doc = r" Value of the field"]
pub struct ITERR {
    bits: u8,
}
impl ITERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODER {
    bits: u8,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RESBPAIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESBPAIRR {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl RESBPAIRR {
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
            RESBPAIRR::PAIR0 => false,
            RESBPAIRR::PAIR1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESBPAIRR {
        match value {
            false => RESBPAIRR::PAIR0,
            true => RESBPAIRR::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline]
    pub fn is_pair0(&self) -> bool {
        *self == RESBPAIRR::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline]
    pub fn is_pair1(&self) -> bool {
        *self == RESBPAIRR::PAIR1
    }
}
#[doc = r" Value of the field"]
pub struct RESOFFR {
    bits: u16,
}
impl RESOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `CSKIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSKIPR {
    #[doc = "No Skip"]
    NO_SKIP,
    #[doc = "Skip if Carry is 1"]
    SKIP_IF_1,
    #[doc = "Skip if Carry is 0"]
    SKIP_IF_0,
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    SET_AND_SKIP,
}
impl CSKIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSKIPR::NO_SKIP => 0,
            CSKIPR::SKIP_IF_1 => 1,
            CSKIPR::SKIP_IF_0 => 2,
            CSKIPR::SET_AND_SKIP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSKIPR {
        match value {
            0 => CSKIPR::NO_SKIP,
            1 => CSKIPR::SKIP_IF_1,
            2 => CSKIPR::SKIP_IF_0,
            3 => CSKIPR::SET_AND_SKIP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SKIP`"]
    #[inline]
    pub fn is_no_skip(&self) -> bool {
        *self == CSKIPR::NO_SKIP
    }
    #[doc = "Checks if the value of the field is `SKIP_IF_1`"]
    #[inline]
    pub fn is_skip_if_1(&self) -> bool {
        *self == CSKIPR::SKIP_IF_1
    }
    #[doc = "Checks if the value of the field is `SKIP_IF_0`"]
    #[inline]
    pub fn is_skip_if_0(&self) -> bool {
        *self == CSKIPR::SKIP_IF_0
    }
    #[doc = "Checks if the value of the field is `SET_AND_SKIP`"]
    #[inline]
    pub fn is_set_and_skip(&self) -> bool {
        *self == CSKIPR::SET_AND_SKIP
    }
}
#[doc = r" Proxy"]
pub struct _ITERW<'a> {
    w: &'a mut W,
}
impl<'a> _ITERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESBPAIR`"]
pub enum RESBPAIRW {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1,
}
impl RESBPAIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESBPAIRW::PAIR0 => false,
            RESBPAIRW::PAIR1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESBPAIRW<'a> {
    w: &'a mut W,
}
impl<'a> _RESBPAIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESBPAIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline]
    pub fn pair0(self) -> &'a mut W {
        self.variant(RESBPAIRW::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline]
    pub fn pair1(self) -> &'a mut W {
        self.variant(RESBPAIRW::PAIR1)
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
pub struct _RESOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RESOFFW<'a> {
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
#[doc = "Values that can be written to the field `CSKIP`"]
pub enum CSKIPW {
    #[doc = "No Skip"]
    NO_SKIP,
    #[doc = "Skip if Carry is 1"]
    SKIP_IF_1,
    #[doc = "Skip if Carry is 0"]
    SKIP_IF_0,
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    SET_AND_SKIP,
}
impl CSKIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSKIPW::NO_SKIP => 0,
            CSKIPW::SKIP_IF_1 => 1,
            CSKIPW::SKIP_IF_0 => 2,
            CSKIPW::SET_AND_SKIP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSKIPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSKIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSKIPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Skip"]
    #[inline]
    pub fn no_skip(self) -> &'a mut W {
        self.variant(CSKIPW::NO_SKIP)
    }
    #[doc = "Skip if Carry is 1"]
    #[inline]
    pub fn skip_if_1(self) -> &'a mut W {
        self.variant(CSKIPW::SKIP_IF_1)
    }
    #[doc = "Skip if Carry is 0"]
    #[inline]
    pub fn skip_if_0(self) -> &'a mut W {
        self.variant(CSKIPW::SKIP_IF_0)
    }
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    #[inline]
    pub fn set_and_skip(self) -> &'a mut W {
        self.variant(CSKIPW::SET_AND_SKIP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline]
    pub fn iter(&self) -> ITERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITERR { bits }
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MODER { bits }
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline]
    pub fn resbpair(&self) -> RESBPAIRR {
        RESBPAIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline]
    pub fn resoff(&self) -> RESOFFR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESOFFR { bits }
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline]
    pub fn cskip(&self) -> CSKIPR {
        CSKIPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:7 - Iteration counter. Is number_cycles - 1. write 0 means Does one cycle - does not iterate."]
    #[inline]
    pub fn iter(&mut self) -> _ITERW {
        _ITERW { w: self }
    }
    #[doc = "Bits 8:15 - Operation mode to perform. write 0 means Accelerator is inactive. write others means accelerator is active."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset RESOFF is within. This must be 0 if only 2-up. Ideally this is not the same bank as ABBPAIR (when 4-up supported)"]
    #[inline]
    pub fn resbpair(&mut self) -> _RESBPAIRW {
        _RESBPAIRW { w: self }
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of result. Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB and CD values"]
    #[inline]
    pub fn resoff(&mut self) -> _RESOFFW {
        _RESOFFW { w: self }
    }
    #[doc = "Bits 30:31 - Skip rules on Carry if needed. This operation will be skipped based on Carry value (from previous operation) if not 0:"]
    #[inline]
    pub fn cskip(&mut self) -> _CSKIPW {
        _CSKIPW { w: self }
    }
}
