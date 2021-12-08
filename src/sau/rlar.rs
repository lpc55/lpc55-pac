#[doc = "Register `RLAR` reader"]
pub struct R(crate::R<RLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLAR` writer"]
pub struct W(crate::W<RLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLAR_SPEC>;
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
impl From<crate::W<RLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable. SAU region enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: SAU region is enabled."]
    ENABLED = 0,
    #[doc = "1: SAU region is disabled."]
    DISABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable. SAU region enable."]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLED,
            true => ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable. SAU region enable."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SAU region is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
    #[doc = "SAU region is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
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
#[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSC_A {
    #[doc = "0: Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE = 0,
    #[doc = "1: Region is Non-secure callable."]
    NON_SECURE_CALLABLE = 1,
}
impl From<NSC_A> for bool {
    #[inline(always)]
    fn from(variant: NSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSC` reader - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
pub struct NSC_R(crate::FieldReader<bool, NSC_A>);
impl NSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSC_A {
        match self.bits {
            false => NSC_A::NOT_NON_SECURE_CALLABLE,
            true => NSC_A::NON_SECURE_CALLABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_not_non_secure_callable(&self) -> bool {
        **self == NSC_A::NOT_NON_SECURE_CALLABLE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_non_secure_callable(&self) -> bool {
        **self == NSC_A::NON_SECURE_CALLABLE
    }
}
impl core::ops::Deref for NSC_R {
    type Target = crate::FieldReader<bool, NSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSC` writer - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
pub struct NSC_W<'a> {
    w: &'a mut W,
}
impl<'a> NSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Region is not Non-secure callable."]
    #[inline(always)]
    pub fn not_non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NOT_NON_SECURE_CALLABLE)
    }
    #[doc = "Region is Non-secure callable."]
    #[inline(always)]
    pub fn non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NON_SECURE_CALLABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LADDR` reader - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
pub struct LADDR_R(crate::FieldReader<u32, u32>);
impl LADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LADDR` writer - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
pub struct LADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | ((value as u32 & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub fn nsc(&mut self) -> NSC_W {
        NSC_W { w: self }
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub fn laddr(&mut self) -> LADDR_W {
        LADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Region Limit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlar](index.html) module"]
pub struct RLAR_SPEC;
impl crate::RegisterSpec for RLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlar::R](R) reader structure"]
impl crate::Readable for RLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlar::W](W) writer structure"]
impl crate::Writable for RLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
