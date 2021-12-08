#[doc = "Register `BOD_DCDC_INT_CTRL` reader"]
pub struct R(crate::R<BOD_DCDC_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_DCDC_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_DCDC_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_DCDC_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD_DCDC_INT_CTRL` writer"]
pub struct W(crate::W<BOD_DCDC_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_DCDC_INT_CTRL_SPEC>;
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
impl From<crate::W<BOD_DCDC_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_DCDC_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BOD VBAT interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_ENABLE_A {
    #[doc = "0: BOD VBAT interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: BOD VBAT interrupt is enabled."]
    ENABLE = 1,
}
impl From<BODVBAT_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` reader - BOD VBAT interrupt control."]
pub struct BODVBAT_INT_ENABLE_R(crate::FieldReader<bool, BODVBAT_INT_ENABLE_A>);
impl BODVBAT_INT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVBAT_INT_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_INT_ENABLE_A {
        match self.bits {
            false => BODVBAT_INT_ENABLE_A::DISABLE,
            true => BODVBAT_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODVBAT_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODVBAT_INT_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for BODVBAT_INT_ENABLE_R {
    type Target = crate::FieldReader<bool, BODVBAT_INT_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVBAT_INT_ENABLE` writer - BOD VBAT interrupt control."]
pub struct BODVBAT_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVBAT_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODVBAT_INT_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::ENABLE)
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
#[doc = "Field `BODVBAT_INT_CLEAR` reader - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct BODVBAT_INT_CLEAR_R(crate::FieldReader<bool, bool>);
impl BODVBAT_INT_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVBAT_INT_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVBAT_INT_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVBAT_INT_CLEAR` writer - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct BODVBAT_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVBAT_INT_CLEAR_W<'a> {
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
#[doc = "BOD CORE interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_ENABLE_A {
    #[doc = "0: BOD CORE interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: BOD CORE interrupt is enabled."]
    ENABLE = 1,
}
impl From<BODCORE_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORE_INT_ENABLE` reader - BOD CORE interrupt control."]
pub struct BODCORE_INT_ENABLE_R(crate::FieldReader<bool, BODCORE_INT_ENABLE_A>);
impl BODCORE_INT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODCORE_INT_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_INT_ENABLE_A {
        match self.bits {
            false => BODCORE_INT_ENABLE_A::DISABLE,
            true => BODCORE_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODCORE_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODCORE_INT_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for BODCORE_INT_ENABLE_R {
    type Target = crate::FieldReader<bool, BODCORE_INT_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCORE_INT_ENABLE` writer - BOD CORE interrupt control."]
pub struct BODCORE_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCORE_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODCORE_INT_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BODCORE_INT_CLEAR` reader - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct BODCORE_INT_CLEAR_R(crate::FieldReader<bool, bool>);
impl BODCORE_INT_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODCORE_INT_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODCORE_INT_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODCORE_INT_CLEAR` writer - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct BODCORE_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCORE_INT_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "DCDC interrupt control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_ENABLE_A {
    #[doc = "0: DCDC interrupt is disabled."]
    DISABLE = 0,
    #[doc = "1: DCDC interrupt is enabled."]
    ENABLE = 1,
}
impl From<DCDC_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_INT_ENABLE` reader - DCDC interrupt control."]
pub struct DCDC_INT_ENABLE_R(crate::FieldReader<bool, DCDC_INT_ENABLE_A>);
impl DCDC_INT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_INT_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_INT_ENABLE_A {
        match self.bits {
            false => DCDC_INT_ENABLE_A::DISABLE,
            true => DCDC_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DCDC_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DCDC_INT_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for DCDC_INT_ENABLE_R {
    type Target = crate::FieldReader<bool, DCDC_INT_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_INT_ENABLE` writer - DCDC interrupt control."]
pub struct DCDC_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_INT_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DCDC interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::DISABLE)
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DCDC_INT_CLEAR` reader - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct DCDC_INT_CLEAR_R(crate::FieldReader<bool, bool>);
impl DCDC_INT_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_INT_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_INT_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_INT_CLEAR` writer - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
pub struct DCDC_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_INT_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&self) -> BODVBAT_INT_ENABLE_R {
        BODVBAT_INT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&self) -> BODVBAT_INT_CLEAR_R {
        BODVBAT_INT_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&self) -> BODCORE_INT_ENABLE_R {
        BODCORE_INT_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&self) -> BODCORE_INT_CLEAR_R {
        BODCORE_INT_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&self) -> DCDC_INT_ENABLE_R {
        DCDC_INT_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&self) -> DCDC_INT_CLEAR_R {
        DCDC_INT_CLEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&mut self) -> BODVBAT_INT_ENABLE_W {
        BODVBAT_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&mut self) -> BODVBAT_INT_CLEAR_W {
        BODVBAT_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&mut self) -> BODCORE_INT_ENABLE_W {
        BODCORE_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&mut self) -> BODCORE_INT_CLEAR_W {
        BODCORE_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&mut self) -> DCDC_INT_ENABLE_W {
        DCDC_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&mut self) -> DCDC_INT_CLEAR_W {
        DCDC_INT_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod_dcdc_int_ctrl](index.html) module"]
pub struct BOD_DCDC_INT_CTRL_SPEC;
impl crate::RegisterSpec for BOD_DCDC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod_dcdc_int_ctrl::R](R) reader structure"]
impl crate::Readable for BOD_DCDC_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod_dcdc_int_ctrl::W](W) writer structure"]
impl crate::Writable for BOD_DCDC_INT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD_DCDC_INT_CTRL to value 0"]
impl crate::Resettable for BOD_DCDC_INT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
