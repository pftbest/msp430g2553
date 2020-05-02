#[doc = "Reader of register ADC10SA"]
pub type R = crate::R<u16, super::ADC10SA>;
#[doc = "Writer for register ADC10SA"]
pub type W = crate::W<u16, super::ADC10SA>;
#[doc = "Register ADC10SA `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10SA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10SA`"]
pub type ADC10SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC10SA`"]
pub struct ADC10SA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&self) -> ADC10SA_R {
        ADC10SA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&mut self) -> ADC10SA_W {
        ADC10SA_W { w: self }
    }
}
