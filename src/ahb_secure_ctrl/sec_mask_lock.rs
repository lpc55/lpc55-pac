#[doc = "Register `SEC_MASK_LOCK` reader"]
pub struct R(crate::R<SEC_MASK_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_MASK_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_MASK_LOCK_SPEC>> for R {
    fn from(reader: crate::R<SEC_MASK_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_MASK_LOCK` writer"]
pub struct W(crate::W<SEC_MASK_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_MASK_LOCK_SPEC>;
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
impl core::convert::From<crate::W<SEC_MASK_LOCK_SPEC>> for W {
    fn from(writer: crate::W<SEC_MASK_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SEC_GPIO_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK0_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_GPIO_MASK0_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK0_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` reader - SEC_GPIO_MASK0 register write-lock."]
pub struct SEC_GPIO_MASK0_LOCK_R(crate::FieldReader<u8, SEC_GPIO_MASK0_LOCK_A>);
impl SEC_GPIO_MASK0_LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_GPIO_MASK0_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK0_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK0_LOCK_A::BLOCKED),
            2 => Some(SEC_GPIO_MASK0_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == SEC_GPIO_MASK0_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == SEC_GPIO_MASK0_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for SEC_GPIO_MASK0_LOCK_R {
    type Target = crate::FieldReader<u8, SEC_GPIO_MASK0_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` writer - SEC_GPIO_MASK0 register write-lock."]
pub struct SEC_GPIO_MASK0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_MASK0_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_MASK0_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "SEC_GPIO_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK1_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_GPIO_MASK1_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK1_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` reader - SEC_GPIO_MASK1 register write-lock."]
pub struct SEC_GPIO_MASK1_LOCK_R(crate::FieldReader<u8, SEC_GPIO_MASK1_LOCK_A>);
impl SEC_GPIO_MASK1_LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_GPIO_MASK1_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_GPIO_MASK1_LOCK_A> {
        match self.bits {
            1 => Some(SEC_GPIO_MASK1_LOCK_A::BLOCKED),
            2 => Some(SEC_GPIO_MASK1_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == SEC_GPIO_MASK1_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == SEC_GPIO_MASK1_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for SEC_GPIO_MASK1_LOCK_R {
    type Target = crate::FieldReader<u8, SEC_GPIO_MASK1_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` writer - SEC_GPIO_MASK1 register write-lock."]
pub struct SEC_GPIO_MASK1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_MASK1_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_MASK1_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "SEC_CPU_INT_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_CPU1_INT_MASK0_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_CPU1_INT_MASK0_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_CPU1_INT_MASK0_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK0_LOCK` reader - SEC_CPU_INT_MASK0 register write-lock."]
pub struct SEC_CPU1_INT_MASK0_LOCK_R(crate::FieldReader<u8, SEC_CPU1_INT_MASK0_LOCK_A>);
impl SEC_CPU1_INT_MASK0_LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_CPU1_INT_MASK0_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_CPU1_INT_MASK0_LOCK_A> {
        match self.bits {
            1 => Some(SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED),
            2 => Some(SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for SEC_CPU1_INT_MASK0_LOCK_R {
    type Target = crate::FieldReader<u8, SEC_CPU1_INT_MASK0_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK0_LOCK` writer - SEC_CPU_INT_MASK0 register write-lock."]
pub struct SEC_CPU1_INT_MASK0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CPU1_INT_MASK0_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK0_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "SEC_CPU_INT_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_CPU1_INT_MASK1_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_CPU1_INT_MASK1_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_CPU1_INT_MASK1_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK1_LOCK` reader - SEC_CPU_INT_MASK1 register write-lock."]
pub struct SEC_CPU1_INT_MASK1_LOCK_R(crate::FieldReader<u8, SEC_CPU1_INT_MASK1_LOCK_A>);
impl SEC_CPU1_INT_MASK1_LOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_CPU1_INT_MASK1_LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_CPU1_INT_MASK1_LOCK_A> {
        match self.bits {
            1 => Some(SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED),
            2 => Some(SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        **self == SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE
    }
}
impl core::ops::Deref for SEC_CPU1_INT_MASK1_LOCK_R {
    type Target = crate::FieldReader<u8, SEC_CPU1_INT_MASK1_LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK1_LOCK` writer - SEC_CPU_INT_MASK1 register write-lock."]
pub struct SEC_CPU1_INT_MASK1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CPU1_INT_MASK1_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK1_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&self) -> SEC_GPIO_MASK0_LOCK_R {
        SEC_GPIO_MASK0_LOCK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&self) -> SEC_GPIO_MASK1_LOCK_R {
        SEC_GPIO_MASK1_LOCK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&self) -> SEC_CPU1_INT_MASK0_LOCK_R {
        SEC_CPU1_INT_MASK0_LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&self) -> SEC_CPU1_INT_MASK1_LOCK_R {
        SEC_CPU1_INT_MASK1_LOCK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&mut self) -> SEC_GPIO_MASK0_LOCK_W {
        SEC_GPIO_MASK0_LOCK_W { w: self }
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&mut self) -> SEC_GPIO_MASK1_LOCK_W {
        SEC_GPIO_MASK1_LOCK_W { w: self }
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&mut self) -> SEC_CPU1_INT_MASK0_LOCK_W {
        SEC_CPU1_INT_MASK0_LOCK_W { w: self }
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&mut self) -> SEC_CPU1_INT_MASK1_LOCK_W {
        SEC_CPU1_INT_MASK1_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security General Purpose register access control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_mask_lock](index.html) module"]
pub struct SEC_MASK_LOCK_SPEC;
impl crate::RegisterSpec for SEC_MASK_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_mask_lock::R](R) reader structure"]
impl crate::Readable for SEC_MASK_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_mask_lock::W](W) writer structure"]
impl crate::Writable for SEC_MASK_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_MASK_LOCK to value 0x0aaa"]
impl crate::Resettable for SEC_MASK_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0aaa
    }
}
