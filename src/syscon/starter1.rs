#[doc = "Reader of register STARTER1"]
pub type R = crate::R<u32, super::STARTER1>;
#[doc = "Writer for register STARTER1"]
pub type W = crate::W<u32, super::STARTER1>;
#[doc = "Register STARTER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `GPIO_INT04`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT04_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GPIO_INT04_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT04_A) -> Self {
        match variant {
            GPIO_INT04_A::DISABLE => false,
            GPIO_INT04_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT04`"]
pub type GPIO_INT04_R = crate::R<bool, GPIO_INT04_A>;
impl GPIO_INT04_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT04_A {
        match self.bits {
            false => GPIO_INT04_A::DISABLE,
            true => GPIO_INT04_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT04_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT04_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_INT04`"]
pub struct GPIO_INT04_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT04_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT04_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT04_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT04_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO_INT05`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT05_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GPIO_INT05_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT05_A) -> Self {
        match variant {
            GPIO_INT05_A::DISABLE => false,
            GPIO_INT05_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT05`"]
pub type GPIO_INT05_R = crate::R<bool, GPIO_INT05_A>;
impl GPIO_INT05_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT05_A {
        match self.bits {
            false => GPIO_INT05_A::DISABLE,
            true => GPIO_INT05_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT05_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT05_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_INT05`"]
pub struct GPIO_INT05_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT05_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT05_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT05_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT05_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO_INT06`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT06_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GPIO_INT06_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT06_A) -> Self {
        match variant {
            GPIO_INT06_A::DISABLE => false,
            GPIO_INT06_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT06`"]
pub type GPIO_INT06_R = crate::R<bool, GPIO_INT06_A>;
impl GPIO_INT06_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT06_A {
        match self.bits {
            false => GPIO_INT06_A::DISABLE,
            true => GPIO_INT06_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT06_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT06_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_INT06`"]
pub struct GPIO_INT06_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT06_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT06_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT06_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT06_A::ENABLE)
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
#[doc = "Possible values of the field `GPIO_INT07`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT07_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<GPIO_INT07_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT07_A) -> Self {
        match variant {
            GPIO_INT07_A::DISABLE => false,
            GPIO_INT07_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT07`"]
pub type GPIO_INT07_R = crate::R<bool, GPIO_INT07_A>;
impl GPIO_INT07_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT07_A {
        match self.bits {
            false => GPIO_INT07_A::DISABLE,
            true => GPIO_INT07_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_INT07_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_INT07_A::ENABLE
    }
}
#[doc = "Write proxy for field `GPIO_INT07`"]
pub struct GPIO_INT07_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT07_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT07_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT07_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT07_A::ENABLE)
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
#[doc = "Possible values of the field `CTIMER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER2_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CTIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER2_A) -> Self {
        match variant {
            CTIMER2_A::DISABLE => false,
            CTIMER2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER2`"]
pub type CTIMER2_R = crate::R<bool, CTIMER2_A>;
impl CTIMER2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER2_A {
        match self.bits {
            false => CTIMER2_A::DISABLE,
            true => CTIMER2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER2_A::ENABLE
    }
}
#[doc = "Write proxy for field `CTIMER2`"]
pub struct CTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER2_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER2_A::ENABLE)
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
#[doc = "Possible values of the field `CTIMER4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER4_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CTIMER4_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER4_A) -> Self {
        match variant {
            CTIMER4_A::DISABLE => false,
            CTIMER4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER4`"]
pub type CTIMER4_R = crate::R<bool, CTIMER4_A>;
impl CTIMER4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER4_A {
        match self.bits {
            false => CTIMER4_A::DISABLE,
            true => CTIMER4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTIMER4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTIMER4_A::ENABLE
    }
}
#[doc = "Write proxy for field `CTIMER4`"]
pub struct CTIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER4_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER4_A::ENABLE)
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
#[doc = "Possible values of the field `OS_EVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OS_EVENT_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<OS_EVENT_A> for bool {
    #[inline(always)]
    fn from(variant: OS_EVENT_A) -> Self {
        match variant {
            OS_EVENT_A::DISABLE => false,
            OS_EVENT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `OS_EVENT`"]
pub type OS_EVENT_R = crate::R<bool, OS_EVENT_A>;
impl OS_EVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_A {
        match self.bits {
            false => OS_EVENT_A::DISABLE,
            true => OS_EVENT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OS_EVENT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OS_EVENT_A::ENABLE
    }
}
#[doc = "Write proxy for field `OS_EVENT`"]
pub struct OS_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> OS_EVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OS_EVENT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OS_EVENT_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OS_EVENT_A::ENABLE)
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
#[doc = "Possible values of the field `SDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
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
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `USB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<USB1_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_A) -> Self {
        match variant {
            USB1_A::DISABLE => false,
            USB1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1`"]
pub type USB1_R = crate::R<bool, USB1_A>;
impl USB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_A {
        match self.bits {
            false => USB1_A::DISABLE,
            true => USB1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1`"]
pub struct USB1_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_NEEDCLK_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<USB1_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_NEEDCLK_A) -> Self {
        match variant {
            USB1_NEEDCLK_A::DISABLE => false,
            USB1_NEEDCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_NEEDCLK`"]
pub type USB1_NEEDCLK_R = crate::R<bool, USB1_NEEDCLK_A>;
impl USB1_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_NEEDCLK_A {
        match self.bits {
            false => USB1_NEEDCLK_A::DISABLE,
            true => USB1_NEEDCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB1_NEEDCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB1_NEEDCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB1_NEEDCLK`"]
pub struct USB1_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::ENABLE)
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
#[doc = "Possible values of the field `SEC_HYPERVISOR_CALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_HYPERVISOR_CALL_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SEC_HYPERVISOR_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_HYPERVISOR_CALL_A) -> Self {
        match variant {
            SEC_HYPERVISOR_CALL_A::DISABLE => false,
            SEC_HYPERVISOR_CALL_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_HYPERVISOR_CALL`"]
pub type SEC_HYPERVISOR_CALL_R = crate::R<bool, SEC_HYPERVISOR_CALL_A>;
impl SEC_HYPERVISOR_CALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_HYPERVISOR_CALL_A {
        match self.bits {
            false => SEC_HYPERVISOR_CALL_A::DISABLE,
            true => SEC_HYPERVISOR_CALL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_A::ENABLE
    }
}
#[doc = "Write proxy for field `SEC_HYPERVISOR_CALL`"]
pub struct SEC_HYPERVISOR_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_HYPERVISOR_CALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_HYPERVISOR_CALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_A::ENABLE)
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
#[doc = "Possible values of the field `SEC_GPIO_INT00`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT00_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SEC_GPIO_INT00_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT00_A) -> Self {
        match variant {
            SEC_GPIO_INT00_A::DISABLE => false,
            SEC_GPIO_INT00_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_GPIO_INT00`"]
pub type SEC_GPIO_INT00_R = crate::R<bool, SEC_GPIO_INT00_A>;
impl SEC_GPIO_INT00_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT00_A {
        match self.bits {
            false => SEC_GPIO_INT00_A::DISABLE,
            true => SEC_GPIO_INT00_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_GPIO_INT00_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_GPIO_INT00_A::ENABLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_INT00`"]
pub struct SEC_GPIO_INT00_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT00_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_INT00_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT00_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT00_A::ENABLE)
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
#[doc = "Possible values of the field `SEC_GPIO_INT01`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT01_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SEC_GPIO_INT01_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT01_A) -> Self {
        match variant {
            SEC_GPIO_INT01_A::DISABLE => false,
            SEC_GPIO_INT01_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_GPIO_INT01`"]
pub type SEC_GPIO_INT01_R = crate::R<bool, SEC_GPIO_INT01_A>;
impl SEC_GPIO_INT01_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT01_A {
        match self.bits {
            false => SEC_GPIO_INT01_A::DISABLE,
            true => SEC_GPIO_INT01_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_GPIO_INT01_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_GPIO_INT01_A::ENABLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_INT01`"]
pub struct SEC_GPIO_INT01_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_INT01_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT01_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT01_A::ENABLE)
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
#[doc = "Possible values of the field `PLU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<PLU_A> for bool {
    #[inline(always)]
    fn from(variant: PLU_A) -> Self {
        match variant {
            PLU_A::DISABLE => false,
            PLU_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PLU`"]
pub type PLU_R = crate::R<bool, PLU_A>;
impl PLU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_A {
        match self.bits {
            false => PLU_A::DISABLE,
            true => PLU_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLU_A::ENABLE
    }
}
#[doc = "Write proxy for field `PLU`"]
pub struct PLU_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLU_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLU_A::ENABLE)
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
#[doc = "Possible values of the field `SEC_VIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SEC_VIO_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_A) -> Self {
        match variant {
            SEC_VIO_A::DISABLE => false,
            SEC_VIO_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_VIO`"]
pub type SEC_VIO_R = crate::R<bool, SEC_VIO_A>;
impl SEC_VIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_A {
        match self.bits {
            false => SEC_VIO_A::DISABLE,
            true => SEC_VIO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_VIO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_VIO_A::ENABLE
    }
}
#[doc = "Write proxy for field `SEC_VIO`"]
pub struct SEC_VIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_VIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_VIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_VIO_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_VIO_A::ENABLE)
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
#[doc = "Possible values of the field `SHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHA_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SHA_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_A) -> Self {
        match variant {
            SHA_A::DISABLE => false,
            SHA_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SHA`"]
pub type SHA_R = crate::R<bool, SHA_A>;
impl SHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_A {
        match self.bits {
            false => SHA_A::DISABLE,
            true => SHA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SHA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SHA_A::ENABLE
    }
}
#[doc = "Write proxy for field `SHA`"]
pub struct SHA_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SHA_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SHA_A::ENABLE)
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
#[doc = "Possible values of the field `CASER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASER_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<CASER_A> for bool {
    #[inline(always)]
    fn from(variant: CASER_A) -> Self {
        match variant {
            CASER_A::DISABLE => false,
            CASER_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CASER`"]
pub type CASER_R = crate::R<bool, CASER_A>;
impl CASER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASER_A {
        match self.bits {
            false => CASER_A::DISABLE,
            true => CASER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CASER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CASER_A::ENABLE
    }
}
#[doc = "Write proxy for field `CASER`"]
pub struct CASER_W<'a> {
    w: &'a mut W,
}
impl<'a> CASER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASER_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASER_A::ENABLE)
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
#[doc = "Possible values of the field `QDDKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDDKEY_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<QDDKEY_A> for bool {
    #[inline(always)]
    fn from(variant: QDDKEY_A) -> Self {
        match variant {
            QDDKEY_A::DISABLE => false,
            QDDKEY_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `QDDKEY`"]
pub type QDDKEY_R = crate::R<bool, QDDKEY_A>;
impl QDDKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QDDKEY_A {
        match self.bits {
            false => QDDKEY_A::DISABLE,
            true => QDDKEY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == QDDKEY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == QDDKEY_A::ENABLE
    }
}
#[doc = "Write proxy for field `QDDKEY`"]
pub struct QDDKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> QDDKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QDDKEY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(QDDKEY_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(QDDKEY_A::ENABLE)
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
#[doc = "Possible values of the field `PQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
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
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQ_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `SDMA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<SDMA1_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA1_A) -> Self {
        match variant {
            SDMA1_A::DISABLE => false,
            SDMA1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SDMA1`"]
pub type SDMA1_R = crate::R<bool, SDMA1_A>;
impl SDMA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_A {
        match self.bits {
            false => SDMA1_A::DISABLE,
            true => SDMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDMA1_A::ENABLE
    }
}
#[doc = "Write proxy for field `SDMA1`"]
pub struct SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA1_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `LSPI_HS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPI_HS_A {
    #[doc = "Wake-up disabled."]
    DISABLE,
    #[doc = "Wake-up enabled."]
    ENABLE,
}
impl From<LSPI_HS_A> for bool {
    #[inline(always)]
    fn from(variant: LSPI_HS_A) -> Self {
        match variant {
            LSPI_HS_A::DISABLE => false,
            LSPI_HS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `LSPI_HS`"]
pub type LSPI_HS_R = crate::R<bool, LSPI_HS_A>;
impl LSPI_HS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPI_HS_A {
        match self.bits {
            false => LSPI_HS_A::DISABLE,
            true => LSPI_HS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LSPI_HS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LSPI_HS_A::ENABLE
    }
}
#[doc = "Write proxy for field `LSPI_HS`"]
pub struct LSPI_HS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPI_HS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPI_HS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LSPI_HS_A::DISABLE)
    }
    #[doc = "Wake-up enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LSPI_HS_A::ENABLE)
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
#[doc = "Reader of field `WAKEUPPADS`"]
pub type WAKEUPPADS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUPPADS`"]
pub struct WAKEUPPADS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPPADS_W<'a> {
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
    #[doc = "Bit 0 - GPIO_INT04 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int04(&self) -> GPIO_INT04_R {
        GPIO_INT04_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO_INT05 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int05(&self) -> GPIO_INT05_R {
        GPIO_INT05_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO_INT06 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int06(&self) -> GPIO_INT06_R {
        GPIO_INT06_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO_INT07 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int07(&self) -> GPIO_INT07_R {
        GPIO_INT07_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTIMER2 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTIMER4 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OS_EVENT interrupt wake-up."]
    #[inline(always)]
    pub fn os_event(&self) -> OS_EVENT_R {
        OS_EVENT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SDIO interrupt wake-up."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB1 interrupt wake-up."]
    #[inline(always)]
    pub fn usb1(&self) -> USB1_R {
        USB1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB1_NEEDCLK interrupt wake-up."]
    #[inline(always)]
    pub fn usb1_needclk(&self) -> USB1_NEEDCLK_R {
        USB1_NEEDCLK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SEC_HYPERVISOR_CALL interrupt wake-up."]
    #[inline(always)]
    pub fn sec_hypervisor_call(&self) -> SEC_HYPERVISOR_CALL_R {
        SEC_HYPERVISOR_CALL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SEC_GPIO_INT00 interrupt wake-up."]
    #[inline(always)]
    pub fn sec_gpio_int00(&self) -> SEC_GPIO_INT00_R {
        SEC_GPIO_INT00_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SEC_GPIO_INT01 interrupt wake-up."]
    #[inline(always)]
    pub fn sec_gpio_int01(&self) -> SEC_GPIO_INT01_R {
        SEC_GPIO_INT01_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLU interrupt wake-up."]
    #[inline(always)]
    pub fn plu(&self) -> PLU_R {
        PLU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SEC_VIO interrupt wake-up."]
    #[inline(always)]
    pub fn sec_vio(&self) -> SEC_VIO_R {
        SEC_VIO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SHA interrupt wake-up."]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CASER interrupt wake-up."]
    #[inline(always)]
    pub fn caser(&self) -> CASER_R {
        CASER_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - QDDKEY interrupt wake-up."]
    #[inline(always)]
    pub fn qddkey(&self) -> QDDKEY_R {
        QDDKEY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PQ interrupt wake-up."]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SDMA1 interrupt wake-up."]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LSPI_HS interrupt wake-up."]
    #[inline(always)]
    pub fn lspi_hs(&self) -> LSPI_HS_R {
        LSPI_HS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WAKEUPPADS interrupt wake-up."]
    #[inline(always)]
    pub fn wakeuppads(&self) -> WAKEUPPADS_R {
        WAKEUPPADS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO_INT04 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int04(&mut self) -> GPIO_INT04_W {
        GPIO_INT04_W { w: self }
    }
    #[doc = "Bit 1 - GPIO_INT05 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int05(&mut self) -> GPIO_INT05_W {
        GPIO_INT05_W { w: self }
    }
    #[doc = "Bit 2 - GPIO_INT06 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int06(&mut self) -> GPIO_INT06_W {
        GPIO_INT06_W { w: self }
    }
    #[doc = "Bit 3 - GPIO_INT07 interrupt wake-up."]
    #[inline(always)]
    pub fn gpio_int07(&mut self) -> GPIO_INT07_W {
        GPIO_INT07_W { w: self }
    }
    #[doc = "Bit 4 - CTIMER2 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W {
        CTIMER2_W { w: self }
    }
    #[doc = "Bit 5 - CTIMER4 interrupt wake-up."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W {
        CTIMER4_W { w: self }
    }
    #[doc = "Bit 6 - OS_EVENT interrupt wake-up."]
    #[inline(always)]
    pub fn os_event(&mut self) -> OS_EVENT_W {
        OS_EVENT_W { w: self }
    }
    #[doc = "Bit 10 - SDIO interrupt wake-up."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 15 - USB1 interrupt wake-up."]
    #[inline(always)]
    pub fn usb1(&mut self) -> USB1_W {
        USB1_W { w: self }
    }
    #[doc = "Bit 16 - USB1_NEEDCLK interrupt wake-up."]
    #[inline(always)]
    pub fn usb1_needclk(&mut self) -> USB1_NEEDCLK_W {
        USB1_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 17 - SEC_HYPERVISOR_CALL interrupt wake-up."]
    #[inline(always)]
    pub fn sec_hypervisor_call(&mut self) -> SEC_HYPERVISOR_CALL_W {
        SEC_HYPERVISOR_CALL_W { w: self }
    }
    #[doc = "Bit 18 - SEC_GPIO_INT00 interrupt wake-up."]
    #[inline(always)]
    pub fn sec_gpio_int00(&mut self) -> SEC_GPIO_INT00_W {
        SEC_GPIO_INT00_W { w: self }
    }
    #[doc = "Bit 19 - SEC_GPIO_INT01 interrupt wake-up."]
    #[inline(always)]
    pub fn sec_gpio_int01(&mut self) -> SEC_GPIO_INT01_W {
        SEC_GPIO_INT01_W { w: self }
    }
    #[doc = "Bit 20 - PLU interrupt wake-up."]
    #[inline(always)]
    pub fn plu(&mut self) -> PLU_W {
        PLU_W { w: self }
    }
    #[doc = "Bit 21 - SEC_VIO interrupt wake-up."]
    #[inline(always)]
    pub fn sec_vio(&mut self) -> SEC_VIO_W {
        SEC_VIO_W { w: self }
    }
    #[doc = "Bit 22 - SHA interrupt wake-up."]
    #[inline(always)]
    pub fn sha(&mut self) -> SHA_W {
        SHA_W { w: self }
    }
    #[doc = "Bit 23 - CASER interrupt wake-up."]
    #[inline(always)]
    pub fn caser(&mut self) -> CASER_W {
        CASER_W { w: self }
    }
    #[doc = "Bit 24 - QDDKEY interrupt wake-up."]
    #[inline(always)]
    pub fn qddkey(&mut self) -> QDDKEY_W {
        QDDKEY_W { w: self }
    }
    #[doc = "Bit 25 - PQ interrupt wake-up."]
    #[inline(always)]
    pub fn pq(&mut self) -> PQ_W {
        PQ_W { w: self }
    }
    #[doc = "Bit 26 - SDMA1 interrupt wake-up."]
    #[inline(always)]
    pub fn sdma1(&mut self) -> SDMA1_W {
        SDMA1_W { w: self }
    }
    #[doc = "Bit 27 - LSPI_HS interrupt wake-up."]
    #[inline(always)]
    pub fn lspi_hs(&mut self) -> LSPI_HS_W {
        LSPI_HS_W { w: self }
    }
    #[doc = "Bit 31 - WAKEUPPADS interrupt wake-up."]
    #[inline(always)]
    pub fn wakeuppads(&mut self) -> WAKEUPPADS_W {
        WAKEUPPADS_W { w: self }
    }
}
