#[doc = "Reader of register MISC_CFG"]
pub type R = crate::R<u32, super::MISC_CFG>;
#[doc = "Writer for register MISC_CFG"]
pub type W = crate::W<u32, super::MISC_CFG>;
#[doc = "Register MISC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_RESEED`"]
pub type AES_RESEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_RESEED`"]
pub struct AES_RESEED_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_RESEED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `AES_DT_CFG`"]
pub type AES_DT_CFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_DT_CFG`"]
pub struct AES_DT_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_DT_CFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&self) -> AES_RESEED_R {
        AES_RESEED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&self) -> AES_DT_CFG_R {
        AES_DT_CFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&mut self) -> AES_RESEED_W {
        AES_RESEED_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&mut self) -> AES_DT_CFG_W {
        AES_DT_CFG_W { w: self }
    }
}
