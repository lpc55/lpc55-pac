#[doc = "Register `OFSTRIM` reader"]
pub struct R(crate::R<OFSTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFSTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OFSTRIM_SPEC>> for R {
    fn from(reader: crate::R<OFSTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFSTRIM` writer"]
pub struct W(crate::W<OFSTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFSTRIM_SPEC>;
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
impl core::convert::From<crate::W<OFSTRIM_SPEC>> for W {
    fn from(writer: crate::W<OFSTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFSTRIM_A` reader - Trim for offset"]
pub struct OFSTRIM_A_R(crate::FieldReader<u8, u8>);
impl OFSTRIM_A_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFSTRIM_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFSTRIM_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFSTRIM_A` writer - Trim for offset"]
pub struct OFSTRIM_A_W<'a> {
    w: &'a mut W,
}
impl<'a> OFSTRIM_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `OFSTRIM_B` reader - Trim for offset"]
pub struct OFSTRIM_B_R(crate::FieldReader<u8, u8>);
impl OFSTRIM_B_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFSTRIM_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFSTRIM_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFSTRIM_B` writer - Trim for offset"]
pub struct OFSTRIM_B_W<'a> {
    w: &'a mut W,
}
impl<'a> OFSTRIM_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&self) -> OFSTRIM_A_R {
        OFSTRIM_A_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&self) -> OFSTRIM_B_R {
        OFSTRIM_B_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&mut self) -> OFSTRIM_A_W {
        OFSTRIM_A_W { w: self }
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&mut self) -> OFSTRIM_B_W {
        OFSTRIM_B_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Offset Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofstrim](index.html) module"]
pub struct OFSTRIM_SPEC;
impl crate::RegisterSpec for OFSTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofstrim::R](R) reader structure"]
impl crate::Readable for OFSTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofstrim::W](W) writer structure"]
impl crate::Writable for OFSTRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFSTRIM to value 0"]
impl crate::Resettable for OFSTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
