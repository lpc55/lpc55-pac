#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
pub struct FIFO_RX_WATERMARKR {
    bits: bool,
}
impl FIFO_RX_WATERMARKR {
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
pub struct FIFO_TX_WATERMARKR {
    bits: bool,
}
impl FIFO_TX_WATERMARKR {
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
pub struct FIFO_EMPTYR {
    bits: bool,
}
impl FIFO_EMPTYR {
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
pub struct FIFO_FULLR {
    bits: bool,
}
impl FIFO_FULLR {
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
pub struct CMDFSMSTATESR {
    bits: u8,
}
impl CMDFSMSTATESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_3_STATUSR {
    bits: bool,
}
impl DATA_3_STATUSR {
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
pub struct DATA_BUSYR {
    bits: bool,
}
impl DATA_BUSYR {
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
pub struct DATA_STATE_MC_BUSYR {
    bits: bool,
}
impl DATA_STATE_MC_BUSYR {
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
pub struct RESPONSE_INDEXR {
    bits: u8,
}
impl RESPONSE_INDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFO_COUNTR {
    bits: u16,
}
impl FIFO_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMA_ACKR {
    bits: bool,
}
impl DMA_ACKR {
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
pub struct DMA_REQR {
    bits: bool,
}
impl DMA_REQR {
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
pub struct _FIFO_RX_WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_RX_WATERMARKW<'a> {
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
pub struct _FIFO_TX_WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_TX_WATERMARKW<'a> {
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
pub struct _FIFO_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_EMPTYW<'a> {
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
#[doc = r" Proxy"]
pub struct _FIFO_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_FULLW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDFSMSTATESW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDFSMSTATESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_3_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_3_STATUSW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATA_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BUSYW<'a> {
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
pub struct _DATA_STATE_MC_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_STATE_MC_BUSYW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_INDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_COUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ACKW<'a> {
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
pub struct _DMA_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REQW<'a> {
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
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFO_RX_WATERMARKR { bits }
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFO_TX_WATERMARKR { bits }
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline]
    pub fn fifo_empty(&self) -> FIFO_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFO_EMPTYR { bits }
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline]
    pub fn fifo_full(&self) -> FIFO_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FIFO_FULLR { bits }
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline]
    pub fn cmdfsmstates(&self) -> CMDFSMSTATESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDFSMSTATESR { bits }
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline]
    pub fn data_3_status(&self) -> DATA_3_STATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_3_STATUSR { bits }
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline]
    pub fn data_busy(&self) -> DATA_BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_BUSYR { bits }
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_STATE_MC_BUSYR { bits }
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline]
    pub fn response_index(&self) -> RESPONSE_INDEXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESPONSE_INDEXR { bits }
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline]
    pub fn fifo_count(&self) -> FIFO_COUNTR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FIFO_COUNTR { bits }
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline]
    pub fn dma_ack(&self) -> DMA_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_ACKR { bits }
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline]
    pub fn dma_req(&self) -> DMA_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMA_REQR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1030 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline]
    pub fn fifo_rx_watermark(&mut self) -> _FIFO_RX_WATERMARKW {
        _FIFO_RX_WATERMARKW { w: self }
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline]
    pub fn fifo_tx_watermark(&mut self) -> _FIFO_TX_WATERMARKW {
        _FIFO_TX_WATERMARKW { w: self }
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline]
    pub fn fifo_empty(&mut self) -> _FIFO_EMPTYW {
        _FIFO_EMPTYW { w: self }
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline]
    pub fn fifo_full(&mut self) -> _FIFO_FULLW {
        _FIFO_FULLW { w: self }
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline]
    pub fn cmdfsmstates(&mut self) -> _CMDFSMSTATESW {
        _CMDFSMSTATESW { w: self }
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline]
    pub fn data_3_status(&mut self) -> _DATA_3_STATUSW {
        _DATA_3_STATUSW { w: self }
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline]
    pub fn data_busy(&mut self) -> _DATA_BUSYW {
        _DATA_BUSYW { w: self }
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline]
    pub fn data_state_mc_busy(&mut self) -> _DATA_STATE_MC_BUSYW {
        _DATA_STATE_MC_BUSYW { w: self }
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline]
    pub fn response_index(&mut self) -> _RESPONSE_INDEXW {
        _RESPONSE_INDEXW { w: self }
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline]
    pub fn fifo_count(&mut self) -> _FIFO_COUNTW {
        _FIFO_COUNTW { w: self }
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline]
    pub fn dma_ack(&mut self) -> _DMA_ACKW {
        _DMA_ACKW { w: self }
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline]
    pub fn dma_req(&mut self) -> _DMA_REQW {
        _DMA_REQW { w: self }
    }
}
