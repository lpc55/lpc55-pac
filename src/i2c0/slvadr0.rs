#[doc = "Register `SLVADR0` reader"]
pub struct R(crate::R<SLVADR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVADR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVADR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVADR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVADR0` writer"]
pub struct W(crate::W<SLVADR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVADR0_SPEC>;
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
impl From<crate::W<SLVADR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVADR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADISABLE_A {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    ENABLED = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    DISABLED = 1,
}
impl From<SADISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SADISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SADISABLE` reader - Slave Address n Disable."]
pub struct SADISABLE_R(crate::FieldReader<bool, SADISABLE_A>);
impl SADISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SADISABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADISABLE_A {
        match self.bits {
            false => SADISABLE_A::ENABLED,
            true => SADISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SADISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SADISABLE_A::DISABLED
    }
}
impl core::ops::Deref for SADISABLE_R {
    type Target = crate::FieldReader<bool, SADISABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADISABLE` writer - Slave Address n Disable."]
pub struct SADISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SADISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SADISABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::ENABLED)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::DISABLED)
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
#[doc = "Field `SLVADR` reader - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub struct SLVADR_R(crate::FieldReader<u8, u8>);
impl SLVADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLVADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVADR` writer - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub struct SLVADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTONACK_A {
    #[doc = "0: Normal operation, matching I2C addresses are not ignored."]
    NORMAL = 0,
    #[doc = "1: Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC = 1,
}
impl From<AUTONACK_A> for bool {
    #[inline(always)]
    fn from(variant: AUTONACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTONACK` reader - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
pub struct AUTONACK_R(crate::FieldReader<bool, AUTONACK_A>);
impl AUTONACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTONACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTONACK_A {
        match self.bits {
            false => AUTONACK_A::NORMAL,
            true => AUTONACK_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == AUTONACK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == AUTONACK_A::AUTOMATIC
    }
}
impl core::ops::Deref for AUTONACK_R {
    type Target = crate::FieldReader<bool, AUTONACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTONACK` writer - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
pub struct AUTONACK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTONACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTONACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AUTONACK_A::NORMAL)
    }
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTONACK_A::AUTOMATIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SADISABLE_R {
        SADISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SLVADR_R {
        SLVADR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn autonack(&self) -> AUTONACK_R {
        AUTONACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&mut self) -> SADISABLE_W {
        SADISABLE_W { w: self }
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&mut self) -> SLVADR_W {
        SLVADR_W { w: self }
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn autonack(&mut self) -> AUTONACK_W {
        AUTONACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvadr0](index.html) module"]
pub struct SLVADR0_SPEC;
impl crate::RegisterSpec for SLVADR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvadr0::R](R) reader structure"]
impl crate::Readable for SLVADR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvadr0::W](W) writer structure"]
impl crate::Writable for SLVADR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLVADR0 to value 0x01"]
impl crate::Resettable for SLVADR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
