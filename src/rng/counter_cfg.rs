#[doc = "Reader of register COUNTER_CFG"]
pub type R = crate::R<u32, super::COUNTER_CFG>;
#[doc = "Writer for register COUNTER_CFG"]
pub type W = crate::W<u32, super::COUNTER_CFG>;
#[doc = "Register COUNTER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNTER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CLOCK_SEL`"]
pub type CLOCK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLOCK_SEL`"]
pub struct CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `SHIFT4X`"]
pub type SHIFT4X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFT4X`"]
pub struct SHIFT4X_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT4X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&self) -> SHIFT4X_R {
        SHIFT4X_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&mut self) -> SHIFT4X_W {
        SHIFT4X_W { w: self }
    }
}
