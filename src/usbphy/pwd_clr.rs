#[doc = "Reader of register PWD_CLR"]
pub type R = crate::R<u32, super::PWD_CLR>;
#[doc = "Writer for register PWD_CLR"]
pub type W = crate::W<u32, super::PWD_CLR>;
#[doc = "Register PWD_CLR `reset()`'s with value 0x001e_1c00"]
impl crate::ResetValue for super::PWD_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001e_1c00
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDFS_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 1,
}
impl From<TXPWDFS_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPWDFS`"]
pub type TXPWDFS_R = crate::R<bool, TXPWDFS_A>;
impl TXPWDFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDFS_A {
        match self.bits {
            false => TXPWDFS_A::VALUE0,
            true => TXPWDFS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDFS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDFS_A::VALUE1
    }
}
#[doc = "Write proxy for field `TXPWDFS`"]
pub struct TXPWDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWDFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDFS_A::VALUE0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDFS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDIBIAS_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 1,
}
impl From<TXPWDIBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDIBIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPWDIBIAS`"]
pub type TXPWDIBIAS_R = crate::R<bool, TXPWDIBIAS_A>;
impl TXPWDIBIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDIBIAS_A {
        match self.bits {
            false => TXPWDIBIAS_A::VALUE0,
            true => TXPWDIBIAS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDIBIAS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDIBIAS_A::VALUE1
    }
}
#[doc = "Write proxy for field `TXPWDIBIAS`"]
pub struct TXPWDIBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDIBIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWDIBIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::VALUE0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDV2I_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 1,
}
impl From<TXPWDV2I_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDV2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPWDV2I`"]
pub type TXPWDV2I_R = crate::R<bool, TXPWDV2I_A>;
impl TXPWDV2I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDV2I_A {
        match self.bits {
            false => TXPWDV2I_A::VALUE0,
            true => TXPWDV2I_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDV2I_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDV2I_A::VALUE1
    }
}
#[doc = "Write proxy for field `TXPWDV2I`"]
pub struct TXPWDV2I_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDV2I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPWDV2I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::VALUE0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDENV_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 1,
}
impl From<RXPWDENV_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDENV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPWDENV`"]
pub type RXPWDENV_R = crate::R<bool, RXPWDENV_A>;
impl RXPWDENV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDENV_A {
        match self.bits {
            false => RXPWDENV_A::VALUE0,
            true => RXPWDENV_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDENV_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDENV_A::VALUE1
    }
}
#[doc = "Write proxy for field `RXPWDENV`"]
pub struct RXPWDENV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDENV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWDENV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDENV_A::VALUE0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDENV_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWD1PT1_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB full-speed differential receiver."]
    VALUE1 = 1,
}
impl From<RXPWD1PT1_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWD1PT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPWD1PT1`"]
pub type RXPWD1PT1_R = crate::R<bool, RXPWD1PT1_A>;
impl RXPWD1PT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWD1PT1_A {
        match self.bits {
            false => RXPWD1PT1_A::VALUE0,
            true => RXPWD1PT1_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWD1PT1_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWD1PT1_A::VALUE1
    }
}
#[doc = "Write proxy for field `RXPWD1PT1`"]
pub struct RXPWD1PT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWD1PT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWD1PT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::VALUE0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDDIFF_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB high-speed differential receive"]
    VALUE1 = 1,
}
impl From<RXPWDDIFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDDIFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPWDDIFF`"]
pub type RXPWDDIFF_R = crate::R<bool, RXPWDDIFF_A>;
impl RXPWDDIFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDDIFF_A {
        match self.bits {
            false => RXPWDDIFF_A::VALUE0,
            true => RXPWDDIFF_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDDIFF_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDDIFF_A::VALUE1
    }
}
#[doc = "Write proxy for field `RXPWDDIFF`"]
pub struct RXPWDDIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDDIFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWDDIFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::VALUE0)
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDRX_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 1,
}
impl From<RXPWDRX_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPWDRX`"]
pub type RXPWDRX_R = crate::R<bool, RXPWDRX_A>;
impl RXPWDRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDRX_A {
        match self.bits {
            false => RXPWDRX_A::VALUE0,
            true => RXPWDRX_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDRX_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDRX_A::VALUE1
    }
}
#[doc = "Write proxy for field `RXPWDRX`"]
pub struct RXPWDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPWDRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDRX_A::VALUE0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDRX_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TXPWDFS_R {
        TXPWDFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TXPWDIBIAS_R {
        TXPWDIBIAS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> TXPWDV2I_R {
        TXPWDV2I_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RXPWDENV_R {
        RXPWDENV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1_R {
        RXPWD1PT1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RXPWDDIFF_R {
        RXPWDDIFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RXPWDRX_R {
        RXPWDRX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&mut self) -> TXPWDFS_W {
        TXPWDFS_W { w: self }
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&mut self) -> TXPWDIBIAS_W {
        TXPWDIBIAS_W { w: self }
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&mut self) -> TXPWDV2I_W {
        TXPWDV2I_W { w: self }
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&mut self) -> RXPWDENV_W {
        RXPWDENV_W { w: self }
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&mut self) -> RXPWD1PT1_W {
        RXPWD1PT1_W { w: self }
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&mut self) -> RXPWDDIFF_W {
        RXPWDDIFF_W { w: self }
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&mut self) -> RXPWDRX_W {
        RXPWDRX_W { w: self }
    }
}
