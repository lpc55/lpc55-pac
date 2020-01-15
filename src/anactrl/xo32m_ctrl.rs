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
#[doc = "Bypass enable of XO AC buffer enable in pll and top level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACBUF_PASS_ENABLE_A {
    #[doc = "0: XO AC buffer bypass is disabled."]
    DISABLE = 0,
    #[doc = "1: XO AC buffer bypass is enabled."]
    ENABLE = 1,
}
impl From<ACBUF_PASS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ACBUF_PASS_ENABLE_A) -> Self {
        variant as u8 != 0
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
#[doc = "Enable High speed Crystal oscillator output to USB HS PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_PLL_USB_OUT_A {
    #[doc = "0: High speed Crystal oscillator output to USB HS PLL is disabled."]
    DISABLE = 0,
    #[doc = "1: High speed Crystal oscillator output to USB HS PLL is enabled."]
    ENABLE = 1,
}
impl From<ENABLE_PLL_USB_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_PLL_USB_OUT_A) -> Self {
        variant as u8 != 0
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
    #[doc = "High speed Crystal oscillator output to USB HS PLL is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_PLL_USB_OUT_A::DISABLE)
    }
    #[doc = "High speed Crystal oscillator output to USB HS PLL is enabled."]
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
#[doc = "Enable XO 32 MHz output to CPU system.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_SYSTEM_CLK_OUT_A {
    #[doc = "0: High speed Crystal oscillator output to CPU system is disabled."]
    DISABLE = 0,
    #[doc = "1: High speed Crystal oscillator output to CPU system is enabled."]
    ENABLE = 1,
}
impl From<ENABLE_SYSTEM_CLK_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_SYSTEM_CLK_OUT_A) -> Self {
        variant as u8 != 0
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
    #[doc = "High speed Crystal oscillator output to CPU system is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUT_A::DISABLE)
    }
    #[doc = "High speed Crystal oscillator output to CPU system is enabled."]
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
impl R {
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&self) -> ACBUF_PASS_ENABLE_R {
        ACBUF_PASS_ENABLE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&self) -> ENABLE_PLL_USB_OUT_R {
        ENABLE_PLL_USB_OUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&self) -> ENABLE_SYSTEM_CLK_OUT_R {
        ENABLE_SYSTEM_CLK_OUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&mut self) -> ACBUF_PASS_ENABLE_W {
        ACBUF_PASS_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub fn enable_pll_usb_out(&mut self) -> ENABLE_PLL_USB_OUT_W {
        ENABLE_PLL_USB_OUT_W { w: self }
    }
    #[doc = "Bit 24 - Enable XO 32 MHz output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&mut self) -> ENABLE_SYSTEM_CLK_OUT_W {
        ENABLE_SYSTEM_CLK_OUT_W { w: self }
    }
}
