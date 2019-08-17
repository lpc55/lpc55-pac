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
#[doc = "Possible values of the field `SECLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECLOCK_A {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK,
}
impl From<SECLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SECLOCK_A) -> Self {
        match variant {
            SECLOCK_A::UNLOCK => 0,
            SECLOCK_A::LOCK => 1,
        }
    }
}
#[doc = "Reader of field `SECLOCK`"]
pub type SECLOCK_R = crate::R<u8, SECLOCK_A>;
impl SECLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECLOCK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECLOCK_A::UNLOCK),
            1 => Val(SECLOCK_A::LOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == SECLOCK_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == SECLOCK_A::LOCK
    }
}
#[doc = "Write proxy for field `SECLOCK`"]
pub struct SECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECLOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(SECLOCK_A::UNLOCK)
    }
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(SECLOCK_A::LOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PATTERN`"]
pub type PATTERN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PATTERN`"]
pub struct PATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> PATTERN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub fn pattern(&self) -> PATTERN_R {
        PATTERN_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub fn seclock(&mut self) -> SECLOCK_W {
        SECLOCK_W { w: self }
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub fn pattern(&mut self) -> PATTERN_W {
        PATTERN_W { w: self }
    }
}
