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
pub type CAPD0_R = crate::BitReader<bool>;
#[doc = "Field `CAPD0` writer - Comp. A Disable Input Buffer of Port Register .0"]
pub type CAPD0_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD` reader - Comparator A Port Disable register"]
pub type CAPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPD` writer - Comparator A Port Disable register"]
pub type CAPD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CAPD_SPEC, u8, u8, 8, O>;
#[doc = "Field `CAPD1` reader - Comp. A Disable Input Buffer of Port Register .1"]
pub type CAPD1_R = crate::BitReader<bool>;
#[doc = "Field `CAPD1` writer - Comp. A Disable Input Buffer of Port Register .1"]
pub type CAPD1_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD2` reader - Comp. A Disable Input Buffer of Port Register .2"]
pub type CAPD2_R = crate::BitReader<bool>;
#[doc = "Field `CAPD2` writer - Comp. A Disable Input Buffer of Port Register .2"]
pub type CAPD2_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD3` reader - Comp. A Disable Input Buffer of Port Register .3"]
pub type CAPD3_R = crate::BitReader<bool>;
#[doc = "Field `CAPD3` writer - Comp. A Disable Input Buffer of Port Register .3"]
pub type CAPD3_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD4` reader - Comp. A Disable Input Buffer of Port Register .4"]
pub type CAPD4_R = crate::BitReader<bool>;
#[doc = "Field `CAPD4` writer - Comp. A Disable Input Buffer of Port Register .4"]
pub type CAPD4_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD5` reader - Comp. A Disable Input Buffer of Port Register .5"]
pub type CAPD5_R = crate::BitReader<bool>;
#[doc = "Field `CAPD5` writer - Comp. A Disable Input Buffer of Port Register .5"]
pub type CAPD5_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD6` reader - Comp. A Disable Input Buffer of Port Register .6"]
pub type CAPD6_R = crate::BitReader<bool>;
#[doc = "Field `CAPD6` writer - Comp. A Disable Input Buffer of Port Register .6"]
pub type CAPD6_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
#[doc = "Field `CAPD7` reader - Comp. A Disable Input Buffer of Port Register .7"]
pub type CAPD7_R = crate::BitReader<bool>;
#[doc = "Field `CAPD7` writer - Comp. A Disable Input Buffer of Port Register .7"]
pub type CAPD7_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CAPD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&self) -> CAPD0_R {
        CAPD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&self) -> CAPD_R {
        CAPD_R::new(self.bits)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&self) -> CAPD1_R {
        CAPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&self) -> CAPD2_R {
        CAPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&self) -> CAPD3_R {
        CAPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&self) -> CAPD4_R {
        CAPD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&self) -> CAPD5_R {
        CAPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&self) -> CAPD6_R {
        CAPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&self) -> CAPD7_R {
        CAPD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Disable Input Buffer of Port Register .0"]
    #[inline(always)]
    pub fn capd0(&mut self) -> CAPD0_W<0> {
        CAPD0_W::new(self)
    }
    #[doc = "Bits 0:7 - Comparator A Port Disable register"]
    #[inline(always)]
    pub fn capd(&mut self) -> CAPD_W<0> {
        CAPD_W::new(self)
    }
    #[doc = "Bit 1 - Comp. A Disable Input Buffer of Port Register .1"]
    #[inline(always)]
    pub fn capd1(&mut self) -> CAPD1_W<1> {
        CAPD1_W::new(self)
    }
    #[doc = "Bit 2 - Comp. A Disable Input Buffer of Port Register .2"]
    #[inline(always)]
    pub fn capd2(&mut self) -> CAPD2_W<2> {
        CAPD2_W::new(self)
    }
    #[doc = "Bit 3 - Comp. A Disable Input Buffer of Port Register .3"]
    #[inline(always)]
    pub fn capd3(&mut self) -> CAPD3_W<3> {
        CAPD3_W::new(self)
    }
    #[doc = "Bit 4 - Comp. A Disable Input Buffer of Port Register .4"]
    #[inline(always)]
    pub fn capd4(&mut self) -> CAPD4_W<4> {
        CAPD4_W::new(self)
    }
    #[doc = "Bit 5 - Comp. A Disable Input Buffer of Port Register .5"]
    #[inline(always)]
    pub fn capd5(&mut self) -> CAPD5_W<5> {
        CAPD5_W::new(self)
    }
    #[doc = "Bit 6 - Comp. A Disable Input Buffer of Port Register .6"]
    #[inline(always)]
    pub fn capd6(&mut self) -> CAPD6_W<6> {
        CAPD6_W::new(self)
    }
    #[doc = "Bit 7 - Comp. A Disable Input Buffer of Port Register .7"]
    #[inline(always)]
    pub fn capd7(&mut self) -> CAPD7_W<7> {
        CAPD7_W::new(self)
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
