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
#[doc = "BCSCTL1 Calibration Data for 12MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_12mhz](calbc1_12mhz) module"]
pub type CALBC1_12MHZ = crate::Reg<u8, _CALBC1_12MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALBC1_12MHZ;
#[doc = "`read()` method returns [calbc1_12mhz::R](calbc1_12mhz::R) reader structure"]
impl crate::Readable for CALBC1_12MHZ {}
#[doc = "`write(|w| ..)` method takes [calbc1_12mhz::W](calbc1_12mhz::W) writer structure"]
impl crate::Writable for CALBC1_12MHZ {}
#[doc = "BCSCTL1 Calibration Data for 12MHz"]
pub mod calbc1_12mhz;
#[doc = "BCSCTL1 Calibration Data for 16MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_16mhz](calbc1_16mhz) module"]
pub type CALBC1_16MHZ = crate::Reg<u8, _CALBC1_16MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALBC1_16MHZ;
#[doc = "`read()` method returns [calbc1_16mhz::R](calbc1_16mhz::R) reader structure"]
impl crate::Readable for CALBC1_16MHZ {}
#[doc = "`write(|w| ..)` method takes [calbc1_16mhz::W](calbc1_16mhz::W) writer structure"]
impl crate::Writable for CALBC1_16MHZ {}
#[doc = "BCSCTL1 Calibration Data for 16MHz"]
pub mod calbc1_16mhz;
#[doc = "BCSCTL1 Calibration Data for 1MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_1mhz](calbc1_1mhz) module"]
pub type CALBC1_1MHZ = crate::Reg<u8, _CALBC1_1MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALBC1_1MHZ;
#[doc = "`read()` method returns [calbc1_1mhz::R](calbc1_1mhz::R) reader structure"]
impl crate::Readable for CALBC1_1MHZ {}
#[doc = "`write(|w| ..)` method takes [calbc1_1mhz::W](calbc1_1mhz::W) writer structure"]
impl crate::Writable for CALBC1_1MHZ {}
#[doc = "BCSCTL1 Calibration Data for 1MHz"]
pub mod calbc1_1mhz;
#[doc = "BCSCTL1 Calibration Data for 8MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calbc1_8mhz](calbc1_8mhz) module"]
pub type CALBC1_8MHZ = crate::Reg<u8, _CALBC1_8MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALBC1_8MHZ;
#[doc = "`read()` method returns [calbc1_8mhz::R](calbc1_8mhz::R) reader structure"]
impl crate::Readable for CALBC1_8MHZ {}
#[doc = "`write(|w| ..)` method takes [calbc1_8mhz::W](calbc1_8mhz::W) writer structure"]
impl crate::Writable for CALBC1_8MHZ {}
#[doc = "BCSCTL1 Calibration Data for 8MHz"]
pub mod calbc1_8mhz;
#[doc = "DCOCTL Calibration Data for 12MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_12mhz](caldco_12mhz) module"]
pub type CALDCO_12MHZ = crate::Reg<u8, _CALDCO_12MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALDCO_12MHZ;
#[doc = "`read()` method returns [caldco_12mhz::R](caldco_12mhz::R) reader structure"]
impl crate::Readable for CALDCO_12MHZ {}
#[doc = "`write(|w| ..)` method takes [caldco_12mhz::W](caldco_12mhz::W) writer structure"]
impl crate::Writable for CALDCO_12MHZ {}
#[doc = "DCOCTL Calibration Data for 12MHz"]
pub mod caldco_12mhz;
#[doc = "DCOCTL Calibration Data for 16MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_16mhz](caldco_16mhz) module"]
pub type CALDCO_16MHZ = crate::Reg<u8, _CALDCO_16MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALDCO_16MHZ;
#[doc = "`read()` method returns [caldco_16mhz::R](caldco_16mhz::R) reader structure"]
impl crate::Readable for CALDCO_16MHZ {}
#[doc = "`write(|w| ..)` method takes [caldco_16mhz::W](caldco_16mhz::W) writer structure"]
impl crate::Writable for CALDCO_16MHZ {}
#[doc = "DCOCTL Calibration Data for 16MHz"]
pub mod caldco_16mhz;
#[doc = "DCOCTL Calibration Data for 1MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_1mhz](caldco_1mhz) module"]
pub type CALDCO_1MHZ = crate::Reg<u8, _CALDCO_1MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALDCO_1MHZ;
#[doc = "`read()` method returns [caldco_1mhz::R](caldco_1mhz::R) reader structure"]
impl crate::Readable for CALDCO_1MHZ {}
#[doc = "`write(|w| ..)` method takes [caldco_1mhz::W](caldco_1mhz::W) writer structure"]
impl crate::Writable for CALDCO_1MHZ {}
#[doc = "DCOCTL Calibration Data for 1MHz"]
pub mod caldco_1mhz;
#[doc = "DCOCTL Calibration Data for 8MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_8mhz](caldco_8mhz) module"]
pub type CALDCO_8MHZ = crate::Reg<u8, _CALDCO_8MHZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALDCO_8MHZ;
#[doc = "`read()` method returns [caldco_8mhz::R](caldco_8mhz::R) reader structure"]
impl crate::Readable for CALDCO_8MHZ {}
#[doc = "`write(|w| ..)` method takes [caldco_8mhz::W](caldco_8mhz::W) writer structure"]
impl crate::Writable for CALDCO_8MHZ {}
#[doc = "DCOCTL Calibration Data for 8MHz"]
pub mod caldco_8mhz;
