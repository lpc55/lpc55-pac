#[doc = "Register `COMP_INT_STATUS` reader"]
pub struct R(crate::R<COMP_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<COMP_INT_STATUS_SPEC>> for R {
    fn from(reader: crate::R<COMP_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_INT_STATUS` writer"]
pub struct W(crate::W<COMP_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_INT_STATUS_SPEC>;
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
impl core::convert::From<crate::W<COMP_INT_STATUS_SPEC>> for W {
    fn from(writer: crate::W<COMP_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt status BEFORE Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: no interrupt pending."]
    NO_INT = 0,
    #[doc = "1: interrupt pending."]
    PENDING = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Interrupt status BEFORE Interrupt Enable."]
pub struct STATUS_R(crate::FieldReader<bool, STATUS_A>);
impl STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NO_INT,
            true => STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        **self == STATUS_A::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == STATUS_A::PENDING
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status AFTER Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_STATUS_A {
    #[doc = "0: no interrupt pending."]
    NO_INT = 0,
    #[doc = "1: interrupt pending."]
    PENDING = 1,
}
impl From<INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_STATUS` reader - Interrupt status AFTER Interrupt Enable."]
pub struct INT_STATUS_R(crate::FieldReader<bool, INT_STATUS_A>);
impl INT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_STATUS_A {
        match self.bits {
            false => INT_STATUS_A::NO_INT,
            true => INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INT`"]
    #[inline(always)]
    pub fn is_no_int(&self) -> bool {
        **self == INT_STATUS_A::NO_INT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == INT_STATUS_A::PENDING
    }
}
impl core::ops::Deref for INT_STATUS_R {
    type Target = crate::FieldReader<bool, INT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "comparator analog output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VAL_A {
    #[doc = "0: P+ is smaller than P-."]
    SMALLER = 0,
    #[doc = "1: P+ is greater than P-."]
    GREATER = 1,
}
impl From<VAL_A> for bool {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VAL` reader - comparator analog output."]
pub struct VAL_R(crate::FieldReader<bool, VAL_A>);
impl VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VAL_A {
        match self.bits {
            false => VAL_A::SMALLER,
            true => VAL_A::GREATER,
        }
    }
    #[doc = "Checks if the value of the field is `SMALLER`"]
    #[inline(always)]
    pub fn is_smaller(&self) -> bool {
        **self == VAL_A::SMALLER
    }
    #[doc = "Checks if the value of the field is `GREATER`"]
    #[inline(always)]
    pub fn is_greater(&self) -> bool {
        **self == VAL_A::GREATER
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<bool, VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status BEFORE Interrupt Enable."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status AFTER Interrupt Enable."]
    #[inline(always)]
    pub fn int_status(&self) -> INT_STATUS_R {
        INT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - comparator analog output."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_int_status](index.html) module"]
pub struct COMP_INT_STATUS_SPEC;
impl crate::RegisterSpec for COMP_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_int_status::R](R) reader structure"]
impl crate::Readable for COMP_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_int_status::W](W) writer structure"]
impl crate::Writable for COMP_INT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_INT_STATUS to value 0"]
impl crate::Resettable for COMP_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
