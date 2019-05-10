#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - Configuration for shared functions."]
    pub cfg: CFG,
    #[doc = "0x804 - Status register for Master, Slave, and Monitor functions."]
    pub stat: STAT,
    #[doc = "0x808 - Interrupt Enable Set and read register."]
    pub intenset: INTENSET,
    #[doc = "0x80c - Interrupt Enable Clear register."]
    pub intenclr: INTENCLR,
    #[doc = "0x810 - Time-out value register."]
    pub timeout: TIMEOUT,
    #[doc = "0x814 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    pub clkdiv: CLKDIV,
    #[doc = "0x818 - Interrupt Status register for Master, Slave, and Monitor functions."]
    pub intstat: INTSTAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x820 - Master control register."]
    pub mstctl: MSTCTL,
    #[doc = "0x824 - Master timing configuration."]
    pub msttime: MSTTIME,
    #[doc = "0x828 - Combined Master receiver and transmitter data register."]
    pub mstdat: MSTDAT,
    _reserved2: [u8; 20usize],
    #[doc = "0x840 - Slave control register."]
    pub slvctl: SLVCTL,
    #[doc = "0x844 - Combined Slave receiver and transmitter data register."]
    pub slvdat: SLVDAT,
    #[doc = "0x848 - Slave address register."]
    pub slvadr: [SLVADR; 4],
    #[doc = "0x858 - Slave Qualification for address 0."]
    pub slvqual0: SLVQUAL0,
    _reserved3: [u8; 36usize],
    #[doc = "0x880 - Monitor receiver data register."]
    pub monrxdat: MONRXDAT,
    _reserved4: [u8; 1912usize],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "Configuration for shared functions."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "Interrupt Enable Set and read register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "Time-out value register."]
pub struct TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub mod clkdiv;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "Master control register."]
pub struct MSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "Master timing configuration."]
pub struct MSTTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "Combined Master receiver and transmitter data register."]
pub struct MSTDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "Slave control register."]
pub struct SLVCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "Combined Slave receiver and transmitter data register."]
pub struct SLVDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "Slave address register."]
pub struct SLVADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave address register."]
pub mod slvadr;
#[doc = "Slave Qualification for address 0."]
pub struct SLVQUAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "Monitor receiver data register."]
pub struct MONRXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
#[doc = "Peripheral identification register."]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral identification register."]
pub mod id;
