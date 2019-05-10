#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOOT_CFG {
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
#[doc = "Possible values of the field `DEFAULT_ISP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEFAULT_ISP_MODER {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEFAULT_ISP_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEFAULT_ISP_MODER::VALUE_0 => 0,
            DEFAULT_ISP_MODER::VALUE_1 => 1,
            DEFAULT_ISP_MODER::VALUE_2 => 2,
            DEFAULT_ISP_MODER::VALUE_3 => 3,
            DEFAULT_ISP_MODER::VALUE_7 => 7,
            DEFAULT_ISP_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEFAULT_ISP_MODER {
        match value {
            0 => DEFAULT_ISP_MODER::VALUE_0,
            1 => DEFAULT_ISP_MODER::VALUE_1,
            2 => DEFAULT_ISP_MODER::VALUE_2,
            3 => DEFAULT_ISP_MODER::VALUE_3,
            7 => DEFAULT_ISP_MODER::VALUE_7,
            i => DEFAULT_ISP_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == DEFAULT_ISP_MODER::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == DEFAULT_ISP_MODER::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline]
    pub fn is_value_2(&self) -> bool {
        *self == DEFAULT_ISP_MODER::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline]
    pub fn is_value_3(&self) -> bool {
        *self == DEFAULT_ISP_MODER::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_7`"]
    #[inline]
    pub fn is_value_7(&self) -> bool {
        *self == DEFAULT_ISP_MODER::VALUE_7
    }
}
#[doc = "Possible values of the field `BOOT_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_SPEEDR {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0,
    #[doc = "48MHz FRO"]
    VALUE_1,
    #[doc = "96MHz FRO"]
    VALUE_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BOOT_SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BOOT_SPEEDR::VALUE_0 => 0,
            BOOT_SPEEDR::VALUE_1 => 1,
            BOOT_SPEEDR::VALUE_2 => 2,
            BOOT_SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BOOT_SPEEDR {
        match value {
            0 => BOOT_SPEEDR::VALUE_0,
            1 => BOOT_SPEEDR::VALUE_1,
            2 => BOOT_SPEEDR::VALUE_2,
            i => BOOT_SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == BOOT_SPEEDR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == BOOT_SPEEDR::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SPEEDR::VALUE_2
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_FAILURE_PINR {
    bits: u8,
}
impl BOOT_FAILURE_PINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DEFAULT_ISP_MODE`"]
pub enum DEFAULT_ISP_MODEW {
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
impl DEFAULT_ISP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEFAULT_ISP_MODEW::VALUE_0 => 0,
            DEFAULT_ISP_MODEW::VALUE_1 => 1,
            DEFAULT_ISP_MODEW::VALUE_2 => 2,
            DEFAULT_ISP_MODEW::VALUE_3 => 3,
            DEFAULT_ISP_MODEW::VALUE_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEFAULT_ISP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEFAULT_ISP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEFAULT_ISP_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Auto ISP"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODEW::VALUE_0)
    }
    #[doc = "USB_HID_MSC"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODEW::VALUE_1)
    }
    #[doc = "SPI Slave ISP"]
    #[inline]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODEW::VALUE_2)
    }
    #[doc = "I2C Slave ISP"]
    #[inline]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODEW::VALUE_3)
    }
    #[doc = "Disable ISP fall through"]
    #[inline]
    pub fn value_7(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODEW::VALUE_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOT_SPEED`"]
pub enum BOOT_SPEEDW {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0,
    #[doc = "48MHz FRO"]
    VALUE_1,
    #[doc = "96MHz FRO"]
    VALUE_2,
}
impl BOOT_SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BOOT_SPEEDW::VALUE_0 => 0,
            BOOT_SPEEDW::VALUE_1 => 1,
            BOOT_SPEEDW::VALUE_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOT_SPEEDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(BOOT_SPEEDW::VALUE_0)
    }
    #[doc = "48MHz FRO"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(BOOT_SPEEDW::VALUE_1)
    }
    #[doc = "96MHz FRO"]
    #[inline]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SPEEDW::VALUE_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_FAILURE_PINW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_FAILURE_PINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline]
    pub fn default_isp_mode(&self) -> DEFAULT_ISP_MODER {
        DEFAULT_ISP_MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline]
    pub fn boot_speed(&self) -> BOOT_SPEEDR {
        BOOT_SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline]
    pub fn boot_failure_pin(&self) -> BOOT_FAILURE_PINR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BOOT_FAILURE_PINR { bits }
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
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline]
    pub fn default_isp_mode(&mut self) -> _DEFAULT_ISP_MODEW {
        _DEFAULT_ISP_MODEW { w: self }
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline]
    pub fn boot_speed(&mut self) -> _BOOT_SPEEDW {
        _BOOT_SPEEDW { w: self }
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline]
    pub fn boot_failure_pin(&mut self) -> _BOOT_FAILURE_PINW {
        _BOOT_FAILURE_PINW { w: self }
    }
}
