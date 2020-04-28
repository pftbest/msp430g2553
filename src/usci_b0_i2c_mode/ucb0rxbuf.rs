#[doc = "Reader of register UCB0RXBUF"]
pub type R = crate::R<u8, super::UCB0RXBUF>;
#[doc = "Writer for register UCB0RXBUF"]
pub type W = crate::W<u8, super::UCB0RXBUF>;
#[doc = "Register UCB0RXBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0RXBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCB0RXBUF`"]
pub type UCB0RXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCB0RXBUF`"]
pub struct UCB0RXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCB0RXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI B0 Receive Buffer register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&self) -> UCB0RXBUF_R {
        UCB0RXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI B0 Receive Buffer register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&mut self) -> UCB0RXBUF_W {
        UCB0RXBUF_W { w: self }
    }
}
