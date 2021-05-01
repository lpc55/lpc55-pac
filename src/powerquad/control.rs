#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONTROL_SPEC>> for R {
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl core::convert::From<crate::W<CONTROL_SPEC>> for W {
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `decode_opcode` reader - opcode specific to decode_machine"]
pub struct DECODE_OPCODE_R(crate::FieldReader<u8, u8>);
impl DECODE_OPCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECODE_OPCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECODE_OPCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `decode_opcode` writer - opcode specific to decode_machine"]
pub struct DECODE_OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODE_OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `decode_machine` reader - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
pub struct DECODE_MACHINE_R(crate::FieldReader<u8, u8>);
impl DECODE_MACHINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECODE_MACHINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECODE_MACHINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `decode_machine` writer - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
pub struct DECODE_MACHINE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODE_MACHINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `inst_busy` reader - Instruction busy signal when high indicates processing is on"]
pub struct INST_BUSY_R(crate::FieldReader<bool, bool>);
impl INST_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        INST_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INST_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - opcode specific to decode_machine"]
    #[inline(always)]
    pub fn decode_opcode(&self) -> DECODE_OPCODE_R {
        DECODE_OPCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub fn decode_machine(&self) -> DECODE_MACHINE_R {
        DECODE_MACHINE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Instruction busy signal when high indicates processing is on"]
    #[inline(always)]
    pub fn inst_busy(&self) -> INST_BUSY_R {
        INST_BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - opcode specific to decode_machine"]
    #[inline(always)]
    pub fn decode_opcode(&mut self) -> DECODE_OPCODE_W {
        DECODE_OPCODE_W { w: self }
    }
    #[doc = "Bits 4:7 - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub fn decode_machine(&mut self) -> DECODE_MACHINE_W {
        DECODE_MACHINE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PowerQuad Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
