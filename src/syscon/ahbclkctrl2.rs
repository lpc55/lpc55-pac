#[doc = "Reader of register AHBCLKCTRL2"]
pub type R = crate::R<u32, super::AHBCLKCTRL2>;
#[doc = "Writer for register AHBCLKCTRL2"]
pub type W = crate::W<u32, super::AHBCLKCTRL2>;
#[doc = "Register AHBCLKCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCLKCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<DMA1_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_A) -> Self {
        match variant {
            DMA1_A::DISABLE => false,
            DMA1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `DMA1`"]
pub type DMA1_R = crate::R<bool, DMA1_A>;
impl DMA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_A {
        match self.bits {
            false => DMA1_A::DISABLE,
            true => DMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA1_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMA1`"]
pub struct DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA1_A::ENABLE)
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
#[doc = "Possible values of the field `COMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<COMP_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        match variant {
            COMP_A::DISABLE => false,
            COMP_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<bool, COMP_A>;
impl COMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_A {
        match self.bits {
            false => COMP_A::DISABLE,
            true => COMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMP_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMP_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMP_A::ENABLE)
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
#[doc = "Possible values of the field `SDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SDIO_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_A) -> Self {
        match variant {
            SDIO_A::DISABLE => false,
            SDIO_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SDIO`"]
pub type SDIO_R = crate::R<bool, SDIO_A>;
impl SDIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_A {
        match self.bits {
            false => SDIO_A::DISABLE,
            true => SDIO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDIO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDIO_A::ENABLE
    }
}
#[doc = "Write proxy for field `SDIO`"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIO_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_HOST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOST_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB1_HOST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_HOST_A) -> Self {
        match variant {
            USB1_HOST_A::DISABLE => false,
            USB1_HOST_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_HOST`"]
pub type USB1_HOST_R = crate::R<bool, USB1_HOST_A>;
impl USB1_HOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_HOST_A {
        match self.bits {
            false => USB1_HOST_A::DISABLE,
            true => USB1_HOST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_HOST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_HOST_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1_HOST`"]
pub struct USB1_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_HOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_HOST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_HOST_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_HOST_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_DEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEV_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB1_DEV_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_DEV_A) -> Self {
        match variant {
            USB1_DEV_A::DISABLE => false,
            USB1_DEV_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_DEV`"]
pub type USB1_DEV_R = crate::R<bool, USB1_DEV_A>;
impl USB1_DEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_DEV_A {
        match self.bits {
            false => USB1_DEV_A::DISABLE,
            true => USB1_DEV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_DEV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_DEV_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1_DEV`"]
pub struct USB1_DEV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_DEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_DEV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_DEV_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_DEV_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_RAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAM_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB1_RAM_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_RAM_A) -> Self {
        match variant {
            USB1_RAM_A::DISABLE => false,
            USB1_RAM_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_RAM`"]
pub type USB1_RAM_R = crate::R<bool, USB1_RAM_A>;
impl USB1_RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_RAM_A {
        match self.bits {
            false => USB1_RAM_A::DISABLE,
            true => USB1_RAM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_RAM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_RAM_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1_RAM`"]
pub struct USB1_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_RAM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_RAM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_RAM_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_PHY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB1_PHY_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_A) -> Self {
        match variant {
            USB1_PHY_A::DISABLE => false,
            USB1_PHY_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_PHY`"]
pub type USB1_PHY_R = crate::R<bool, USB1_PHY_A>;
impl USB1_PHY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_A {
        match self.bits {
            false => USB1_PHY_A::DISABLE,
            true => USB1_PHY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_PHY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_PHY_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1_PHY`"]
pub struct USB1_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_PHY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_PHY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_PHY_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_PHY_A::ENABLE)
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
#[doc = "Possible values of the field `FREQME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FREQME_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_A) -> Self {
        match variant {
            FREQME_A::DISABLE => false,
            FREQME_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FREQME`"]
pub type FREQME_R = crate::R<bool, FREQME_A>;
impl FREQME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_A {
        match self.bits {
            false => FREQME_A::DISABLE,
            true => FREQME_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FREQME_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FREQME_A::ENABLE
    }
}
#[doc = "Write proxy for field `FREQME`"]
pub struct FREQME_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREQME_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREQME_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO4_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4_A) -> Self {
        match variant {
            GPIO4_A::DISABLE => false,
            GPIO4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO4`"]
pub type GPIO4_R = crate::R<bool, GPIO4_A>;
impl GPIO4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4_A {
        match self.bits {
            false => GPIO4_A::DISABLE,
            true => GPIO4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO4_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO4`"]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO5_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5_A) -> Self {
        match variant {
            GPIO5_A::DISABLE => false,
            GPIO5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO5`"]
pub type GPIO5_R = crate::R<bool, GPIO5_A>;
impl GPIO5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5_A {
        match self.bits {
            false => GPIO5_A::DISABLE,
            true => GPIO5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO5_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO5`"]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO5_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO5_A::ENABLE)
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
#[doc = "Possible values of the field `OTP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTP_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<OTP_A> for bool {
    #[inline(always)]
    fn from(variant: OTP_A) -> Self {
        match variant {
            OTP_A::DISABLE => false,
            OTP_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `OTP`"]
pub type OTP_R = crate::R<bool, OTP_A>;
impl OTP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTP_A {
        match self.bits {
            false => OTP_A::DISABLE,
            true => OTP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OTP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OTP_A::ENABLE
    }
}
#[doc = "Write proxy for field `OTP`"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OTP_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OTP_A::ENABLE)
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
#[doc = "Possible values of the field `RNG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        match variant {
            RNG_A::DISABLE => false,
            RNG_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RNG`"]
pub type RNG_R = crate::R<bool, RNG_A>;
impl RNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::DISABLE,
            true => RNG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RNG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RNG_A::ENABLE
    }
}
#[doc = "Write proxy for field `RNG`"]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RNG_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RNG_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `MUX1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<MUX1_A> for bool {
    #[inline(always)]
    fn from(variant: MUX1_A) -> Self {
        match variant {
            MUX1_A::DISABLE => false,
            MUX1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `MUX1`"]
pub type MUX1_R = crate::R<bool, MUX1_A>;
impl MUX1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX1_A {
        match self.bits {
            false => MUX1_A::DISABLE,
            true => MUX1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MUX1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MUX1_A::ENABLE
    }
}
#[doc = "Write proxy for field `MUX1`"]
pub struct MUX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `USB0_HOSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTM_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB0_HOSTM_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTM_A) -> Self {
        match variant {
            USB0_HOSTM_A::DISABLE => false,
            USB0_HOSTM_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0_HOSTM`"]
pub type USB0_HOSTM_R = crate::R<bool, USB0_HOSTM_A>;
impl USB0_HOSTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTM_A {
        match self.bits {
            false => USB0_HOSTM_A::DISABLE,
            true => USB0_HOSTM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_HOSTM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_HOSTM_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0_HOSTM`"]
pub struct USB0_HOSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTM_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `USB0_HOSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTS_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB0_HOSTS_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTS_A) -> Self {
        match variant {
            USB0_HOSTS_A::DISABLE => false,
            USB0_HOSTS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0_HOSTS`"]
pub type USB0_HOSTS_R = crate::R<bool, USB0_HOSTS_A>;
impl USB0_HOSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTS_A {
        match self.bits {
            false => USB0_HOSTS_A::DISABLE,
            true => USB0_HOSTS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_HOSTS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_HOSTS_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0_HOSTS`"]
pub struct USB0_HOSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTS_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTS_A::ENABLE)
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
#[doc = "Possible values of the field `HASH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<HASH0_A> for bool {
    #[inline(always)]
    fn from(variant: HASH0_A) -> Self {
        match variant {
            HASH0_A::DISABLE => false,
            HASH0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `HASH0`"]
pub type HASH0_R = crate::R<bool, HASH0_A>;
impl HASH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH0_A {
        match self.bits {
            false => HASH0_A::DISABLE,
            true => HASH0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HASH0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HASH0_A::ENABLE
    }
}
#[doc = "Write proxy for field `HASH0`"]
pub struct HASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASH0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASH0_A::ENABLE)
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
#[doc = "Possible values of the field `PQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<PQ_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_A) -> Self {
        match variant {
            PQ_A::DISABLE => false,
            PQ_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PQ`"]
pub type PQ_R = crate::R<bool, PQ_A>;
impl PQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_A {
        match self.bits {
            false => PQ_A::DISABLE,
            true => PQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PQ_A::ENABLE
    }
}
#[doc = "Write proxy for field `PQ`"]
pub struct PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQ_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PQ_A::ENABLE)
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
#[doc = "Possible values of the field `PLULUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<PLULUT_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_A) -> Self {
        match variant {
            PLULUT_A::DISABLE => false,
            PLULUT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PLULUT`"]
pub type PLULUT_R = crate::R<bool, PLULUT_A>;
impl PLULUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_A {
        match self.bits {
            false => PLULUT_A::DISABLE,
            true => PLULUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLULUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLULUT_A::ENABLE
    }
}
#[doc = "Write proxy for field `PLULUT`"]
pub struct PLULUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLULUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLULUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLULUT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLULUT_A::ENABLE)
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
#[doc = "Possible values of the field `TIMER3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<TIMER3_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_A) -> Self {
        match variant {
            TIMER3_A::DISABLE => false,
            TIMER3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TIMER3`"]
pub type TIMER3_R = crate::R<bool, TIMER3_A>;
impl TIMER3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_A {
        match self.bits {
            false => TIMER3_A::DISABLE,
            true => TIMER3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER3_A::ENABLE
    }
}
#[doc = "Write proxy for field `TIMER3`"]
pub struct TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `TIMER4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<TIMER4_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_A) -> Self {
        match variant {
            TIMER4_A::DISABLE => false,
            TIMER4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TIMER4`"]
pub type TIMER4_R = crate::R<bool, TIMER4_A>;
impl TIMER4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_A {
        match self.bits {
            false => TIMER4_A::DISABLE,
            true => TIMER4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER4_A::ENABLE
    }
}
#[doc = "Write proxy for field `TIMER4`"]
pub struct TIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `PUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<PUF_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        match variant {
            PUF_A::DISABLE => false,
            PUF_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PUF`"]
pub type PUF_R = crate::R<bool, PUF_A>;
impl PUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            false => PUF_A::DISABLE,
            true => PUF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PUF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PUF_A::ENABLE
    }
}
#[doc = "Write proxy for field `PUF`"]
pub struct PUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUF_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUF_A::ENABLE)
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
#[doc = "Possible values of the field `CASPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        match variant {
            CASPER_A::DISABLE => false,
            CASPER_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CASPER`"]
pub type CASPER_R = crate::R<bool, CASPER_A>;
impl CASPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::DISABLE,
            true => CASPER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CASPER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CASPER_A::ENABLE
    }
}
#[doc = "Write proxy for field `CASPER`"]
pub struct CASPER_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPER_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPER_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `CAPT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<CAPT0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT0_A) -> Self {
        match variant {
            CAPT0_A::DISABLE => false,
            CAPT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CAPT0`"]
pub type CAPT0_R = crate::R<bool, CAPT0_A>;
impl CAPT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT0_A {
        match self.bits {
            false => CAPT0_A::DISABLE,
            true => CAPT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPT0`"]
pub struct CAPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPT0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPT0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `ANALOG_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<ANALOG_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_A) -> Self {
        match variant {
            ANALOG_CTRL_A::DISABLE => false,
            ANALOG_CTRL_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ANALOG_CTRL`"]
pub type ANALOG_CTRL_R = crate::R<bool, ANALOG_CTRL_A>;
impl ANALOG_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_A {
        match self.bits {
            false => ANALOG_CTRL_A::DISABLE,
            true => ANALOG_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ANALOG_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ANALOG_CTRL_A::ENABLE
    }
}
#[doc = "Write proxy for field `ANALOG_CTRL`"]
pub struct ANALOG_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANALOG_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `HS_LSPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<HS_LSPI_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_A) -> Self {
        match variant {
            HS_LSPI_A::DISABLE => false,
            HS_LSPI_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `HS_LSPI`"]
pub type HS_LSPI_R = crate::R<bool, HS_LSPI_A>;
impl HS_LSPI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_A {
        match self.bits {
            false => HS_LSPI_A::DISABLE,
            true => HS_LSPI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HS_LSPI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HS_LSPI_A::ENABLE
    }
}
#[doc = "Write proxy for field `HS_LSPI`"]
pub struct HS_LSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO_SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO_SEC_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_A) -> Self {
        match variant {
            GPIO_SEC_A::DISABLE => false,
            GPIO_SEC_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_SEC`"]
pub type GPIO_SEC_R = crate::R<bool, GPIO_SEC_A>;
impl GPIO_SEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_A {
        match self.bits {
            false => GPIO_SEC_A::DISABLE,
            true => GPIO_SEC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SEC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SEC_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_SEC`"]
pub struct GPIO_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO_SEC_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<GPIO_SEC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_A) -> Self {
        match variant {
            GPIO_SEC_INT_A::DISABLE => false,
            GPIO_SEC_INT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_SEC_INT`"]
pub type GPIO_SEC_INT_R = crate::R<bool, GPIO_SEC_INT_A>;
impl GPIO_SEC_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_A {
        match self.bits {
            false => GPIO_SEC_INT_A::DISABLE,
            true => GPIO_SEC_INT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SEC_INT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SEC_INT_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_SEC_INT`"]
pub struct GPIO_SEC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline(always)]
    pub fn usb1_host(&self) -> USB1_HOST_R {
        USB1_HOST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline(always)]
    pub fn usb1_dev(&self) -> USB1_DEV_R {
        USB1_DEV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline(always)]
    pub fn usb1_ram(&self) -> USB1_RAM_R {
        USB1_RAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline(always)]
    pub fn usb1_phy(&self) -> USB1_PHY_R {
        USB1_PHY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&self) -> FREQME_R {
        FREQME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the Peripheral Input Mux 1."]
    #[inline(always)]
    pub fn mux1(&self) -> MUX1_R {
        MUX1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&self) -> USB0_HOSTM_R {
        USB0_HOSTM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&self) -> USB0_HOSTS_R {
        USB0_HOSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH0."]
    #[inline(always)]
    pub fn hash0(&self) -> HASH0_R {
        HASH0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&self) -> PLULUT_R {
        PLULUT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the CAPT0."]
    #[inline(always)]
    pub fn capt0(&self) -> CAPT0_R {
        CAPT0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&self) -> ANALOG_CTRL_R {
        ANALOG_CTRL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&self) -> HS_LSPI_R {
        HS_LSPI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&self) -> GPIO_SEC_R {
        GPIO_SEC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&self) -> GPIO_SEC_INT_R {
        GPIO_SEC_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W {
        DMA1_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline(always)]
    pub fn usb1_host(&mut self) -> USB1_HOST_W {
        USB1_HOST_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline(always)]
    pub fn usb1_dev(&mut self) -> USB1_DEV_W {
        USB1_DEV_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline(always)]
    pub fn usb1_ram(&mut self) -> USB1_RAM_W {
        USB1_RAM_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline(always)]
    pub fn usb1_phy(&mut self) -> USB1_PHY_W {
        USB1_PHY_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&mut self) -> FREQME_W {
        FREQME_W { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the OTP."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the Peripheral Input Mux 1."]
    #[inline(always)]
    pub fn mux1(&mut self) -> MUX1_W {
        MUX1_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&mut self) -> USB0_HOSTM_W {
        USB0_HOSTM_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&mut self) -> USB0_HOSTS_W {
        USB0_HOSTS_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the HASH0."]
    #[inline(always)]
    pub fn hash0(&mut self) -> HASH0_W {
        HASH0_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline(always)]
    pub fn pq(&mut self) -> PQ_W {
        PQ_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&mut self) -> PLULUT_W {
        PLULUT_W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&mut self) -> PUF_W {
        PUF_W { w: self }
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&mut self) -> CASPER_W {
        CASPER_W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the CAPT0."]
    #[inline(always)]
    pub fn capt0(&mut self) -> CAPT0_W {
        CAPT0_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&mut self) -> ANALOG_CTRL_W {
        ANALOG_CTRL_W { w: self }
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&mut self) -> HS_LSPI_W {
        HS_LSPI_W { w: self }
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&mut self) -> GPIO_SEC_W {
        GPIO_SEC_W { w: self }
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&mut self) -> GPIO_SEC_INT_W {
        GPIO_SEC_INT_W { w: self }
    }
}
