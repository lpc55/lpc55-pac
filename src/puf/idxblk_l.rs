#[doc = "Reader of register IDXBLK_L"]
pub type R = crate::R<u32, super::IDXBLK_L>;
#[doc = "Writer for register IDXBLK_L"]
pub type W = crate::W<u32, super::IDXBLK_L>;
#[doc = "Register IDXBLK_L `reset()`'s with value 0x8000_aaaa"]
impl crate::ResetValue for super::IDXBLK_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_aaaa
    }
}
#[doc = "Reader of field `IDX1`"]
pub type IDX1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX1`"]
pub struct IDX1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IDX2`"]
pub type IDX2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX2`"]
pub struct IDX2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `IDX3`"]
pub type IDX3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX3`"]
pub struct IDX3_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IDX4`"]
pub type IDX4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX4`"]
pub struct IDX4_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IDX5`"]
pub type IDX5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX5`"]
pub struct IDX5_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IDX6`"]
pub type IDX6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX6`"]
pub struct IDX6_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `IDX7`"]
pub type IDX7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDX7`"]
pub struct IDX7_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX7_W<'a> {
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
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&self) -> IDX1_R {
        IDX1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&self) -> IDX2_R {
        IDX2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&self) -> IDX3_R {
        IDX3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&self) -> IDX4_R {
        IDX4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&self) -> IDX5_R {
        IDX5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&self) -> IDX6_R {
        IDX6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&self) -> IDX7_R {
        IDX7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&mut self) -> IDX1_W {
        IDX1_W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&mut self) -> IDX2_W {
        IDX2_W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&mut self) -> IDX3_W {
        IDX3_W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&mut self) -> IDX4_W {
        IDX4_W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&mut self) -> IDX5_W {
        IDX5_W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&mut self) -> IDX6_W {
        IDX6_W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&mut self) -> IDX7_W {
        IDX7_W { w: self }
    }
    #[doc = "Bits 30:31 - Lock 0 to 7 PUF key indexes"]
    #[inline(always)]
    pub fn lock_idx(&mut self) -> LOCK_IDX_W {
        LOCK_IDX_W { w: self }
    }
}
