#[doc = "Reader of register UCA0RXBUF"]
pub type R = crate::R<u8, super::UCA0RXBUF>;
#[doc = "Writer for register UCA0RXBUF"]
pub type W = crate::W<u8, super::UCA0RXBUF>;
#[doc = "Register UCA0RXBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0RXBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCA0RXBUF`"]
pub type UCA0RXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCA0RXBUF`"]
pub struct UCA0RXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0RXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> UCA0RXBUF_R {
        UCA0RXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&mut self) -> UCA0RXBUF_W {
        UCA0RXBUF_W { w: self }
    }
}
