#[doc = "Register `RTCOSC32K` reader"]
pub struct R(crate::R<RTCOSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCOSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCOSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOSC32K` writer"]
pub struct W(crate::W<RTCOSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOSC32K_SPEC>;
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
impl From<crate::W<RTCOSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCOSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: FRO 32 KHz."]
    FRO32K = 0,
    #[doc = "1: XTAL 32KHz."]
    XTAL32K = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::FRO32K,
            true => SEL_A::XTAL32K,
        }
    }
    #[doc = "Checks if the value of the field is `FRO32K`"]
    #[inline(always)]
    pub fn is_fro32k(&self) -> bool {
        **self == SEL_A::FRO32K
    }
    #[doc = "Checks if the value of the field is `XTAL32K`"]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        **self == SEL_A::XTAL32K
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FRO 32 KHz."]
    #[inline(always)]
    pub fn fro32k(self) -> &'a mut W {
        self.variant(SEL_A::FRO32K)
    }
    #[doc = "XTAL 32KHz."]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut W {
        self.variant(SEL_A::XTAL32K)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CLK1KHZDIV` reader - Actual division ratio is : 28 + CLK1KHZDIV."]
pub struct CLK1KHZDIV_R(crate::FieldReader<u8, u8>);
impl CLK1KHZDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK1KHZDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1KHZDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1KHZDIV` writer - Actual division ratio is : 28 + CLK1KHZDIV."]
pub struct CLK1KHZDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1KHZDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `CLK1KHZDIVUPDATEREQ` reader - RTC 1KHz clock Divider status flag."]
pub struct CLK1KHZDIVUPDATEREQ_R(crate::FieldReader<bool, bool>);
impl CLK1KHZDIVUPDATEREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK1KHZDIVUPDATEREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1KHZDIVUPDATEREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1KHZDIVUPDATEREQ` writer - RTC 1KHz clock Divider status flag."]
pub struct CLK1KHZDIVUPDATEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1KHZDIVUPDATEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CLK1HZDIV` reader - Actual division ratio is : 31744 + CLK1HZDIV."]
pub struct CLK1HZDIV_R(crate::FieldReader<u16, u16>);
impl CLK1HZDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLK1HZDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1HZDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1HZDIV` writer - Actual division ratio is : 31744 + CLK1HZDIV."]
pub struct CLK1HZDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `CLK1HZDIVHALT` reader - Halts the divider counter."]
pub struct CLK1HZDIVHALT_R(crate::FieldReader<bool, bool>);
impl CLK1HZDIVHALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK1HZDIVHALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1HZDIVHALT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1HZDIVHALT` writer - Halts the divider counter."]
pub struct CLK1HZDIVHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIVHALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CLK1HZDIVUPDATEREQ` reader - RTC 1Hz Divider status flag."]
pub struct CLK1HZDIVUPDATEREQ_R(crate::FieldReader<bool, bool>);
impl CLK1HZDIVUPDATEREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK1HZDIVUPDATEREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK1HZDIVUPDATEREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK1HZDIVUPDATEREQ` writer - RTC 1Hz Divider status flag."]
pub struct CLK1HZDIVUPDATEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK1HZDIVUPDATEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&self) -> CLK1KHZDIV_R {
        CLK1KHZDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&self) -> CLK1KHZDIVUPDATEREQ_R {
        CLK1KHZDIVUPDATEREQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&self) -> CLK1HZDIV_R {
        CLK1HZDIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&self) -> CLK1HZDIVHALT_R {
        CLK1HZDIVHALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&self) -> CLK1HZDIVUPDATEREQ_R {
        CLK1HZDIVUPDATEREQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&mut self) -> CLK1KHZDIV_W {
        CLK1KHZDIV_W { w: self }
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&mut self) -> CLK1KHZDIVUPDATEREQ_W {
        CLK1KHZDIVUPDATEREQ_W { w: self }
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&mut self) -> CLK1HZDIV_W {
        CLK1HZDIV_W { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&mut self) -> CLK1HZDIVHALT_W {
        CLK1HZDIVHALT_W { w: self }
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&mut self) -> CLK1HZDIVUPDATEREQ_W {
        CLK1HZDIVUPDATEREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcosc32k](index.html) module"]
pub struct RTCOSC32K_SPEC;
impl crate::RegisterSpec for RTCOSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcosc32k::R](R) reader structure"]
impl crate::Readable for RTCOSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcosc32k::W](W) writer structure"]
impl crate::Writable for RTCOSC32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOSC32K to value 0x03ff_0008"]
impl crate::Resettable for RTCOSC32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff_0008
    }
}
