#[doc = "Reader of register CPU0_LOCK_REG"]
pub type R = crate::R<u32, super::CPU0_LOCK_REG>;
#[doc = "Writer for register CPU0_LOCK_REG"]
pub type W = crate::W<u32, super::CPU0_LOCK_REG>;
#[doc = "Register CPU0_LOCK_REG `reset()`'s with value 0x8000_02aa"]
impl crate::ResetValue for super::CPU0_LOCK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_02aa
    }
}
#[doc = "Cortex M33 (CPU0) VTOR_NS register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_NS_VTOR_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<LOCK_NS_VTOR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_VTOR_A) -> Self {
        variant as _
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
#[doc = "Cortex M33 (CPU0) non-secure MPU register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_NS_MPU_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<LOCK_NS_MPU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_NS_MPU_A) -> Self {
        variant as _
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
#[doc = "Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_S_VTAIRCR_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<LOCK_S_VTAIRCR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_S_VTAIRCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_S_VTAIRCR`"]
pub type LOCK_S_VTAIRCR_R = crate::R<u8, LOCK_S_VTAIRCR_A>;
impl LOCK_S_VTAIRCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_S_VTAIRCR_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LOCK_S_VTAIRCR_A::BLOCKED),
            2 => Val(LOCK_S_VTAIRCR_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_S_VTAIRCR_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_S_VTAIRCR_A::WRITABLE
    }
}
#[doc = "Write proxy for field `LOCK_S_VTAIRCR`"]
pub struct LOCK_S_VTAIRCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_S_VTAIRCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_S_VTAIRCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_S_VTAIRCR_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_S_VTAIRCR_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Cortex M33 (CPU0) Secure MPU registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_S_MPU_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<LOCK_S_MPU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_S_MPU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_S_MPU`"]
pub type LOCK_S_MPU_R = crate::R<u8, LOCK_S_MPU_A>;
impl LOCK_S_MPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_S_MPU_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LOCK_S_MPU_A::BLOCKED),
            2 => Val(LOCK_S_MPU_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_S_MPU_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_S_MPU_A::WRITABLE
    }
}
#[doc = "Write proxy for field `LOCK_S_MPU`"]
pub struct LOCK_S_MPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_S_MPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_S_MPU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_S_MPU_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_S_MPU_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Cortex M33 (CPU0) SAU registers write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_SAU_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<LOCK_SAU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_SAU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_SAU`"]
pub type LOCK_SAU_R = crate::R<u8, LOCK_SAU_A>;
impl LOCK_SAU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_SAU_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(LOCK_SAU_A::BLOCKED),
            2 => Val(LOCK_SAU_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_SAU_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_SAU_A::WRITABLE
    }
}
#[doc = "Write proxy for field `LOCK_SAU`"]
pub struct LOCK_SAU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_SAU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_SAU_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_SAU_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_SAU_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CPU0_LOCK_REG write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU0_LOCK_REG_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<CPU0_LOCK_REG_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_LOCK_REG_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPU0_LOCK_REG_LOCK`"]
pub type CPU0_LOCK_REG_LOCK_R = crate::R<u8, CPU0_LOCK_REG_LOCK_A>;
impl CPU0_LOCK_REG_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_LOCK_REG_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CPU0_LOCK_REG_LOCK_A::BLOCKED),
            2 => Val(CPU0_LOCK_REG_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == CPU0_LOCK_REG_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == CPU0_LOCK_REG_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `CPU0_LOCK_REG_LOCK`"]
pub struct CPU0_LOCK_REG_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0_LOCK_REG_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0_LOCK_REG_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(CPU0_LOCK_REG_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(CPU0_LOCK_REG_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTOR_R {
        LOCK_NS_VTOR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPU_R {
        LOCK_NS_MPU_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub fn lock_s_vtaircr(&self) -> LOCK_S_VTAIRCR_R {
        LOCK_S_VTAIRCR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub fn lock_s_mpu(&self) -> LOCK_S_MPU_R {
        LOCK_S_MPU_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub fn lock_sau(&self) -> LOCK_SAU_R {
        LOCK_SAU_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu0_lock_reg_lock(&self) -> CPU0_LOCK_REG_LOCK_R {
        CPU0_LOCK_REG_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Cortex M33 (CPU0) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&mut self) -> LOCK_NS_VTOR_W {
        LOCK_NS_VTOR_W { w: self }
    }
    #[doc = "Bits 2:3 - Cortex M33 (CPU0) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&mut self) -> LOCK_NS_MPU_W {
        LOCK_NS_MPU_W { w: self }
    }
    #[doc = "Bits 4:5 - Cortex M33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline(always)]
    pub fn lock_s_vtaircr(&mut self) -> LOCK_S_VTAIRCR_W {
        LOCK_S_VTAIRCR_W { w: self }
    }
    #[doc = "Bits 6:7 - Cortex M33 (CPU0) Secure MPU registers write-lock."]
    #[inline(always)]
    pub fn lock_s_mpu(&mut self) -> LOCK_S_MPU_W {
        LOCK_S_MPU_W { w: self }
    }
    #[doc = "Bits 8:9 - Cortex M33 (CPU0) SAU registers write-lock."]
    #[inline(always)]
    pub fn lock_sau(&mut self) -> LOCK_SAU_W {
        LOCK_SAU_W { w: self }
    }
    #[doc = "Bits 30:31 - CPU0_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu0_lock_reg_lock(&mut self) -> CPU0_LOCK_REG_LOCK_W {
        CPU0_LOCK_REG_LOCK_W { w: self }
    }
}
