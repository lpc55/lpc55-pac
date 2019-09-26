#[doc = "Reader of register DATAPAYLOAD"]
pub type R = crate::R<u32, super::DATAPAYLOAD>;
#[doc = "Writer for register DATAPAYLOAD"]
pub type W = crate::W<u32, super::DATAPAYLOAD>;
#[doc = "Register DATAPAYLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::DATAPAYLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT_BASE`"]
pub type DAT_BASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DAT_BASE`"]
pub struct DAT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&self) -> DAT_BASE_R {
        DAT_BASE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base address to be used by the hardware to find the start of the data payload section."]
    #[inline(always)]
    pub fn dat_base(&mut self) -> DAT_BASE_W {
        DAT_BASE_W { w: self }
    }
}
