#[doc = "Reader of register TACCR1"]
pub type R = crate::R<u16, super::TACCR1>;
#[doc = "Writer for register TACCR1"]
pub type W = crate::W<u16, super::TACCR1>;
#[doc = "Register TACCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TACCR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TACCR1`"]
pub type TACCR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TACCR1`"]
pub struct TACCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&self) -> TACCR1_R {
        TACCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 1"]
    #[inline(always)]
    pub fn taccr1(&mut self) -> TACCR1_W {
        TACCR1_W { w: self }
    }
}
