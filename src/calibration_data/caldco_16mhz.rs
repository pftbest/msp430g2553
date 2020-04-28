#[doc = "Reader of register CALDCO_16MHZ"]
pub type R = crate::R<u8, super::CALDCO_16MHZ>;
#[doc = "Writer for register CALDCO_16MHZ"]
pub type W = crate::W<u8, super::CALDCO_16MHZ>;
#[doc = "Register CALDCO_16MHZ `reset()`'s with value 0"]
impl crate::ResetValue for super::CALDCO_16MHZ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALDCO_16MHZ`"]
pub type CALDCO_16MHZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALDCO_16MHZ`"]
pub struct CALDCO_16MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDCO_16MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&self) -> CALDCO_16MHZ_R {
        CALDCO_16MHZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 16MHz register"]
    #[inline(always)]
    pub fn caldco_16mhz(&mut self) -> CALDCO_16MHZ_W {
        CALDCO_16MHZ_W { w: self }
    }
}
