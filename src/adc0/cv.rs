#[doc = "Reader of register CV%s"]
pub type R = crate::R<u32, super::CV>;
#[doc = "Writer for register CV%s"]
pub type W = crate::W<u32, super::CV>;
#[doc = "Register CV%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CVL`"]
pub type CVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CVL`"]
pub struct CVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CVH`"]
pub type CVH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CVH`"]
pub struct CVH_W<'a> {
    w: &'a mut W,
}
impl<'a> CVH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Value Low."]
    #[inline(always)]
    pub fn cvl(&self) -> CVL_R {
        CVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value High."]
    #[inline(always)]
    pub fn cvh(&self) -> CVH_R {
        CVH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value Low."]
    #[inline(always)]
    pub fn cvl(&mut self) -> CVL_W {
        CVL_W { w: self }
    }
    #[doc = "Bits 16:31 - Compare Value High."]
    #[inline(always)]
    pub fn cvh(&mut self) -> CVH_W {
        CVH_W { w: self }
    }
}
