#[doc = "Register `UPDATELCKOUT` reader"]
pub struct R(crate::R<UPDATELCKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATELCKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UPDATELCKOUT_SPEC>> for R {
    fn from(reader: crate::R<UPDATELCKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATELCKOUT` writer"]
pub struct W(crate::W<UPDATELCKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATELCKOUT_SPEC>;
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
impl core::convert::From<crate::W<UPDATELCKOUT_SPEC>> for W {
    fn from(writer: crate::W<UPDATELCKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "All Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATELCKOUT_A {
    #[doc = "0: Normal Mode. Can be written to."]
    NORMAL_MODE = 0,
    #[doc = "1: Protected Mode. Cannot be written to."]
    PROTECTED_MODE = 1,
}
impl From<UPDATELCKOUT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATELCKOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATELCKOUT` reader - All Registers"]
pub struct UPDATELCKOUT_R(crate::FieldReader<bool, UPDATELCKOUT_A>);
impl UPDATELCKOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDATELCKOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATELCKOUT_A {
        match self.bits {
            false => UPDATELCKOUT_A::NORMAL_MODE,
            true => UPDATELCKOUT_A::PROTECTED_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        **self == UPDATELCKOUT_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `PROTECTED_MODE`"]
    #[inline(always)]
    pub fn is_protected_mode(&self) -> bool {
        **self == UPDATELCKOUT_A::PROTECTED_MODE
    }
}
impl core::ops::Deref for UPDATELCKOUT_R {
    type Target = crate::FieldReader<bool, UPDATELCKOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATELCKOUT` writer - All Registers"]
pub struct UPDATELCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATELCKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATELCKOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode. Can be written to."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::NORMAL_MODE)
    }
    #[doc = "Protected Mode. Cannot be written to."]
    #[inline(always)]
    pub fn protected_mode(self) -> &'a mut W {
        self.variant(UPDATELCKOUT_A::PROTECTED_MODE)
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
impl R {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&self) -> UPDATELCKOUT_R {
        UPDATELCKOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&mut self) -> UPDATELCKOUT_W {
        UPDATELCKOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "update lock out control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [updatelckout](index.html) module"]
pub struct UPDATELCKOUT_SPEC;
impl crate::RegisterSpec for UPDATELCKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [updatelckout::R](R) reader structure"]
impl crate::Readable for UPDATELCKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [updatelckout::W](W) writer structure"]
impl crate::Writable for UPDATELCKOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPDATELCKOUT to value 0"]
impl crate::Resettable for UPDATELCKOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
