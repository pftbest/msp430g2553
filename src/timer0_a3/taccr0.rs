#[doc = "Reader of register TACCR0"]
pub type R = crate::R<u16, super::TACCR0>;
#[doc = "Writer for register TACCR0"]
pub type W = crate::W<u16, super::TACCR0>;
#[doc = "Register TACCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TACCR0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TACCR0`"]
pub type TACCR0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TACCR0`"]
pub struct TACCR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TACCR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&self) -> TACCR0_R {
        TACCR0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer A Capture/Compare register 0"]
    #[inline(always)]
    pub fn taccr0(&mut self) -> TACCR0_W {
        TACCR0_W { w: self }
    }
}
