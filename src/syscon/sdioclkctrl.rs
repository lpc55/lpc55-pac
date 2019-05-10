#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDIOCLKCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `CCLK_DRV_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_DRV_PHASER {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl CCLK_DRV_PHASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCLK_DRV_PHASER::ENUM_0_DEG => 0,
            CCLK_DRV_PHASER::ENUM_90_DEG => 1,
            CCLK_DRV_PHASER::ENUM_180_DEG => 2,
            CCLK_DRV_PHASER::ENUM_270_DEG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCLK_DRV_PHASER {
        match value {
            0 => CCLK_DRV_PHASER::ENUM_0_DEG,
            1 => CCLK_DRV_PHASER::ENUM_90_DEG,
            2 => CCLK_DRV_PHASER::ENUM_180_DEG,
            3 => CCLK_DRV_PHASER::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_DRV_PHASER::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_DRV_PHASER::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_DRV_PHASER::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_DRV_PHASER::ENUM_270_DEG
    }
}
#[doc = "Possible values of the field `CCLK_SAMPLE_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_SAMPLE_PHASER {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl CCLK_SAMPLE_PHASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCLK_SAMPLE_PHASER::ENUM_0_DEG => 0,
            CCLK_SAMPLE_PHASER::ENUM_90_DEG => 1,
            CCLK_SAMPLE_PHASER::ENUM_180_DEG => 2,
            CCLK_SAMPLE_PHASER::ENUM_270_DEG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCLK_SAMPLE_PHASER {
        match value {
            0 => CCLK_SAMPLE_PHASER::ENUM_0_DEG,
            1 => CCLK_SAMPLE_PHASER::ENUM_90_DEG,
            2 => CCLK_SAMPLE_PHASER::ENUM_180_DEG,
            3 => CCLK_SAMPLE_PHASER::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASER::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASER::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASER::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASER::ENUM_270_DEG
    }
}
#[doc = "Possible values of the field `PHASE_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_ACTIVER {
    #[doc = "Bypassed."]
    BYPASSED,
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT,
}
impl PHASE_ACTIVER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PHASE_ACTIVER::BYPASSED => false,
            PHASE_ACTIVER::PH_SHIFT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHASE_ACTIVER {
        match value {
            false => PHASE_ACTIVER::BYPASSED,
            true => PHASE_ACTIVER::PH_SHIFT,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == PHASE_ACTIVER::BYPASSED
    }
    #[doc = "Checks if the value of the field is `PH_SHIFT`"]
    #[inline]
    pub fn is_ph_shift(&self) -> bool {
        *self == PHASE_ACTIVER::PH_SHIFT
    }
}
#[doc = r" Value of the field"]
pub struct CCLK_DRV_DELAYR {
    bits: u8,
}
impl CCLK_DRV_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CCLK_DRV_DELAY_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_DRV_DELAY_ACTIVER {
    #[doc = "Disable drive delay."]
    DISABLE,
    #[doc = "Enable drive delay."]
    ENABLE,
}
impl CCLK_DRV_DELAY_ACTIVER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCLK_DRV_DELAY_ACTIVER::DISABLE => false,
            CCLK_DRV_DELAY_ACTIVER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCLK_DRV_DELAY_ACTIVER {
        match value {
            false => CCLK_DRV_DELAY_ACTIVER::DISABLE,
            true => CCLK_DRV_DELAY_ACTIVER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVER::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct CCLK_SAMPLE_DELAYR {
    bits: u8,
}
impl CCLK_SAMPLE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CCLK_SAMPLE_DELAY_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_SAMPLE_DELAY_ACTIVER {
    #[doc = "Disables sample delay."]
    DISABLE,
    #[doc = "Enables sample delay."]
    ENABLE,
}
impl CCLK_SAMPLE_DELAY_ACTIVER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCLK_SAMPLE_DELAY_ACTIVER::DISABLE => false,
            CCLK_SAMPLE_DELAY_ACTIVER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCLK_SAMPLE_DELAY_ACTIVER {
        match value {
            false => CCLK_SAMPLE_DELAY_ACTIVER::DISABLE,
            true => CCLK_SAMPLE_DELAY_ACTIVER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVER::ENABLE
    }
}
#[doc = "Values that can be written to the field `CCLK_DRV_PHASE`"]
pub enum CCLK_DRV_PHASEW {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl CCLK_DRV_PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCLK_DRV_PHASEW::ENUM_0_DEG => 0,
            CCLK_DRV_PHASEW::ENUM_90_DEG => 1,
            CCLK_DRV_PHASEW::ENUM_180_DEG => 2,
            CCLK_DRV_PHASEW::ENUM_270_DEG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_DRV_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_DRV_PHASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCLK_DRV_PHASEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 degree shift."]
    #[inline]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASEW::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASEW::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASEW::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASEW::ENUM_270_DEG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCLK_SAMPLE_PHASE`"]
pub enum CCLK_SAMPLE_PHASEW {
    #[doc = "0 degree shift."]
    ENUM_0_DEG,
    #[doc = "90 degree shift."]
    ENUM_90_DEG,
    #[doc = "180 degree shift."]
    ENUM_180_DEG,
    #[doc = "270 degree shift."]
    ENUM_270_DEG,
}
impl CCLK_SAMPLE_PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCLK_SAMPLE_PHASEW::ENUM_0_DEG => 0,
            CCLK_SAMPLE_PHASEW::ENUM_90_DEG => 1,
            CCLK_SAMPLE_PHASEW::ENUM_180_DEG => 2,
            CCLK_SAMPLE_PHASEW::ENUM_270_DEG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_SAMPLE_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_SAMPLE_PHASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCLK_SAMPLE_PHASEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 degree shift."]
    #[inline]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASEW::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASEW::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASEW::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASEW::ENUM_270_DEG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PHASE_ACTIVE`"]
pub enum PHASE_ACTIVEW {
    #[doc = "Bypassed."]
    BYPASSED,
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT,
}
impl PHASE_ACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHASE_ACTIVEW::BYPASSED => false,
            PHASE_ACTIVEW::PH_SHIFT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHASE_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE_ACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHASE_ACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bypassed."]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(PHASE_ACTIVEW::BYPASSED)
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline]
    pub fn ph_shift(self) -> &'a mut W {
        self.variant(PHASE_ACTIVEW::PH_SHIFT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_DRV_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_DRV_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCLK_DRV_DELAY_ACTIVE`"]
pub enum CCLK_DRV_DELAY_ACTIVEW {
    #[doc = "Disable drive delay."]
    DISABLE,
    #[doc = "Enable drive delay."]
    ENABLE,
}
impl CCLK_DRV_DELAY_ACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCLK_DRV_DELAY_ACTIVEW::DISABLE => false,
            CCLK_DRV_DELAY_ACTIVEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_DRV_DELAY_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_DRV_DELAY_ACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCLK_DRV_DELAY_ACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable drive delay."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVEW::DISABLE)
    }
    #[doc = "Enable drive delay."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVEW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_SAMPLE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_SAMPLE_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCLK_SAMPLE_DELAY_ACTIVE`"]
pub enum CCLK_SAMPLE_DELAY_ACTIVEW {
    #[doc = "Disables sample delay."]
    DISABLE,
    #[doc = "Enables sample delay."]
    ENABLE,
}
impl CCLK_SAMPLE_DELAY_ACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCLK_SAMPLE_DELAY_ACTIVEW::DISABLE => false,
            CCLK_SAMPLE_DELAY_ACTIVEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCLK_SAMPLE_DELAY_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLK_SAMPLE_DELAY_ACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCLK_SAMPLE_DELAY_ACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables sample delay."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVEW::DISABLE)
    }
    #[doc = "Enables sample delay."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVEW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASER {
        CCLK_DRV_PHASER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASER {
        CCLK_SAMPLE_PHASER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline]
    pub fn phase_active(&self) -> PHASE_ACTIVER {
        PHASE_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCLK_DRV_DELAYR { bits }
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVER {
        CCLK_DRV_DELAY_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCLK_SAMPLE_DELAYR { bits }
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVER {
        CCLK_SAMPLE_DELAY_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline]
    pub fn cclk_drv_phase(&mut self) -> _CCLK_DRV_PHASEW {
        _CCLK_DRV_PHASEW { w: self }
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_sample_phase(&mut self) -> _CCLK_SAMPLE_PHASEW {
        _CCLK_SAMPLE_PHASEW { w: self }
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline]
    pub fn phase_active(&mut self) -> _PHASE_ACTIVEW {
        _PHASE_ACTIVEW { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_drv_delay(&mut self) -> _CCLK_DRV_DELAYW {
        _CCLK_DRV_DELAYW { w: self }
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline]
    pub fn cclk_drv_delay_active(&mut self) -> _CCLK_DRV_DELAY_ACTIVEW {
        _CCLK_DRV_DELAY_ACTIVEW { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline]
    pub fn cclk_sample_delay(&mut self) -> _CCLK_SAMPLE_DELAYW {
        _CCLK_SAMPLE_DELAYW { w: self }
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline]
    pub fn cclk_sample_delay_active(&mut self) -> _CCLK_SAMPLE_DELAY_ACTIVEW {
        _CCLK_SAMPLE_DELAY_ACTIVEW { w: self }
    }
}
