#[doc = "Register `XO32M_CTRL` reader"]
pub struct R(crate::R<XO32M_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XO32M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XO32M_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XO32M_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XO32M_CTRL` writer"]
pub struct W(crate::W<XO32M_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XO32M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<XO32M_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XO32M_CTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `ACBUF_PASS_ENABLE` reader - Bypass enable of XO AC buffer enable in pll and top level."]
pub struct ACBUF_PASS_ENABLE_R(crate::FieldReader<bool, ACBUF_PASS_ENABLE_A>);
impl ACBUF_PASS_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACBUF_PASS_ENABLE_R(crate::FieldReader::new(bits))
    }
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
        **self == ACBUF_PASS_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ACBUF_PASS_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ACBUF_PASS_ENABLE_R {
    type Target = crate::FieldReader<bool, ACBUF_PASS_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACBUF_PASS_ENABLE` writer - Bypass enable of XO AC buffer enable in pll and top level."]
pub struct ACBUF_PASS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACBUF_PASS_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACBUF_PASS_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
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
#[doc = "Field `ENABLE_PLL_USB_OUT` reader - Enable High speed Crystal oscillator output to USB HS PLL."]
pub struct ENABLE_PLL_USB_OUT_R(crate::FieldReader<bool, ENABLE_PLL_USB_OUT_A>);
impl ENABLE_PLL_USB_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_PLL_USB_OUT_R(crate::FieldReader::new(bits))
    }
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
        **self == ENABLE_PLL_USB_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_PLL_USB_OUT_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_PLL_USB_OUT_R {
    type Target = crate::FieldReader<bool, ENABLE_PLL_USB_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_PLL_USB_OUT` writer - Enable High speed Crystal oscillator output to USB HS PLL."]
pub struct ENABLE_PLL_USB_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_PLL_USB_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_PLL_USB_OUT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
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
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` reader - Enable XO 32 MHz output to CPU system."]
pub struct ENABLE_SYSTEM_CLK_OUT_R(crate::FieldReader<bool, ENABLE_SYSTEM_CLK_OUT_A>);
impl ENABLE_SYSTEM_CLK_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_SYSTEM_CLK_OUT_R(crate::FieldReader::new(bits))
    }
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
        **self == ENABLE_SYSTEM_CLK_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_SYSTEM_CLK_OUT_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_SYSTEM_CLK_OUT_R {
    type Target = crate::FieldReader<bool, ENABLE_SYSTEM_CLK_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` writer - Enable XO 32 MHz output to CPU system."]
pub struct ENABLE_SYSTEM_CLK_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SYSTEM_CLK_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_SYSTEM_CLK_OUT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High speed Crystal Oscillator Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xo32m_ctrl](index.html) module"]
pub struct XO32M_CTRL_SPEC;
impl crate::RegisterSpec for XO32M_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xo32m_ctrl::R](R) reader structure"]
impl crate::Readable for XO32M_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xo32m_ctrl::W](W) writer structure"]
impl crate::Writable for XO32M_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XO32M_CTRL to value 0x0021_428a"]
impl crate::Resettable for XO32M_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0021_428a
    }
}
