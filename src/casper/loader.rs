#[doc = "Register `LOADER` reader"]
pub struct R(crate::R<LOADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOADER` writer"]
pub struct W(crate::W<LOADER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOADER_SPEC>;
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
impl From<crate::W<LOADER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOADER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub struct COUNT_R(crate::FieldReader<u8, u8>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<CTRLBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLBPAIR` reader - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub struct CTRLBPAIR_R(crate::FieldReader<bool, CTRLBPAIR_A>);
impl CTRLBPAIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRLBPAIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLBPAIR_A {
        match self.bits {
            false => CTRLBPAIR_A::PAIR0,
            true => CTRLBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        **self == CTRLBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        **self == CTRLBPAIR_A::PAIR1
    }
}
impl core::ops::Deref for CTRLBPAIR_R {
    type Target = crate::FieldReader<bool, CTRLBPAIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLBPAIR` writer - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub struct CTRLBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRLBPAIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CTRLOFF` reader - DWord Offset of CTRL pair to load next."]
pub struct CTRLOFF_R(crate::FieldReader<u16, u16>);
impl CTRLOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CTRLOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLOFF` writer - DWord Offset of CTRL pair to load next."]
pub struct CTRLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | ((value as u32 & 0x07ff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&self) -> CTRLBPAIR_R {
        CTRLBPAIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&self) -> CTRLOFF_R {
        CTRLOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&mut self) -> CTRLBPAIR_W {
        CTRLBPAIR_W { w: self }
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&mut self) -> CTRLOFF_W {
        CTRLOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loader](index.html) module"]
pub struct LOADER_SPEC;
impl crate::RegisterSpec for LOADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loader::R](R) reader structure"]
impl crate::Readable for LOADER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loader::W](W) writer structure"]
impl crate::Writable for LOADER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOADER to value 0"]
impl crate::Resettable for LOADER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
