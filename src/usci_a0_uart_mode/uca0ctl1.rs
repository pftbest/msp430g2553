#[doc = "Register `UCA0CTL1` reader"]
pub struct R(crate::R<UCA0CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0CTL1` writer"]
pub struct W(crate::W<UCA0CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0CTL1_SPEC>;
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
impl From<crate::W<UCA0CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSWRST` reader - USCI Software Reset"]
pub type UCSWRST_R = crate::BitReader<bool>;
#[doc = "Field `UCSWRST` writer - USCI Software Reset"]
pub type UCSWRST_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCTXBRK` reader - Send next Data as Break"]
pub type UCTXBRK_R = crate::BitReader<bool>;
#[doc = "Field `UCTXBRK` writer - Send next Data as Break"]
pub type UCTXBRK_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCTXADDR` reader - Send next Data as Address"]
pub type UCTXADDR_R = crate::BitReader<bool>;
#[doc = "Field `UCTXADDR` writer - Send next Data as Address"]
pub type UCTXADDR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCDORM` reader - Dormant (Sleep) Mode"]
pub type UCDORM_R = crate::BitReader<bool>;
#[doc = "Field `UCDORM` writer - Dormant (Sleep) Mode"]
pub type UCDORM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCBRKIE` reader - Break interrupt enable"]
pub type UCBRKIE_R = crate::BitReader<bool>;
#[doc = "Field `UCBRKIE` writer - Break interrupt enable"]
pub type UCBRKIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCRXEIE` reader - RX Error interrupt enable"]
pub type UCRXEIE_R = crate::BitReader<bool>;
#[doc = "Field `UCRXEIE` writer - RX Error interrupt enable"]
pub type UCRXEIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCA0CTL1_SPEC, bool, O>;
#[doc = "Field `UCSSEL` reader - USCI 0 Clock Source Select 1"]
pub type UCSSEL_R = crate::FieldReader<u8, UCSSEL_A>;
#[doc = "USCI 0 Clock Source Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCSSEL_A {
    #[doc = "0: USCI 0 Clock Source: 0"]
    UCSSEL_0 = 0,
    #[doc = "1: USCI 0 Clock Source: 1"]
    UCSSEL_1 = 1,
    #[doc = "2: USCI 0 Clock Source: 2"]
    UCSSEL_2 = 2,
    #[doc = "3: USCI 0 Clock Source: 3"]
    UCSSEL_3 = 3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        variant as _
    }
}
impl UCSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCSSEL_0,
            1 => UCSSEL_A::UCSSEL_1,
            2 => UCSSEL_A::UCSSEL_2,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCSSEL_0`"]
    #[inline(always)]
    pub fn is_ucssel_0(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_0
    }
    #[doc = "Checks if the value of the field is `UCSSEL_1`"]
    #[inline(always)]
    pub fn is_ucssel_1(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_1
    }
    #[doc = "Checks if the value of the field is `UCSSEL_2`"]
    #[inline(always)]
    pub fn is_ucssel_2(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_2
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Field `UCSSEL` writer - USCI 0 Clock Source Select 1"]
pub type UCSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCA0CTL1_SPEC, u8, UCSSEL_A, 2, O>;
impl<'a, const O: u8> UCSSEL_W<'a, O> {
    #[doc = "USCI 0 Clock Source: 0"]
    #[inline(always)]
    pub fn ucssel_0(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_0)
    }
    #[doc = "USCI 0 Clock Source: 1"]
    #[inline(always)]
    pub fn ucssel_1(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_1)
    }
    #[doc = "USCI 0 Clock Source: 2"]
    #[inline(always)]
    pub fn ucssel_2(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_2)
    }
    #[doc = "USCI 0 Clock Source: 3"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
}
impl R {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send next Data as Break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UCTXBRK_R {
        UCTXBRK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Send next Data as Address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UCTXADDR_R {
        UCTXADDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dormant (Sleep) Mode"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UCDORM_R {
        UCDORM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UCBRKIE_R {
        UCBRKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UCRXEIE_R {
        UCRXEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI 0 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Software Reset"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W<0> {
        UCSWRST_W::new(self)
    }
    #[doc = "Bit 1 - Send next Data as Break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UCTXBRK_W<1> {
        UCTXBRK_W::new(self)
    }
    #[doc = "Bit 2 - Send next Data as Address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UCTXADDR_W<2> {
        UCTXADDR_W::new(self)
    }
    #[doc = "Bit 3 - Dormant (Sleep) Mode"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UCDORM_W<3> {
        UCDORM_W::new(self)
    }
    #[doc = "Bit 4 - Break interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UCBRKIE_W<4> {
        UCBRKIE_W::new(self)
    }
    #[doc = "Bit 5 - RX Error interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UCRXEIE_W<5> {
        UCRXEIE_W::new(self)
    }
    #[doc = "Bits 6:7 - USCI 0 Clock Source Select 1"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W<6> {
        UCSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ctl1](index.html) module"]
pub struct UCA0CTL1_SPEC;
impl crate::RegisterSpec for UCA0CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ctl1::R](R) reader structure"]
impl crate::Readable for UCA0CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ctl1::W](W) writer structure"]
impl crate::Writable for UCA0CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0CTL1 to value 0"]
impl crate::Resettable for UCA0CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
