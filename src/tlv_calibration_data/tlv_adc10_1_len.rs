#[doc = "Reader of register TLV_ADC10_1_LEN"]
pub type R = crate::R<u8, super::TLV_ADC10_1_LEN>;
#[doc = "Writer for register TLV_ADC10_1_LEN"]
pub type W = crate::W<u8, super::TLV_ADC10_1_LEN>;
#[doc = "Register TLV_ADC10_1_LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::TLV_ADC10_1_LEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLV_ADC10_1_LEN`"]
pub type TLV_ADC10_1_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLV_ADC10_1_LEN`"]
pub struct TLV_ADC10_1_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_ADC10_1_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TLV ADC10_1 LEN register"]
    #[inline(always)]
    pub fn tlv_adc10_1_len(&self) -> TLV_ADC10_1_LEN_R {
        TLV_ADC10_1_LEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV ADC10_1 LEN register"]
    #[inline(always)]
    pub fn tlv_adc10_1_len(&mut self) -> TLV_ADC10_1_LEN_W {
        TLV_ADC10_1_LEN_W { w: self }
    }
}
