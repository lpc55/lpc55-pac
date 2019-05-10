#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMP {
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
#[doc = "Possible values of the field `HYST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTR {
    #[doc = "Hysteresis is disable."]
    DISABLE,
    #[doc = "Hysteresis is enable."]
    ENABLE,
}
impl HYSTR {
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
            HYSTR::DISABLE => false,
            HYSTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSTR {
        match value {
            false => HYSTR::DISABLE,
            true => HYSTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HYSTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HYSTR::ENABLE
    }
}
#[doc = "Possible values of the field `VREFINPUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFINPUTR {
    #[doc = "Select internal VREF."]
    INTERNALREF,
    #[doc = "Select VDDA."]
    VDDA,
}
impl VREFINPUTR {
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
            VREFINPUTR::INTERNALREF => false,
            VREFINPUTR::VDDA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREFINPUTR {
        match value {
            false => VREFINPUTR::INTERNALREF,
            true => VREFINPUTR::VDDA,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNALREF`"]
    #[inline]
    pub fn is_internalref(&self) -> bool {
        *self == VREFINPUTR::INTERNALREF
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline]
    pub fn is_vdda(&self) -> bool {
        *self == VREFINPUTR::VDDA
    }
}
#[doc = "Possible values of the field `LOWPOWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWERR {
    #[doc = "High speed mode."]
    HIGHSPEED,
    #[doc = "Low power mode (Low speed)."]
    LOWSPEED,
}
impl LOWPOWERR {
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
            LOWPOWERR::HIGHSPEED => false,
            LOWPOWERR::LOWSPEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOWPOWERR {
        match value {
            false => LOWPOWERR::HIGHSPEED,
            true => LOWPOWERR::LOWSPEED,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline]
    pub fn is_highspeed(&self) -> bool {
        *self == LOWPOWERR::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline]
    pub fn is_lowspeed(&self) -> bool {
        *self == LOWPOWERR::LOWSPEED
    }
}
#[doc = "Possible values of the field `PMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMUXR {
    #[doc = "VREF (See fiedl VREFINPUT)."]
    VREF,
    #[doc = "Pin P0_0."]
    CMP0_A,
    #[doc = "Pin P0_9."]
    CMP0_B,
    #[doc = "Pin P0_18."]
    CMP0_C,
    #[doc = "Pin P1_14."]
    CMP0_D,
    #[doc = "Pin P2_23."]
    CMP0_E,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMUXR::VREF => 0,
            PMUXR::CMP0_A => 1,
            PMUXR::CMP0_B => 2,
            PMUXR::CMP0_C => 3,
            PMUXR::CMP0_D => 4,
            PMUXR::CMP0_E => 5,
            PMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMUXR {
        match value {
            0 => PMUXR::VREF,
            1 => PMUXR::CMP0_A,
            2 => PMUXR::CMP0_B,
            3 => PMUXR::CMP0_C,
            4 => PMUXR::CMP0_D,
            5 => PMUXR::CMP0_E,
            i => PMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline]
    pub fn is_vref(&self) -> bool {
        *self == PMUXR::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline]
    pub fn is_cmp0_a(&self) -> bool {
        *self == PMUXR::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline]
    pub fn is_cmp0_b(&self) -> bool {
        *self == PMUXR::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline]
    pub fn is_cmp0_c(&self) -> bool {
        *self == PMUXR::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline]
    pub fn is_cmp0_d(&self) -> bool {
        *self == PMUXR::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline]
    pub fn is_cmp0_e(&self) -> bool {
        *self == PMUXR::CMP0_E
    }
}
#[doc = "Possible values of the field `NMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMUXR {
    #[doc = "VREF (See field VREFINPUT)."]
    VREF,
    #[doc = "Pin P0_0."]
    CMP0_A,
    #[doc = "Pin P0_9."]
    CMP0_B,
    #[doc = "Pin P0_18."]
    CMP0_C,
    #[doc = "Pin P1_14."]
    CMP0_D,
    #[doc = "Pin P2_23."]
    CMP0_E,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NMUXR::VREF => 0,
            NMUXR::CMP0_A => 1,
            NMUXR::CMP0_B => 2,
            NMUXR::CMP0_C => 3,
            NMUXR::CMP0_D => 4,
            NMUXR::CMP0_E => 5,
            NMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NMUXR {
        match value {
            0 => NMUXR::VREF,
            1 => NMUXR::CMP0_A,
            2 => NMUXR::CMP0_B,
            3 => NMUXR::CMP0_C,
            4 => NMUXR::CMP0_D,
            5 => NMUXR::CMP0_E,
            i => NMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline]
    pub fn is_vref(&self) -> bool {
        *self == NMUXR::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline]
    pub fn is_cmp0_a(&self) -> bool {
        *self == NMUXR::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline]
    pub fn is_cmp0_b(&self) -> bool {
        *self == NMUXR::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline]
    pub fn is_cmp0_c(&self) -> bool {
        *self == NMUXR::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline]
    pub fn is_cmp0_d(&self) -> bool {
        *self == NMUXR::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline]
    pub fn is_cmp0_e(&self) -> bool {
        *self == NMUXR::CMP0_E
    }
}
#[doc = r" Value of the field"]
pub struct VREFR {
    bits: u8,
}
impl VREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILTERCGF_SAMPLEMODER {
    bits: u8,
}
impl FILTERCGF_SAMPLEMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILTERCGF_CLKDIVR {
    bits: u8,
}
impl FILTERCGF_CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PMUXCAPTR {
    bits: u8,
}
impl PMUXCAPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `HYST`"]
pub enum HYSTW {
    #[doc = "Hysteresis is disable."]
    DISABLE,
    #[doc = "Hysteresis is enable."]
    ENABLE,
}
impl HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSTW::DISABLE => false,
            HYSTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hysteresis is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSTW::DISABLE)
    }
    #[doc = "Hysteresis is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSTW::ENABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VREFINPUT`"]
pub enum VREFINPUTW {
    #[doc = "Select internal VREF."]
    INTERNALREF,
    #[doc = "Select VDDA."]
    VDDA,
}
impl VREFINPUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREFINPUTW::INTERNALREF => false,
            VREFINPUTW::VDDA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREFINPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFINPUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREFINPUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select internal VREF."]
    #[inline]
    pub fn internalref(self) -> &'a mut W {
        self.variant(VREFINPUTW::INTERNALREF)
    }
    #[doc = "Select VDDA."]
    #[inline]
    pub fn vdda(self) -> &'a mut W {
        self.variant(VREFINPUTW::VDDA)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOWPOWER`"]
pub enum LOWPOWERW {
    #[doc = "High speed mode."]
    HIGHSPEED,
    #[doc = "Low power mode (Low speed)."]
    LOWSPEED,
}
impl LOWPOWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOWPOWERW::HIGHSPEED => false,
            LOWPOWERW::LOWSPEED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOWPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWPOWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOWPOWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High speed mode."]
    #[inline]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(LOWPOWERW::HIGHSPEED)
    }
    #[doc = "Low power mode (Low speed)."]
    #[inline]
    pub fn lowspeed(self) -> &'a mut W {
        self.variant(LOWPOWERW::LOWSPEED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PMUX`"]
pub enum PMUXW {
    #[doc = "VREF (See fiedl VREFINPUT)."]
    VREF,
    #[doc = "Pin P0_0."]
    CMP0_A,
    #[doc = "Pin P0_9."]
    CMP0_B,
    #[doc = "Pin P0_18."]
    CMP0_C,
    #[doc = "Pin P1_14."]
    CMP0_D,
    #[doc = "Pin P2_23."]
    CMP0_E,
}
impl PMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PMUXW::VREF => 0,
            PMUXW::CMP0_A => 1,
            PMUXW::CMP0_B => 2,
            PMUXW::CMP0_C => 3,
            PMUXW::CMP0_D => 4,
            PMUXW::CMP0_E => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VREF (See fiedl VREFINPUT)."]
    #[inline]
    pub fn vref(self) -> &'a mut W {
        self.variant(PMUXW::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(PMUXW::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(PMUXW::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(PMUXW::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(PMUXW::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(PMUXW::CMP0_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NMUX`"]
pub enum NMUXW {
    #[doc = "VREF (See field VREFINPUT)."]
    VREF,
    #[doc = "Pin P0_0."]
    CMP0_A,
    #[doc = "Pin P0_9."]
    CMP0_B,
    #[doc = "Pin P0_18."]
    CMP0_C,
    #[doc = "Pin P1_14."]
    CMP0_D,
    #[doc = "Pin P2_23."]
    CMP0_E,
}
impl NMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NMUXW::VREF => 0,
            NMUXW::CMP0_A => 1,
            NMUXW::CMP0_B => 2,
            NMUXW::CMP0_C => 3,
            NMUXW::CMP0_D => 4,
            NMUXW::CMP0_E => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _NMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VREF (See field VREFINPUT)."]
    #[inline]
    pub fn vref(self) -> &'a mut W {
        self.variant(NMUXW::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(NMUXW::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(NMUXW::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(NMUXW::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(NMUXW::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(NMUXW::CMP0_E)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VREFW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTERCGF_SAMPLEMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERCGF_SAMPLEMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTERCGF_CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERCGF_CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PMUXCAPTW<'a> {
    w: &'a mut W,
}
impl<'a> _PMUXCAPTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        HYSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline]
    pub fn vrefinput(&self) -> VREFINPUTR {
        VREFINPUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline]
    pub fn lowpower(&self) -> LOWPOWERR {
        LOWPOWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline]
    pub fn pmux(&self) -> PMUXR {
        PMUXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline]
    pub fn nmux(&self) -> NMUXR {
        NMUXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline]
    pub fn vref(&self) -> VREFR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VREFR { bits }
    }
    #[doc = "Bits 16:17 - Filter Sample mode."]
    #[inline]
    pub fn filtercgf_samplemode(&self) -> FILTERCGF_SAMPLEMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILTERCGF_SAMPLEMODER { bits }
    }
    #[doc = "Bits 18:20 - Filter Clock div ."]
    #[inline]
    pub fn filtercgf_clkdiv(&self) -> FILTERCGF_CLKDIVR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILTERCGF_CLKDIVR { bits }
    }
    #[doc = "Bits 21:23 - Control word for P multiplexer for Capacitive Touch Controller."]
    #[inline]
    pub fn pmuxcapt(&self) -> PMUXCAPTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PMUXCAPTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 10 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline]
    pub fn vrefinput(&mut self) -> _VREFINPUTW {
        _VREFINPUTW { w: self }
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline]
    pub fn lowpower(&mut self) -> _LOWPOWERW {
        _LOWPOWERW { w: self }
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline]
    pub fn pmux(&mut self) -> _PMUXW {
        _PMUXW { w: self }
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline]
    pub fn nmux(&mut self) -> _NMUXW {
        _NMUXW { w: self }
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline]
    pub fn vref(&mut self) -> _VREFW {
        _VREFW { w: self }
    }
    #[doc = "Bits 16:17 - Filter Sample mode."]
    #[inline]
    pub fn filtercgf_samplemode(&mut self) -> _FILTERCGF_SAMPLEMODEW {
        _FILTERCGF_SAMPLEMODEW { w: self }
    }
    #[doc = "Bits 18:20 - Filter Clock div ."]
    #[inline]
    pub fn filtercgf_clkdiv(&mut self) -> _FILTERCGF_CLKDIVW {
        _FILTERCGF_CLKDIVW { w: self }
    }
    #[doc = "Bits 21:23 - Control word for P multiplexer for Capacitive Touch Controller."]
    #[inline]
    pub fn pmuxcapt(&mut self) -> _PMUXCAPTW {
        _PMUXCAPTW { w: self }
    }
}
