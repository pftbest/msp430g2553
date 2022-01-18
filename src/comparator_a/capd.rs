#[doc = "Register `CAPD` reader"]
pub struct R(crate::R<CAPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPD` writer"]
pub struct W(crate::W<CAPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPD_SPEC>;
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
impl From<crate::W<CAPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPD0` reader - Comp. A Disable Input Buffer of Port Register .0"]
pub struct CAPD0_R(crate::FieldReader<bool, bool>);
impl CAPD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD0` writer - Comp. A Disable Input Buffer of Port Register .0"]
pub struct CAPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `CAPD1` reader - Comp. A Disable Input Buffer of Port Register .1"]
pub struct CAPD1_R(crate::FieldReader<bool, bool>);
impl CAPD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD1` writer - Comp. A Disable Input Buffer of Port Register .1"]
pub struct CAPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CAPD2` reader - Comp. A Disable Input Buffer of Port Register .2"]
pub struct CAPD2_R(crate::FieldReader<bool, bool>);
impl CAPD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD2` writer - Comp. A Disable Input Buffer of Port Register .2"]
pub struct CAPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CAPD3` reader - Comp. A Disable Input Buffer of Port Register .3"]
pub struct CAPD3_R(crate::FieldReader<bool, bool>);
impl CAPD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD3` writer - Comp. A Disable Input Buffer of Port Register .3"]
pub struct CAPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CAPD4` reader - Comp. A Disable Input Buffer of Port Register .4"]
pub struct CAPD4_R(crate::FieldReader<bool, bool>);
impl CAPD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD4` writer - Comp. A Disable Input Buffer of Port Register .4"]
pub struct CAPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CAPD5` reader - Comp. A Disable Input Buffer of Port Register .5"]
pub struct CAPD5_R(crate::FieldReader<bool, bool>);
impl CAPD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD5` writer - Comp. A Disable Input Buffer of Port Register .5"]
pub struct CAPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CAPD6` reader - Comp. A Disable Input Buffer of Port Register .6"]
pub struct CAPD6_R(crate::FieldReader<bool, bool>);
impl CAPD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD6` writer - Comp. A Disable Input Buffer of Port Register .6"]
pub struct CAPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CAPD7` reader - Comp. A Disable Input Buffer of Port Register .7"]
pub struct CAPD7_R(crate::FieldReader<bool, bool>);
impl CAPD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD7` writer - Comp. A Disable Input Buffer of Port Register .7"]
pub struct CAPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CAPD` reader - Comparator A Port Disable register"]
pub struct CAPD_R(crate::FieldReader<u8, u8>);
impl CAPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPD` writer - Comparator A Port Disable register"]
pub struct CAPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&self) -> CAPD0_R {
        CAPD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&self) -> CAPD1_R {
        CAPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&self) -> CAPD2_R {
        CAPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&self) -> CAPD3_R {
        CAPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&self) -> CAPD4_R {
        CAPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&self) -> CAPD5_R {
        CAPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&self) -> CAPD6_R {
        CAPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&self) -> CAPD7_R {
        CAPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&self) -> CAPD_R {
        CAPD_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&mut self) -> CAPD0_W {
        CAPD0_W { w: self }
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&mut self) -> CAPD1_W {
        CAPD1_W { w: self }
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&mut self) -> CAPD2_W {
        CAPD2_W { w: self }
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&mut self) -> CAPD3_W {
        CAPD3_W { w: self }
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&mut self) -> CAPD4_W {
        CAPD4_W { w: self }
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&mut self) -> CAPD5_W {
        CAPD5_W { w: self }
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&mut self) -> CAPD6_W {
        CAPD6_W { w: self }
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&mut self) -> CAPD7_W {
        CAPD7_W { w: self }
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&mut self) -> CAPD_W {
        CAPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator A Port Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capd](index.html) module"]
pub struct CAPD_SPEC;
impl crate::RegisterSpec for CAPD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [capd::R](R) reader structure"]
impl crate::Readable for CAPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capd::W](W) writer structure"]
impl crate::Writable for CAPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPD to value 0"]
impl crate::Resettable for CAPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
