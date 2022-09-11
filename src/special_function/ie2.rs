#[doc = "Register `IE2` reader"]
pub struct R(crate::R<IE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE2` writer"]
pub struct W(crate::W<IE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE2_SPEC>;
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
impl From<crate::W<IE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0RXIE` reader - UCA0RXIE"]
pub type UCA0RXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCA0RXIE` writer - UCA0RXIE"]
pub type UCA0RXIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IE2_SPEC, bool, O>;
#[doc = "Field `UCA0TXIE` reader - UCA0TXIE"]
pub type UCA0TXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCA0TXIE` writer - UCA0TXIE"]
pub type UCA0TXIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IE2_SPEC, bool, O>;
#[doc = "Field `UCB0RXIE` reader - UCB0RXIE"]
pub type UCB0RXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCB0RXIE` writer - UCB0RXIE"]
pub type UCB0RXIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IE2_SPEC, bool, O>;
#[doc = "Field `UCB0TXIE` reader - UCB0TXIE"]
pub type UCB0TXIE_R = crate::BitReader<bool>;
#[doc = "Field `UCB0TXIE` writer - UCB0TXIE"]
pub type UCB0TXIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IE2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&self) -> UCA0RXIE_R {
        UCA0RXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&self) -> UCA0TXIE_R {
        UCA0TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&self) -> UCB0RXIE_R {
        UCB0RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&self) -> UCB0TXIE_R {
        UCB0TXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIE"]
    #[inline(always)]
    pub fn uca0rxie(&mut self) -> UCA0RXIE_W<0> {
        UCA0RXIE_W::new(self)
    }
    #[doc = "Bit 1 - UCA0TXIE"]
    #[inline(always)]
    pub fn uca0txie(&mut self) -> UCA0TXIE_W<1> {
        UCA0TXIE_W::new(self)
    }
    #[doc = "Bit 2 - UCB0RXIE"]
    #[inline(always)]
    pub fn ucb0rxie(&mut self) -> UCB0RXIE_W<2> {
        UCB0RXIE_W::new(self)
    }
    #[doc = "Bit 3 - UCB0TXIE"]
    #[inline(always)]
    pub fn ucb0txie(&mut self) -> UCB0TXIE_W<3> {
        UCB0TXIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie2](index.html) module"]
pub struct IE2_SPEC;
impl crate::RegisterSpec for IE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ie2::R](R) reader structure"]
impl crate::Readable for IE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie2::W](W) writer structure"]
impl crate::Writable for IE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE2 to value 0"]
impl crate::Resettable for IE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
