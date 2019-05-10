#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SECURE_BOOT_CFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RSA4KR {
    bits: u8,
}
impl RSA4KR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DICE_ENC_NXP_CFGR {
    bits: u8,
}
impl DICE_ENC_NXP_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DICE_CUST_CFGR {
    bits: u8,
}
impl DICE_CUST_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SKIP_DICER {
    bits: u8,
}
impl SKIP_DICER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TZM_IMAGE_TYPER {
    bits: u8,
}
impl TZM_IMAGE_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BLOCK_SET_KEYR {
    bits: u8,
}
impl BLOCK_SET_KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BLOCK_ENROLLR {
    bits: u8,
}
impl BLOCK_ENROLLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEC_BOOT_ENR {
    bits: u8,
}
impl SEC_BOOT_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RSA4KW<'a> {
    w: &'a mut W,
}
impl<'a> _RSA4KW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DICE_ENC_NXP_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DICE_ENC_NXP_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DICE_CUST_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DICE_CUST_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SKIP_DICEW<'a> {
    w: &'a mut W,
}
impl<'a> _SKIP_DICEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TZM_IMAGE_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TZM_IMAGE_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_SET_KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_SET_KEYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLOCK_ENROLLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCK_ENROLLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEC_BOOT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_BOOT_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline]
    pub fn rsa4k(&self) -> RSA4KR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSA4KR { bits }
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline]
    pub fn dice_enc_nxp_cfg(&self) -> DICE_ENC_NXP_CFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DICE_ENC_NXP_CFGR { bits }
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline]
    pub fn dice_cust_cfg(&self) -> DICE_CUST_CFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DICE_CUST_CFGR { bits }
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline]
    pub fn skip_dice(&self) -> SKIP_DICER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SKIP_DICER { bits }
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline]
    pub fn tzm_image_type(&self) -> TZM_IMAGE_TYPER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TZM_IMAGE_TYPER { bits }
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline]
    pub fn block_set_key(&self) -> BLOCK_SET_KEYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLOCK_SET_KEYR { bits }
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline]
    pub fn block_enroll(&self) -> BLOCK_ENROLLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLOCK_ENROLLR { bits }
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline]
    pub fn sec_boot_en(&self) -> SEC_BOOT_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEC_BOOT_ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline]
    pub fn rsa4k(&mut self) -> _RSA4KW {
        _RSA4KW { w: self }
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline]
    pub fn dice_enc_nxp_cfg(&mut self) -> _DICE_ENC_NXP_CFGW {
        _DICE_ENC_NXP_CFGW { w: self }
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline]
    pub fn dice_cust_cfg(&mut self) -> _DICE_CUST_CFGW {
        _DICE_CUST_CFGW { w: self }
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline]
    pub fn skip_dice(&mut self) -> _SKIP_DICEW {
        _SKIP_DICEW { w: self }
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline]
    pub fn tzm_image_type(&mut self) -> _TZM_IMAGE_TYPEW {
        _TZM_IMAGE_TYPEW { w: self }
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline]
    pub fn block_set_key(&mut self) -> _BLOCK_SET_KEYW {
        _BLOCK_SET_KEYW { w: self }
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline]
    pub fn block_enroll(&mut self) -> _BLOCK_ENROLLW {
        _BLOCK_ENROLLW { w: self }
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline]
    pub fn sec_boot_en(&mut self) -> _SEC_BOOT_ENW {
        _SEC_BOOT_ENW { w: self }
    }
}
