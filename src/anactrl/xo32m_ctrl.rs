#[doc = "Reader of register XO32M_CTRL"]
pub type R = crate::R<u32, super::XO32M_CTRL>;
#[doc = "Writer for register XO32M_CTRL"]
pub type W = crate::W<u32, super::XO32M_CTRL>;
#[doc = "Register XO32M_CTRL `reset()`'s with value 0x0021_428a"]
impl crate::ResetValue for super::XO32M_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0021_428a
    }
}
#[doc = "Reader of field `GM`"]
pub type GM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GM`"]
pub struct GM_W<'a> {
    w: &'a mut W,
}
impl<'a> GM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `SLAVE`"]
pub type SLAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE`"]
pub struct SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AMP`"]
pub type AMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP`"]
pub struct AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `OSC_CAP_IN`"]
pub type OSC_CAP_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSC_CAP_IN`"]
pub struct OSC_CAP_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_CAP_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSC_CAP_OUT`"]
pub type OSC_CAP_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSC_CAP_OUT`"]
pub struct OSC_CAP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_CAP_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 15)) | (((value as u32) & 0x7f) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `ACBUF_PASS_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACBUF_PASS_ENABLE_A {
    #[doc = "XO AC buffer bypass is disabled."]
    DISABLE,
    #[doc = "XO AC buffer bypass is enabled."]
    ENABLE,
}
impl From<ACBUF_PASS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ACBUF_PASS_ENABLE_A) -> Self {
        match variant {
            ACBUF_PASS_ENABLE_A::DISABLE => false,
            ACBUF_PASS_ENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ACBUF_PASS_ENABLE`"]
pub type ACBUF_PASS_ENABLE_R = crate::R<bool, ACBUF_PASS_ENABLE_A>;
impl ACBUF_PASS_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACBUF_PASS_ENABLE_A {
        match self.bits {
            false => ACBUF_PASS_ENABLE_A::DISABLE,
            true => ACBUF_PASS_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACBUF_PASS_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACBUF_PASS_ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `ACBUF_PASS_ENABLE`"]
pub struct ACBUF_PASS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACBUF_PASS_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACBUF_PASS_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XO AC buffer bypass is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLE_A::DISABLE)
    }
    #[doc = "XO AC buffer bypass is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLE_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLE_PLL_USB_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_PLL_USB_OUT_A {
    #[doc = "XO 32 MHz output to USB HS PLL is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to USB HS PLL is enabled."]
    ENABLE,
}
impl From<ENABLE_PLL_USB_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_PLL_USB_OUT_A) -> Self {
        match variant {
            ENABLE_PLL_USB_OUT_A::DISABLE => false,
            ENABLE_PLL_USB_OUT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENABLE_PLL_USB_OUT`"]
pub type ENABLE_PLL_USB_OUT_R = crate::R<bool, ENABLE_PLL_USB_OUT_A>;
impl ENABLE_PLL_USB_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_PLL_USB_OUT_A {
        match self.bits {
            false => ENABLE_PLL_USB_OUT_A::DISABLE,
            true => ENABLE_PLL_USB_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_PLL_USB_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_PLL_USB_OUT_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE_PLL_USB_OUT`"]
pub struct ENABLE_PLL_USB_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_PLL_USB_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_PLL_USB_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XO 32 MHz output to USB HS PLL is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_PLL_USB_OUT_A::DISABLE)
    }
    #[doc = "XO 32 MHz output to USB HS PLL is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_PLL_USB_OUT_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLE_SYSTEM_CLK_OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_SYSTEM_CLK_OUT_A {
    #[doc = "XO 32 MHz output to CPU system is disabled."]
    DISABLE,
    #[doc = "XO 32 MHz output to CPU system is enabled."]
    ENABLE,
}
impl From<ENABLE_SYSTEM_CLK_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_SYSTEM_CLK_OUT_A) -> Self {
        match variant {
            ENABLE_SYSTEM_CLK_OUT_A::DISABLE => false,
            ENABLE_SYSTEM_CLK_OUT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENABLE_SYSTEM_CLK_OUT`"]
pub type ENABLE_SYSTEM_CLK_OUT_R = crate::R<bool, ENABLE_SYSTEM_CLK_OUT_A>;
impl ENABLE_SYSTEM_CLK_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_SYSTEM_CLK_OUT_A {
        match self.bits {
            false => ENABLE_SYSTEM_CLK_OUT_A::DISABLE,
            true => ENABLE_SYSTEM_CLK_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUT_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENABLE_SYSTEM_CLK_OUT`"]
pub struct ENABLE_SYSTEM_CLK_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SYSTEM_CLK_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_SYSTEM_CLK_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XO 32 MHz output to CPU system is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUT_A::DISABLE)
    }
    #[doc = "XO 32 MHz output to CPU system is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUT_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `CAPTESTSTARTSRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTSTARTSRCSEL_A {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPTEST,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl From<CAPTESTSTARTSRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTESTSTARTSRCSEL_A) -> Self {
        match variant {
            CAPTESTSTARTSRCSEL_A::CAPTEST => false,
            CAPTESTSTARTSRCSEL_A::CALIB => true,
        }
    }
}
#[doc = "Reader of field `CAPTESTSTARTSRCSEL`"]
pub type CAPTESTSTARTSRCSEL_R = crate::R<bool, CAPTESTSTARTSRCSEL_A>;
impl CAPTESTSTARTSRCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTESTSTARTSRCSEL_A {
        match self.bits {
            false => CAPTESTSTARTSRCSEL_A::CAPTEST,
            true => CAPTESTSTARTSRCSEL_A::CALIB,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTEST`"]
    #[inline(always)]
    pub fn is_captest(&self) -> bool {
        *self == CAPTESTSTARTSRCSEL_A::CAPTEST
    }
    #[doc = "Checks if the value of the field is `CALIB`"]
    #[inline(always)]
    pub fn is_calib(&self) -> bool {
        *self == CAPTESTSTARTSRCSEL_A::CALIB
    }
}
#[doc = "Write proxy for field `CAPTESTSTARTSRCSEL`"]
pub struct CAPTESTSTARTSRCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTSTARTSRCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTESTSTARTSRCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline(always)]
    pub fn captest(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSEL_A::CAPTEST)
    }
    #[doc = "Sourced from calibration."]
    #[inline(always)]
    pub fn calib(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSEL_A::CALIB)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CAPTESTSTART`"]
pub type CAPTESTSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTESTSTART`"]
pub struct CAPTESTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTSTART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `CAPTESTENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTENABLE_A {
    #[doc = "Captest is disabled."]
    DISABLE,
    #[doc = "Captest is enabled."]
    ENABLE,
}
impl From<CAPTESTENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTESTENABLE_A) -> Self {
        match variant {
            CAPTESTENABLE_A::DISABLE => false,
            CAPTESTENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CAPTESTENABLE`"]
pub type CAPTESTENABLE_R = crate::R<bool, CAPTESTENABLE_A>;
impl CAPTESTENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTESTENABLE_A {
        match self.bits {
            false => CAPTESTENABLE_A::DISABLE,
            true => CAPTESTENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTESTENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPTESTENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPTESTENABLE`"]
pub struct CAPTESTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTESTENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Captest is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTESTENABLE_A::DISABLE)
    }
    #[doc = "Captest is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTESTENABLE_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `CAPTESTOSCINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTOSCINSEL_A {
    #[doc = "osc_out (oscillator output) pin."]
    OSCOUT,
    #[doc = "osc_in (oscillator) pin."]
    OSCIN,
}
impl From<CAPTESTOSCINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTESTOSCINSEL_A) -> Self {
        match variant {
            CAPTESTOSCINSEL_A::OSCOUT => false,
            CAPTESTOSCINSEL_A::OSCIN => true,
        }
    }
}
#[doc = "Reader of field `CAPTESTOSCINSEL`"]
pub type CAPTESTOSCINSEL_R = crate::R<bool, CAPTESTOSCINSEL_A>;
impl CAPTESTOSCINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTESTOSCINSEL_A {
        match self.bits {
            false => CAPTESTOSCINSEL_A::OSCOUT,
            true => CAPTESTOSCINSEL_A::OSCIN,
        }
    }
    #[doc = "Checks if the value of the field is `OSCOUT`"]
    #[inline(always)]
    pub fn is_oscout(&self) -> bool {
        *self == CAPTESTOSCINSEL_A::OSCOUT
    }
    #[doc = "Checks if the value of the field is `OSCIN`"]
    #[inline(always)]
    pub fn is_oscin(&self) -> bool {
        *self == CAPTESTOSCINSEL_A::OSCIN
    }
}
#[doc = "Write proxy for field `CAPTESTOSCINSEL`"]
pub struct CAPTESTOSCINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTESTOSCINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTESTOSCINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "osc_out (oscillator output) pin."]
    #[inline(always)]
    pub fn oscout(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSEL_A::OSCOUT)
    }
    #[doc = "osc_in (oscillator) pin."]
    #[inline(always)]
    pub fn oscin(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSEL_A::OSCIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline(always)]
    pub fn amp(&self) -> AMP_R {
        AMP_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Tune capa banks of Crystal 32-MHz input pin"]
    #[inline(always)]
    pub fn osc_cap_in(&self) -> OSC_CAP_IN_R {
        OSC_CAP_IN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Tune capa banks of Crystal 32-MHz output pin"]
    #[inline(always)]
    pub fn osc_cap_out(&self) -> OSC_CAP_OUT_R {
        OSC_CAP_OUT_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&self) -> ACBUF_PASS_ENABLE_R {
        ACBUF_PASS_ENABLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable XO 32 MHz output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&self) -> ENABLE_PLL_USB_OUT_R {
        ENABLE_PLL_USB_OUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&self) -> ENABLE_SYSTEM_CLK_OUT_R {
        ENABLE_SYSTEM_CLK_OUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Source selection for 'xo32k_captest_start' signal."]
    #[inline(always)]
    pub fn capteststartsrcsel(&self) -> CAPTESTSTARTSRCSEL_R {
        CAPTESTSTARTSRCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1: Start CapTest."]
    #[inline(always)]
    pub fn capteststart(&self) -> CAPTESTSTART_R {
        CAPTESTSTART_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable signal for captest."]
    #[inline(always)]
    pub fn captestenable(&self) -> CAPTESTENABLE_R {
        CAPTESTENABLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&self) -> CAPTESTOSCINSEL_R {
        CAPTESTOSCINSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W {
        GM_W { w: self }
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W {
        SLAVE_W { w: self }
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline(always)]
    pub fn amp(&mut self) -> AMP_W {
        AMP_W { w: self }
    }
    #[doc = "Bits 8:14 - Tune capa banks of Crystal 32-MHz input pin"]
    #[inline(always)]
    pub fn osc_cap_in(&mut self) -> OSC_CAP_IN_W {
        OSC_CAP_IN_W { w: self }
    }
    #[doc = "Bits 15:21 - Tune capa banks of Crystal 32-MHz output pin"]
    #[inline(always)]
    pub fn osc_cap_out(&mut self) -> OSC_CAP_OUT_W {
        OSC_CAP_OUT_W { w: self }
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&mut self) -> ACBUF_PASS_ENABLE_W {
        ACBUF_PASS_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - Enable XO 32 MHz output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&mut self) -> ENABLE_PLL_USB_OUT_W {
        ENABLE_PLL_USB_OUT_W { w: self }
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&mut self) -> ENABLE_SYSTEM_CLK_OUT_W {
        ENABLE_SYSTEM_CLK_OUT_W { w: self }
    }
    #[doc = "Bit 25 - Source selection for 'xo32k_captest_start' signal."]
    #[inline(always)]
    pub fn capteststartsrcsel(&mut self) -> CAPTESTSTARTSRCSEL_W {
        CAPTESTSTARTSRCSEL_W { w: self }
    }
    #[doc = "Bit 26 - 1: Start CapTest."]
    #[inline(always)]
    pub fn capteststart(&mut self) -> CAPTESTSTART_W {
        CAPTESTSTART_W { w: self }
    }
    #[doc = "Bit 27 - Enable signal for captest."]
    #[inline(always)]
    pub fn captestenable(&mut self) -> CAPTESTENABLE_W {
        CAPTESTENABLE_W { w: self }
    }
    #[doc = "Bit 28 - Select the input for test."]
    #[inline(always)]
    pub fn captestoscinsel(&mut self) -> CAPTESTOSCINSEL_W {
        CAPTESTOSCINSEL_W { w: self }
    }
}
