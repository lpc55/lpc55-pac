#[doc = "Writer for register KEY_BLOCK"]
pub type W = crate::W<u32, super::KEY_BLOCK>;
#[doc = "Register KEY_BLOCK `reset()`'s with value 0x3cc3_5aa5"]
impl crate::ResetValue for super::KEY_BLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3cc3_5aa5
    }
}
#[doc = "Write proxy for field `KEY_BLOCK`"]
pub struct KEY_BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_BLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write a value to block quiddikey/PUF all index."]
    #[inline(always)]
    pub fn key_block(&mut self) -> KEY_BLOCK_W {
        KEY_BLOCK_W { w: self }
    }
}
