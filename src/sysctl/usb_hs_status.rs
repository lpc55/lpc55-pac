#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB_HS_STATUS {
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
#[doc = "Possible values of the field `USBHS_3V_NOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHS_3V_NOKR {
    #[doc = "3v3 supply is good."]
    SUPPLY_3V_OK,
    #[doc = "3v3 supply is too low."]
    SUPPLY_3V_LOW,
}
impl USBHS_3V_NOKR {
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
            USBHS_3V_NOKR::SUPPLY_3V_OK => false,
            USBHS_3V_NOKR::SUPPLY_3V_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBHS_3V_NOKR {
        match value {
            false => USBHS_3V_NOKR::SUPPLY_3V_OK,
            true => USBHS_3V_NOKR::SUPPLY_3V_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLY_3V_OK`"]
    #[inline]
    pub fn is_supply_3v_ok(&self) -> bool {
        *self == USBHS_3V_NOKR::SUPPLY_3V_OK
    }
    #[doc = "Checks if the value of the field is `SUPPLY_3V_LOW`"]
    #[inline]
    pub fn is_supply_3v_low(&self) -> bool {
        *self == USBHS_3V_NOKR::SUPPLY_3V_LOW
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB_HS: Low voltage detection on 3.3V supply."]
    #[inline]
    pub fn usbhs_3v_nok(&self) -> USBHS_3V_NOKR {
        USBHS_3V_NOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
}
