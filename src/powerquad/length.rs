#[doc = "Reader of register LENGTH"]
pub type R = crate::R<u32, super::LENGTH>;
#[doc = "Writer for register LENGTH"]
pub type W = crate::W<u32, super::LENGTH>;
#[doc = "Register LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `inst_length`"]
pub type INST_LENGTH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `inst_length`"]
pub struct INST_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
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
        INST_LENGTH_R::new((self.bits & 0xffff_ffff) as u32)
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
}
