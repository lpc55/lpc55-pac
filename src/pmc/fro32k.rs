#[doc = "Reader of register FRO32K"]
pub type R = crate::R<u32, super::FRO32K>;
#[doc = "Writer for register FRO32K"]
pub type W = crate::W<u32, super::FRO32K>;
#[doc = "Register FRO32K `reset()`'s with value 0x90b6"]
impl crate::ResetValue for super::FRO32K {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x90b6
    }
}
#[doc = "Reader of field `NTAT`"]
pub type NTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTAT`"]
pub struct NTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> NTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `PTAT`"]
pub type PTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PTAT`"]
pub struct PTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CAPCAL`"]
pub type CAPCAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPCAL`"]
pub struct CAPCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | (((value as u32) & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Reader of field `ATBCTRL`"]
pub type ATBCTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATBCTRL`"]
pub struct ATBCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATBCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:3 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn ntat(&self) -> NTAT_R {
        NTAT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn ptat(&self) -> PTAT_R {
        PTAT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:15 - Capacitive dac calibration bits (fine frequency trimming)."]
    #[inline(always)]
    pub fn capcal(&self) -> CAPCAL_R {
        CAPCAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Debug control bits to set the analog/digital test modes."]
    #[inline(always)]
    pub fn atbctrl(&self) -> ATBCTRL_R {
        ATBCTRL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn ntat(&mut self) -> NTAT_W {
        NTAT_W { w: self }
    }
    #[doc = "Bits 4:6 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn ptat(&mut self) -> PTAT_W {
        PTAT_W { w: self }
    }
    #[doc = "Bits 7:15 - Capacitive dac calibration bits (fine frequency trimming)."]
    #[inline(always)]
    pub fn capcal(&mut self) -> CAPCAL_W {
        CAPCAL_W { w: self }
    }
    #[doc = "Bits 16:17 - Debug control bits to set the analog/digital test modes."]
    #[inline(always)]
    pub fn atbctrl(&mut self) -> ATBCTRL_W {
        ATBCTRL_W { w: self }
    }
}
