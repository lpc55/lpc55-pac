#[doc = "Register `LENGTH` reader"]
pub struct R(crate::R<LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LENGTH` writer"]
pub struct W(crate::W<LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LENGTH_SPEC>;
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
impl From<crate::W<LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inst_length` reader - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
pub struct INST_LENGTH_R(crate::FieldReader<u32, u32>);
impl INST_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INST_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INST_LENGTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inst_length` writer - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
pub struct INST_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub fn inst_length(&self) -> INST_LENGTH_R {
        INST_LENGTH_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub fn inst_length(&mut self) -> INST_LENGTH_W {
        INST_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](index.html) module"]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [length::R](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [length::W](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
