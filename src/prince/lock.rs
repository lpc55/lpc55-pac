#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock Region 0 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG0_A {
    #[doc = "0: Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    DISABLED = 0,
    #[doc = "1: Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    ENABLED = 1,
}
impl From<LOCKREG0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKREG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG0` reader - Lock Region 0 registers."]
pub struct LOCKREG0_R(crate::FieldReader<bool, LOCKREG0_A>);
impl LOCKREG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKREG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKREG0_A {
        match self.bits {
            false => LOCKREG0_A::DISABLED,
            true => LOCKREG0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOCKREG0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOCKREG0_A::ENABLED
    }
}
impl core::ops::Deref for LOCKREG0_R {
    type Target = crate::FieldReader<bool, LOCKREG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKREG0` writer - Lock Region 0 registers."]
pub struct LOCKREG0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG0_A::DISABLED)
    }
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Lock Region 1 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG1_A {
    #[doc = "0: Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    DISABLED = 0,
    #[doc = "1: Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    ENABLED = 1,
}
impl From<LOCKREG1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKREG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG1` reader - Lock Region 1 registers."]
pub struct LOCKREG1_R(crate::FieldReader<bool, LOCKREG1_A>);
impl LOCKREG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKREG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKREG1_A {
        match self.bits {
            false => LOCKREG1_A::DISABLED,
            true => LOCKREG1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOCKREG1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOCKREG1_A::ENABLED
    }
}
impl core::ops::Deref for LOCKREG1_R {
    type Target = crate::FieldReader<bool, LOCKREG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKREG1` writer - Lock Region 1 registers."]
pub struct LOCKREG1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG1_A::DISABLED)
    }
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Lock Region 2 registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG2_A {
    #[doc = "0: Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    DISABLED = 0,
    #[doc = "1: Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    ENABLED = 1,
}
impl From<LOCKREG2_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKREG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKREG2` reader - Lock Region 2 registers."]
pub struct LOCKREG2_R(crate::FieldReader<bool, LOCKREG2_A>);
impl LOCKREG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKREG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKREG2_A {
        match self.bits {
            false => LOCKREG2_A::DISABLED,
            true => LOCKREG2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOCKREG2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOCKREG2_A::ENABLED
    }
}
impl core::ops::Deref for LOCKREG2_R {
    type Target = crate::FieldReader<bool, LOCKREG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKREG2` writer - Lock Region 2 registers."]
pub struct LOCKREG2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG2_A::DISABLED)
    }
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Lock the Mask registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKMASK_A {
    #[doc = "0: Disabled. MASK_LSB, and MASK_MSB are writable.."]
    DISABLED = 0,
    #[doc = "1: Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    ENABLED = 1,
}
impl From<LOCKMASK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKMASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKMASK` reader - Lock the Mask registers."]
pub struct LOCKMASK_R(crate::FieldReader<bool, LOCKMASK_A>);
impl LOCKMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKMASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKMASK_A {
        match self.bits {
            false => LOCKMASK_A::DISABLED,
            true => LOCKMASK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LOCKMASK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LOCKMASK_A::ENABLED
    }
}
impl core::ops::Deref for LOCKMASK_R {
    type Target = crate::FieldReader<bool, LOCKMASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKMASK` writer - Lock the Mask registers."]
pub struct LOCKMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKMASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKMASK_A::DISABLED)
    }
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKMASK_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline(always)]
    pub fn lockreg0(&self) -> LOCKREG0_R {
        LOCKREG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline(always)]
    pub fn lockreg1(&self) -> LOCKREG1_R {
        LOCKREG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline(always)]
    pub fn lockreg2(&self) -> LOCKREG2_R {
        LOCKREG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline(always)]
    pub fn lockmask(&self) -> LOCKMASK_R {
        LOCKMASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline(always)]
    pub fn lockreg0(&mut self) -> LOCKREG0_W {
        LOCKREG0_W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline(always)]
    pub fn lockreg1(&mut self) -> LOCKREG1_W {
        LOCKREG1_W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline(always)]
    pub fn lockreg2(&mut self) -> LOCKREG2_W {
        LOCKREG2_W { w: self }
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline(always)]
    pub fn lockmask(&mut self) -> LOCKMASK_W {
        LOCKMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
