#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCOCTL Calibration Data for 16MHz"]
    pub caldco_16mhz: CALDCO_16MHZ,
    #[doc = "0x01 - BCSCTL1 Calibration Data for 16MHz"]
    pub calbc1_16mhz: CALBC1_16MHZ,
    #[doc = "0x02 - DCOCTL Calibration Data for 12MHz"]
    pub caldco_12mhz: CALDCO_12MHZ,
    #[doc = "0x03 - BCSCTL1 Calibration Data for 12MHz"]
    pub calbc1_12mhz: CALBC1_12MHZ,
    #[doc = "0x04 - DCOCTL Calibration Data for 8MHz"]
    pub caldco_8mhz: CALDCO_8MHZ,
    #[doc = "0x05 - BCSCTL1 Calibration Data for 8MHz"]
    pub calbc1_8mhz: CALBC1_8MHZ,
    #[doc = "0x06 - DCOCTL Calibration Data for 1MHz"]
    pub caldco_1mhz: CALDCO_1MHZ,
    #[doc = "0x07 - BCSCTL1 Calibration Data for 1MHz"]
    pub calbc1_1mhz: CALBC1_1MHZ,
}
#[doc = "CALDCO_16MHZ (rw) register accessor: an alias for `Reg<CALDCO_16MHZ_SPEC>`"]
pub type CALDCO_16MHZ = crate::Reg<caldco_16mhz::CALDCO_16MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 16MHz"]
pub mod caldco_16mhz;
#[doc = "CALBC1_16MHZ (rw) register accessor: an alias for `Reg<CALBC1_16MHZ_SPEC>`"]
pub type CALBC1_16MHZ = crate::Reg<calbc1_16mhz::CALBC1_16MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 16MHz"]
pub mod calbc1_16mhz;
#[doc = "CALDCO_12MHZ (rw) register accessor: an alias for `Reg<CALDCO_12MHZ_SPEC>`"]
pub type CALDCO_12MHZ = crate::Reg<caldco_12mhz::CALDCO_12MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 12MHz"]
pub mod caldco_12mhz;
#[doc = "CALBC1_12MHZ (rw) register accessor: an alias for `Reg<CALBC1_12MHZ_SPEC>`"]
pub type CALBC1_12MHZ = crate::Reg<calbc1_12mhz::CALBC1_12MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 12MHz"]
pub mod calbc1_12mhz;
#[doc = "CALDCO_8MHZ (rw) register accessor: an alias for `Reg<CALDCO_8MHZ_SPEC>`"]
pub type CALDCO_8MHZ = crate::Reg<caldco_8mhz::CALDCO_8MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 8MHz"]
pub mod caldco_8mhz;
#[doc = "CALBC1_8MHZ (rw) register accessor: an alias for `Reg<CALBC1_8MHZ_SPEC>`"]
pub type CALBC1_8MHZ = crate::Reg<calbc1_8mhz::CALBC1_8MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 8MHz"]
pub mod calbc1_8mhz;
#[doc = "CALDCO_1MHZ (rw) register accessor: an alias for `Reg<CALDCO_1MHZ_SPEC>`"]
pub type CALDCO_1MHZ = crate::Reg<caldco_1mhz::CALDCO_1MHZ_SPEC>;
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "CALBC1_1MHZ (rw) register accessor: an alias for `Reg<CALBC1_1MHZ_SPEC>`"]
pub type CALBC1_1MHZ = crate::Reg<calbc1_1mhz::CALBC1_1MHZ_SPEC>;
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
