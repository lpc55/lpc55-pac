#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `zeroize`"]
pub type ZEROIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `zeroize`"]
pub struct ZEROIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROIZE_W<'a> {
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
#[doc = "Reader of field `enroll`"]
pub type ENROLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enroll`"]
pub struct ENROLL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENROLL_W<'a> {
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
#[doc = "Reader of field `start`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `start`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `GENERATEKEY`"]
pub type GENERATEKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GENERATEKEY`"]
pub struct GENERATEKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERATEKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SETKEY`"]
pub type SETKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETKEY`"]
pub struct SETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SETKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `GETKEY`"]
pub type GETKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GETKEY`"]
pub struct GETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> GETKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&self) -> ZEROIZE_R {
        ZEROIZE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&self) -> ENROLL_R {
        ENROLL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&self) -> GENERATEKEY_R {
        GENERATEKEY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&self) -> SETKEY_R {
        SETKEY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&self) -> GETKEY_R {
        GETKEY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&mut self) -> ZEROIZE_W {
        ZEROIZE_W { w: self }
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&mut self) -> ENROLL_W {
        ENROLL_W { w: self }
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&mut self) -> GENERATEKEY_W {
        GENERATEKEY_W { w: self }
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&mut self) -> SETKEY_W {
        SETKEY_W { w: self }
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&mut self) -> GETKEY_W {
        GETKEY_W { w: self }
    }
}
