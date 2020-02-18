#[doc = "Reader of register PINTSECSEL[%s]"]
pub type R = crate::R<u32, super::PINTSECSEL>;
#[doc = "Writer for register PINTSECSEL[%s]"]
pub type W = crate::W<u32, super::PINTSECSEL>;
#[doc = "Register PINTSECSEL[%s]
`reset()`'s with value 0x3f"]
impl crate::ResetValue for super::PINTSECSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `INTPIN`"]
pub type INTPIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTPIN`"]
pub struct INTPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&mut self) -> INTPIN_W {
        INTPIN_W { w: self }
    }
}
