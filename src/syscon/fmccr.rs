#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMCCR {
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
#[doc = "Possible values of the field `FETCHCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETCHCTLR {
    #[doc = "No buffering (bypass always used) for Fetch cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Fetch cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Fetch cycles"]
    ALLBUF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FETCHCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FETCHCTLR::NOBUF => 0,
            FETCHCTLR::ONEBUF => 1,
            FETCHCTLR::ALLBUF => 2,
            FETCHCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FETCHCTLR {
        match value {
            0 => FETCHCTLR::NOBUF,
            1 => FETCHCTLR::ONEBUF,
            2 => FETCHCTLR::ALLBUF,
            i => FETCHCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline]
    pub fn is_nobuf(&self) -> bool {
        *self == FETCHCTLR::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline]
    pub fn is_onebuf(&self) -> bool {
        *self == FETCHCTLR::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline]
    pub fn is_allbuf(&self) -> bool {
        *self == FETCHCTLR::ALLBUF
    }
}
#[doc = "Possible values of the field `DATACTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATACTLR {
    #[doc = "No buffering (bypass always used) for Data cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Data cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Data cycles"]
    ALLBUF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATACTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATACTLR::NOBUF => 0,
            DATACTLR::ONEBUF => 1,
            DATACTLR::ALLBUF => 2,
            DATACTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATACTLR {
        match value {
            0 => DATACTLR::NOBUF,
            1 => DATACTLR::ONEBUF,
            2 => DATACTLR::ALLBUF,
            i => DATACTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline]
    pub fn is_nobuf(&self) -> bool {
        *self == DATACTLR::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline]
    pub fn is_onebuf(&self) -> bool {
        *self == DATACTLR::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline]
    pub fn is_allbuf(&self) -> bool {
        *self == DATACTLR::ALLBUF
    }
}
#[doc = r" Value of the field"]
pub struct ACCELR {
    bits: bool,
}
impl ACCELR {
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
#[doc = r" Value of the field"]
pub struct PREFENR {
    bits: bool,
}
impl PREFENR {
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
#[doc = r" Value of the field"]
pub struct PREFOVRR {
    bits: bool,
}
impl PREFOVRR {
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
#[doc = r" Value of the field"]
pub struct PREFCRIR {
    bits: u8,
}
impl PREFCRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FMCTIMR {
    bits: u8,
}
impl FMCTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PFISLRUR {
    bits: bool,
}
impl PFISLRUR {
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
#[doc = r" Value of the field"]
pub struct PFADAPR {
    bits: bool,
}
impl PFADAPR {
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
#[doc = "Values that can be written to the field `FETCHCTL`"]
pub enum FETCHCTLW {
    #[doc = "No buffering (bypass always used) for Fetch cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Fetch cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Fetch cycles"]
    ALLBUF,
}
impl FETCHCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FETCHCTLW::NOBUF => 0,
            FETCHCTLW::ONEBUF => 1,
            FETCHCTLW::ALLBUF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FETCHCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _FETCHCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FETCHCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No buffering (bypass always used) for Fetch cycles"]
    #[inline]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(FETCHCTLW::NOBUF)
    }
    #[doc = "One buffer is used for all Fetch cycles"]
    #[inline]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(FETCHCTLW::ONEBUF)
    }
    #[doc = "All buffers can be used for Fetch cycles"]
    #[inline]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(FETCHCTLW::ALLBUF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATACTL`"]
pub enum DATACTLW {
    #[doc = "No buffering (bypass always used) for Data cycles"]
    NOBUF,
    #[doc = "One buffer is used for all Data cycles"]
    ONEBUF,
    #[doc = "All buffers can be used for Data cycles"]
    ALLBUF,
}
impl DATACTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATACTLW::NOBUF => 0,
            DATACTLW::ONEBUF => 1,
            DATACTLW::ALLBUF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATACTLW<'a> {
    w: &'a mut W,
}
impl<'a> _DATACTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATACTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No buffering (bypass always used) for Data cycles"]
    #[inline]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(DATACTLW::NOBUF)
    }
    #[doc = "One buffer is used for all Data cycles"]
    #[inline]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(DATACTLW::ONEBUF)
    }
    #[doc = "All buffers can be used for Data cycles"]
    #[inline]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(DATACTLW::ALLBUF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACCELW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCELW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREFENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREFOVRW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFOVRW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREFCRIW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFCRIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FMCTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FMCTIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFISLRUW<'a> {
    w: &'a mut W,
}
impl<'a> _PFISLRUW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFADAPW<'a> {
    w: &'a mut W,
}
impl<'a> _PFADAPW<'a> {
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
    #[doc = "Bits 0:1 - Fetch control"]
    #[inline]
    pub fn fetchctl(&self) -> FETCHCTLR {
        FETCHCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Data control"]
    #[inline]
    pub fn datactl(&self) -> DATACTLR {
        DATACTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - ACCEL"]
    #[inline]
    pub fn accel(&self) -> ACCELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCELR { bits }
    }
    #[doc = "Bit 5 - Pref enable"]
    #[inline]
    pub fn prefen(&self) -> PREFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREFENR { bits }
    }
    #[doc = "Bit 6 - Pref ovr"]
    #[inline]
    pub fn prefovr(&self) -> PREFOVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREFOVRR { bits }
    }
    #[doc = "Bits 8:10 - Pref CRI"]
    #[inline]
    pub fn prefcri(&self) -> PREFCRIR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PREFCRIR { bits }
    }
    #[doc = "Bits 12:16 - TMC time"]
    #[inline]
    pub fn fmctim(&self) -> FMCTIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FMCTIMR { bits }
    }
    #[doc = "Bit 17 - When set, prefetch uses LRU buffer replacement policy"]
    #[inline]
    pub fn pfislru(&self) -> PFISLRUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFISLRUR { bits }
    }
    #[doc = "Bit 18 - When set, prefetch will adaptively select between parent and LRU buffer replacement policies."]
    #[inline]
    pub fn pfadap(&self) -> PFADAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFADAPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12288 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Fetch control"]
    #[inline]
    pub fn fetchctl(&mut self) -> _FETCHCTLW {
        _FETCHCTLW { w: self }
    }
    #[doc = "Bits 2:3 - Data control"]
    #[inline]
    pub fn datactl(&mut self) -> _DATACTLW {
        _DATACTLW { w: self }
    }
    #[doc = "Bit 4 - ACCEL"]
    #[inline]
    pub fn accel(&mut self) -> _ACCELW {
        _ACCELW { w: self }
    }
    #[doc = "Bit 5 - Pref enable"]
    #[inline]
    pub fn prefen(&mut self) -> _PREFENW {
        _PREFENW { w: self }
    }
    #[doc = "Bit 6 - Pref ovr"]
    #[inline]
    pub fn prefovr(&mut self) -> _PREFOVRW {
        _PREFOVRW { w: self }
    }
    #[doc = "Bits 8:10 - Pref CRI"]
    #[inline]
    pub fn prefcri(&mut self) -> _PREFCRIW {
        _PREFCRIW { w: self }
    }
    #[doc = "Bits 12:16 - TMC time"]
    #[inline]
    pub fn fmctim(&mut self) -> _FMCTIMW {
        _FMCTIMW { w: self }
    }
    #[doc = "Bit 17 - When set, prefetch uses LRU buffer replacement policy"]
    #[inline]
    pub fn pfislru(&mut self) -> _PFISLRUW {
        _PFISLRUW { w: self }
    }
    #[doc = "Bit 18 - When set, prefetch will adaptively select between parent and LRU buffer replacement policies."]
    #[inline]
    pub fn pfadap(&mut self) -> _PFADAPW {
        _PFADAPW { w: self }
    }
}
