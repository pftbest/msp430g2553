#[doc = "Register `UCA0RXBUF` reader"]
pub struct R(crate::R<UCA0RXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0RXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0RXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0RXBUF` writer"]
pub struct W(crate::W<UCA0RXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCA0RXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0RXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0RXBUF` reader - USCI A0 Receive Buffer register"]
pub struct UCA0RXBUF_R(crate::FieldReader<u8, u8>);
impl UCA0RXBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UCA0RXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCA0RXBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCA0RXBUF` writer - USCI A0 Receive Buffer register"]
pub struct UCA0RXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCA0RXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> UCA0RXBUF_R {
        UCA0RXBUF_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USCI A0 Receive Buffer register"]
    #[inline(always)]
    pub fn uca0rxbuf(&mut self) -> UCA0RXBUF_W {
        UCA0RXBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "USCI A0 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0rxbuf](index.html) module"]
pub struct UCA0RXBUF_SPEC;
impl crate::RegisterSpec for UCA0RXBUF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0rxbuf::R](R) reader structure"]
impl crate::Readable for UCA0RXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0rxbuf::W](W) writer structure"]
impl crate::Writable for UCA0RXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0RXBUF to value 0"]
impl crate::Resettable for UCA0RXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
