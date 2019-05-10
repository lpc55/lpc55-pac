#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL0SSCG1 {
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
pub struct MD_MBSR {
    bits: bool,
}
impl MD_MBSR {
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
pub struct MD_REQR {
    bits: bool,
}
impl MD_REQR {
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
pub struct MFR {
    bits: u8,
}
impl MFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MRR {
    bits: u8,
}
impl MRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCR {
    bits: u8,
}
impl MCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MDIV_EXTR {
    bits: u16,
}
impl MDIV_EXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MREQR {
    bits: bool,
}
impl MREQR {
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
pub struct DITHERR {
    bits: bool,
}
impl DITHERR {
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
pub struct SEL_EXTR {
    bits: bool,
}
impl SEL_EXTR {
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
pub struct _MD_MBSW<'a> {
    w: &'a mut W,
}
impl<'a> _MD_MBSW<'a> {
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
#[doc = r" Proxy"]
pub struct _MD_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _MD_REQW<'a> {
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
pub struct _MFW<'a> {
    w: &'a mut W,
}
impl<'a> _MFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MRW<'a> {
    w: &'a mut W,
}
impl<'a> _MRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCW<'a> {
    w: &'a mut W,
}
impl<'a> _MCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MDIV_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIV_EXTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MREQW<'a> {
    w: &'a mut W,
}
impl<'a> _MREQW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DITHERW<'a> {
    w: &'a mut W,
}
impl<'a> _DITHERW<'a> {
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
pub struct _SEL_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_EXTW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline]
    pub fn md_mbs(&self) -> MD_MBSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MD_MBSR { bits }
    }
    #[doc = "Bit 1 - md change request."]
    #[inline]
    pub fn md_req(&self) -> MD_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MD_REQR { bits }
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline]
    pub fn mf(&self) -> MFR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MFR { bits }
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline]
    pub fn mr(&self) -> MRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MRR { bits }
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline]
    pub fn mc(&self) -> MCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCR { bits }
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline]
    pub fn mdiv_ext(&self) -> MDIV_EXTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MDIV_EXTR { bits }
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline]
    pub fn mreq(&self) -> MREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MREQR { bits }
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline]
    pub fn dither(&self) -> DITHERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DITHERR { bits }
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline]
    pub fn sel_ext(&self) -> SEL_EXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEL_EXTR { bits }
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
    #[doc = "Bit 0 - input word of the wrapper bit 32."]
    #[inline]
    pub fn md_mbs(&mut self) -> _MD_MBSW {
        _MD_MBSW { w: self }
    }
    #[doc = "Bit 1 - md change request."]
    #[inline]
    pub fn md_req(&mut self) -> _MD_REQW {
        _MD_REQW { w: self }
    }
    #[doc = "Bits 2:4 - programmable modulation frequency fm = Fref/Nss mf\\[2:0\\] = 000 => Nss=512 (fm ~ 3."]
    #[inline]
    pub fn mf(&mut self) -> _MFW {
        _MFW { w: self }
    }
    #[doc = "Bits 5:7 - programmable frequency modulation depth Dfmodpk-pk = Fref*kss/Fcco = kss/(2*md\\[32:25\\]dec) mr\\[2:0\\] = 000 => kss = 0 (no spread spectrum) mr\\[2:0\\] = 001 => kss ~ 1 mr\\[2:0\\] = 010 => kss ~ 1."]
    #[inline]
    pub fn mr(&mut self) -> _MRW {
        _MRW { w: self }
    }
    #[doc = "Bits 8:9 - modulation waveform control Compensation for low pass filtering of the PLL to get a triangular modulation at the output of the PLL, giving a flat frequency spectrum."]
    #[inline]
    pub fn mc(&mut self) -> _MCW {
        _MCW { w: self }
    }
    #[doc = "Bits 10:25 - to select an external mdiv value."]
    #[inline]
    pub fn mdiv_ext(&mut self) -> _MDIV_EXTW {
        _MDIV_EXTW { w: self }
    }
    #[doc = "Bit 26 - to select an external mreq value."]
    #[inline]
    pub fn mreq(&mut self) -> _MREQW {
        _MREQW { w: self }
    }
    #[doc = "Bit 27 - dithering between two modulation frequencies in a random way or in a pseudo random way (white noise), in order to decrease the probability that the modulated waveform will occur with the same phase on a particular point on the screen."]
    #[inline]
    pub fn dither(&mut self) -> _DITHERW {
        _DITHERW { w: self }
    }
    #[doc = "Bit 28 - to select mdiv_ext and mreq_ext sel_ext = 0: mdiv ~ md\\[32:0\\], mreq = 1 sel_ext = 1 : mdiv = mdiv_ext, mreq = mreq_ext."]
    #[inline]
    pub fn sel_ext(&mut self) -> _SEL_EXTW {
        _SEL_EXTW { w: self }
    }
}
