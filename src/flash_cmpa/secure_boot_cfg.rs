#[doc = "Reader of register SECURE_BOOT_CFG"]
pub type R = crate::R<u32, super::SECURE_BOOT_CFG>;
#[doc = "Writer for register SECURE_BOOT_CFG"]
pub type W = crate::W<u32, super::SECURE_BOOT_CFG>;
#[doc = "Register SECURE_BOOT_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SECURE_BOOT_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSA4K`"]
pub type RSA4K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSA4K`"]
pub struct RSA4K_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA4K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DICE_ENC_NXP_CFG`"]
pub type DICE_ENC_NXP_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DICE_ENC_NXP_CFG`"]
pub struct DICE_ENC_NXP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_ENC_NXP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DICE_CUST_CFG`"]
pub type DICE_CUST_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DICE_CUST_CFG`"]
pub struct DICE_CUST_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_CUST_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SKIP_DICE`"]
pub type SKIP_DICE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SKIP_DICE`"]
pub struct SKIP_DICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_DICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TZM_IMAGE_TYPE`"]
pub type TZM_IMAGE_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TZM_IMAGE_TYPE`"]
pub struct TZM_IMAGE_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZM_IMAGE_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `BLOCK_SET_KEY`"]
pub type BLOCK_SET_KEY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLOCK_SET_KEY`"]
pub struct BLOCK_SET_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SET_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `BLOCK_ENROLL`"]
pub type BLOCK_ENROLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLOCK_ENROLL`"]
pub struct BLOCK_ENROLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_ENROLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SEC_BOOT_EN`"]
pub type SEC_BOOT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC_BOOT_EN`"]
pub struct SEC_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_BOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline(always)]
    pub fn rsa4k(&self) -> RSA4K_R {
        RSA4K_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_enc_nxp_cfg(&self) -> DICE_ENC_NXP_CFG_R {
        DICE_ENC_NXP_CFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_cust_cfg(&self) -> DICE_CUST_CFG_R {
        DICE_CUST_CFG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline(always)]
    pub fn skip_dice(&self) -> SKIP_DICE_R {
        SKIP_DICE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline(always)]
    pub fn tzm_image_type(&self) -> TZM_IMAGE_TYPE_R {
        TZM_IMAGE_TYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline(always)]
    pub fn block_set_key(&self) -> BLOCK_SET_KEY_R {
        BLOCK_SET_KEY_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline(always)]
    pub fn block_enroll(&self) -> BLOCK_ENROLL_R {
        BLOCK_ENROLL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn sec_boot_en(&self) -> SEC_BOOT_EN_R {
        SEC_BOOT_EN_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline(always)]
    pub fn rsa4k(&mut self) -> RSA4K_W {
        RSA4K_W { w: self }
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_enc_nxp_cfg(&mut self) -> DICE_ENC_NXP_CFG_W {
        DICE_ENC_NXP_CFG_W { w: self }
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_cust_cfg(&mut self) -> DICE_CUST_CFG_W {
        DICE_CUST_CFG_W { w: self }
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline(always)]
    pub fn skip_dice(&mut self) -> SKIP_DICE_W {
        SKIP_DICE_W { w: self }
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline(always)]
    pub fn tzm_image_type(&mut self) -> TZM_IMAGE_TYPE_W {
        TZM_IMAGE_TYPE_W { w: self }
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline(always)]
    pub fn block_set_key(&mut self) -> BLOCK_SET_KEY_W {
        BLOCK_SET_KEY_W { w: self }
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline(always)]
    pub fn block_enroll(&mut self) -> BLOCK_ENROLL_W {
        BLOCK_ENROLL_W { w: self }
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn sec_boot_en(&mut self) -> SEC_BOOT_EN_W {
        SEC_BOOT_EN_W { w: self }
    }
}
