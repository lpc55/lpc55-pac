#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `decode_opcode`"]
pub type DECODE_OPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `decode_opcode`"]
pub struct DECODE_OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODE_OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `decode_machine`"]
pub type DECODE_MACHINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `decode_machine`"]
pub struct DECODE_MACHINE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECODE_MACHINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `inst_busy`"]
pub type INST_BUSY_R = crate::R<bool, bool>;
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
}
