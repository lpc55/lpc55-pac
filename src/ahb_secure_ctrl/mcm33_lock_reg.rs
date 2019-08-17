#[doc = "Reader of register MCM33_LOCK_REG"]
pub type R = crate::R<u32, super::MCM33_LOCK_REG>;
#[doc = "Writer for register MCM33_LOCK_REG"]
pub type W = crate::W<u32, super::MCM33_LOCK_REG>;
#[doc = "Register MCM33_LOCK_REG `reset()`'s with value 0x8000_000a"]
impl crate::ResetValue for super::MCM33_LOCK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_000a
    }
}
#[doc = "Possible values of the field `LOCK_NS_VTOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NS_VTOR_A {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl From<LOCK_NS_VTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_VTOR_A) -> Self {
        match variant {
            LOCK_NS_VTOR_A::BLOCKED => 1,
            LOCK_NS_VTOR_A::WRITABLE => 2,
        }
    }
}
#[doc = "Reader of field `LOCK_NS_VTOR`"]
pub type LOCK_NS_VTOR_R = crate::R<u8, LOCK_NS_VTOR_A>;
impl LOCK_NS_VTOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_NS_VTOR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LOCK_NS_VTOR_A::BLOCKED),
            2 => Val(LOCK_NS_VTOR_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_NS_VTOR_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_NS_VTOR_A::WRITABLE
    }
}
#[doc = "Write proxy for field `LOCK_NS_VTOR`"]
pub struct LOCK_NS_VTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_NS_VTOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_NS_VTOR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_NS_VTOR_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_NS_VTOR_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `LOCK_NS_MPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NS_MPU_A {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl From<LOCK_NS_MPU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_MPU_A) -> Self {
        match variant {
            LOCK_NS_MPU_A::BLOCKED => 1,
            LOCK_NS_MPU_A::WRITABLE => 2,
        }
    }
}
#[doc = "Reader of field `LOCK_NS_MPU`"]
pub type LOCK_NS_MPU_R = crate::R<u8, LOCK_NS_MPU_A>;
impl LOCK_NS_MPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_NS_MPU_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LOCK_NS_MPU_A::BLOCKED),
            2 => Val(LOCK_NS_MPU_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_NS_MPU_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_NS_MPU_A::WRITABLE
    }
}
#[doc = "Write proxy for field `LOCK_NS_MPU`"]
pub struct LOCK_NS_MPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_NS_MPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_NS_MPU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_NS_MPU_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_NS_MPU_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `MCM33_LOCK_REG_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_LOCK_REG_LOCK_A {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl From<MCM33_LOCK_REG_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MCM33_LOCK_REG_LOCK_A) -> Self {
        match variant {
            MCM33_LOCK_REG_LOCK_A::BLOCKED => 1,
            MCM33_LOCK_REG_LOCK_A::WRITABLE => 2,
        }
    }
}
#[doc = "Reader of field `MCM33_LOCK_REG_LOCK`"]
pub type MCM33_LOCK_REG_LOCK_R = crate::R<u8, MCM33_LOCK_REG_LOCK_A>;
impl MCM33_LOCK_REG_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCM33_LOCK_REG_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(MCM33_LOCK_REG_LOCK_A::BLOCKED),
            2 => Val(MCM33_LOCK_REG_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MCM33_LOCK_REG_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == MCM33_LOCK_REG_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `MCM33_LOCK_REG_LOCK`"]
pub struct MCM33_LOCK_REG_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCM33_LOCK_REG_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCM33_LOCK_REG_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(MCM33_LOCK_REG_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(MCM33_LOCK_REG_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - micro-CM33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTOR_R {
        LOCK_NS_VTOR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - micro-CM33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPU_R {
        LOCK_NS_MPU_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - MCM33_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn mcm33_lock_reg_lock(&self) -> MCM33_LOCK_REG_LOCK_R {
        MCM33_LOCK_REG_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - micro-CM33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&mut self) -> LOCK_NS_VTOR_W {
        LOCK_NS_VTOR_W { w: self }
    }
    #[doc = "Bits 2:3 - micro-CM33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&mut self) -> LOCK_NS_MPU_W {
        LOCK_NS_MPU_W { w: self }
    }
    #[doc = "Bits 30:31 - MCM33_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn mcm33_lock_reg_lock(&mut self) -> MCM33_LOCK_REG_LOCK_W {
        MCM33_LOCK_REG_LOCK_W { w: self }
    }
}
