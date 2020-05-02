#[doc = "Reader of register UCA0TXBUF"]
pub type R = crate::R<u8, super::UCA0TXBUF>;
#[doc = "Writer for register UCA0TXBUF"]
pub type W = crate::W<u8, super::UCA0TXBUF>;
#[doc = "Register UCA0TXBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0TXBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCA0TXBUF`"]
pub type UCA0TXBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UCA0TXBUF`"]
pub struct UCA0TXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0TXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> UCA0TXBUF_R {
        UCA0TXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Transmit Buffer register"]
    #[inline(always)]
    pub fn uca0txbuf(&mut self) -> UCA0TXBUF_W {
        UCA0TXBUF_W { w: self }
    }
}
