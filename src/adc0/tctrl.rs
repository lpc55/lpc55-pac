#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCTRL {
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
#[doc = "Possible values of the field `HTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTENR {
    #[doc = "Hardware trigger source disabled"]
    HTEN_0,
    #[doc = "Hardware trigger source enabled"]
    HTEN_1,
}
impl HTENR {
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
            HTENR::HTEN_0 => false,
            HTENR::HTEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTENR {
        match value {
            false => HTENR::HTEN_0,
            true => HTENR::HTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HTEN_0`"]
    #[inline]
    pub fn is_hten_0(&self) -> bool {
        *self == HTENR::HTEN_0
    }
    #[doc = "Checks if the value of the field is `HTEN_1`"]
    #[inline]
    pub fn is_hten_1(&self) -> bool {
        *self == HTENR::HTEN_1
    }
}
#[doc = "Possible values of the field `FIFO_SEL_A`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_SEL_AR {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_A_0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_A_1,
}
impl FIFO_SEL_AR {
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
            FIFO_SEL_AR::FIFO_SEL_A_0 => false,
            FIFO_SEL_AR::FIFO_SEL_A_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_SEL_AR {
        match value {
            false => FIFO_SEL_AR::FIFO_SEL_A_0,
            true => FIFO_SEL_AR::FIFO_SEL_A_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_A_0`"]
    #[inline]
    pub fn is_fifo_sel_a_0(&self) -> bool {
        *self == FIFO_SEL_AR::FIFO_SEL_A_0
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_A_1`"]
    #[inline]
    pub fn is_fifo_sel_a_1(&self) -> bool {
        *self == FIFO_SEL_AR::FIFO_SEL_A_1
    }
}
#[doc = "Possible values of the field `FIFO_SEL_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_SEL_BR {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_B_0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_B_1,
}
impl FIFO_SEL_BR {
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
            FIFO_SEL_BR::FIFO_SEL_B_0 => false,
            FIFO_SEL_BR::FIFO_SEL_B_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIFO_SEL_BR {
        match value {
            false => FIFO_SEL_BR::FIFO_SEL_B_0,
            true => FIFO_SEL_BR::FIFO_SEL_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_B_0`"]
    #[inline]
    pub fn is_fifo_sel_b_0(&self) -> bool {
        *self == FIFO_SEL_BR::FIFO_SEL_B_0
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_B_1`"]
    #[inline]
    pub fn is_fifo_sel_b_1(&self) -> bool {
        *self == FIFO_SEL_BR::FIFO_SEL_B_1
    }
}
#[doc = "Possible values of the field `TPRI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRIR {
    #[doc = "Set to highest priority, Level 1"]
    TPRI_0,
    #[doc = "Set to corresponding priority level"]
    TPRI_1,
    #[doc = "Set to corresponding priority level"]
    TPRI_2,
    #[doc = "Set to corresponding priority level"]
    TPRI_3,
    #[doc = "Set to corresponding priority level"]
    TPRI_4,
    #[doc = "Set to corresponding priority level"]
    TPRI_5,
    #[doc = "Set to corresponding priority level"]
    TPRI_6,
    #[doc = "Set to corresponding priority level"]
    TPRI_7,
    #[doc = "Set to corresponding priority level"]
    TPRI_8,
    #[doc = "Set to corresponding priority level"]
    TPRI_9,
    #[doc = "Set to lowest priority, Level 16"]
    TPRI_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPRIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPRIR::TPRI_0 => 0,
            TPRIR::TPRI_1 => 1,
            TPRIR::TPRI_2 => 2,
            TPRIR::TPRI_3 => 3,
            TPRIR::TPRI_4 => 4,
            TPRIR::TPRI_5 => 5,
            TPRIR::TPRI_6 => 6,
            TPRIR::TPRI_7 => 7,
            TPRIR::TPRI_8 => 8,
            TPRIR::TPRI_9 => 9,
            TPRIR::TPRI_15 => 15,
            TPRIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPRIR {
        match value {
            0 => TPRIR::TPRI_0,
            1 => TPRIR::TPRI_1,
            2 => TPRIR::TPRI_2,
            3 => TPRIR::TPRI_3,
            4 => TPRIR::TPRI_4,
            5 => TPRIR::TPRI_5,
            6 => TPRIR::TPRI_6,
            7 => TPRIR::TPRI_7,
            8 => TPRIR::TPRI_8,
            9 => TPRIR::TPRI_9,
            15 => TPRIR::TPRI_15,
            i => TPRIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TPRI_0`"]
    #[inline]
    pub fn is_tpri_0(&self) -> bool {
        *self == TPRIR::TPRI_0
    }
    #[doc = "Checks if the value of the field is `TPRI_1`"]
    #[inline]
    pub fn is_tpri_1(&self) -> bool {
        *self == TPRIR::TPRI_1
    }
    #[doc = "Checks if the value of the field is `TPRI_2`"]
    #[inline]
    pub fn is_tpri_2(&self) -> bool {
        *self == TPRIR::TPRI_2
    }
    #[doc = "Checks if the value of the field is `TPRI_3`"]
    #[inline]
    pub fn is_tpri_3(&self) -> bool {
        *self == TPRIR::TPRI_3
    }
    #[doc = "Checks if the value of the field is `TPRI_4`"]
    #[inline]
    pub fn is_tpri_4(&self) -> bool {
        *self == TPRIR::TPRI_4
    }
    #[doc = "Checks if the value of the field is `TPRI_5`"]
    #[inline]
    pub fn is_tpri_5(&self) -> bool {
        *self == TPRIR::TPRI_5
    }
    #[doc = "Checks if the value of the field is `TPRI_6`"]
    #[inline]
    pub fn is_tpri_6(&self) -> bool {
        *self == TPRIR::TPRI_6
    }
    #[doc = "Checks if the value of the field is `TPRI_7`"]
    #[inline]
    pub fn is_tpri_7(&self) -> bool {
        *self == TPRIR::TPRI_7
    }
    #[doc = "Checks if the value of the field is `TPRI_8`"]
    #[inline]
    pub fn is_tpri_8(&self) -> bool {
        *self == TPRIR::TPRI_8
    }
    #[doc = "Checks if the value of the field is `TPRI_9`"]
    #[inline]
    pub fn is_tpri_9(&self) -> bool {
        *self == TPRIR::TPRI_9
    }
    #[doc = "Checks if the value of the field is `TPRI_15`"]
    #[inline]
    pub fn is_tpri_15(&self) -> bool {
        *self == TPRIR::TPRI_15
    }
}
#[doc = r" Value of the field"]
pub struct RSYNCR {
    bits: bool,
}
impl RSYNCR {
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
pub struct TDLYR {
    bits: u8,
}
impl TDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMDR {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0,
    #[doc = "CMD1 is executed"]
    TCMD_1,
    #[doc = "Corresponding CMD is executed"]
    TCMD_2,
    #[doc = "Corresponding CMD is executed"]
    TCMD_3,
    #[doc = "Corresponding CMD is executed"]
    TCMD_4,
    #[doc = "Corresponding CMD is executed"]
    TCMD_5,
    #[doc = "Corresponding CMD is executed"]
    TCMD_6,
    #[doc = "Corresponding CMD is executed"]
    TCMD_7,
    #[doc = "Corresponding CMD is executed"]
    TCMD_8,
    #[doc = "Corresponding CMD is executed"]
    TCMD_9,
    #[doc = "CMD15 is executed"]
    TCMD_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCMDR::TCMD_0 => 0,
            TCMDR::TCMD_1 => 1,
            TCMDR::TCMD_2 => 2,
            TCMDR::TCMD_3 => 3,
            TCMDR::TCMD_4 => 4,
            TCMDR::TCMD_5 => 5,
            TCMDR::TCMD_6 => 6,
            TCMDR::TCMD_7 => 7,
            TCMDR::TCMD_8 => 8,
            TCMDR::TCMD_9 => 9,
            TCMDR::TCMD_15 => 15,
            TCMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCMDR {
        match value {
            0 => TCMDR::TCMD_0,
            1 => TCMDR::TCMD_1,
            2 => TCMDR::TCMD_2,
            3 => TCMDR::TCMD_3,
            4 => TCMDR::TCMD_4,
            5 => TCMDR::TCMD_5,
            6 => TCMDR::TCMD_6,
            7 => TCMDR::TCMD_7,
            8 => TCMDR::TCMD_8,
            9 => TCMDR::TCMD_9,
            15 => TCMDR::TCMD_15,
            i => TCMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCMD_0`"]
    #[inline]
    pub fn is_tcmd_0(&self) -> bool {
        *self == TCMDR::TCMD_0
    }
    #[doc = "Checks if the value of the field is `TCMD_1`"]
    #[inline]
    pub fn is_tcmd_1(&self) -> bool {
        *self == TCMDR::TCMD_1
    }
    #[doc = "Checks if the value of the field is `TCMD_2`"]
    #[inline]
    pub fn is_tcmd_2(&self) -> bool {
        *self == TCMDR::TCMD_2
    }
    #[doc = "Checks if the value of the field is `TCMD_3`"]
    #[inline]
    pub fn is_tcmd_3(&self) -> bool {
        *self == TCMDR::TCMD_3
    }
    #[doc = "Checks if the value of the field is `TCMD_4`"]
    #[inline]
    pub fn is_tcmd_4(&self) -> bool {
        *self == TCMDR::TCMD_4
    }
    #[doc = "Checks if the value of the field is `TCMD_5`"]
    #[inline]
    pub fn is_tcmd_5(&self) -> bool {
        *self == TCMDR::TCMD_5
    }
    #[doc = "Checks if the value of the field is `TCMD_6`"]
    #[inline]
    pub fn is_tcmd_6(&self) -> bool {
        *self == TCMDR::TCMD_6
    }
    #[doc = "Checks if the value of the field is `TCMD_7`"]
    #[inline]
    pub fn is_tcmd_7(&self) -> bool {
        *self == TCMDR::TCMD_7
    }
    #[doc = "Checks if the value of the field is `TCMD_8`"]
    #[inline]
    pub fn is_tcmd_8(&self) -> bool {
        *self == TCMDR::TCMD_8
    }
    #[doc = "Checks if the value of the field is `TCMD_9`"]
    #[inline]
    pub fn is_tcmd_9(&self) -> bool {
        *self == TCMDR::TCMD_9
    }
    #[doc = "Checks if the value of the field is `TCMD_15`"]
    #[inline]
    pub fn is_tcmd_15(&self) -> bool {
        *self == TCMDR::TCMD_15
    }
}
#[doc = "Values that can be written to the field `HTEN`"]
pub enum HTENW {
    #[doc = "Hardware trigger source disabled"]
    HTEN_0,
    #[doc = "Hardware trigger source enabled"]
    HTEN_1,
}
impl HTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HTENW::HTEN_0 => false,
            HTENW::HTEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HTENW<'a> {
    w: &'a mut W,
}
impl<'a> _HTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware trigger source disabled"]
    #[inline]
    pub fn hten_0(self) -> &'a mut W {
        self.variant(HTENW::HTEN_0)
    }
    #[doc = "Hardware trigger source enabled"]
    #[inline]
    pub fn hten_1(self) -> &'a mut W {
        self.variant(HTENW::HTEN_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIFO_SEL_A`"]
pub enum FIFO_SEL_AW {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_A_0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_A_1,
}
impl FIFO_SEL_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFO_SEL_AW::FIFO_SEL_A_0 => false,
            FIFO_SEL_AW::FIFO_SEL_A_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_SEL_AW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_SEL_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFO_SEL_AW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result written to FIFO 0"]
    #[inline]
    pub fn fifo_sel_a_0(self) -> &'a mut W {
        self.variant(FIFO_SEL_AW::FIFO_SEL_A_0)
    }
    #[doc = "Result written to FIFO 1"]
    #[inline]
    pub fn fifo_sel_a_1(self) -> &'a mut W {
        self.variant(FIFO_SEL_AW::FIFO_SEL_A_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIFO_SEL_B`"]
pub enum FIFO_SEL_BW {
    #[doc = "Result written to FIFO 0"]
    FIFO_SEL_B_0,
    #[doc = "Result written to FIFO 1"]
    FIFO_SEL_B_1,
}
impl FIFO_SEL_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFO_SEL_BW::FIFO_SEL_B_0 => false,
            FIFO_SEL_BW::FIFO_SEL_B_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_SEL_BW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_SEL_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFO_SEL_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Result written to FIFO 0"]
    #[inline]
    pub fn fifo_sel_b_0(self) -> &'a mut W {
        self.variant(FIFO_SEL_BW::FIFO_SEL_B_0)
    }
    #[doc = "Result written to FIFO 1"]
    #[inline]
    pub fn fifo_sel_b_1(self) -> &'a mut W {
        self.variant(FIFO_SEL_BW::FIFO_SEL_B_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPRI`"]
pub enum TPRIW {
    #[doc = "Set to highest priority, Level 1"]
    TPRI_0,
    #[doc = "Set to corresponding priority level"]
    TPRI_1,
    #[doc = "Set to corresponding priority level"]
    TPRI_2,
    #[doc = "Set to corresponding priority level"]
    TPRI_3,
    #[doc = "Set to corresponding priority level"]
    TPRI_4,
    #[doc = "Set to corresponding priority level"]
    TPRI_5,
    #[doc = "Set to corresponding priority level"]
    TPRI_6,
    #[doc = "Set to corresponding priority level"]
    TPRI_7,
    #[doc = "Set to corresponding priority level"]
    TPRI_8,
    #[doc = "Set to corresponding priority level"]
    TPRI_9,
    #[doc = "Set to lowest priority, Level 16"]
    TPRI_15,
}
impl TPRIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPRIW::TPRI_0 => 0,
            TPRIW::TPRI_1 => 1,
            TPRIW::TPRI_2 => 2,
            TPRIW::TPRI_3 => 3,
            TPRIW::TPRI_4 => 4,
            TPRIW::TPRI_5 => 5,
            TPRIW::TPRI_6 => 6,
            TPRIW::TPRI_7 => 7,
            TPRIW::TPRI_8 => 8,
            TPRIW::TPRI_9 => 9,
            TPRIW::TPRI_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPRIW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPRIW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set to highest priority, Level 1"]
    #[inline]
    pub fn tpri_0(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_0)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_1(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_1)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_2(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_2)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_3(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_3)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_4(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_4)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_5(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_5)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_6(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_6)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_7(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_7)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_8(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_8)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline]
    pub fn tpri_9(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_9)
    }
    #[doc = "Set to lowest priority, Level 16"]
    #[inline]
    pub fn tpri_15(self) -> &'a mut W {
        self.variant(TPRIW::TPRI_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSYNCW<'a> {
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
pub struct _TDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _TDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCMD`"]
pub enum TCMDW {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0,
    #[doc = "CMD1 is executed"]
    TCMD_1,
    #[doc = "Corresponding CMD is executed"]
    TCMD_2,
    #[doc = "Corresponding CMD is executed"]
    TCMD_3,
    #[doc = "Corresponding CMD is executed"]
    TCMD_4,
    #[doc = "Corresponding CMD is executed"]
    TCMD_5,
    #[doc = "Corresponding CMD is executed"]
    TCMD_6,
    #[doc = "Corresponding CMD is executed"]
    TCMD_7,
    #[doc = "Corresponding CMD is executed"]
    TCMD_8,
    #[doc = "Corresponding CMD is executed"]
    TCMD_9,
    #[doc = "CMD15 is executed"]
    TCMD_15,
}
impl TCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCMDW::TCMD_0 => 0,
            TCMDW::TCMD_1 => 1,
            TCMDW::TCMD_2 => 2,
            TCMDW::TCMD_3 => 3,
            TCMDW::TCMD_4 => 4,
            TCMDW::TCMD_5 => 5,
            TCMDW::TCMD_6 => 6,
            TCMDW::TCMD_7 => 7,
            TCMDW::TCMD_8 => 8,
            TCMDW::TCMD_9 => 9,
            TCMDW::TCMD_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _TCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    #[inline]
    pub fn tcmd_0(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_0)
    }
    #[doc = "CMD1 is executed"]
    #[inline]
    pub fn tcmd_1(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_1)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_2(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_2)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_3(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_3)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_4(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_4)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_5(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_5)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_6(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_6)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_7(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_7)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_8(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_8)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline]
    pub fn tcmd_9(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_9)
    }
    #[doc = "CMD15 is executed"]
    #[inline]
    pub fn tcmd_15(self) -> &'a mut W {
        self.variant(TCMDW::TCMD_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Trigger enable"]
    #[inline]
    pub fn hten(&self) -> HTENR {
        HTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline]
    pub fn fifo_sel_a(&self) -> FIFO_SEL_AR {
        FIFO_SEL_AR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SAR Result Destination For Channel B"]
    #[inline]
    pub fn fifo_sel_b(&self) -> FIFO_SEL_BR {
        FIFO_SEL_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline]
    pub fn tpri(&self) -> TPRIR {
        TPRIR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline]
    pub fn rsync(&self) -> RSYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSYNCR { bits }
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline]
    pub fn tdly(&self) -> TDLYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDLYR { bits }
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline]
    pub fn tcmd(&self) -> TCMDR {
        TCMDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Trigger enable"]
    #[inline]
    pub fn hten(&mut self) -> _HTENW {
        _HTENW { w: self }
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline]
    pub fn fifo_sel_a(&mut self) -> _FIFO_SEL_AW {
        _FIFO_SEL_AW { w: self }
    }
    #[doc = "Bit 2 - SAR Result Destination For Channel B"]
    #[inline]
    pub fn fifo_sel_b(&mut self) -> _FIFO_SEL_BW {
        _FIFO_SEL_BW { w: self }
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline]
    pub fn tpri(&mut self) -> _TPRIW {
        _TPRIW { w: self }
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline]
    pub fn rsync(&mut self) -> _RSYNCW {
        _RSYNCW { w: self }
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline]
    pub fn tdly(&mut self) -> _TDLYW {
        _TDLYW { w: self }
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline]
    pub fn tcmd(&mut self) -> _TCMDW {
        _TCMDW { w: self }
    }
}
