#[doc = "Reader of register TACCR2"]
pub type R = crate::R<u16, super::TACCR2>;
#[doc = "Writer for register TACCR2"]
pub type W = crate::W<u16, super::TACCR2>;
#[doc = "Register TACCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TACCR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TACCR2`"]
pub type TACCR2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TACCR2`"]
pub struct TACCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&self) -> TACCR2_R {
        TACCR2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 2"]
    #[inline(always)]
    pub fn taccr2(&mut self) -> TACCR2_W {
        TACCR2_W { w: self }
    }
}
