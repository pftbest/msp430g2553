#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC10 Data Transfer Control 0"]
    pub adc10dtc0: crate::Reg<adc10dtc0::ADC10DTC0_SPEC>,
    #[doc = "0x01 - ADC10 Data Transfer Control 1"]
    pub adc10dtc1: crate::Reg<adc10dtc1::ADC10DTC1_SPEC>,
    #[doc = "0x02 - ADC10 Analog Enable 0"]
    pub adc10ae0: crate::Reg<adc10ae0::ADC10AE0_SPEC>,
    _reserved3: [u8; 0x0165],
    #[doc = "0x168 - ADC10 Control 0"]
    pub adc10ctl0: crate::Reg<adc10ctl0::ADC10CTL0_SPEC>,
    #[doc = "0x16a - ADC10 Control 1"]
    pub adc10ctl1: crate::Reg<adc10ctl1::ADC10CTL1_SPEC>,
    #[doc = "0x16c - ADC10 Memory"]
    pub adc10mem: crate::Reg<adc10mem::ADC10MEM_SPEC>,
    _reserved6: [u8; 0x06],
    #[doc = "0x174 - ADC10 Data Transfer Start Address"]
    pub adc10sa: crate::Reg<adc10sa::ADC10SA_SPEC>,
}
#[doc = "ADC10DTC0 register accessor: an alias for `Reg<ADC10DTC0_SPEC>`"]
pub type ADC10DTC0 = crate::Reg<adc10dtc0::ADC10DTC0_SPEC>;
#[doc = "ADC10 Data Transfer Control 0"]
pub mod adc10dtc0;
#[doc = "ADC10DTC1 register accessor: an alias for `Reg<ADC10DTC1_SPEC>`"]
pub type ADC10DTC1 = crate::Reg<adc10dtc1::ADC10DTC1_SPEC>;
#[doc = "ADC10 Data Transfer Control 1"]
pub mod adc10dtc1;
#[doc = "ADC10AE0 register accessor: an alias for `Reg<ADC10AE0_SPEC>`"]
pub type ADC10AE0 = crate::Reg<adc10ae0::ADC10AE0_SPEC>;
#[doc = "ADC10 Analog Enable 0"]
pub mod adc10ae0;
#[doc = "ADC10CTL0 register accessor: an alias for `Reg<ADC10CTL0_SPEC>`"]
pub type ADC10CTL0 = crate::Reg<adc10ctl0::ADC10CTL0_SPEC>;
#[doc = "ADC10 Control 0"]
pub mod adc10ctl0;
#[doc = "ADC10CTL1 register accessor: an alias for `Reg<ADC10CTL1_SPEC>`"]
pub type ADC10CTL1 = crate::Reg<adc10ctl1::ADC10CTL1_SPEC>;
#[doc = "ADC10 Control 1"]
pub mod adc10ctl1;
#[doc = "ADC10MEM register accessor: an alias for `Reg<ADC10MEM_SPEC>`"]
pub type ADC10MEM = crate::Reg<adc10mem::ADC10MEM_SPEC>;
#[doc = "ADC10 Memory"]
pub mod adc10mem;
#[doc = "ADC10SA register accessor: an alias for `Reg<ADC10SA_SPEC>`"]
pub type ADC10SA = crate::Reg<adc10sa::ADC10SA_SPEC>;
#[doc = "ADC10 Data Transfer Start Address"]
pub mod adc10sa;
