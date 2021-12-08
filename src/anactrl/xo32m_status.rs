#[doc = "Register `XO32M_STATUS` reader"]
pub struct R(crate::R<XO32M_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XO32M_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XO32M_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XO32M_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates XO out frequency statibilty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO_READY_A {
    #[doc = "0: XO output frequency is not yet stable."]
    NOT_STABLE = 0,
    #[doc = "1: XO output frequency is stable."]
    STABLE = 1,
}
impl From<XO_READY_A> for bool {
    #[inline(always)]
    fn from(variant: XO_READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XO_READY` reader - Indicates XO out frequency statibilty."]
pub struct XO_READY_R(crate::FieldReader<bool, XO_READY_A>);
impl XO_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XO_READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XO_READY_A {
        match self.bits {
            false => XO_READY_A::NOT_STABLE,
            true => XO_READY_A::STABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_STABLE`"]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        **self == XO_READY_A::NOT_STABLE
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        **self == XO_READY_A::STABLE
    }
}
impl core::ops::Deref for XO_READY_R {
    type Target = crate::FieldReader<bool, XO_READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates XO out frequency statibilty."]
    #[inline(always)]
    pub fn xo_ready(&self) -> XO_READY_R {
        XO_READY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "High speed Crystal Oscillator Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xo32m_status](index.html) module"]
pub struct XO32M_STATUS_SPEC;
impl crate::RegisterSpec for XO32M_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xo32m_status::R](R) reader structure"]
impl crate::Readable for XO32M_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XO32M_STATUS to value 0"]
impl crate::Resettable for XO32M_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
