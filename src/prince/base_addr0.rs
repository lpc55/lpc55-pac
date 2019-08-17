#[doc = "Reader of register BASE_ADDR0"]
pub type R = crate::R<u32, super::BASE_ADDR0>;
#[doc = "Writer for register BASE_ADDR0"]
pub type W = crate::W<u32, super::BASE_ADDR0>;
#[doc = "Register BASE_ADDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BASE_ADDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_FIXED`"]
pub type ADDR_FIXED_R = crate::R<u32, u32>;
#[doc = "Reader of field `ADDR_PRG`"]
pub type ADDR_PRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR_PRG`"]
pub struct ADDR_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_fixed(&self) -> ADDR_FIXED_R {
        ADDR_FIXED_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&self) -> ADDR_PRG_R {
        ADDR_PRG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&mut self) -> ADDR_PRG_W {
        ADDR_PRG_W { w: self }
    }
}
