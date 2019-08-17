#[doc = "Reader of register SDIOCLKCTRL"]
pub type R = crate::R<u32, super::SDIOCLKCTRL>;
#[doc = "Writer for register SDIOCLKCTRL"]
pub type W = crate::W<u32, super::SDIOCLKCTRL>;
#[doc = "Register SDIOCLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIOCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CCLK_DRV_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_DRV_PHASE_A {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl From<CCLK_DRV_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_DRV_PHASE_A) -> Self {
        match variant {
            CCLK_DRV_PHASE_A::ENUM_0_DEG => 0,
            CCLK_DRV_PHASE_A::ENUM_90_DEG => 1,
            CCLK_DRV_PHASE_A::ENUM_180_DEG => 2,
            CCLK_DRV_PHASE_A::ENUM_270_DEG => 3,
        }
    }
}
#[doc = "Reader of field `CCLK_DRV_PHASE`"]
pub type CCLK_DRV_PHASE_R = crate::R<u8, CCLK_DRV_PHASE_A>;
impl CCLK_DRV_PHASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_DRV_PHASE_A {
        match self.bits {
            0 => CCLK_DRV_PHASE_A::ENUM_0_DEG,
            1 => CCLK_DRV_PHASE_A::ENUM_90_DEG,
            2 => CCLK_DRV_PHASE_A::ENUM_180_DEG,
            3 => CCLK_DRV_PHASE_A::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_270_DEG
    }
}
#[doc = "Write proxy for field `CCLK_DRV_PHASE`"]
pub struct CCLK_DRV_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_DRV_PHASE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_270_DEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CCLK_SAMPLE_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_SAMPLE_PHASE_A {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl From<CCLK_SAMPLE_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_PHASE_A) -> Self {
        match variant {
            CCLK_SAMPLE_PHASE_A::ENUM_0_DEG => 0,
            CCLK_SAMPLE_PHASE_A::ENUM_90_DEG => 1,
            CCLK_SAMPLE_PHASE_A::ENUM_180_DEG => 2,
            CCLK_SAMPLE_PHASE_A::ENUM_270_DEG => 3,
        }
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_PHASE`"]
pub type CCLK_SAMPLE_PHASE_R = crate::R<u8, CCLK_SAMPLE_PHASE_A>;
impl CCLK_SAMPLE_PHASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_SAMPLE_PHASE_A {
        match self.bits {
            0 => CCLK_SAMPLE_PHASE_A::ENUM_0_DEG,
            1 => CCLK_SAMPLE_PHASE_A::ENUM_90_DEG,
            2 => CCLK_SAMPLE_PHASE_A::ENUM_180_DEG,
            3 => CCLK_SAMPLE_PHASE_A::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_270_DEG
    }
}
#[doc = "Write proxy for field `CCLK_SAMPLE_PHASE`"]
pub struct CCLK_SAMPLE_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_SAMPLE_PHASE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_270_DEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PHASE_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_ACTIVE_A {
    #[doc = "Bypassed."]
    BYPASSED,
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT,
}
impl From<PHASE_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_ACTIVE_A) -> Self {
        match variant {
            PHASE_ACTIVE_A::BYPASSED => false,
            PHASE_ACTIVE_A::PH_SHIFT => true,
        }
    }
}
#[doc = "Reader of field `PHASE_ACTIVE`"]
pub type PHASE_ACTIVE_R = crate::R<bool, PHASE_ACTIVE_A>;
impl PHASE_ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_ACTIVE_A {
        match self.bits {
            false => PHASE_ACTIVE_A::BYPASSED,
            true => PHASE_ACTIVE_A::PH_SHIFT,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == PHASE_ACTIVE_A::BYPASSED
    }
    #[doc = "Checks if the value of the field is `PH_SHIFT`"]
    #[inline(always)]
    pub fn is_ph_shift(&self) -> bool {
        *self == PHASE_ACTIVE_A::PH_SHIFT
    }
}
#[doc = "Write proxy for field `PHASE_ACTIVE`"]
pub struct PHASE_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_ACTIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bypassed."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_A::BYPASSED)
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline(always)]
    pub fn ph_shift(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_A::PH_SHIFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CCLK_DRV_DELAY`"]
pub type CCLK_DRV_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_DRV_DELAY`"]
pub struct CCLK_DRV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CCLK_DRV_DELAY_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_DRV_DELAY_ACTIVE_A {
    #[doc = "Disable drive delay."]
    DISABLE,
    #[doc = "Enable drive delay."]
    ENABLE,
}
impl From<CCLK_DRV_DELAY_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_DRV_DELAY_ACTIVE_A) -> Self {
        match variant {
            CCLK_DRV_DELAY_ACTIVE_A::DISABLE => false,
            CCLK_DRV_DELAY_ACTIVE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CCLK_DRV_DELAY_ACTIVE`"]
pub type CCLK_DRV_DELAY_ACTIVE_R = crate::R<bool, CCLK_DRV_DELAY_ACTIVE_A>;
impl CCLK_DRV_DELAY_ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_DRV_DELAY_ACTIVE_A {
        match self.bits {
            false => CCLK_DRV_DELAY_ACTIVE_A::DISABLE,
            true => CCLK_DRV_DELAY_ACTIVE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVE_A::ENABLE
    }
}
#[doc = "Write proxy for field `CCLK_DRV_DELAY_ACTIVE`"]
pub struct CCLK_DRV_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_DRV_DELAY_ACTIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable drive delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_A::DISABLE)
    }
    #[doc = "Enable drive delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_DELAY`"]
pub type CCLK_SAMPLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_SAMPLE_DELAY`"]
pub struct CCLK_SAMPLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `CCLK_SAMPLE_DELAY_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_SAMPLE_DELAY_ACTIVE_A {
    #[doc = "Disables sample delay."]
    DISABLE,
    #[doc = "Enables sample delay."]
    ENABLE,
}
impl From<CCLK_SAMPLE_DELAY_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_DELAY_ACTIVE_A) -> Self {
        match variant {
            CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE => false,
            CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_DELAY_ACTIVE`"]
pub type CCLK_SAMPLE_DELAY_ACTIVE_R = crate::R<bool, CCLK_SAMPLE_DELAY_ACTIVE_A>;
impl CCLK_SAMPLE_DELAY_ACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_A {
        match self.bits {
            false => CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE,
            true => CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE
    }
}
#[doc = "Write proxy for field `CCLK_SAMPLE_DELAY_ACTIVE`"]
pub struct CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_SAMPLE_DELAY_ACTIVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables sample delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE)
    }
    #[doc = "Enables sample delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASE_R {
        CCLK_DRV_PHASE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASE_R {
        CCLK_SAMPLE_PHASE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&self) -> PHASE_ACTIVE_R {
        PHASE_ACTIVE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAY_R {
        CCLK_DRV_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVE_R {
        CCLK_DRV_DELAY_ACTIVE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAY_R {
        CCLK_SAMPLE_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_R {
        CCLK_SAMPLE_DELAY_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W {
        CCLK_DRV_PHASE_W { w: self }
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W {
        CCLK_SAMPLE_PHASE_W { w: self }
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W {
        PHASE_ACTIVE_W { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W {
        CCLK_DRV_DELAY_W { w: self }
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W {
        CCLK_DRV_DELAY_ACTIVE_W { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W {
        CCLK_SAMPLE_DELAY_W { w: self }
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W {
        CCLK_SAMPLE_DELAY_ACTIVE_W { w: self }
    }
}
