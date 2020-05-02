#[doc = "Reader of register ADC10MEM"]
pub type R = crate::R<u16, super::ADC10MEM>;
#[doc = "Writer for register ADC10MEM"]
pub type W = crate::W<u16, super::ADC10MEM>;
#[doc = "Register ADC10MEM `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC10MEM {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC10MEM`"]
pub type ADC10MEM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADC10MEM`"]
pub struct ADC10MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10MEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> ADC10MEM_R {
        ADC10MEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&mut self) -> ADC10MEM_W {
        ADC10MEM_W { w: self }
    }
}
