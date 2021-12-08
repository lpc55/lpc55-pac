#[doc = "Register `SECURE_BOOT_CFG` reader"]
pub struct R(crate::R<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_BOOT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_BOOT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_BOOT_CFG` writer"]
pub struct W(crate::W<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SECURE_BOOT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_BOOT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSA4K` reader - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
pub struct RSA4K_R(crate::FieldReader<u8, u8>);
impl RSA4K_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RSA4K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSA4K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSA4K` writer - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
pub struct RSA4K_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA4K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DICE_ENC_NXP_CFG` reader - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
pub struct DICE_ENC_NXP_CFG_R(crate::FieldReader<u8, u8>);
impl DICE_ENC_NXP_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DICE_ENC_NXP_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DICE_ENC_NXP_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DICE_ENC_NXP_CFG` writer - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
pub struct DICE_ENC_NXP_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_ENC_NXP_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DICE_CUST_CFG` reader - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
pub struct DICE_CUST_CFG_R(crate::FieldReader<u8, u8>);
impl DICE_CUST_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DICE_CUST_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DICE_CUST_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DICE_CUST_CFG` writer - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
pub struct DICE_CUST_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_CUST_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `SKIP_DICE` reader - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
pub struct SKIP_DICE_R(crate::FieldReader<u8, u8>);
impl SKIP_DICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SKIP_DICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIP_DICE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIP_DICE` writer - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
pub struct SKIP_DICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_DICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TZM_IMAGE_TYPE` reader - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
pub struct TZM_IMAGE_TYPE_R(crate::FieldReader<u8, u8>);
impl TZM_IMAGE_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TZM_IMAGE_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZM_IMAGE_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZM_IMAGE_TYPE` writer - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
pub struct TZM_IMAGE_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZM_IMAGE_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `BLOCK_SET_KEY` reader - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
pub struct BLOCK_SET_KEY_R(crate::FieldReader<u8, u8>);
impl BLOCK_SET_KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLOCK_SET_KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_SET_KEY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_SET_KEY` writer - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
pub struct BLOCK_SET_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SET_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `BLOCK_ENROLL` reader - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
pub struct BLOCK_ENROLL_R(crate::FieldReader<u8, u8>);
impl BLOCK_ENROLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLOCK_ENROLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_ENROLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_ENROLL` writer - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
pub struct BLOCK_ENROLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_ENROLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DICE_INC_SEC_EPOCH` reader - Include security EPOCH in DICE"]
pub struct DICE_INC_SEC_EPOCH_R(crate::FieldReader<u8, u8>);
impl DICE_INC_SEC_EPOCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DICE_INC_SEC_EPOCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DICE_INC_SEC_EPOCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DICE_INC_SEC_EPOCH` writer - Include security EPOCH in DICE"]
pub struct DICE_INC_SEC_EPOCH_W<'a> {
    w: &'a mut W,
}
impl<'a> DICE_INC_SEC_EPOCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `SEC_BOOT_EN` reader - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
pub struct SEC_BOOT_EN_R(crate::FieldReader<u8, u8>);
impl SEC_BOOT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_BOOT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_BOOT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_BOOT_EN` writer - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
pub struct SEC_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_BOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
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
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&self) -> DICE_INC_SEC_EPOCH_R {
        DICE_INC_SEC_EPOCH_R::new(((self.bits >> 14) & 0x03) as u8)
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
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&mut self) -> DICE_INC_SEC_EPOCH_W {
        DICE_INC_SEC_EPOCH_W { w: self }
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn sec_boot_en(&mut self) -> SEC_BOOT_EN_W {
        SEC_BOOT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_boot_cfg](index.html) module"]
pub struct SECURE_BOOT_CFG_SPEC;
impl crate::RegisterSpec for SECURE_BOOT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_boot_cfg::R](R) reader structure"]
impl crate::Readable for SECURE_BOOT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_boot_cfg::W](W) writer structure"]
impl crate::Writable for SECURE_BOOT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURE_BOOT_CFG to value 0"]
impl crate::Resettable for SECURE_BOOT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
