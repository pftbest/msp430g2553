#[doc = "Register `IFG2` reader"]
pub struct R(crate::R<IFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFG2` writer"]
pub struct W(crate::W<IFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFG2_SPEC>;
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
impl From<crate::W<IFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCA0RXIFG` reader - UCA0RXIFG"]
pub type UCA0RXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCA0RXIFG` writer - UCA0RXIFG"]
pub type UCA0RXIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG2_SPEC, bool, O>;
#[doc = "Field `UCA0TXIFG` reader - UCA0TXIFG"]
pub type UCA0TXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCA0TXIFG` writer - UCA0TXIFG"]
pub type UCA0TXIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG2_SPEC, bool, O>;
#[doc = "Field `UCB0RXIFG` reader - UCB0RXIFG"]
pub type UCB0RXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCB0RXIFG` writer - UCB0RXIFG"]
pub type UCB0RXIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG2_SPEC, bool, O>;
#[doc = "Field `UCB0TXIFG` reader - UCB0TXIFG"]
pub type UCB0TXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCB0TXIFG` writer - UCB0TXIFG"]
pub type UCB0TXIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, IFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&self) -> UCA0RXIFG_R {
        UCA0RXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&self) -> UCA0TXIFG_R {
        UCA0TXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&self) -> UCB0RXIFG_R {
        UCB0RXIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&self) -> UCB0TXIFG_R {
        UCB0TXIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UCA0RXIFG"]
    #[inline(always)]
    pub fn uca0rxifg(&mut self) -> UCA0RXIFG_W<0> {
        UCA0RXIFG_W::new(self)
    }
    #[doc = "Bit 1 - UCA0TXIFG"]
    #[inline(always)]
    pub fn uca0txifg(&mut self) -> UCA0TXIFG_W<1> {
        UCA0TXIFG_W::new(self)
    }
    #[doc = "Bit 2 - UCB0RXIFG"]
    #[inline(always)]
    pub fn ucb0rxifg(&mut self) -> UCB0RXIFG_W<2> {
        UCB0RXIFG_W::new(self)
    }
    #[doc = "Bit 3 - UCB0TXIFG"]
    #[inline(always)]
    pub fn ucb0txifg(&mut self) -> UCB0TXIFG_W<3> {
        UCB0TXIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifg2](index.html) module"]
pub struct IFG2_SPEC;
impl crate::RegisterSpec for IFG2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ifg2::R](R) reader structure"]
impl crate::Readable for IFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifg2::W](W) writer structure"]
impl crate::Writable for IFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFG2 to value 0"]
impl crate::Resettable for IFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
