#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL_SET {
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
pub struct ENHOSTDISCONDETECTR {
    bits: bool,
}
impl ENHOSTDISCONDETECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HOSTDISCONDETECT_IRQR {
    bits: bool,
}
impl HOSTDISCONDETECT_IRQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ENDEVPLUGINDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEVPLUGINDETR {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1,
}
impl ENDEVPLUGINDETR {
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
            ENDEVPLUGINDETR::VALUE0 => false,
            ENDEVPLUGINDETR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEVPLUGINDETR {
        match value {
            false => ENDEVPLUGINDETR::VALUE0,
            true => ENDEVPLUGINDETR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == ENDEVPLUGINDETR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ENDEVPLUGINDETR::VALUE1
    }
}
#[doc = r" Value of the field"]
pub struct DEVPLUGIN_IRQR {
    bits: bool,
}
impl DEVPLUGIN_IRQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENUTMILEVEL2R {
    bits: bool,
}
impl ENUTMILEVEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENUTMILEVEL3R {
    bits: bool,
}
impl ENUTMILEVEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct AUTORESUME_ENR {
    bits: bool,
}
impl AUTORESUME_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENAUTOCLR_CLKGATER {
    bits: bool,
}
impl ENAUTOCLR_CLKGATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct ENAUTOCLR_PHY_PWDR {
    bits: bool,
}
impl ENAUTOCLR_PHY_PWDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct FSDLL_RST_ENR {
    bits: bool,
}
impl FSDLL_RST_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct OTG_ID_VALUER {
    bits: bool,
}
impl OTG_ID_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HOST_FORCE_LS_SE0R {
    bits: bool,
}
impl HOST_FORCE_LS_SE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct UTMI_SUSPENDMR {
    bits: bool,
}
impl UTMI_SUSPENDMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SFTRSTR {
    bits: bool,
}
impl SFTRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _ENHOSTDISCONDETECTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENHOSTDISCONDETECTW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOSTDISCONDETECT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTDISCONDETECT_IRQW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENDEVPLUGINDET`"]
pub enum ENDEVPLUGINDETW {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1,
}
impl ENDEVPLUGINDETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEVPLUGINDETW::VALUE0 => false,
            ENDEVPLUGINDETW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEVPLUGINDETW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEVPLUGINDETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEVPLUGINDETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDETW::VALUE0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDETW::VALUE1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEVPLUGIN_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVPLUGIN_IRQW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENUTMILEVEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUTMILEVEL2W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENUTMILEVEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _ENUTMILEVEL3W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTORESUME_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTORESUME_ENW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENAUTOCLR_CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAUTOCLR_CLKGATEW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENAUTOCLR_PHY_PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAUTOCLR_PHY_PWDW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSDLL_RST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDLL_RST_ENW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OTG_ID_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG_ID_VALUEW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOST_FORCE_LS_SE0W<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_FORCE_LS_SE0W<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UTMI_SUSPENDMW<'a> {
    w: &'a mut W,
}
impl<'a> _UTMI_SUSPENDMW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTW<'a> {
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
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENHOSTDISCONDETECTR { bits }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HOSTDISCONDETECT_IRQR { bits }
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline]
    pub fn endevplugindet(&self) -> ENDEVPLUGINDETR {
        ENDEVPLUGINDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEVPLUGIN_IRQR { bits }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENUTMILEVEL2R { bits }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENUTMILEVEL3R { bits }
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline]
    pub fn autoresume_en(&self) -> AUTORESUME_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTORESUME_ENR { bits }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAUTOCLR_CLKGATER { bits }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENAUTOCLR_PHY_PWDR { bits }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSDLL_RST_ENR { bits }
    }
    #[doc = "Bit 27 - Indicates the results of USB_ID pin while monitoring the cable plugged into the Micro- or Mini-AB receptacle"]
    #[inline]
    pub fn otg_id_value(&self) -> OTG_ID_VALUER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTG_ID_VALUER { bits }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HOST_FORCE_LS_SE0R { bits }
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMI_SUSPENDMR { bits }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline]
    pub fn sftrst(&self) -> SFTRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3221225472 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline]
    pub fn enhostdiscondetect(&mut self) -> _ENHOSTDISCONDETECTW {
        _ENHOSTDISCONDETECTW { w: self }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline]
    pub fn hostdiscondetect_irq(&mut self) -> _HOSTDISCONDETECT_IRQW {
        _HOSTDISCONDETECT_IRQW { w: self }
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline]
    pub fn endevplugindet(&mut self) -> _ENDEVPLUGINDETW {
        _ENDEVPLUGINDETW { w: self }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline]
    pub fn devplugin_irq(&mut self) -> _DEVPLUGIN_IRQW {
        _DEVPLUGIN_IRQW { w: self }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline]
    pub fn enutmilevel2(&mut self) -> _ENUTMILEVEL2W {
        _ENUTMILEVEL2W { w: self }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline]
    pub fn enutmilevel3(&mut self) -> _ENUTMILEVEL3W {
        _ENUTMILEVEL3W { w: self }
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline]
    pub fn autoresume_en(&mut self) -> _AUTORESUME_ENW {
        _AUTORESUME_ENW { w: self }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_clkgate(&mut self) -> _ENAUTOCLR_CLKGATEW {
        _ENAUTOCLR_CLKGATEW { w: self }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline]
    pub fn enautoclr_phy_pwd(&mut self) -> _ENAUTOCLR_PHY_PWDW {
        _ENAUTOCLR_PHY_PWDW { w: self }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline]
    pub fn fsdll_rst_en(&mut self) -> _FSDLL_RST_ENW {
        _FSDLL_RST_ENW { w: self }
    }
    #[doc = "Bit 27 - Indicates the results of USB_ID pin while monitoring the cable plugged into the Micro- or Mini-AB receptacle"]
    #[inline]
    pub fn otg_id_value(&mut self) -> _OTG_ID_VALUEW {
        _OTG_ID_VALUEW { w: self }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline]
    pub fn host_force_ls_se0(&mut self) -> _HOST_FORCE_LS_SE0W {
        _HOST_FORCE_LS_SE0W { w: self }
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline]
    pub fn utmi_suspendm(&mut self) -> _UTMI_SUSPENDMW {
        _UTMI_SUSPENDMW { w: self }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline]
    pub fn sftrst(&mut self) -> _SFTRSTW {
        _SFTRSTW { w: self }
    }
}
