#[doc = "Register `CPU1_LOCK_REG` reader"]
pub struct R(crate::R<CPU1_LOCK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU1_LOCK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPU1_LOCK_REG_SPEC>> for R {
    fn from(reader: crate::R<CPU1_LOCK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU1_LOCK_REG` writer"]
pub struct W(crate::W<CPU1_LOCK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU1_LOCK_REG_SPEC>;
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
impl core::convert::From<crate::W<CPU1_LOCK_REG_SPEC>> for W {
    fn from(writer: crate::W<CPU1_LOCK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock.\n\nValue on reset: 2"]
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
#[doc = "Field `LOCK_NS_VTOR` reader - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
pub struct LOCK_NS_VTOR_R(crate::FieldReader<u8, LOCK_NS_VTOR_A>);
impl LOCK_NS_VTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_NS_VTOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_NS_VTOR_A> {
        match self.bits {
            1 => Some(LOCK_NS_VTOR_A::BLOCKED),
            2 => Some(LOCK_NS_VTOR_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == LOCK_NS_VTOR_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == LOCK_NS_VTOR_A::WRITABLE
    }
}
impl core::ops::Deref for LOCK_NS_VTOR_R {
    type Target = crate::FieldReader<u8, LOCK_NS_VTOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_NS_VTOR` writer - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock.\n\nValue on reset: 2"]
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
#[doc = "Field `LOCK_NS_MPU` reader - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
pub struct LOCK_NS_MPU_R(crate::FieldReader<u8, LOCK_NS_MPU_A>);
impl LOCK_NS_MPU_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_NS_MPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_NS_MPU_A> {
        match self.bits {
            1 => Some(LOCK_NS_MPU_A::BLOCKED),
            2 => Some(LOCK_NS_MPU_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == LOCK_NS_MPU_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == LOCK_NS_MPU_A::WRITABLE
    }
}
impl core::ops::Deref for LOCK_NS_MPU_R {
    type Target = crate::FieldReader<u8, LOCK_NS_MPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_NS_MPU` writer - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "CPU1_LOCK_REG write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPU1_LOCK_REG_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<CPU1_LOCK_REG_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1_LOCK_REG_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPU1_LOCK_REG_LOCK` reader - CPU1_LOCK_REG write-lock."]
pub struct CPU1_LOCK_REG_LOCK_R(crate::FieldReader<u8, CPU1_LOCK_REG_LOCK_A>);
impl CPU1_LOCK_REG_LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU1_LOCK_REG_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU1_LOCK_REG_LOCK_A> {
        match self.bits {
            1 => Some(CPU1_LOCK_REG_LOCK_A::BLOCKED),
            2 => Some(CPU1_LOCK_REG_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == CPU1_LOCK_REG_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == CPU1_LOCK_REG_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for CPU1_LOCK_REG_LOCK_R {
    type Target = crate::FieldReader<u8, CPU1_LOCK_REG_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1_LOCK_REG_LOCK` writer - CPU1_LOCK_REG write-lock."]
pub struct CPU1_LOCK_REG_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_LOCK_REG_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_LOCK_REG_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(CPU1_LOCK_REG_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(CPU1_LOCK_REG_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTOR_R {
        LOCK_NS_VTOR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPU_R {
        LOCK_NS_MPU_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu1_lock_reg_lock(&self) -> CPU1_LOCK_REG_LOCK_R {
        CPU1_LOCK_REG_LOCK_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&mut self) -> LOCK_NS_VTOR_W {
        LOCK_NS_VTOR_W { w: self }
    }
    #[doc = "Bits 2:3 - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&mut self) -> LOCK_NS_MPU_W {
        LOCK_NS_MPU_W { w: self }
    }
    #[doc = "Bits 30:31 - CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu1_lock_reg_lock(&mut self) -> CPU1_LOCK_REG_LOCK_W {
        CPU1_LOCK_REG_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu1_lock_reg](index.html) module"]
pub struct CPU1_LOCK_REG_SPEC;
impl crate::RegisterSpec for CPU1_LOCK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu1_lock_reg::R](R) reader structure"]
impl crate::Readable for CPU1_LOCK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu1_lock_reg::W](W) writer structure"]
impl crate::Writable for CPU1_LOCK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU1_LOCK_REG to value 0x8000_000a"]
impl crate::Resettable for CPU1_LOCK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_000a
    }
}
