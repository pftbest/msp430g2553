#[doc = "Reader of register ADC10AE0"]
pub type R = crate::R<u8, super::ADC10AE0>;
#[doc = "Writer for register ADC10AE0"]
pub type W = crate::W<u8, super::ADC10AE0>;
#[doc = "Register ADC10AE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10AE0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10AE0`"]
pub type ADC10AE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC10AE0`"]
pub struct ADC10AE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10AE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC10 Analog Enable 0 register"]
    #[inline(always)]
    pub fn adc10ae0(&self) -> ADC10AE0_R {
        ADC10AE0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC10 Analog Enable 0 register"]
    #[inline(always)]
    pub fn adc10ae0(&mut self) -> ADC10AE0_W {
        ADC10AE0_W { w: self }
    }
}
