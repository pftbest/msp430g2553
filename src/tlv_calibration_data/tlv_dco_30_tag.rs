#[doc = "Register `TLV_DCO_30_TAG` reader"]
pub struct R(crate::R<TLV_DCO_30_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_DCO_30_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TLV_DCO_30_TAG_SPEC>> for R {
    fn from(reader: crate::R<TLV_DCO_30_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TLV_DCO_30_TAG` writer"]
pub struct W(crate::W<TLV_DCO_30_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLV_DCO_30_TAG_SPEC>;
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
impl core::convert::From<crate::W<TLV_DCO_30_TAG_SPEC>> for W {
    fn from(writer: crate::W<TLV_DCO_30_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLV_DCO_30_TAG` reader - TLV TAG_DCO30 TAG register"]
pub struct TLV_DCO_30_TAG_R(crate::FieldReader<u8, u8>);
impl TLV_DCO_30_TAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLV_DCO_30_TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLV_DCO_30_TAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLV_DCO_30_TAG` writer - TLV TAG_DCO30 TAG register"]
pub struct TLV_DCO_30_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_DCO_30_TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u8 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&self) -> TLV_DCO_30_TAG_R {
        TLV_DCO_30_TAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&mut self) -> TLV_DCO_30_TAG_W {
        TLV_DCO_30_TAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TLV TAG_DCO30 TAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_dco_30_tag](index.html) module"]
pub struct TLV_DCO_30_TAG_SPEC;
impl crate::RegisterSpec for TLV_DCO_30_TAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tlv_dco_30_tag::R](R) reader structure"]
impl crate::Readable for TLV_DCO_30_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tlv_dco_30_tag::W](W) writer structure"]
impl crate::Writable for TLV_DCO_30_TAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TLV_DCO_30_TAG to value 0"]
impl crate::Resettable for TLV_DCO_30_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
