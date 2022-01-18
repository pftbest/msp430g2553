#[doc = "Register `TLV_ADC10_1_TAG` reader"]
pub struct R(crate::R<TLV_ADC10_1_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_ADC10_1_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLV_ADC10_1_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLV_ADC10_1_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TLV_ADC10_1_TAG` writer"]
pub struct W(crate::W<TLV_ADC10_1_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLV_ADC10_1_TAG_SPEC>;
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
impl From<crate::W<TLV_ADC10_1_TAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLV_ADC10_1_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLV_ADC10_1_TAG` reader - TLV ADC10_1 TAG register"]
pub struct TLV_ADC10_1_TAG_R(crate::FieldReader<u8, u8>);
impl TLV_ADC10_1_TAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TLV_ADC10_1_TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLV_ADC10_1_TAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLV_ADC10_1_TAG` writer - TLV ADC10_1 TAG register"]
pub struct TLV_ADC10_1_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_ADC10_1_TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&self) -> TLV_ADC10_1_TAG_R {
        TLV_ADC10_1_TAG_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV ADC10_1 TAG register"]
    #[inline(always)]
    pub fn tlv_adc10_1_tag(&mut self) -> TLV_ADC10_1_TAG_W {
        TLV_ADC10_1_TAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "TLV ADC10_1 TAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_adc10_1_tag](index.html) module"]
pub struct TLV_ADC10_1_TAG_SPEC;
impl crate::RegisterSpec for TLV_ADC10_1_TAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tlv_adc10_1_tag::R](R) reader structure"]
impl crate::Readable for TLV_ADC10_1_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tlv_adc10_1_tag::W](W) writer structure"]
impl crate::Writable for TLV_ADC10_1_TAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TLV_ADC10_1_TAG to value 0"]
impl crate::Resettable for TLV_ADC10_1_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
