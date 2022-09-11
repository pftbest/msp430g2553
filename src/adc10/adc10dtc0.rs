#[doc = "Register `ADC10DTC0` reader"]
pub struct R(crate::R<ADC10DTC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC10DTC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC10DTC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC10DTC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC10DTC0` writer"]
pub struct W(crate::W<ADC10DTC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC10DTC0_SPEC>;
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
impl From<crate::W<ADC10DTC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC10DTC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10FETCH` reader - This bit should normally be reset"]
pub type ADC10FETCH_R = crate::BitReader<bool>;
#[doc = "Field `ADC10FETCH` writer - This bit should normally be reset"]
pub type ADC10FETCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, ADC10DTC0_SPEC, bool, O>;
#[doc = "Field `ADC10B1` reader - ADC10 block one"]
pub type ADC10B1_R = crate::BitReader<bool>;
#[doc = "Field `ADC10B1` writer - ADC10 block one"]
pub type ADC10B1_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, ADC10DTC0_SPEC, bool, O>;
#[doc = "Field `ADC10CT` reader - ADC10 continuous transfer"]
pub type ADC10CT_R = crate::BitReader<bool>;
#[doc = "Field `ADC10CT` writer - ADC10 continuous transfer"]
pub type ADC10CT_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, ADC10DTC0_SPEC, bool, O>;
#[doc = "Field `ADC10TB` reader - ADC10 two-block mode"]
pub type ADC10TB_R = crate::BitReader<bool>;
#[doc = "Field `ADC10TB` writer - ADC10 two-block mode"]
pub type ADC10TB_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, ADC10DTC0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&self) -> ADC10FETCH_R {
        ADC10FETCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&self) -> ADC10B1_R {
        ADC10B1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&self) -> ADC10CT_R {
        ADC10CT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&self) -> ADC10TB_R {
        ADC10TB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&mut self) -> ADC10FETCH_W<0> {
        ADC10FETCH_W::new(self)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&mut self) -> ADC10B1_W<1> {
        ADC10B1_W::new(self)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&mut self) -> ADC10CT_W<2> {
        ADC10CT_W::new(self)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&mut self) -> ADC10TB_W<3> {
        ADC10TB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC10 Data Transfer Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc10dtc0](index.html) module"]
pub struct ADC10DTC0_SPEC;
impl crate::RegisterSpec for ADC10DTC0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adc10dtc0::R](R) reader structure"]
impl crate::Readable for ADC10DTC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc10dtc0::W](W) writer structure"]
impl crate::Writable for ADC10DTC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC10DTC0 to value 0"]
impl crate::Resettable for ADC10DTC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
