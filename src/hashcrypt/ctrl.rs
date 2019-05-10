#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `Mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "SHA1 is enabled"]
    SHA1,
    #[doc = "SHA2-256 is enabled"]
    SHA2_256,
    #[doc = "SHA2-512 is enabled (if available)"]
    SHA2_512,
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    AES,
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    ICB_AES,
    #[doc = "Salsa20/20 if available (including XSalsa - see also CRYPTCFG register)"]
    SALSA20,
    #[doc = "ChaCha20 if available (see also CRYPTCFG register for more controls)"]
    CHACHA20,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::DISABLED => 0,
            MODER::SHA1 => 1,
            MODER::SHA2_256 => 2,
            MODER::SHA2_512 => 3,
            MODER::AES => 4,
            MODER::ICB_AES => 5,
            MODER::SALSA20 => 6,
            MODER::CHACHA20 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::DISABLED,
            1 => MODER::SHA1,
            2 => MODER::SHA2_256,
            3 => MODER::SHA2_512,
            4 => MODER::AES,
            5 => MODER::ICB_AES,
            6 => MODER::SALSA20,
            7 => MODER::CHACHA20,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline]
    pub fn is_sha1(&self) -> bool {
        *self == MODER::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA2_256`"]
    #[inline]
    pub fn is_sha2_256(&self) -> bool {
        *self == MODER::SHA2_256
    }
    #[doc = "Checks if the value of the field is `SHA2_512`"]
    #[inline]
    pub fn is_sha2_512(&self) -> bool {
        *self == MODER::SHA2_512
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline]
    pub fn is_aes(&self) -> bool {
        *self == MODER::AES
    }
    #[doc = "Checks if the value of the field is `ICB_AES`"]
    #[inline]
    pub fn is_icb_aes(&self) -> bool {
        *self == MODER::ICB_AES
    }
    #[doc = "Checks if the value of the field is `SALSA20`"]
    #[inline]
    pub fn is_salsa20(&self) -> bool {
        *self == MODER::SALSA20
    }
    #[doc = "Checks if the value of the field is `CHACHA20`"]
    #[inline]
    pub fn is_chacha20(&self) -> bool {
        *self == MODER::CHACHA20
    }
}
#[doc = "Possible values of the field `New_Hash`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEW_HASHR {
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    START,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl NEW_HASHR {
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
            NEW_HASHR::START => true,
            NEW_HASHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEW_HASHR {
        match value {
            true => NEW_HASHR::START,
            i => NEW_HASHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == NEW_HASHR::START
    }
}
#[doc = "Possible values of the field `DMA_I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_IR {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED,
    #[doc = "DMA will push in the data."]
    PUSH,
}
impl DMA_IR {
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
            DMA_IR::NOT_USED => false,
            DMA_IR::PUSH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_IR {
        match value {
            false => DMA_IR::NOT_USED,
            true => DMA_IR::PUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline]
    pub fn is_not_used(&self) -> bool {
        *self == DMA_IR::NOT_USED
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline]
    pub fn is_push(&self) -> bool {
        *self == DMA_IR::PUSH
    }
}
#[doc = "Possible values of the field `DMA_O`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_OR {
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_OR {
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
            DMA_OR::NOTUSED => false,
            DMA_OR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_OR {
        match value {
            false => DMA_OR::NOTUSED,
            i => DMA_OR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTUSED`"]
    #[inline]
    pub fn is_notused(&self) -> bool {
        *self == DMA_OR::NOTUSED
    }
}
#[doc = r" Value of the field"]
pub struct HASHSWPBR {
    bits: bool,
}
impl HASHSWPBR {
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
#[doc = "Values that can be written to the field `Mode`"]
pub enum MODEW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "SHA1 is enabled"]
    SHA1,
    #[doc = "SHA2-256 is enabled"]
    SHA2_256,
    #[doc = "SHA2-512 is enabled (if available)"]
    SHA2_512,
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    AES,
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    ICB_AES,
    #[doc = "Salsa20/20 if available (including XSalsa - see also CRYPTCFG register)"]
    SALSA20,
    #[doc = "ChaCha20 if available (see also CRYPTCFG register for more controls)"]
    CHACHA20,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::DISABLED => 0,
            MODEW::SHA1 => 1,
            MODEW::SHA2_256 => 2,
            MODEW::SHA2_512 => 3,
            MODEW::AES => 4,
            MODEW::ICB_AES => 5,
            MODEW::SALSA20 => 6,
            MODEW::CHACHA20 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODEW::DISABLED)
    }
    #[doc = "SHA1 is enabled"]
    #[inline]
    pub fn sha1(self) -> &'a mut W {
        self.variant(MODEW::SHA1)
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline]
    pub fn sha2_256(self) -> &'a mut W {
        self.variant(MODEW::SHA2_256)
    }
    #[doc = "SHA2-512 is enabled (if available)"]
    #[inline]
    pub fn sha2_512(self) -> &'a mut W {
        self.variant(MODEW::SHA2_512)
    }
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    #[inline]
    pub fn aes(self) -> &'a mut W {
        self.variant(MODEW::AES)
    }
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    #[inline]
    pub fn icb_aes(self) -> &'a mut W {
        self.variant(MODEW::ICB_AES)
    }
    #[doc = "Salsa20/20 if available (including XSalsa - see also CRYPTCFG register)"]
    #[inline]
    pub fn salsa20(self) -> &'a mut W {
        self.variant(MODEW::SALSA20)
    }
    #[doc = "ChaCha20 if available (see also CRYPTCFG register for more controls)"]
    #[inline]
    pub fn chacha20(self) -> &'a mut W {
        self.variant(MODEW::CHACHA20)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `New_Hash`"]
pub enum NEW_HASHW {
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    START,
}
impl NEW_HASHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEW_HASHW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEW_HASHW<'a> {
    w: &'a mut W,
}
impl<'a> _NEW_HASHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEW_HASHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(NEW_HASHW::START)
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
#[doc = "Values that can be written to the field `DMA_I`"]
pub enum DMA_IW {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED,
    #[doc = "DMA will push in the data."]
    PUSH,
}
impl DMA_IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_IW::NOT_USED => false,
            DMA_IW::PUSH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_IW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMA_IW::NOT_USED)
    }
    #[doc = "DMA will push in the data."]
    #[inline]
    pub fn push(self) -> &'a mut W {
        self.variant(DMA_IW::PUSH)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMA_O`"]
pub enum DMA_OW {
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED,
}
impl DMA_OW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMA_OW::NOTUSED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMA_OW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_OW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_OW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline]
    pub fn notused(self) -> &'a mut W {
        self.variant(DMA_OW::NOTUSED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HASHSWPBW<'a> {
    w: &'a mut W,
}
impl<'a> _HASHSWPBW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline]
    pub fn new_hash(&self) -> NEW_HASHR {
        NEW_HASHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline]
    pub fn dma_i(&self) -> DMA_IR {
        DMA_IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline]
    pub fn dma_o(&self) -> DMA_OR {
        DMA_OR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline]
    pub fn hashswpb(&self) -> HASHSWPBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HASHSWPBR { bits }
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
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 4 - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline]
    pub fn new_hash(&mut self) -> _NEW_HASHW {
        _NEW_HASHW { w: self }
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline]
    pub fn dma_i(&mut self) -> _DMA_IW {
        _DMA_IW { w: self }
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline]
    pub fn dma_o(&mut self) -> _DMA_OW {
        _DMA_OW { w: self }
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline]
    pub fn hashswpb(&mut self) -> _HASHSWPBW {
        _HASHSWPBW { w: self }
    }
}
