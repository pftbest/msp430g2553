#[doc = "Reader of register CALDCO_1MHZ"]
pub type R = crate::R<u8, super::CALDCO_1MHZ>;
#[doc = "Writer for register CALDCO_1MHZ"]
pub type W = crate::W<u8, super::CALDCO_1MHZ>;
#[doc = "Register CALDCO_1MHZ `reset()`'s with value 0"]
impl crate::ResetValue for super::CALDCO_1MHZ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALDCO_1MHZ`"]
pub type CALDCO_1MHZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALDCO_1MHZ`"]
pub struct CALDCO_1MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDCO_1MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn caldco_1mhz(&self) -> CALDCO_1MHZ_R {
        CALDCO_1MHZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn caldco_1mhz(&mut self) -> CALDCO_1MHZ_W {
        CALDCO_1MHZ_W { w: self }
    }
}
