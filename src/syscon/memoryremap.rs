#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMORYREMAP {
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
#[doc = "Possible values of the field `MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAPR {
    #[doc = "Vector Table in ROM."]
    ROM0,
    #[doc = "Vector Table in RAM."]
    RAM1,
    #[doc = "Vector Table in Flash."]
    FLASH0,
    #[doc = "Vector Table in Flash."]
    FLASH1,
}
impl MAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAPR::ROM0 => 0,
            MAPR::RAM1 => 1,
            MAPR::FLASH0 => 2,
            MAPR::FLASH1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAPR {
        match value {
            0 => MAPR::ROM0,
            1 => MAPR::RAM1,
            2 => MAPR::FLASH0,
            3 => MAPR::FLASH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ROM0`"]
    #[inline]
    pub fn is_rom0(&self) -> bool {
        *self == MAPR::ROM0
    }
    #[doc = "Checks if the value of the field is `RAM1`"]
    #[inline]
    pub fn is_ram1(&self) -> bool {
        *self == MAPR::RAM1
    }
    #[doc = "Checks if the value of the field is `FLASH0`"]
    #[inline]
    pub fn is_flash0(&self) -> bool {
        *self == MAPR::FLASH0
    }
    #[doc = "Checks if the value of the field is `FLASH1`"]
    #[inline]
    pub fn is_flash1(&self) -> bool {
        *self == MAPR::FLASH1
    }
}
#[doc = "Values that can be written to the field `MAP`"]
pub enum MAPW {
    #[doc = "Vector Table in ROM."]
    ROM0,
    #[doc = "Vector Table in RAM."]
    RAM1,
    #[doc = "Vector Table in Flash."]
    FLASH0,
    #[doc = "Vector Table in Flash."]
    FLASH1,
}
impl MAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MAPW::ROM0 => 0,
            MAPW::RAM1 => 1,
            MAPW::FLASH0 => 2,
            MAPW::FLASH1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAPW<'a> {
    w: &'a mut W,
}
impl<'a> _MAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Vector Table in ROM."]
    #[inline]
    pub fn rom0(self) -> &'a mut W {
        self.variant(MAPW::ROM0)
    }
    #[doc = "Vector Table in RAM."]
    #[inline]
    pub fn ram1(self) -> &'a mut W {
        self.variant(MAPW::RAM1)
    }
    #[doc = "Vector Table in Flash."]
    #[inline]
    pub fn flash0(self) -> &'a mut W {
        self.variant(MAPW::FLASH0)
    }
    #[doc = "Vector Table in Flash."]
    #[inline]
    pub fn flash1(self) -> &'a mut W {
        self.variant(MAPW::FLASH1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline]
    pub fn map(&self) -> MAPR {
        MAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline]
    pub fn map(&mut self) -> _MAPW {
        _MAPW { w: self }
    }
}
