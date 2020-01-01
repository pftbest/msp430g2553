#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TLV CHECK SUM"]
    pub tlv_checksum: TLV_CHECKSUM,
    _reserved1: [u8; 24usize],
    #[doc = "0x1a - TLV ADC10_1 TAG"]
    pub tlv_adc10_1_tag: TLV_ADC10_1_TAG,
    #[doc = "0x1b - TLV ADC10_1 LEN"]
    pub tlv_adc10_1_len: TLV_ADC10_1_LEN,
    _reserved3: [u8; 26usize],
    #[doc = "0x36 - TLV TAG_DCO30 TAG"]
    pub tlv_dco_30_tag: TLV_DCO_30_TAG,
    #[doc = "0x37 - TLV TAG_DCO30 LEN"]
    pub tlv_dco_30_len: TLV_DCO_30_LEN,
}
#[doc = "TLV ADC10_1 TAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_adc10_1_tag](tlv_adc10_1_tag) module"]
pub type TLV_ADC10_1_TAG = crate::Reg<u8, _TLV_ADC10_1_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_ADC10_1_TAG;
#[doc = "`read()` method returns [tlv_adc10_1_tag::R](tlv_adc10_1_tag::R) reader structure"]
impl crate::Readable for TLV_ADC10_1_TAG {}
#[doc = "`write(|w| ..)` method takes [tlv_adc10_1_tag::W](tlv_adc10_1_tag::W) writer structure"]
impl crate::Writable for TLV_ADC10_1_TAG {}
#[doc = "TLV ADC10_1 TAG"]
pub mod tlv_adc10_1_tag;
#[doc = "TLV ADC10_1 LEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_adc10_1_len](tlv_adc10_1_len) module"]
pub type TLV_ADC10_1_LEN = crate::Reg<u8, _TLV_ADC10_1_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_ADC10_1_LEN;
#[doc = "`read()` method returns [tlv_adc10_1_len::R](tlv_adc10_1_len::R) reader structure"]
impl crate::Readable for TLV_ADC10_1_LEN {}
#[doc = "`write(|w| ..)` method takes [tlv_adc10_1_len::W](tlv_adc10_1_len::W) writer structure"]
impl crate::Writable for TLV_ADC10_1_LEN {}
#[doc = "TLV ADC10_1 LEN"]
pub mod tlv_adc10_1_len;
#[doc = "TLV TAG_DCO30 TAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_dco_30_tag](tlv_dco_30_tag) module"]
pub type TLV_DCO_30_TAG = crate::Reg<u8, _TLV_DCO_30_TAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_DCO_30_TAG;
#[doc = "`read()` method returns [tlv_dco_30_tag::R](tlv_dco_30_tag::R) reader structure"]
impl crate::Readable for TLV_DCO_30_TAG {}
#[doc = "`write(|w| ..)` method takes [tlv_dco_30_tag::W](tlv_dco_30_tag::W) writer structure"]
impl crate::Writable for TLV_DCO_30_TAG {}
#[doc = "TLV TAG_DCO30 TAG"]
pub mod tlv_dco_30_tag;
#[doc = "TLV TAG_DCO30 LEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_dco_30_len](tlv_dco_30_len) module"]
pub type TLV_DCO_30_LEN = crate::Reg<u8, _TLV_DCO_30_LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_DCO_30_LEN;
#[doc = "`read()` method returns [tlv_dco_30_len::R](tlv_dco_30_len::R) reader structure"]
impl crate::Readable for TLV_DCO_30_LEN {}
#[doc = "`write(|w| ..)` method takes [tlv_dco_30_len::W](tlv_dco_30_len::W) writer structure"]
impl crate::Writable for TLV_DCO_30_LEN {}
#[doc = "TLV TAG_DCO30 LEN"]
pub mod tlv_dco_30_len;
#[doc = "TLV CHECK SUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_checksum](tlv_checksum) module"]
pub type TLV_CHECKSUM = crate::Reg<u16, _TLV_CHECKSUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TLV_CHECKSUM;
#[doc = "`read()` method returns [tlv_checksum::R](tlv_checksum::R) reader structure"]
impl crate::Readable for TLV_CHECKSUM {}
#[doc = "`write(|w| ..)` method takes [tlv_checksum::W](tlv_checksum::W) writer structure"]
impl crate::Writable for TLV_CHECKSUM {}
#[doc = "TLV CHECK SUM"]
pub mod tlv_checksum;
