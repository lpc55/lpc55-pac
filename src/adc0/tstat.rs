#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSTAT {
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
#[doc = "Possible values of the field `TEXC_NUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_NUMR {
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    TEXC_NUM_0,
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    TEXC_NUM_1,
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    TEXC_NUM_2,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_3,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_4,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_5,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_6,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_7,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_8,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_9,
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    TEXC_NUM_65535,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TEXC_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TEXC_NUMR::TEXC_NUM_0 => 0,
            TEXC_NUMR::TEXC_NUM_1 => 1,
            TEXC_NUMR::TEXC_NUM_2 => 2,
            TEXC_NUMR::TEXC_NUM_3 => 3,
            TEXC_NUMR::TEXC_NUM_4 => 4,
            TEXC_NUMR::TEXC_NUM_5 => 5,
            TEXC_NUMR::TEXC_NUM_6 => 6,
            TEXC_NUMR::TEXC_NUM_7 => 7,
            TEXC_NUMR::TEXC_NUM_8 => 8,
            TEXC_NUMR::TEXC_NUM_9 => 9,
            TEXC_NUMR::TEXC_NUM_65535 => 65535,
            TEXC_NUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TEXC_NUMR {
        match value {
            0 => TEXC_NUMR::TEXC_NUM_0,
            1 => TEXC_NUMR::TEXC_NUM_1,
            2 => TEXC_NUMR::TEXC_NUM_2,
            3 => TEXC_NUMR::TEXC_NUM_3,
            4 => TEXC_NUMR::TEXC_NUM_4,
            5 => TEXC_NUMR::TEXC_NUM_5,
            6 => TEXC_NUMR::TEXC_NUM_6,
            7 => TEXC_NUMR::TEXC_NUM_7,
            8 => TEXC_NUMR::TEXC_NUM_8,
            9 => TEXC_NUMR::TEXC_NUM_9,
            65535 => TEXC_NUMR::TEXC_NUM_65535,
            i => TEXC_NUMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_0`"]
    #[inline]
    pub fn is_texc_num_0(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_0
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_1`"]
    #[inline]
    pub fn is_texc_num_1(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_1
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_2`"]
    #[inline]
    pub fn is_texc_num_2(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_2
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_3`"]
    #[inline]
    pub fn is_texc_num_3(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_3
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_4`"]
    #[inline]
    pub fn is_texc_num_4(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_4
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_5`"]
    #[inline]
    pub fn is_texc_num_5(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_5
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_6`"]
    #[inline]
    pub fn is_texc_num_6(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_6
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_7`"]
    #[inline]
    pub fn is_texc_num_7(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_7
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_8`"]
    #[inline]
    pub fn is_texc_num_8(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_8
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_9`"]
    #[inline]
    pub fn is_texc_num_9(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_9
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_65535`"]
    #[inline]
    pub fn is_texc_num_65535(&self) -> bool {
        *self == TEXC_NUMR::TEXC_NUM_65535
    }
}
#[doc = "Possible values of the field `TCOMP_FLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCOMP_FLAGR {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    TCOMP_FLAG_0,
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    TCOMP_FLAG_1,
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    TCOMP_FLAG_2,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_3,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_4,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_5,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_6,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_7,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_8,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_9,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    TCOMP_FLAG_65535,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TCOMP_FLAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TCOMP_FLAGR::TCOMP_FLAG_0 => 0,
            TCOMP_FLAGR::TCOMP_FLAG_1 => 1,
            TCOMP_FLAGR::TCOMP_FLAG_2 => 2,
            TCOMP_FLAGR::TCOMP_FLAG_3 => 3,
            TCOMP_FLAGR::TCOMP_FLAG_4 => 4,
            TCOMP_FLAGR::TCOMP_FLAG_5 => 5,
            TCOMP_FLAGR::TCOMP_FLAG_6 => 6,
            TCOMP_FLAGR::TCOMP_FLAG_7 => 7,
            TCOMP_FLAGR::TCOMP_FLAG_8 => 8,
            TCOMP_FLAGR::TCOMP_FLAG_9 => 9,
            TCOMP_FLAGR::TCOMP_FLAG_65535 => 65535,
            TCOMP_FLAGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TCOMP_FLAGR {
        match value {
            0 => TCOMP_FLAGR::TCOMP_FLAG_0,
            1 => TCOMP_FLAGR::TCOMP_FLAG_1,
            2 => TCOMP_FLAGR::TCOMP_FLAG_2,
            3 => TCOMP_FLAGR::TCOMP_FLAG_3,
            4 => TCOMP_FLAGR::TCOMP_FLAG_4,
            5 => TCOMP_FLAGR::TCOMP_FLAG_5,
            6 => TCOMP_FLAGR::TCOMP_FLAG_6,
            7 => TCOMP_FLAGR::TCOMP_FLAG_7,
            8 => TCOMP_FLAGR::TCOMP_FLAG_8,
            9 => TCOMP_FLAGR::TCOMP_FLAG_9,
            65535 => TCOMP_FLAGR::TCOMP_FLAG_65535,
            i => TCOMP_FLAGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_0`"]
    #[inline]
    pub fn is_tcomp_flag_0(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_1`"]
    #[inline]
    pub fn is_tcomp_flag_1(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_2`"]
    #[inline]
    pub fn is_tcomp_flag_2(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_3`"]
    #[inline]
    pub fn is_tcomp_flag_3(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_4`"]
    #[inline]
    pub fn is_tcomp_flag_4(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_5`"]
    #[inline]
    pub fn is_tcomp_flag_5(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_6`"]
    #[inline]
    pub fn is_tcomp_flag_6(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_7`"]
    #[inline]
    pub fn is_tcomp_flag_7(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_8`"]
    #[inline]
    pub fn is_tcomp_flag_8(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_9`"]
    #[inline]
    pub fn is_tcomp_flag_9(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_65535`"]
    #[inline]
    pub fn is_tcomp_flag_65535(&self) -> bool {
        *self == TCOMP_FLAGR::TCOMP_FLAG_65535
    }
}
#[doc = "Values that can be written to the field `TEXC_NUM`"]
pub enum TEXC_NUMW {
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    TEXC_NUM_0,
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    TEXC_NUM_1,
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    TEXC_NUM_2,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_3,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_4,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_5,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_6,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_7,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_8,
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_9,
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    TEXC_NUM_65535,
}
impl TEXC_NUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            TEXC_NUMW::TEXC_NUM_0 => 0,
            TEXC_NUMW::TEXC_NUM_1 => 1,
            TEXC_NUMW::TEXC_NUM_2 => 2,
            TEXC_NUMW::TEXC_NUM_3 => 3,
            TEXC_NUMW::TEXC_NUM_4 => 4,
            TEXC_NUMW::TEXC_NUM_5 => 5,
            TEXC_NUMW::TEXC_NUM_6 => 6,
            TEXC_NUMW::TEXC_NUM_7 => 7,
            TEXC_NUMW::TEXC_NUM_8 => 8,
            TEXC_NUMW::TEXC_NUM_9 => 9,
            TEXC_NUMW::TEXC_NUM_65535 => 65535,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEXC_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _TEXC_NUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEXC_NUMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\] = 1."]
    #[inline]
    pub fn texc_num_0(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_0)
    }
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_1(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_1)
    }
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_2(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_2)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_3(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_3)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_4(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_4)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_5(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_5)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_6(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_6)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_7(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_7)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_8(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_8)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_9(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_9)
    }
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    #[inline]
    pub fn texc_num_65535(self) -> &'a mut W {
        self.variant(TEXC_NUMW::TEXC_NUM_65535)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCOMP_FLAG`"]
pub enum TCOMP_FLAGW {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    TCOMP_FLAG_0,
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    TCOMP_FLAG_1,
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    TCOMP_FLAG_2,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_3,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_4,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_5,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_6,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_7,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_8,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_9,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    TCOMP_FLAG_65535,
}
impl TCOMP_FLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            TCOMP_FLAGW::TCOMP_FLAG_0 => 0,
            TCOMP_FLAGW::TCOMP_FLAG_1 => 1,
            TCOMP_FLAGW::TCOMP_FLAG_2 => 2,
            TCOMP_FLAGW::TCOMP_FLAG_3 => 3,
            TCOMP_FLAGW::TCOMP_FLAG_4 => 4,
            TCOMP_FLAGW::TCOMP_FLAG_5 => 5,
            TCOMP_FLAGW::TCOMP_FLAG_6 => 6,
            TCOMP_FLAGW::TCOMP_FLAG_7 => 7,
            TCOMP_FLAGW::TCOMP_FLAG_8 => 8,
            TCOMP_FLAGW::TCOMP_FLAG_9 => 9,
            TCOMP_FLAGW::TCOMP_FLAG_65535 => 65535,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCOMP_FLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TCOMP_FLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCOMP_FLAGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    #[inline]
    pub fn tcomp_flag_0(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_0)
    }
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_1(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_1)
    }
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_2(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_2)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_3(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_3)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_4(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_4)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_5(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_5)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_6(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_6)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_7(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_7)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_8(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_8)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_9(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_9)
    }
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    #[inline]
    pub fn tcomp_flag_65535(self) -> &'a mut W {
        self.variant(TCOMP_FLAGW::TCOMP_FLAG_65535)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline]
    pub fn texc_num(&self) -> TEXC_NUMR {
        TEXC_NUMR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline]
    pub fn tcomp_flag(&self) -> TCOMP_FLAGR {
        TCOMP_FLAGR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline]
    pub fn texc_num(&mut self) -> _TEXC_NUMW {
        _TEXC_NUMW { w: self }
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline]
    pub fn tcomp_flag(&mut self) -> _TCOMP_FLAGW {
        _TCOMP_FLAGW { w: self }
    }
}
