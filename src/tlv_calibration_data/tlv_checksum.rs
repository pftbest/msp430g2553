#[doc = "Register `TLV_CHECKSUM` reader"]
pub struct R(crate::R<TLV_CHECKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_CHECKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLV_CHECKSUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLV_CHECKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TLV_CHECKSUM` writer"]
pub struct W(crate::W<TLV_CHECKSUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLV_CHECKSUM_SPEC>;
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
impl From<crate::W<TLV_CHECKSUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLV_CHECKSUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLV_CHECKSUM` reader - TLV CHECK SUM register"]
pub struct TLV_CHECKSUM_R(crate::FieldReader<u16, u16>);
impl TLV_CHECKSUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TLV_CHECKSUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLV_CHECKSUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLV_CHECKSUM` writer - TLV CHECK SUM register"]
pub struct TLV_CHECKSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_CHECKSUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value as u16;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&self) -> TLV_CHECKSUM_R {
        TLV_CHECKSUM_R::new(self.bits as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TLV CHECK SUM register"]
    #[inline(always)]
    pub fn tlv_checksum(&mut self) -> TLV_CHECKSUM_W {
        TLV_CHECKSUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "TLV CHECK SUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_checksum](index.html) module"]
pub struct TLV_CHECKSUM_SPEC;
impl crate::RegisterSpec for TLV_CHECKSUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tlv_checksum::R](R) reader structure"]
impl crate::Readable for TLV_CHECKSUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tlv_checksum::W](W) writer structure"]
impl crate::Writable for TLV_CHECKSUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TLV_CHECKSUM to value 0"]
impl crate::Resettable for TLV_CHECKSUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
