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
#[doc = "Reader of field `LOCKREG0`"]
pub type LOCKREG0_R = crate::R<bool, LOCKREG0_A>;
impl LOCKREG0_R {
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
        *self == LOCKREG0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG0_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOCKREG0`"]
pub struct LOCKREG0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `LOCKREG1`"]
pub type LOCKREG1_R = crate::R<bool, LOCKREG1_A>;
impl LOCKREG1_R {
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
        *self == LOCKREG1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG1_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOCKREG1`"]
pub struct LOCKREG1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Reader of field `LOCKREG2`"]
pub type LOCKREG2_R = crate::R<bool, LOCKREG2_A>;
impl LOCKREG2_R {
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
        *self == LOCKREG2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG2_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOCKREG2`"]
pub struct LOCKREG2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKREG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKREG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Reader of field `LOCKMASK`"]
pub type LOCKMASK_R = crate::R<bool, LOCKMASK_A>;
impl LOCKMASK_R {
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
        *self == LOCKMASK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKMASK_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOCKMASK`"]
pub struct LOCKMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKMASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
}
