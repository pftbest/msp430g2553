#[doc = "Reader of register ADC10DTC1"]
pub type R = crate::R<u8, super::ADC10DTC1>;
#[doc = "Writer for register ADC10DTC1"]
pub type W = crate::W<u8, super::ADC10DTC1>;
#[doc = "Register ADC10DTC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10DTC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10DTC1`"]
pub type ADC10DTC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC10DTC1`"]
pub struct ADC10DTC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10DTC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&self) -> ADC10DTC1_R {
        ADC10DTC1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&mut self) -> ADC10DTC1_W {
        ADC10DTC1_W { w: self }
    }
}
