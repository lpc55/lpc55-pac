#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
    pub hcrevision: HCREVISION,
    #[doc = "0x04 - Defines the operating modes of the HC"]
    pub hccontrol: HCCONTROL,
    #[doc = "0x08 - This register is used to receive the commands from the Host Controller Driver (HCD)"]
    pub hccommandstatus: HCCOMMANDSTATUS,
    #[doc = "0x0c - Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
    pub hcinterruptstatus: HCINTERRUPTSTATUS,
    #[doc = "0x10 - Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
    pub hcinterruptenable: HCINTERRUPTENABLE,
    #[doc = "0x14 - The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
    pub hcinterruptdisable: HCINTERRUPTDISABLE,
    #[doc = "0x18 - Contains the physical address of the host controller communication area"]
    pub hchcca: HCHCCA,
    #[doc = "0x1c - Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
    pub hcperiodcurrented: HCPERIODCURRENTED,
    #[doc = "0x20 - Contains the physical address of the first endpoint descriptor of the control list"]
    pub hccontrolheaded: HCCONTROLHEADED,
    #[doc = "0x24 - Contains the physical address of the current endpoint descriptor of the control list"]
    pub hccontrolcurrented: HCCONTROLCURRENTED,
    #[doc = "0x28 - Contains the physical address of the first endpoint descriptor of the bulk list"]
    pub hcbulkheaded: HCBULKHEADED,
    #[doc = "0x2c - Contains the physical address of the current endpoint descriptor of the bulk list"]
    pub hcbulkcurrented: HCBULKCURRENTED,
    #[doc = "0x30 - Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
    pub hcdonehead: HCDONEHEAD,
    #[doc = "0x34 - Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
    pub hcfminterval: HCFMINTERVAL,
    #[doc = "0x38 - A 14-bit counter showing the bit time remaining in the current frame"]
    pub hcfmremaining: HCFMREMAINING,
    #[doc = "0x3c - Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
    pub hcfmnumber: HCFMNUMBER,
    #[doc = "0x40 - Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
    pub hcperiodicstart: HCPERIODICSTART,
    #[doc = "0x44 - Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
    pub hclsthreshold: HCLSTHRESHOLD,
    #[doc = "0x48 - First of the two registers which describes the characteristics of the root hub"]
    pub hcrhdescriptora: HCRHDESCRIPTORA,
    #[doc = "0x4c - Second of the two registers which describes the characteristics of the Root Hub"]
    pub hcrhdescriptorb: HCRHDESCRIPTORB,
    #[doc = "0x50 - This register is divided into two parts"]
    pub hcrhstatus: HCRHSTATUS,
    #[doc = "0x54 - Controls and reports the port events on a per-port basis"]
    pub hcrhportstatus: HCRHPORTSTATUS,
    _reserved0: [u8; 4usize],
    #[doc = "0x5c - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
pub struct HCREVISION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
pub mod hcrevision;
#[doc = "Defines the operating modes of the HC"]
pub struct HCCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Defines the operating modes of the HC"]
pub mod hccontrol;
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
pub struct HCCOMMANDSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
pub mod hccommandstatus;
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
pub struct HCINTERRUPTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
pub mod hcinterruptstatus;
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
pub struct HCINTERRUPTENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
pub mod hcinterruptenable;
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
pub struct HCINTERRUPTDISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
pub mod hcinterruptdisable;
#[doc = "Contains the physical address of the host controller communication area"]
pub struct HCHCCA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the host controller communication area"]
pub mod hchcca;
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
pub struct HCPERIODCURRENTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
pub mod hcperiodcurrented;
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
pub struct HCCONTROLHEADED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
pub mod hccontrolheaded;
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
pub struct HCCONTROLCURRENTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
pub mod hccontrolcurrented;
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
pub struct HCBULKHEADED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
pub mod hcbulkheaded;
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
pub struct HCBULKCURRENTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
pub mod hcbulkcurrented;
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
pub struct HCDONEHEAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
pub mod hcdonehead;
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
pub struct HCFMINTERVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
pub mod hcfminterval;
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
pub struct HCFMREMAINING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
pub mod hcfmremaining;
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
pub struct HCFMNUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
pub mod hcfmnumber;
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
pub struct HCPERIODICSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
pub mod hcperiodicstart;
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
pub struct HCLSTHRESHOLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
pub mod hclsthreshold;
#[doc = "First of the two registers which describes the characteristics of the root hub"]
pub struct HCRHDESCRIPTORA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "First of the two registers which describes the characteristics of the root hub"]
pub mod hcrhdescriptora;
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
pub struct HCRHDESCRIPTORB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
pub mod hcrhdescriptorb;
#[doc = "This register is divided into two parts"]
pub struct HCRHSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register is divided into two parts"]
pub mod hcrhstatus;
#[doc = "Controls and reports the port events on a per-port basis"]
pub struct HCRHPORTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls and reports the port events on a per-port basis"]
pub mod hcrhportstatus;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub struct PORTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
