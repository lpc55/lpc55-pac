#[doc = "Reader of register IDXBLK_H"]
pub type R = crate::R<u32, super::IDXBLK_H>;
#[doc = "Writer for register IDXBLK_H"]
pub type W = crate::W<u32, super::IDXBLK_H>;
#[doc = "Register IDXBLK_H `reset()`'s with value 0x8000_aaaa"]
impl crate::ResetValue for super::IDXBLK_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_aaaa
    }
}
#[doc = "Reader of field `IDX8`"]
pub type IDX8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX8`"]
pub struct IDX8_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IDX9`"]
pub type IDX9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX9`"]
pub struct IDX9_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IDX10`"]
pub type IDX10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX10`"]
pub struct IDX10_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `IDX11`"]
pub type IDX11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX11`"]
pub struct IDX11_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IDX12`"]
pub type IDX12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX12`"]
pub struct IDX12_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IDX13`"]
pub type IDX13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX13`"]
pub struct IDX13_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IDX14`"]
pub type IDX14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX14`"]
pub struct IDX14_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `IDX15`"]
pub type IDX15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX15`"]
pub struct IDX15_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `LOCK_IDX`"]
pub struct LOCK_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&self) -> IDX8_R {
        IDX8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&self) -> IDX9_R {
        IDX9_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&self) -> IDX10_R {
        IDX10_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&self) -> IDX11_R {
        IDX11_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&self) -> IDX12_R {
        IDX12_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&self) -> IDX13_R {
        IDX13_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&self) -> IDX14_R {
        IDX14_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&self) -> IDX15_R {
        IDX15_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&mut self) -> IDX8_W {
        IDX8_W { w: self }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&mut self) -> IDX9_W {
        IDX9_W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&mut self) -> IDX10_W {
        IDX10_W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&mut self) -> IDX11_W {
        IDX11_W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&mut self) -> IDX12_W {
        IDX12_W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&mut self) -> IDX13_W {
        IDX13_W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&mut self) -> IDX14_W {
        IDX14_W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&mut self) -> IDX15_W {
        IDX15_W { w: self }
    }
    #[doc = "Bits 30:31 - Lock 8 to 15 PUF key indexes"]
    #[inline(always)]
    pub fn lock_idx(&mut self) -> LOCK_IDX_W {
        LOCK_IDX_W { w: self }
    }
}
