#[doc = "Reader of register UCB0BR1"]
pub type R = crate::R<u8, super::UCB0BR1>;
#[doc = "Writer for register UCB0BR1"]
pub type W = crate::W<u8, super::UCB0BR1>;
#[doc = "Register UCB0BR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0BR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCB0BR1`"]
pub type UCB0BR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCB0BR1`"]
pub struct UCB0BR1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0BR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn ucb0br1(&self) -> UCB0BR1_R {
        UCB0BR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Baud Rate 1 register"]
    #[inline(always)]
    pub fn ucb0br1(&mut self) -> UCB0BR1_W {
        UCB0BR1_W { w: self }
    }
}
