#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    pub caplength_chipid: CAPLENGTH_CHIPID,
    #[doc = "0x04 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x08 - Host Controller Capability Parameters"]
    pub hccparams: HCCPARAMS,
    #[doc = "0x0c - Frame Length Adjustment"]
    pub fladj_frindex: FLADJ_FRINDEX,
    #[doc = "0x10 - Memory base address where ATL PTD0 is stored"]
    pub atl_ptd_base_addr: ATL_PTD_BASE_ADDR,
    #[doc = "0x14 - Memory base address where ISO PTD0 is stored"]
    pub iso_ptd_base_addr: ISO_PTD_BASE_ADDR,
    #[doc = "0x18 - Memory base address where INT PTD0 is stored"]
    pub int_ptd_base_addr: INT_PTD_BASE_ADDR,
    #[doc = "0x1c - Memory base address that indicates the start of the data payload buffers"]
    pub data_payload_base_addr: DATA_PAYLOAD_BASE_ADDR,
    #[doc = "0x20 - USB Command register"]
    pub usbcmd: USBCMD,
    #[doc = "0x24 - USB Interrupt Status register"]
    pub usbsts: USBSTS,
    #[doc = "0x28 - USB Interrupt Enable register"]
    pub usbintr: USBINTR,
    #[doc = "0x2c - Port Status and Control register"]
    pub portsc1: PORTSC1,
    #[doc = "0x30 - Done map for each ATL PTD"]
    pub atl_ptd_done_map: ATL_PTD_DONE_MAP,
    #[doc = "0x34 - Skip map for each ATL PTD"]
    pub atl_ptd_skip_map: ATL_PTD_SKIP_MAP,
    #[doc = "0x38 - Done map for each ISO PTD"]
    pub iso_ptd_done_map: ISO_PTD_DONE_MAP,
    #[doc = "0x3c - Skip map for each ISO PTD"]
    pub iso_ptd_skip_map: ISO_PTD_SKIP_MAP,
    #[doc = "0x40 - Done map for each INT PTD"]
    pub int_ptd_done_map: INT_PTD_DONE_MAP,
    #[doc = "0x44 - Skip map for each INT PTD"]
    pub int_ptd_skip_map: INT_PTD_SKIP_MAP,
    #[doc = "0x48 - Marks the last PTD in the list for ISO, INT and ATL"]
    pub last_ptd_inuse: LAST_PTD_INUSE,
    #[doc = "0x4c - Register to read/write registers in the attached USB PHY"]
    pub utmiplus_ulpi_debug: UTMIPLUS_ULPI_DEBUG,
    #[doc = "0x50 - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub struct CAPLENGTH_CHIPID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub mod caplength_chipid;
#[doc = "Host Controller Structural Parameters"]
pub struct HCSPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters"]
pub struct HCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "Frame Length Adjustment"]
pub struct FLADJ_FRINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "Memory base address where ATL PTD0 is stored"]
pub struct ATL_PTD_BASE_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory base address where ATL PTD0 is stored"]
pub mod atl_ptd_base_addr;
#[doc = "Memory base address where ISO PTD0 is stored"]
pub struct ISO_PTD_BASE_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory base address where ISO PTD0 is stored"]
pub mod iso_ptd_base_addr;
#[doc = "Memory base address where INT PTD0 is stored"]
pub struct INT_PTD_BASE_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory base address where INT PTD0 is stored"]
pub mod int_ptd_base_addr;
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub struct DATA_PAYLOAD_BASE_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub mod data_payload_base_addr;
#[doc = "USB Command register"]
pub struct USBCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Command register"]
pub mod usbcmd;
#[doc = "USB Interrupt Status register"]
pub struct USBSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Interrupt Status register"]
pub mod usbsts;
#[doc = "USB Interrupt Enable register"]
pub struct USBINTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Interrupt Enable register"]
pub mod usbintr;
#[doc = "Port Status and Control register"]
pub struct PORTSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Status and Control register"]
pub mod portsc1;
#[doc = "Done map for each ATL PTD"]
pub struct ATL_PTD_DONE_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Done map for each ATL PTD"]
pub mod atl_ptd_done_map;
#[doc = "Skip map for each ATL PTD"]
pub struct ATL_PTD_SKIP_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Skip map for each ATL PTD"]
pub mod atl_ptd_skip_map;
#[doc = "Done map for each ISO PTD"]
pub struct ISO_PTD_DONE_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Done map for each ISO PTD"]
pub mod iso_ptd_done_map;
#[doc = "Skip map for each ISO PTD"]
pub struct ISO_PTD_SKIP_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Skip map for each ISO PTD"]
pub mod iso_ptd_skip_map;
#[doc = "Done map for each INT PTD"]
pub struct INT_PTD_DONE_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Done map for each INT PTD"]
pub mod int_ptd_done_map;
#[doc = "Skip map for each INT PTD"]
pub struct INT_PTD_SKIP_MAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Skip map for each INT PTD"]
pub mod int_ptd_skip_map;
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub struct LAST_PTD_INUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub mod last_ptd_inuse;
#[doc = "Register to read/write registers in the attached USB PHY"]
pub struct UTMIPLUS_ULPI_DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register to read/write registers in the attached USB PHY"]
pub mod utmiplus_ulpi_debug;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub struct PORTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
