#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
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
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIS` reader - WDTIS0"]
pub type WDTIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTIS` writer - WDTIS0"]
pub type WDTIS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, WDTCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDTSSEL` reader - WDTSSEL"]
pub type WDTSSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDTSSEL` writer - WDTSSEL"]
pub type WDTSSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTCNTCL` reader - WDTCNTCL"]
pub type WDTCNTCL_R = crate::BitReader<bool>;
#[doc = "Field `WDTCNTCL` writer - WDTCNTCL"]
pub type WDTCNTCL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTTMSEL` reader - WDTTMSEL"]
pub type WDTTMSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDTTMSEL` writer - WDTTMSEL"]
pub type WDTTMSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTNMI` reader - WDTNMI"]
pub type WDTNMI_R = crate::BitReader<bool>;
#[doc = "Field `WDTNMI` writer - WDTNMI"]
pub type WDTNMI_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTNMIES` reader - WDTNMIES"]
pub type WDTNMIES_R = crate::BitReader<bool>;
#[doc = "Field `WDTNMIES` writer - WDTNMIES"]
pub type WDTNMIES_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTHOLD` reader - WDTHOLD"]
pub type WDTHOLD_R = crate::BitReader<bool>;
#[doc = "Field `WDTHOLD` writer - WDTHOLD"]
pub type WDTHOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WDTPW_R = crate::FieldReader<u8, WDTPWR_A>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWR_A {
    #[doc = "105: Value always read from the Watchdog Password register"]
    PASSWORD = 105,
}
impl From<WDTPWR_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWR_A) -> Self {
        variant as _
    }
}
impl WDTPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTPWR_A> {
        match self.bits {
            105 => Some(WDTPWR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == WDTPWR_A::PASSWORD
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWW_AW {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    PASSWORD = 90,
}
impl From<WDTPWW_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WDTPW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, WDTCTL_SPEC, u8, WDTPWW_AW, 8, O>;
impl<'a, const O: u8> WDTPW_W<'a, O> {
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(WDTPWW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&self) -> WDTNMI_R {
        WDTNMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&self) -> WDTNMIES_R {
        WDTNMIES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDTIS0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W<0> {
        WDTIS_W::new(self)
    }
    #[doc = "Bit 2 - WDTSSEL"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<2> {
        WDTSSEL_W::new(self)
    }
    #[doc = "Bit 3 - WDTCNTCL"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<3> {
        WDTCNTCL_W::new(self)
    }
    #[doc = "Bit 4 - WDTTMSEL"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<4> {
        WDTTMSEL_W::new(self)
    }
    #[doc = "Bit 5 - WDTNMI"]
    #[inline(always)]
    pub fn wdtnmi(&mut self) -> WDTNMI_W<5> {
        WDTNMI_W::new(self)
    }
    #[doc = "Bit 6 - WDTNMIES"]
    #[inline(always)]
    pub fn wdtnmies(&mut self) -> WDTNMIES_W<6> {
        WDTNMIES_W::new(self)
    }
    #[doc = "Bit 7 - WDTHOLD"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W<7> {
        WDTHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W<8> {
        WDTPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
