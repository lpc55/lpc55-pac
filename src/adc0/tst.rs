#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TST {
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
#[doc = "Possible values of the field `CST_LONG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST_LONGR {
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CST_LONG_0,
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    CST_LONG_1,
}
impl CST_LONGR {
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
            CST_LONGR::CST_LONG_0 => false,
            CST_LONGR::CST_LONG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CST_LONGR {
        match value {
            false => CST_LONGR::CST_LONG_0,
            true => CST_LONGR::CST_LONG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CST_LONG_0`"]
    #[inline]
    pub fn is_cst_long_0(&self) -> bool {
        *self == CST_LONGR::CST_LONG_0
    }
    #[doc = "Checks if the value of the field is `CST_LONG_1`"]
    #[inline]
    pub fn is_cst_long_1(&self) -> bool {
        *self == CST_LONGR::CST_LONG_1
    }
}
#[doc = "Possible values of the field `FOFFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFMR {
    #[doc = "Normal operation. No forced offset."]
    FOFFM_0,
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    FOFFM_1,
}
impl FOFFMR {
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
            FOFFMR::FOFFM_0 => false,
            FOFFMR::FOFFM_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFFMR {
        match value {
            false => FOFFMR::FOFFM_0,
            true => FOFFMR::FOFFM_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM_0`"]
    #[inline]
    pub fn is_foffm_0(&self) -> bool {
        *self == FOFFMR::FOFFM_0
    }
    #[doc = "Checks if the value of the field is `FOFFM_1`"]
    #[inline]
    pub fn is_foffm_1(&self) -> bool {
        *self == FOFFMR::FOFFM_1
    }
}
#[doc = "Possible values of the field `FOFFP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFPR {
    #[doc = "Normal operation. No forced offset."]
    FOFFP_0,
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    FOFFP_1,
}
impl FOFFPR {
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
            FOFFPR::FOFFP_0 => false,
            FOFFPR::FOFFP_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFFPR {
        match value {
            false => FOFFPR::FOFFP_0,
            true => FOFFPR::FOFFP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP_0`"]
    #[inline]
    pub fn is_foffp_0(&self) -> bool {
        *self == FOFFPR::FOFFP_0
    }
    #[doc = "Checks if the value of the field is `FOFFP_1`"]
    #[inline]
    pub fn is_foffp_1(&self) -> bool {
        *self == FOFFPR::FOFFP_1
    }
}
#[doc = "Possible values of the field `FOFFM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFM2R {
    #[doc = "Normal operation. No forced offset."]
    FOFFM2_0,
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    FOFFM2_1,
}
impl FOFFM2R {
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
            FOFFM2R::FOFFM2_0 => false,
            FOFFM2R::FOFFM2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFFM2R {
        match value {
            false => FOFFM2R::FOFFM2_0,
            true => FOFFM2R::FOFFM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM2_0`"]
    #[inline]
    pub fn is_foffm2_0(&self) -> bool {
        *self == FOFFM2R::FOFFM2_0
    }
    #[doc = "Checks if the value of the field is `FOFFM2_1`"]
    #[inline]
    pub fn is_foffm2_1(&self) -> bool {
        *self == FOFFM2R::FOFFM2_1
    }
}
#[doc = "Possible values of the field `FOFFP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFP2R {
    #[doc = "Normal operation. No forced offset."]
    FOFFP2_0,
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    FOFFP2_1,
}
impl FOFFP2R {
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
            FOFFP2R::FOFFP2_0 => false,
            FOFFP2R::FOFFP2_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFFP2R {
        match value {
            false => FOFFP2R::FOFFP2_0,
            true => FOFFP2R::FOFFP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP2_0`"]
    #[inline]
    pub fn is_foffp2_0(&self) -> bool {
        *self == FOFFP2R::FOFFP2_0
    }
    #[doc = "Checks if the value of the field is `FOFFP2_1`"]
    #[inline]
    pub fn is_foffp2_1(&self) -> bool {
        *self == FOFFP2R::FOFFP2_1
    }
}
#[doc = "Possible values of the field `TESTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTENR {
    #[doc = "Normal operation. Test configuration not enabled."]
    TESTEN_0,
    #[doc = "Hardware BIST Test in progress."]
    TESTEN_1,
}
impl TESTENR {
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
            TESTENR::TESTEN_0 => false,
            TESTENR::TESTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TESTENR {
        match value {
            false => TESTENR::TESTEN_0,
            true => TESTENR::TESTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TESTEN_0`"]
    #[inline]
    pub fn is_testen_0(&self) -> bool {
        *self == TESTENR::TESTEN_0
    }
    #[doc = "Checks if the value of the field is `TESTEN_1`"]
    #[inline]
    pub fn is_testen_1(&self) -> bool {
        *self == TESTENR::TESTEN_1
    }
}
#[doc = "Values that can be written to the field `CST_LONG`"]
pub enum CST_LONGW {
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CST_LONG_0,
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    CST_LONG_1,
}
impl CST_LONGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CST_LONGW::CST_LONG_0 => false,
            CST_LONGW::CST_LONG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CST_LONGW<'a> {
    w: &'a mut W,
}
impl<'a> _CST_LONGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CST_LONGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    #[inline]
    pub fn cst_long_0(self) -> &'a mut W {
        self.variant(CST_LONGW::CST_LONG_0)
    }
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    #[inline]
    pub fn cst_long_1(self) -> &'a mut W {
        self.variant(CST_LONGW::CST_LONG_1)
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
#[doc = "Values that can be written to the field `FOFFM`"]
pub enum FOFFMW {
    #[doc = "Normal operation. No forced offset."]
    FOFFM_0,
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    FOFFM_1,
}
impl FOFFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFFMW::FOFFM_0 => false,
            FOFFMW::FOFFM_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFFMW<'a> {
    w: &'a mut W,
}
impl<'a> _FOFFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFFMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline]
    pub fn foffm_0(self) -> &'a mut W {
        self.variant(FOFFMW::FOFFM_0)
    }
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    #[inline]
    pub fn foffm_1(self) -> &'a mut W {
        self.variant(FOFFMW::FOFFM_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FOFFP`"]
pub enum FOFFPW {
    #[doc = "Normal operation. No forced offset."]
    FOFFP_0,
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    FOFFP_1,
}
impl FOFFPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFFPW::FOFFP_0 => false,
            FOFFPW::FOFFP_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFFPW<'a> {
    w: &'a mut W,
}
impl<'a> _FOFFPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFFPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline]
    pub fn foffp_0(self) -> &'a mut W {
        self.variant(FOFFPW::FOFFP_0)
    }
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    #[inline]
    pub fn foffp_1(self) -> &'a mut W {
        self.variant(FOFFPW::FOFFP_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FOFFM2`"]
pub enum FOFFM2W {
    #[doc = "Normal operation. No forced offset."]
    FOFFM2_0,
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    FOFFM2_1,
}
impl FOFFM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFFM2W::FOFFM2_0 => false,
            FOFFM2W::FOFFM2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFFM2W<'a> {
    w: &'a mut W,
}
impl<'a> _FOFFM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFFM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline]
    pub fn foffm2_0(self) -> &'a mut W {
        self.variant(FOFFM2W::FOFFM2_0)
    }
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    #[inline]
    pub fn foffm2_1(self) -> &'a mut W {
        self.variant(FOFFM2W::FOFFM2_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FOFFP2`"]
pub enum FOFFP2W {
    #[doc = "Normal operation. No forced offset."]
    FOFFP2_0,
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    FOFFP2_1,
}
impl FOFFP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFFP2W::FOFFP2_0 => false,
            FOFFP2W::FOFFP2_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFFP2W<'a> {
    w: &'a mut W,
}
impl<'a> _FOFFP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFFP2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline]
    pub fn foffp2_0(self) -> &'a mut W {
        self.variant(FOFFP2W::FOFFP2_0)
    }
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    #[inline]
    pub fn foffp2_1(self) -> &'a mut W {
        self.variant(FOFFP2W::FOFFP2_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TESTEN`"]
pub enum TESTENW {
    #[doc = "Normal operation. Test configuration not enabled."]
    TESTEN_0,
    #[doc = "Hardware BIST Test in progress."]
    TESTEN_1,
}
impl TESTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TESTENW::TESTEN_0 => false,
            TESTENW::TESTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TESTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TESTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation. Test configuration not enabled."]
    #[inline]
    pub fn testen_0(self) -> &'a mut W {
        self.variant(TESTENW::TESTEN_0)
    }
    #[doc = "Hardware BIST Test in progress."]
    #[inline]
    pub fn testen_1(self) -> &'a mut W {
        self.variant(TESTENW::TESTEN_1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline]
    pub fn cst_long(&self) -> CST_LONGR {
        CST_LONGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline]
    pub fn foffm(&self) -> FOFFMR {
        FOFFMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline]
    pub fn foffp(&self) -> FOFFPR {
        FOFFPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline]
    pub fn foffm2(&self) -> FOFFM2R {
        FOFFM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline]
    pub fn foffp2(&self) -> FOFFP2R {
        FOFFP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline]
    pub fn testen(&self) -> TESTENR {
        TESTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline]
    pub fn cst_long(&mut self) -> _CST_LONGW {
        _CST_LONGW { w: self }
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline]
    pub fn foffm(&mut self) -> _FOFFMW {
        _FOFFMW { w: self }
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline]
    pub fn foffp(&mut self) -> _FOFFPW {
        _FOFFPW { w: self }
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline]
    pub fn foffm2(&mut self) -> _FOFFM2W {
        _FOFFM2W { w: self }
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline]
    pub fn foffp2(&mut self) -> _FOFFP2W {
        _FOFFP2W { w: self }
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline]
    pub fn testen(&mut self) -> _TESTENW {
        _TESTENW { w: self }
    }
}
