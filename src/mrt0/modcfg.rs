#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODCFG {
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
#[doc = r" Value of the field"]
pub struct NOCR {
    bits: u8,
}
impl NOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NOBR {
    bits: u8,
}
impl NOBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MULTITASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTITASKR {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE,
}
impl MULTITASKR {
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
            MULTITASKR::HARDWARE_STATUS_MODE => false,
            MULTITASKR::MULTI_TASK_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MULTITASKR {
        match value {
            false => MULTITASKR::HARDWARE_STATUS_MODE,
            true => MULTITASKR::MULTI_TASK_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE_STATUS_MODE`"]
    #[inline]
    pub fn is_hardware_status_mode(&self) -> bool {
        *self == MULTITASKR::HARDWARE_STATUS_MODE
    }
    #[doc = "Checks if the value of the field is `MULTI_TASK_MODE`"]
    #[inline]
    pub fn is_multi_task_mode(&self) -> bool {
        *self == MULTITASKR::MULTI_TASK_MODE
    }
}
#[doc = r" Proxy"]
pub struct _NOCW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOBW<'a> {
    w: &'a mut W,
}
impl<'a> _NOBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MULTITASK`"]
pub enum MULTITASKW {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE,
}
impl MULTITASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MULTITASKW::HARDWARE_STATUS_MODE => false,
            MULTITASKW::MULTI_TASK_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTITASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTITASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTITASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline]
    pub fn hardware_status_mode(self) -> &'a mut W {
        self.variant(MULTITASKW::HARDWARE_STATUS_MODE)
    }
    #[doc = "Multi-task mode."]
    #[inline]
    pub fn multi_task_mode(self) -> &'a mut W {
        self.variant(MULTITASKW::MULTI_TASK_MODE)
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
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline]
    pub fn noc(&self) -> NOCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOCR { bits }
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline]
    pub fn nob(&self) -> NOBR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NOBR { bits }
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline]
    pub fn multitask(&self) -> MULTITASKR {
        MULTITASKR::_from({
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
        W { bits: 371 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline]
    pub fn noc(&mut self) -> _NOCW {
        _NOCW { w: self }
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline]
    pub fn nob(&mut self) -> _NOBW {
        _NOBW { w: self }
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline]
    pub fn multitask(&mut self) -> _MULTITASKW {
        _MULTITASKW { w: self }
    }
}
