#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG1 {
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
#[doc = "Possible values of the field `MAINENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAINENABLER {
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    DISABLED,
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    ENABLED,
}
impl MAINENABLER {
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
            MAINENABLER::DISABLED => false,
            MAINENABLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAINENABLER {
        match value {
            false => MAINENABLER::DISABLED,
            true => MAINENABLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MAINENABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MAINENABLER::ENABLED
    }
}
#[doc = "Possible values of the field `DATAPAUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSER {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE,
}
impl DATAPAUSER {
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
            DATAPAUSER::NORMAL => false,
            DATAPAUSER::PAUSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATAPAUSER {
        match value {
            false => DATAPAUSER::NORMAL,
            true => DATAPAUSER::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == DATAPAUSER::NORMAL
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline]
    pub fn is_pause(&self) -> bool {
        *self == DATAPAUSER::PAUSE
    }
}
#[doc = "Possible values of the field `PAIRCOUNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAIRCOUNTR {
    #[doc = "1 I2S channel pairs in this flexcomm"]
    PAIRS_1,
    #[doc = "2 I2S channel pairs in this flexcomm"]
    PAIRS_2,
    #[doc = "3 I2S channel pairs in this flexcomm"]
    PAIRS_3,
    #[doc = "4 I2S channel pairs in this flexcomm"]
    PAIRS_4,
}
impl PAIRCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAIRCOUNTR::PAIRS_1 => 0,
            PAIRCOUNTR::PAIRS_2 => 1,
            PAIRCOUNTR::PAIRS_3 => 2,
            PAIRCOUNTR::PAIRS_4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAIRCOUNTR {
        match value {
            0 => PAIRCOUNTR::PAIRS_1,
            1 => PAIRCOUNTR::PAIRS_2,
            2 => PAIRCOUNTR::PAIRS_3,
            3 => PAIRCOUNTR::PAIRS_4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAIRS_1`"]
    #[inline]
    pub fn is_pairs_1(&self) -> bool {
        *self == PAIRCOUNTR::PAIRS_1
    }
    #[doc = "Checks if the value of the field is `PAIRS_2`"]
    #[inline]
    pub fn is_pairs_2(&self) -> bool {
        *self == PAIRCOUNTR::PAIRS_2
    }
    #[doc = "Checks if the value of the field is `PAIRS_3`"]
    #[inline]
    pub fn is_pairs_3(&self) -> bool {
        *self == PAIRCOUNTR::PAIRS_3
    }
    #[doc = "Checks if the value of the field is `PAIRS_4`"]
    #[inline]
    pub fn is_pairs_4(&self) -> bool {
        *self == PAIRCOUNTR::PAIRS_4
    }
}
#[doc = "Possible values of the field `MSTSLVCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSLVCFGR {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER,
}
impl MSTSLVCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSLVCFGR::NORMAL_SLAVE_MODE => 0,
            MSTSLVCFGR::WS_SYNC_MASTER => 1,
            MSTSLVCFGR::MASTER_USING_SCK => 2,
            MSTSLVCFGR::NORMAL_MASTER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSLVCFGR {
        match value {
            0 => MSTSLVCFGR::NORMAL_SLAVE_MODE,
            1 => MSTSLVCFGR::WS_SYNC_MASTER,
            2 => MSTSLVCFGR::MASTER_USING_SCK,
            3 => MSTSLVCFGR::NORMAL_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_SLAVE_MODE`"]
    #[inline]
    pub fn is_normal_slave_mode(&self) -> bool {
        *self == MSTSLVCFGR::NORMAL_SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `WS_SYNC_MASTER`"]
    #[inline]
    pub fn is_ws_sync_master(&self) -> bool {
        *self == MSTSLVCFGR::WS_SYNC_MASTER
    }
    #[doc = "Checks if the value of the field is `MASTER_USING_SCK`"]
    #[inline]
    pub fn is_master_using_sck(&self) -> bool {
        *self == MSTSLVCFGR::MASTER_USING_SCK
    }
    #[doc = "Checks if the value of the field is `NORMAL_MASTER`"]
    #[inline]
    pub fn is_normal_master(&self) -> bool {
        *self == MSTSLVCFGR::NORMAL_MASTER
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::CLASSIC_MODE => 0,
            MODER::DSP_MODE_WS_50_DUTYCYCLE => 1,
            MODER::DSP_MODE_WS_1_CLOCK => 2,
            MODER::DSP_MODE_WS_1_DATA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::CLASSIC_MODE,
            1 => MODER::DSP_MODE_WS_50_DUTYCYCLE,
            2 => MODER::DSP_MODE_WS_1_CLOCK,
            3 => MODER::DSP_MODE_WS_1_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLASSIC_MODE`"]
    #[inline]
    pub fn is_classic_mode(&self) -> bool {
        *self == MODER::CLASSIC_MODE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_50_DUTYCYCLE`"]
    #[inline]
    pub fn is_dsp_mode_ws_50_dutycycle(&self) -> bool {
        *self == MODER::DSP_MODE_WS_50_DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_CLOCK`"]
    #[inline]
    pub fn is_dsp_mode_ws_1_clock(&self) -> bool {
        *self == MODER::DSP_MODE_WS_1_CLOCK
    }
    #[doc = "Checks if the value of the field is `DSP_MODE_WS_1_DATA`"]
    #[inline]
    pub fn is_dsp_mode_ws_1_data(&self) -> bool {
        *self == MODER::DSP_MODE_WS_1_DATA
    }
}
#[doc = "Possible values of the field `RIGHTLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIGHTLOWR {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW,
}
impl RIGHTLOWR {
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
            RIGHTLOWR::RIGHT_HIGH => false,
            RIGHTLOWR::RIGHT_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIGHTLOWR {
        match value {
            false => RIGHTLOWR::RIGHT_HIGH,
            true => RIGHTLOWR::RIGHT_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_HIGH`"]
    #[inline]
    pub fn is_right_high(&self) -> bool {
        *self == RIGHTLOWR::RIGHT_HIGH
    }
    #[doc = "Checks if the value of the field is `RIGHT_LOW`"]
    #[inline]
    pub fn is_right_low(&self) -> bool {
        *self == RIGHTLOWR::RIGHT_LOW
    }
}
#[doc = "Possible values of the field `LEFTJUST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEFTJUSTR {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED,
}
impl LEFTJUSTR {
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
            LEFTJUSTR::RIGHT_JUSTIFIED => false,
            LEFTJUSTR::LEFT_JUSTIFIED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LEFTJUSTR {
        match value {
            false => LEFTJUSTR::RIGHT_JUSTIFIED,
            true => LEFTJUSTR::LEFT_JUSTIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT_JUSTIFIED`"]
    #[inline]
    pub fn is_right_justified(&self) -> bool {
        *self == LEFTJUSTR::RIGHT_JUSTIFIED
    }
    #[doc = "Checks if the value of the field is `LEFT_JUSTIFIED`"]
    #[inline]
    pub fn is_left_justified(&self) -> bool {
        *self == LEFTJUSTR::LEFT_JUSTIFIED
    }
}
#[doc = "Possible values of the field `ONECHANNEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONECHANNELR {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL,
}
impl ONECHANNELR {
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
            ONECHANNELR::DUAL_CHANNEL => false,
            ONECHANNELR::SINGLE_CHANNEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONECHANNELR {
        match value {
            false => ONECHANNELR::DUAL_CHANNEL,
            true => ONECHANNELR::SINGLE_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `DUAL_CHANNEL`"]
    #[inline]
    pub fn is_dual_channel(&self) -> bool {
        *self == ONECHANNELR::DUAL_CHANNEL
    }
    #[doc = "Checks if the value of the field is `SINGLE_CHANNEL`"]
    #[inline]
    pub fn is_single_channel(&self) -> bool {
        *self == ONECHANNELR::SINGLE_CHANNEL
    }
}
#[doc = "Possible values of the field `PDMDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMDATAR {
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    NORMAL,
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    DMIC_SUBSYSTEM,
}
impl PDMDATAR {
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
            PDMDATAR::NORMAL => false,
            PDMDATAR::DMIC_SUBSYSTEM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDMDATAR {
        match value {
            false => PDMDATAR::NORMAL,
            true => PDMDATAR::DMIC_SUBSYSTEM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == PDMDATAR::NORMAL
    }
    #[doc = "Checks if the value of the field is `DMIC_SUBSYSTEM`"]
    #[inline]
    pub fn is_dmic_subsystem(&self) -> bool {
        *self == PDMDATAR::DMIC_SUBSYSTEM
    }
}
#[doc = "Possible values of the field `SCK_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_POLR {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE,
}
impl SCK_POLR {
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
            SCK_POLR::FALLING_EDGE => false,
            SCK_POLR::RISING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCK_POLR {
        match value {
            false => SCK_POLR::FALLING_EDGE,
            true => SCK_POLR::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == SCK_POLR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == SCK_POLR::RISING_EDGE
    }
}
#[doc = "Possible values of the field `WS_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WS_POLR {
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    NOT_INVERTED,
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    INVERTED,
}
impl WS_POLR {
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
            WS_POLR::NOT_INVERTED => false,
            WS_POLR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WS_POLR {
        match value {
            false => WS_POLR::NOT_INVERTED,
            true => WS_POLR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline]
    pub fn is_not_inverted(&self) -> bool {
        *self == WS_POLR::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == WS_POLR::INVERTED
    }
}
#[doc = r" Value of the field"]
pub struct DATALENR {
    bits: u8,
}
impl DATALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MAINENABLE`"]
pub enum MAINENABLEW {
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    DISABLED,
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    ENABLED,
}
impl MAINENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAINENABLEW::DISABLED => false,
            MAINENABLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAINENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MAINENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAINENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All I 2S channel pairs in this Flexcomm are disabled and the internal state machines, counters, and flags are reset. No other channel pairs can be enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAINENABLEW::DISABLED)
    }
    #[doc = "This I 2S channel pair is enabled. Other channel pairs in this Flexcomm may be enabled in their individual PAIRENABLE bits."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAINENABLEW::ENABLED)
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
#[doc = "Values that can be written to the field `DATAPAUSE`"]
pub enum DATAPAUSEW {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE,
}
impl DATAPAUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATAPAUSEW::NORMAL => false,
            DATAPAUSEW::PAUSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAPAUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAPAUSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAPAUSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(DATAPAUSEW::NORMAL)
    }
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    #[inline]
    pub fn pause(self) -> &'a mut W {
        self.variant(DATAPAUSEW::PAUSE)
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
#[doc = "Values that can be written to the field `PAIRCOUNT`"]
pub enum PAIRCOUNTW {
    #[doc = "1 I2S channel pairs in this flexcomm"]
    PAIRS_1,
    #[doc = "2 I2S channel pairs in this flexcomm"]
    PAIRS_2,
    #[doc = "3 I2S channel pairs in this flexcomm"]
    PAIRS_3,
    #[doc = "4 I2S channel pairs in this flexcomm"]
    PAIRS_4,
}
impl PAIRCOUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAIRCOUNTW::PAIRS_1 => 0,
            PAIRCOUNTW::PAIRS_2 => 1,
            PAIRCOUNTW::PAIRS_3 => 2,
            PAIRCOUNTW::PAIRS_4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAIRCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PAIRCOUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAIRCOUNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 I2S channel pairs in this flexcomm"]
    #[inline]
    pub fn pairs_1(self) -> &'a mut W {
        self.variant(PAIRCOUNTW::PAIRS_1)
    }
    #[doc = "2 I2S channel pairs in this flexcomm"]
    #[inline]
    pub fn pairs_2(self) -> &'a mut W {
        self.variant(PAIRCOUNTW::PAIRS_2)
    }
    #[doc = "3 I2S channel pairs in this flexcomm"]
    #[inline]
    pub fn pairs_3(self) -> &'a mut W {
        self.variant(PAIRCOUNTW::PAIRS_3)
    }
    #[doc = "4 I2S channel pairs in this flexcomm"]
    #[inline]
    pub fn pairs_4(self) -> &'a mut W {
        self.variant(PAIRCOUNTW::PAIRS_4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTSLVCFG`"]
pub enum MSTSLVCFGW {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER,
}
impl MSTSLVCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSLVCFGW::NORMAL_SLAVE_MODE => 0,
            MSTSLVCFGW::WS_SYNC_MASTER => 1,
            MSTSLVCFGW::MASTER_USING_SCK => 2,
            MSTSLVCFGW::NORMAL_MASTER => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSLVCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSLVCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSLVCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    #[inline]
    pub fn normal_slave_mode(self) -> &'a mut W {
        self.variant(MSTSLVCFGW::NORMAL_SLAVE_MODE)
    }
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    #[inline]
    pub fn ws_sync_master(self) -> &'a mut W {
        self.variant(MSTSLVCFGW::WS_SYNC_MASTER)
    }
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    #[inline]
    pub fn master_using_sck(self) -> &'a mut W {
        self.variant(MSTSLVCFGW::MASTER_USING_SCK)
    }
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    #[inline]
    pub fn normal_master(self) -> &'a mut W {
        self.variant(MSTSLVCFGW::NORMAL_MASTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::CLASSIC_MODE => 0,
            MODEW::DSP_MODE_WS_50_DUTYCYCLE => 1,
            MODEW::DSP_MODE_WS_1_CLOCK => 2,
            MODEW::DSP_MODE_WS_1_DATA => 3,
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
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    #[inline]
    pub fn classic_mode(self) -> &'a mut W {
        self.variant(MODEW::CLASSIC_MODE)
    }
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    #[inline]
    pub fn dsp_mode_ws_50_dutycycle(self) -> &'a mut W {
        self.variant(MODEW::DSP_MODE_WS_50_DUTYCYCLE)
    }
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    #[inline]
    pub fn dsp_mode_ws_1_clock(self) -> &'a mut W {
        self.variant(MODEW::DSP_MODE_WS_1_CLOCK)
    }
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    #[inline]
    pub fn dsp_mode_ws_1_data(self) -> &'a mut W {
        self.variant(MODEW::DSP_MODE_WS_1_DATA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIGHTLOW`"]
pub enum RIGHTLOWW {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW,
}
impl RIGHTLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIGHTLOWW::RIGHT_HIGH => false,
            RIGHTLOWW::RIGHT_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIGHTLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _RIGHTLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIGHTLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    #[inline]
    pub fn right_high(self) -> &'a mut W {
        self.variant(RIGHTLOWW::RIGHT_HIGH)
    }
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    #[inline]
    pub fn right_low(self) -> &'a mut W {
        self.variant(RIGHTLOWW::RIGHT_LOW)
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
#[doc = "Values that can be written to the field `LEFTJUST`"]
pub enum LEFTJUSTW {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED,
}
impl LEFTJUSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LEFTJUSTW::RIGHT_JUSTIFIED => false,
            LEFTJUSTW::LEFT_JUSTIFIED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEFTJUSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LEFTJUSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEFTJUSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    #[inline]
    pub fn right_justified(self) -> &'a mut W {
        self.variant(LEFTJUSTW::RIGHT_JUSTIFIED)
    }
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    #[inline]
    pub fn left_justified(self) -> &'a mut W {
        self.variant(LEFTJUSTW::LEFT_JUSTIFIED)
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
#[doc = "Values that can be written to the field `ONECHANNEL`"]
pub enum ONECHANNELW {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL,
}
impl ONECHANNELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONECHANNELW::DUAL_CHANNEL => false,
            ONECHANNELW::SINGLE_CHANNEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONECHANNELW<'a> {
    w: &'a mut W,
}
impl<'a> _ONECHANNELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONECHANNELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    #[inline]
    pub fn dual_channel(self) -> &'a mut W {
        self.variant(ONECHANNELW::DUAL_CHANNEL)
    }
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    #[inline]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(ONECHANNELW::SINGLE_CHANNEL)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDMDATA`"]
pub enum PDMDATAW {
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    NORMAL,
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    DMIC_SUBSYSTEM,
}
impl PDMDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDMDATAW::NORMAL => false,
            PDMDATAW::DMIC_SUBSYSTEM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDMDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PDMDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDMDATAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation, data is transferred to or from the Flexcomm FIFO."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(PDMDATAW::NORMAL)
    }
    #[doc = "The data source is the D-Mic subsystem. When PDMDATA = 1, only the primary channel pair can be used in this Flexcomm. If ONECHANNEL = 1, only the PDM left data is used. the WS rate must match the Fs (sample rate) of the D-Mic decimator. A rate mismatch will at some point cause the I2S to overrun or underrun."]
    #[inline]
    pub fn dmic_subsystem(self) -> &'a mut W {
        self.variant(PDMDATAW::DMIC_SUBSYSTEM)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCK_POL`"]
pub enum SCK_POLW {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE,
}
impl SCK_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCK_POLW::FALLING_EDGE => false,
            SCK_POLW::RISING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCK_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _SCK_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCK_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SCK_POLW::FALLING_EDGE)
    }
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SCK_POLW::RISING_EDGE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WS_POL`"]
pub enum WS_POLW {
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    NOT_INVERTED,
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    INVERTED,
}
impl WS_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WS_POLW::NOT_INVERTED => false,
            WS_POLW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WS_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _WS_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WS_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data frames begin at a falling edge of WS (standard for classic I2S)."]
    #[inline]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(WS_POLW::NOT_INVERTED)
    }
    #[doc = "WS is inverted, resulting in a data frame beginning at a rising edge of WS (standard for most 'non-classic' variations of I2S)."]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(WS_POLW::INVERTED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline]
    pub fn mainenable(&self) -> MAINENABLER {
        MAINENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline]
    pub fn datapause(&self) -> DATAPAUSER {
        DATAPAUSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline]
    pub fn paircount(&self) -> PAIRCOUNTR {
        PAIRCOUNTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline]
    pub fn mstslvcfg(&self) -> MSTSLVCFGR {
        MSTSLVCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline]
    pub fn rightlow(&self) -> RIGHTLOWR {
        RIGHTLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline]
    pub fn leftjust(&self) -> LEFTJUSTR {
        LEFTJUSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline]
    pub fn onechannel(&self) -> ONECHANNELR {
        ONECHANNELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC55xx, this bit applies only to Flexcomm 6,7."]
    #[inline]
    pub fn pdmdata(&self) -> PDMDATAR {
        PDMDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline]
    pub fn sck_pol(&self) -> SCK_POLR {
        SCK_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline]
    pub fn ws_pol(&self) -> WS_POLR {
        WS_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline]
    pub fn datalen(&self) -> DATALENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATALENR { bits }
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
    #[doc = "Bit 0 - Main enable for I 2S function in this Flexcomm"]
    #[inline]
    pub fn mainenable(&mut self) -> _MAINENABLEW {
        _MAINENABLEW { w: self }
    }
    #[doc = "Bit 1 - Data flow Pause. Allows pausing data flow between the I2S serializer/deserializer and the FIFO. This could be done in order to change streams, or while restarting after a data underflow or overflow. When paused, FIFO operations can be done without corrupting data that is in the process of being sent or received. Once a data pause has been requested, the interface may need to complete sending data that was in progress before interrupting the flow of data. Software must check that the pause is actually in effect before taking action. This is done by monitoring the DATAPAUSED flag in the STAT register. When DATAPAUSE is cleared, data transfer will resume at the beginning of the next frame."]
    #[inline]
    pub fn datapause(&mut self) -> _DATAPAUSEW {
        _DATAPAUSEW { w: self }
    }
    #[doc = "Bits 2:3 - Provides the number of I2S channel pairs in this Flexcomm This is a read-only field whose value may be different in other Flexcomms. 00 = there is 1 I2S channel pair in this Flexcomm. 01 = there are 2 I2S channel pairs in this Flexcomm. 10 = there are 3 I2S channel pairs in this Flexcomm. 11 = there are 4 I2S channel pairs in this Flexcomm."]
    #[inline]
    pub fn paircount(&mut self) -> _PAIRCOUNTW {
        _PAIRCOUNTW { w: self }
    }
    #[doc = "Bits 4:5 - Master / slave configuration selection, determining how SCK and WS are used by all channel pairs in this Flexcomm."]
    #[inline]
    pub fn mstslvcfg(&mut self) -> _MSTSLVCFGW {
        _MSTSLVCFGW { w: self }
    }
    #[doc = "Bits 6:7 - Selects the basic I2S operating mode. Other configurations modify this to obtain all supported cases. See Formats and modes for examples."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 8 - Right channel data is in the Low portion of FIFO data. Essentially, this swaps left and right channel data as it is transferred to or from the FIFO. This bit is not used if the data width is greater than 24 bits or if PDMDATA = 1. Note that if the ONECHANNEL field (bit 10 of this register) = 1, the one channel to be used is the nominally the left channel. POSITION can still place that data in the frame where right channel data is normally located. if all enabled channel pairs have ONECHANNEL = 1, then RIGHTLOW = 1 is not allowed."]
    #[inline]
    pub fn rightlow(&mut self) -> _RIGHTLOWW {
        _RIGHTLOWW { w: self }
    }
    #[doc = "Bit 9 - Left Justify data."]
    #[inline]
    pub fn leftjust(&mut self) -> _LEFTJUSTW {
        _LEFTJUSTW { w: self }
    }
    #[doc = "Bit 10 - Single channel mode. Applies to both transmit and receive. This configuration bit applies only to the first I2S channel pair. Other channel pairs may select this mode independently in their separate CFG1 registers."]
    #[inline]
    pub fn onechannel(&mut self) -> _ONECHANNELW {
        _ONECHANNELW { w: self }
    }
    #[doc = "Bit 11 - PDM Data selection. This bit controls the data source for I2S transmit, and cannot be set in Rx mode. This bit only has an effect if the device the Flexcomm resides in includes a D-Mic subsystem. For the LPC55xx, this bit applies only to Flexcomm 6,7."]
    #[inline]
    pub fn pdmdata(&mut self) -> _PDMDATAW {
        _PDMDATAW { w: self }
    }
    #[doc = "Bit 12 - SCK polarity."]
    #[inline]
    pub fn sck_pol(&mut self) -> _SCK_POLW {
        _SCK_POLW { w: self }
    }
    #[doc = "Bit 13 - WS polarity."]
    #[inline]
    pub fn ws_pol(&mut self) -> _WS_POLW {
        _WS_POLW { w: self }
    }
    #[doc = "Bits 16:20 - Data Length, minus 1 encoded, defines the number of data bits to be transmitted or received for all I2S channel pairs in this Flexcomm. Note that data is only driven to or received from SDA for the number of bits defined by DATALEN. DATALEN is also used in these ways by the I2S: Determines the size of data transfers between the FIFO and the I2S serializer/deserializer. See FIFO buffer configurations and usage In mode 1, 2, and 3, determines the location of right data following left data in the frame. In mode 3 (where WS has a one data slot long pulse at the beginning of each data frame) determines the duration of the WS pulse. Values: 0x00 to 0x02 = not supported 0x03 = data is 4 bits in length 0x04 = data is 5 bits in length 0x1F = data is 32 bits in length"]
    #[inline]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
}
