#[doc = "Reader of register ROTKH_REVOKE"]
pub type R = crate::R<u32, super::ROTKH_REVOKE>;
#[doc = "Writer for register ROTKH_REVOKE"]
pub type W = crate::W<u32, super::ROTKH_REVOKE>;
#[doc = "Register ROTKH_REVOKE `reset()`'s with value 0"]
impl crate::ResetValue for super::ROTKH_REVOKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RoTK0_EN`"]
pub type ROTK0_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RoTK0_EN`"]
pub struct ROTK0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTK0_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RoTK1_EN`"]
pub type ROTK1_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RoTK1_EN`"]
pub struct ROTK1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTK1_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RoTK2_EN`"]
pub type ROTK2_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RoTK2_EN`"]
pub struct ROTK2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTK2_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk0_en(&self) -> ROTK0_EN_R {
        ROTK0_EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk1_en(&self) -> ROTK1_EN_R {
        ROTK1_EN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk2_en(&self) -> ROTK2_EN_R {
        ROTK2_EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk0_en(&mut self) -> ROTK0_EN_W {
        ROTK0_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk1_en(&mut self) -> ROTK1_EN_W {
        ROTK1_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk2_en(&mut self) -> ROTK2_EN_W {
        ROTK2_EN_W { w: self }
    }
}
