#[doc = "Reader of register UCB0TXBUF"]
pub type R = crate::R<u8, super::UCB0TXBUF>;
#[doc = "Writer for register UCB0TXBUF"]
pub type W = crate::W<u8, super::UCB0TXBUF>;
#[doc = "Register UCB0TXBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0TXBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCB0TXBUF`"]
pub type UCB0TXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCB0TXBUF`"]
pub struct UCB0TXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0TXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> UCB0TXBUF_R {
        UCB0TXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Transmit Buffer register"]
    #[inline(always)]
    pub fn ucb0txbuf(&mut self) -> UCB0TXBUF_W {
        UCB0TXBUF_W { w: self }
    }
}
