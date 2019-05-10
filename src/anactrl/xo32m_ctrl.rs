#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XO32M_CTRL {
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
pub struct GMR {
    bits: u8,
}
impl GMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SLAVER {
    bits: bool,
}
impl SLAVER {
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
pub struct AMPR {
    bits: u8,
}
impl AMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OSC_CAP_INR {
    bits: u8,
}
impl OSC_CAP_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OSC_CAP_OUTR {
    bits: u8,
}
impl OSC_CAP_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ACBUF_PASS_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACBUF_PASS_ENABLER {
    #[doc = "XO AC buffer bypass is disabled."]
    DISABLE,
    #[doc = "XO AC buffer bypass is enabled."]
    ENABLE,
}
impl ACBUF_PASS_ENABLER {
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
            ACBUF_PASS_ENABLER::DISABLE => false,
            ACBUF_PASS_ENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACBUF_PASS_ENABLER {
        match value {
            false => ACBUF_PASS_ENABLER::DISABLE,
            true => ACBUF_PASS_ENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACBUF_PASS_ENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ACBUF_PASS_ENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `ENABLE_PLL_USB_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_PLL_USB_OUTR {
    #[doc = "XO 32 MHz output to USB HS PLL is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to USB HS PLL is enabled."]
    ENABLE,
}
impl ENABLE_PLL_USB_OUTR {
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
            ENABLE_PLL_USB_OUTR::DISABLE => false,
            ENABLE_PLL_USB_OUTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLE_PLL_USB_OUTR {
        match value {
            false => ENABLE_PLL_USB_OUTR::DISABLE,
            true => ENABLE_PLL_USB_OUTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_PLL_USB_OUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_PLL_USB_OUTR::ENABLE
    }
}
#[doc = "Possible values of the field `ENABLE_SYSTEM_CLK_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_SYSTEM_CLK_OUTR {
    #[doc = "XO 32 MHz output to CPU system is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to CPU system is enabled."]
    ENABLE,
}
impl ENABLE_SYSTEM_CLK_OUTR {
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
            ENABLE_SYSTEM_CLK_OUTR::DISABLE => false,
            ENABLE_SYSTEM_CLK_OUTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLE_SYSTEM_CLK_OUTR {
        match value {
            false => ENABLE_SYSTEM_CLK_OUTR::DISABLE,
            true => ENABLE_SYSTEM_CLK_OUTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUTR::ENABLE
    }
}
#[doc = "Possible values of the field `CAPTESTSTARTSRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTSTARTSRCSELR {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPTEST,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl CAPTESTSTARTSRCSELR {
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
            CAPTESTSTARTSRCSELR::CAPTEST => false,
            CAPTESTSTARTSRCSELR::CALIB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTSTARTSRCSELR {
        match value {
            false => CAPTESTSTARTSRCSELR::CAPTEST,
            true => CAPTESTSTARTSRCSELR::CALIB,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTEST`"]
    #[inline]
    pub fn is_captest(&self) -> bool {
        *self == CAPTESTSTARTSRCSELR::CAPTEST
    }
    #[doc = "Checks if the value of the field is `CALIB`"]
    #[inline]
    pub fn is_calib(&self) -> bool {
        *self == CAPTESTSTARTSRCSELR::CALIB
    }
}
#[doc = r" Value of the field"]
pub struct CAPTESTSTARTR {
    bits: bool,
}
impl CAPTESTSTARTR {
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
#[doc = "Possible values of the field `CAPTESTENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTENABLER {
    #[doc = "Captest is disabled."]
    DISABLE,
    #[doc = "Captest is enabled."]
    ENABLE,
}
impl CAPTESTENABLER {
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
            CAPTESTENABLER::DISABLE => false,
            CAPTESTENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTENABLER {
        match value {
            false => CAPTESTENABLER::DISABLE,
            true => CAPTESTENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTESTENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTESTENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `CAPTESTOSCINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTOSCINSELR {
    #[doc = "osc_out (oscillator output) pin."]
    OSCOUT,
    #[doc = "osc_in (oscillator) pin."]
    OSCIN,
}
impl CAPTESTOSCINSELR {
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
            CAPTESTOSCINSELR::OSCOUT => false,
            CAPTESTOSCINSELR::OSCIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTOSCINSELR {
        match value {
            false => CAPTESTOSCINSELR::OSCOUT,
            true => CAPTESTOSCINSELR::OSCIN,
        }
    }
    #[doc = "Checks if the value of the field is `OSCOUT`"]
    #[inline]
    pub fn is_oscout(&self) -> bool {
        *self == CAPTESTOSCINSELR::OSCOUT
    }
    #[doc = "Checks if the value of the field is `OSCIN`"]
    #[inline]
    pub fn is_oscin(&self) -> bool {
        *self == CAPTESTOSCINSELR::OSCIN
    }
}
#[doc = r" Proxy"]
pub struct _GMW<'a> {
    w: &'a mut W,
}
impl<'a> _GMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVEW<'a> {
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
pub struct _AMPW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSC_CAP_INW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_CAP_INW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSC_CAP_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_CAP_OUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACBUF_PASS_ENABLE`"]
pub enum ACBUF_PASS_ENABLEW {
    #[doc = "XO AC buffer bypass is disabled."]
    DISABLE,
    #[doc = "XO AC buffer bypass is enabled."]
    ENABLE,
}
impl ACBUF_PASS_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACBUF_PASS_ENABLEW::DISABLE => false,
            ACBUF_PASS_ENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACBUF_PASS_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACBUF_PASS_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACBUF_PASS_ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XO AC buffer bypass is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLEW::DISABLE)
    }
    #[doc = "XO AC buffer bypass is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLEW::ENABLE)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE_PLL_USB_OUT`"]
pub enum ENABLE_PLL_USB_OUTW {
    #[doc = "XO 32 MHz output to USB HS PLL is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to USB HS PLL is enabled."]
    ENABLE,
}
impl ENABLE_PLL_USB_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLE_PLL_USB_OUTW::DISABLE => false,
            ENABLE_PLL_USB_OUTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_PLL_USB_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_PLL_USB_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_PLL_USB_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XO 32 MHz output to USB HS PLL is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_PLL_USB_OUTW::DISABLE)
    }
    #[doc = "XO 32 MHz output to USB HS PLL is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_PLL_USB_OUTW::ENABLE)
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
#[doc = "Values that can be written to the field `ENABLE_SYSTEM_CLK_OUT`"]
pub enum ENABLE_SYSTEM_CLK_OUTW {
    #[doc = "XO 32 MHz output to CPU system is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to CPU system is enabled."]
    ENABLE,
}
impl ENABLE_SYSTEM_CLK_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLE_SYSTEM_CLK_OUTW::DISABLE => false,
            ENABLE_SYSTEM_CLK_OUTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_SYSTEM_CLK_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_SYSTEM_CLK_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_SYSTEM_CLK_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "XO 32 MHz output to CPU system is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUTW::DISABLE)
    }
    #[doc = "XO 32 MHz output to CPU system is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUTW::ENABLE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTESTSTARTSRCSEL`"]
pub enum CAPTESTSTARTSRCSELW {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPTEST,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl CAPTESTSTARTSRCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTSTARTSRCSELW::CAPTEST => false,
            CAPTESTSTARTSRCSELW::CALIB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTSTARTSRCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTSTARTSRCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTSTARTSRCSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline]
    pub fn captest(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSELW::CAPTEST)
    }
    #[doc = "Sourced from calibration."]
    #[inline]
    pub fn calib(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSELW::CALIB)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTSTARTW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTESTENABLE`"]
pub enum CAPTESTENABLEW {
    #[doc = "Captest is disabled."]
    DISABLE,
    #[doc = "Captest is enabled."]
    ENABLE,
}
impl CAPTESTENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTENABLEW::DISABLE => false,
            CAPTESTENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Captest is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTESTENABLEW::DISABLE)
    }
    #[doc = "Captest is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTESTENABLEW::ENABLE)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTESTOSCINSEL`"]
pub enum CAPTESTOSCINSELW {
    #[doc = "osc_out (oscillator output) pin."]
    OSCOUT,
    #[doc = "osc_in (oscillator) pin."]
    OSCIN,
}
impl CAPTESTOSCINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTOSCINSELW::OSCOUT => false,
            CAPTESTOSCINSELW::OSCIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTOSCINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTOSCINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTOSCINSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "osc_out (oscillator output) pin."]
    #[inline]
    pub fn oscout(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSELW::OSCOUT)
    }
    #[doc = "osc_in (oscillator) pin."]
    #[inline]
    pub fn oscin(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSELW::OSCIN)
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline]
    pub fn gm(&self) -> GMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GMR { bits }
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline]
    pub fn slave(&self) -> SLAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLAVER { bits }
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline]
    pub fn amp(&self) -> AMPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AMPR { bits }
    }
    #[doc = "Bits 8:14 - Tune capa banks of Crystal 32-MHz input pin"]
    #[inline]
    pub fn osc_cap_in(&self) -> OSC_CAP_INR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSC_CAP_INR { bits }
    }
    #[doc = "Bits 15:21 - Tune capa banks of Crystal 32-MHz output pin"]
    #[inline]
    pub fn osc_cap_out(&self) -> OSC_CAP_OUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OSC_CAP_OUTR { bits }
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline]
    pub fn acbuf_pass_enable(&self) -> ACBUF_PASS_ENABLER {
        ACBUF_PASS_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable XO 32 MHz output to USB HS PLL."]
    #[inline]
    pub fn enable_pll_usb_out(&self) -> ENABLE_PLL_USB_OUTR {
        ENABLE_PLL_USB_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline]
    pub fn enable_system_clk_out(&self) -> ENABLE_SYSTEM_CLK_OUTR {
        ENABLE_SYSTEM_CLK_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Source selection for 'xo32k_captest_start' signal."]
    #[inline]
    pub fn capteststartsrcsel(&self) -> CAPTESTSTARTSRCSELR {
        CAPTESTSTARTSRCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - 1: Start CapTest."]
    #[inline]
    pub fn capteststart(&self) -> CAPTESTSTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAPTESTSTARTR { bits }
    }
    #[doc = "Bit 27 - Enable signal for captest."]
    #[inline]
    pub fn captestenable(&self) -> CAPTESTENABLER {
        CAPTESTENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Select the input for test."]
    #[inline]
    pub fn captestoscinsel(&self) -> CAPTESTOSCINSELR {
        CAPTESTOSCINSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2179722 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline]
    pub fn gm(&mut self) -> _GMW {
        _GMW { w: self }
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline]
    pub fn slave(&mut self) -> _SLAVEW {
        _SLAVEW { w: self }
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline]
    pub fn amp(&mut self) -> _AMPW {
        _AMPW { w: self }
    }
    #[doc = "Bits 8:14 - Tune capa banks of Crystal 32-MHz input pin"]
    #[inline]
    pub fn osc_cap_in(&mut self) -> _OSC_CAP_INW {
        _OSC_CAP_INW { w: self }
    }
    #[doc = "Bits 15:21 - Tune capa banks of Crystal 32-MHz output pin"]
    #[inline]
    pub fn osc_cap_out(&mut self) -> _OSC_CAP_OUTW {
        _OSC_CAP_OUTW { w: self }
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline]
    pub fn acbuf_pass_enable(&mut self) -> _ACBUF_PASS_ENABLEW {
        _ACBUF_PASS_ENABLEW { w: self }
    }
    #[doc = "Bit 23 - Enable XO 32 MHz output to USB HS PLL."]
    #[inline]
    pub fn enable_pll_usb_out(&mut self) -> _ENABLE_PLL_USB_OUTW {
        _ENABLE_PLL_USB_OUTW { w: self }
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline]
    pub fn enable_system_clk_out(&mut self) -> _ENABLE_SYSTEM_CLK_OUTW {
        _ENABLE_SYSTEM_CLK_OUTW { w: self }
    }
    #[doc = "Bit 25 - Source selection for 'xo32k_captest_start' signal."]
    #[inline]
    pub fn capteststartsrcsel(&mut self) -> _CAPTESTSTARTSRCSELW {
        _CAPTESTSTARTSRCSELW { w: self }
    }
    #[doc = "Bit 26 - 1: Start CapTest."]
    #[inline]
    pub fn capteststart(&mut self) -> _CAPTESTSTARTW {
        _CAPTESTSTARTW { w: self }
    }
    #[doc = "Bit 27 - Enable signal for captest."]
    #[inline]
    pub fn captestenable(&mut self) -> _CAPTESTENABLEW {
        _CAPTESTENABLEW { w: self }
    }
    #[doc = "Bit 28 - Select the input for test."]
    #[inline]
    pub fn captestoscinsel(&mut self) -> _CAPTESTOSCINSELW {
        _CAPTESTOSCINSELW { w: self }
    }
}
