#[doc = "Reader of register CALBC1_12MHZ"]
pub type R = crate::R<u8, super::CALBC1_12MHZ>;
#[doc = "Writer for register CALBC1_12MHZ"]
pub type W = crate::W<u8, super::CALBC1_12MHZ>;
#[doc = "Register CALBC1_12MHZ `reset()`'s with value 0"]
impl crate::ResetValue for super::CALBC1_12MHZ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALBC1_12MHZ`"]
pub type CALBC1_12MHZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALBC1_12MHZ`"]
pub struct CALBC1_12MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALBC1_12MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn calbc1_12mhz(&self) -> CALBC1_12MHZ_R {
        CALBC1_12MHZ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCSCTL1 Calibration Data for 12MHz register"]
    #[inline(always)]
    pub fn calbc1_12mhz(&mut self) -> CALBC1_12MHZ_W {
        CALBC1_12MHZ_W { w: self }
    }
}
