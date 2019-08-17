#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "unlock"]
    UNLOCK,
    #[doc = "Lock to current security level"]
    LOCK,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        match variant {
            LOCK_A::UNLOCK => false,
            LOCK_A::LOCK => true,
        }
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCK,
            true => LOCK_A::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LOCK_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "unlock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCK)
    }
    #[doc = "Lock to current security level"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK)
    }
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
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_A {
    #[doc = "If set during write, will allow lock or unlock"]
    KWY_VALUE,
}
impl From<KEY_A> for u16 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        match variant {
            KEY_A::KWY_VALUE => 1853,
        }
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u16, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            1853 => Val(KEY_A::KWY_VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KWY_VALUE`"]
    #[inline(always)]
    pub fn is_kwy_value(&self) -> bool {
        *self == KEY_A::KWY_VALUE
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "If set during write, will allow lock or unlock"]
    #[inline(always)]
    pub fn kwy_value(self) -> &'a mut W {
        self.variant(KEY_A::KWY_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 4)) | (((value as u32) & 0x1fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
