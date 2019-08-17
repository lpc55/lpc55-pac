#[doc = "Reader of register BURST"]
pub type R = crate::R<u32, super::BURST>;
#[doc = "Writer for register BURST"]
pub type W = crate::W<u32, super::BURST>;
#[doc = "Register BURST `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::BURST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `XOR_MASK`"]
pub type XOR_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XOR_MASK`"]
pub struct XOR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> XOR_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
#[doc = "Reader of field `DESCR1`"]
pub type DESCR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DESCR1`"]
pub struct DESCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DESCR2`"]
pub type DESCR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DESCR2`"]
pub struct DESCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `DESCR3`"]
pub type DESCR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DESCR3`"]
pub struct DESCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - burst 2 XOR mask."]
    #[inline(always)]
    pub fn xor_mask(&self) -> XOR_MASK_R {
        XOR_MASK_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 20:23 - Burst 1 descriptor."]
    #[inline(always)]
    pub fn descr1(&self) -> DESCR1_R {
        DESCR1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Burst 2 descriptor."]
    #[inline(always)]
    pub fn descr2(&self) -> DESCR2_R {
        DESCR2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Burst 3 descriptor."]
    #[inline(always)]
    pub fn descr3(&self) -> DESCR3_R {
        DESCR3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - burst 2 XOR mask."]
    #[inline(always)]
    pub fn xor_mask(&mut self) -> XOR_MASK_W {
        XOR_MASK_W { w: self }
    }
    #[doc = "Bits 20:23 - Burst 1 descriptor."]
    #[inline(always)]
    pub fn descr1(&mut self) -> DESCR1_W {
        DESCR1_W { w: self }
    }
    #[doc = "Bits 24:27 - Burst 2 descriptor."]
    #[inline(always)]
    pub fn descr2(&mut self) -> DESCR2_W {
        DESCR2_W { w: self }
    }
    #[doc = "Bits 28:31 - Burst 3 descriptor."]
    #[inline(always)]
    pub fn descr3(&mut self) -> DESCR3_W {
        DESCR3_W { w: self }
    }
}
