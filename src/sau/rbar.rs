#[doc = "Reader of register RBAR"]
pub type R = crate::R<u32, super::RBAR>;
#[doc = "Writer for register RBAR"]
pub type W = crate::W<u32, super::RBAR>;
#[doc = "Register RBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BADDR`"]
pub type BADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BADDR`"]
pub struct BADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&self) -> BADDR_R {
        BADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&mut self) -> BADDR_W {
        BADDR_W { w: self }
    }
}
