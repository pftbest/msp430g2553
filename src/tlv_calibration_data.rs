#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TLV CHECK SUM"]
    pub tlv_checksum: crate::Reg<tlv_checksum::TLV_CHECKSUM_SPEC>,
    _reserved1: [u8; 24usize],
    #[doc = "0x1a - TLV ADC10_1 TAG"]
    pub tlv_adc10_1_tag: crate::Reg<tlv_adc10_1_tag::TLV_ADC10_1_TAG_SPEC>,
    #[doc = "0x1b - TLV ADC10_1 LEN"]
    pub tlv_adc10_1_len: crate::Reg<tlv_adc10_1_len::TLV_ADC10_1_LEN_SPEC>,
    _reserved3: [u8; 26usize],
    #[doc = "0x36 - TLV TAG_DCO30 TAG"]
    pub tlv_dco_30_tag: crate::Reg<tlv_dco_30_tag::TLV_DCO_30_TAG_SPEC>,
    #[doc = "0x37 - TLV TAG_DCO30 LEN"]
    pub tlv_dco_30_len: crate::Reg<tlv_dco_30_len::TLV_DCO_30_LEN_SPEC>,
}
#[doc = "TLV_ADC10_1_LEN register accessor: an alias for `Reg<TLV_ADC10_1_LEN_SPEC>`"]
pub type TLV_ADC10_1_LEN = crate::Reg<tlv_adc10_1_len::TLV_ADC10_1_LEN_SPEC>;
#[doc = "TLV ADC10_1 LEN"]
pub mod tlv_adc10_1_len;
#[doc = "TLV_ADC10_1_TAG register accessor: an alias for `Reg<TLV_ADC10_1_TAG_SPEC>`"]
pub type TLV_ADC10_1_TAG = crate::Reg<tlv_adc10_1_tag::TLV_ADC10_1_TAG_SPEC>;
#[doc = "TLV ADC10_1 TAG"]
pub mod tlv_adc10_1_tag;
#[doc = "TLV_CHECKSUM register accessor: an alias for `Reg<TLV_CHECKSUM_SPEC>`"]
pub type TLV_CHECKSUM = crate::Reg<tlv_checksum::TLV_CHECKSUM_SPEC>;
#[doc = "TLV CHECK SUM"]
pub mod tlv_checksum;
#[doc = "TLV_DCO_30_LEN register accessor: an alias for `Reg<TLV_DCO_30_LEN_SPEC>`"]
pub type TLV_DCO_30_LEN = crate::Reg<tlv_dco_30_len::TLV_DCO_30_LEN_SPEC>;
#[doc = "TLV TAG_DCO30 LEN"]
pub mod tlv_dco_30_len;
#[doc = "TLV_DCO_30_TAG register accessor: an alias for `Reg<TLV_DCO_30_TAG_SPEC>`"]
pub type TLV_DCO_30_TAG = crate::Reg<tlv_dco_30_tag::TLV_DCO_30_TAG_SPEC>;
#[doc = "TLV TAG_DCO30 TAG"]
pub mod tlv_dco_30_tag;
