#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTCTL_SPEC>> for R {
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
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
impl core::convert::From<crate::W<WDTCTL_SPEC>> for W {
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub struct WDTSSEL_R(crate::FieldReader<bool, bool>);
impl WDTSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTSSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub struct WDTCNTCL_R(crate::FieldReader<bool, bool>);
impl WDTCNTCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTCNTCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTCNTCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub struct WDTTMSEL_R(crate::FieldReader<bool, bool>);
impl WDTTMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTTMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTTMSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub struct WDTNMI_R(crate::FieldReader<bool, bool>);
impl WDTNMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTNMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTNMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub struct WDTNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTNMI_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub struct WDTNMIES_R(crate::FieldReader<bool, bool>);
impl WDTNMIES_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTNMIES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTNMIES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub struct WDTNMIES_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTNMIES_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub struct WDTHOLD_R(crate::FieldReader<bool, bool>);
impl WDTHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTPW_A {
    #[doc = "105: Value always read from the Watchdog Password register"]
    PASSWORD = 105,
}
impl From<WDTPW_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub struct WDTPW_R(crate::FieldReader<u8, WDTPW_A>);
impl WDTPW_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTPW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTPW_A> {
        match self.bits {
            105 => Some(WDTPW_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        **self == WDTPW_A::PASSWORD
    }
}
impl core::ops::Deref for WDTPW_R {
    type Target = crate::FieldReader<u8, WDTPW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTPW_AW {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    PASSWORD = 90,
}
impl From<WDTPW_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDTPW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub struct WDTPW_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTPW_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(WDTPW_AW::PASSWORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `WDTIS` reader - WDTIS0"]
pub struct WDTIS_R(crate::FieldReader<u8, u8>);
impl WDTIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIS` writer - WDTIS0"]
pub struct WDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WDTNMI_R {
        WDTNMI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WDTNMIES_R {
        WDTNMIES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WDTNMI_W {
        WDTNMI_W { w: self }
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WDTNMIES_W {
        WDTNMIES_W { w: self }
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W {
        WDTPW_W { w: self }
    }
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W {
        WDTIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
