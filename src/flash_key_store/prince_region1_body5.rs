#[doc = "Reader of register PRINCE_REGION1_BODY5"]
pub type R = crate::R<u32, super::PRINCE_REGION1_BODY5>;
#[doc = "Writer for register PRINCE_REGION1_BODY5"]
pub type W = crate::W<u32, super::PRINCE_REGION1_BODY5>;
#[doc = "Register PRINCE_REGION1_BODY5 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRINCE_REGION1_BODY5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIELD`"]
pub type FIELD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIELD`"]
pub struct FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W {
        FIELD_W { w: self }
    }
}
