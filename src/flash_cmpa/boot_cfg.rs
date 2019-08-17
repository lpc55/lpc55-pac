#[doc = "Reader of register BOOT_CFG"]
pub type R = crate::R<u32, super::BOOT_CFG>;
#[doc = "Writer for register BOOT_CFG"]
pub type W = crate::W<u32, super::BOOT_CFG>;
#[doc = "Register BOOT_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOT_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DEFAULT_ISP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFAULT_ISP_MODE_A {
    #[doc = "Auto ISP"]
    VALUE_0,
    #[doc = "USB_HID_MSC"]
    VALUE_1,
    #[doc = "SPI Slave ISP"]
    VALUE_2,
    #[doc = "I2C Slave ISP"]
    VALUE_3,
    #[doc = "Disable ISP fall through"]
    VALUE_7,
}
impl From<DEFAULT_ISP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFAULT_ISP_MODE_A) -> Self {
        match variant {
            DEFAULT_ISP_MODE_A::VALUE_0 => 0,
            DEFAULT_ISP_MODE_A::VALUE_1 => 1,
            DEFAULT_ISP_MODE_A::VALUE_2 => 2,
            DEFAULT_ISP_MODE_A::VALUE_3 => 3,
            DEFAULT_ISP_MODE_A::VALUE_7 => 7,
        }
    }
}
#[doc = "Reader of field `DEFAULT_ISP_MODE`"]
pub type DEFAULT_ISP_MODE_R = crate::R<u8, DEFAULT_ISP_MODE_A>;
impl DEFAULT_ISP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEFAULT_ISP_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEFAULT_ISP_MODE_A::VALUE_0),
            1 => Val(DEFAULT_ISP_MODE_A::VALUE_1),
            2 => Val(DEFAULT_ISP_MODE_A::VALUE_2),
            3 => Val(DEFAULT_ISP_MODE_A::VALUE_3),
            7 => Val(DEFAULT_ISP_MODE_A::VALUE_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_7`"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_7
    }
}
#[doc = "Write proxy for field `DEFAULT_ISP_MODE`"]
pub struct DEFAULT_ISP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFAULT_ISP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEFAULT_ISP_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Auto ISP"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_0)
    }
    #[doc = "USB_HID_MSC"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_1)
    }
    #[doc = "SPI Slave ISP"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_2)
    }
    #[doc = "I2C Slave ISP"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_3)
    }
    #[doc = "Disable ISP fall through"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `BOOT_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_SPEED_A {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0,
    #[doc = "48MHz FRO"]
    VALUE_1,
    #[doc = "96MHz FRO"]
    VALUE_2,
}
impl From<BOOT_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SPEED_A) -> Self {
        match variant {
            BOOT_SPEED_A::VALUE_0 => 0,
            BOOT_SPEED_A::VALUE_1 => 1,
            BOOT_SPEED_A::VALUE_2 => 2,
        }
    }
}
#[doc = "Reader of field `BOOT_SPEED`"]
pub type BOOT_SPEED_R = crate::R<u8, BOOT_SPEED_A>;
impl BOOT_SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BOOT_SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BOOT_SPEED_A::VALUE_0),
            1 => Val(BOOT_SPEED_A::VALUE_1),
            2 => Val(BOOT_SPEED_A::VALUE_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_2
    }
}
#[doc = "Write proxy for field `BOOT_SPEED`"]
pub struct BOOT_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_0)
    }
    #[doc = "48MHz FRO"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_1)
    }
    #[doc = "96MHz FRO"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `BOOT_FAILURE_PIN`"]
pub type BOOT_FAILURE_PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BOOT_FAILURE_PIN`"]
pub struct BOOT_FAILURE_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_FAILURE_PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&self) -> DEFAULT_ISP_MODE_R {
        DEFAULT_ISP_MODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&self) -> BOOT_SPEED_R {
        BOOT_SPEED_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&self) -> BOOT_FAILURE_PIN_R {
        BOOT_FAILURE_PIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&mut self) -> DEFAULT_ISP_MODE_W {
        DEFAULT_ISP_MODE_W { w: self }
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&mut self) -> BOOT_SPEED_W {
        BOOT_SPEED_W { w: self }
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&mut self) -> BOOT_FAILURE_PIN_W {
        BOOT_FAILURE_PIN_W { w: self }
    }
}
