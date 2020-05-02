#[doc = "Reader of register TLV_ADC10_1_TAG"]
pub type R = crate::R<u8, super::TLV_ADC10_1_TAG>;
#[doc = "Writer for register TLV_ADC10_1_TAG"]
pub type W = crate::W<u8, super::TLV_ADC10_1_TAG>;
#[doc = "Register TLV_ADC10_1_TAG `reset()`'s with value 0"]
impl crate::ResetValue for super::TLV_ADC10_1_TAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLV_ADC10_1_TAG`"]
pub type TLV_ADC10_1_TAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLV_ADC10_1_TAG`"]
pub struct TLV_ADC10_1_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_ADC10_1_TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&self) -> TLV_ADC10_1_TAG_R {
        TLV_ADC10_1_TAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&mut self) -> TLV_ADC10_1_TAG_W {
        TLV_ADC10_1_TAG_W { w: self }
    }
}
