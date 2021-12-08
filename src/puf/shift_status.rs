#[doc = "Register `SHIFT_STATUS` reader"]
pub struct R(crate::R<SHIFT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFT_STATUS` writer"]
pub struct W(crate::W<SHIFT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFT_STATUS_SPEC>;
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
impl From<crate::W<SHIFT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - Index counter from key 0 shift register"]
pub struct KEY0_R(crate::FieldReader<u8, u8>);
impl KEY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1` reader - Index counter from key 1 shift register"]
pub struct KEY1_R(crate::FieldReader<u8, u8>);
impl KEY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2` reader - Index counter from key 2 shift register"]
pub struct KEY2_R(crate::FieldReader<u8, u8>);
impl KEY2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3` reader - Index counter from key 3 shift register"]
pub struct KEY3_R(crate::FieldReader<u8, u8>);
impl KEY3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Index counter from key 0 shift register"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Index counter from key 1 shift register"]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Index counter from key 2 shift register"]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Index counter from key 3 shift register"]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shift_status](index.html) module"]
pub struct SHIFT_STATUS_SPEC;
impl crate::RegisterSpec for SHIFT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shift_status::R](R) reader structure"]
impl crate::Readable for SHIFT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shift_status::W](W) writer structure"]
impl crate::Writable for SHIFT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFT_STATUS to value 0"]
impl crate::Resettable for SHIFT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
