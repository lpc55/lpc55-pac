#[doc = "Register `ANACTRL_SET` reader"]
pub struct R(crate::R<ANACTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL_SET` writer"]
pub struct W(crate::W<ANACTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SET_SPEC>;
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
impl From<crate::W<ANACTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVI_EN` reader - Vow voltage detector enable bit."]
pub struct LVI_EN_R(crate::FieldReader<bool, bool>);
impl LVI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LVI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVI_EN` writer - Vow voltage detector enable bit."]
pub struct LVI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVI_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PFD_CLK_SEL` reader - For normal USB operation, this bit field must remain at value 2'b00."]
pub struct PFD_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl PFD_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PFD_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFD_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFD_CLK_SEL` writer - For normal USB operation, this bit field must remain at value 2'b00."]
pub struct PFD_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "0: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0,
    #[doc = "1: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_PULLDOWN` reader - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub struct DEV_PULLDOWN_R(crate::FieldReader<bool, DEV_PULLDOWN_A>);
impl DEV_PULLDOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEV_PULLDOWN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::VALUE0,
            true => DEV_PULLDOWN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == DEV_PULLDOWN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DEV_PULLDOWN_A::VALUE1
    }
}
impl core::ops::Deref for DEV_PULLDOWN_R {
    type Target = crate::FieldReader<bool, DEV_PULLDOWN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_PULLDOWN` writer - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub struct DEV_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PULLDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_PULLDOWN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&self) -> LVI_EN_R {
        LVI_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&self) -> PFD_CLK_SEL_R {
        PFD_CLK_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&mut self) -> LVI_EN_W {
        LVI_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&mut self) -> PFD_CLK_SEL_W {
        PFD_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W {
        DEV_PULLDOWN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl_set](index.html) module"]
pub struct ANACTRL_SET_SPEC;
impl crate::RegisterSpec for ANACTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anactrl_set::R](R) reader structure"]
impl crate::Readable for ANACTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl_set::W](W) writer structure"]
impl crate::Writable for ANACTRL_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANACTRL_SET to value 0x0a00_0402"]
impl crate::Resettable for ANACTRL_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00_0402
    }
}
