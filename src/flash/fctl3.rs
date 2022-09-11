#[doc = "Register `FCTL3` reader"]
pub struct R(crate::R<FCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTL3` writer"]
pub struct W(crate::W<FCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTL3_SPEC>;
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
impl From<crate::W<FCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - Flash busy: 1"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - Flash busy: 1"]
pub type BUSY_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `KEYV` reader - Flash Key violation flag"]
pub type KEYV_R = crate::BitReader<bool>;
#[doc = "Field `KEYV` writer - Flash Key violation flag"]
pub type KEYV_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `ACCVIFG` reader - Flash Access violation flag"]
pub type ACCVIFG_R = crate::BitReader<bool>;
#[doc = "Field `ACCVIFG` writer - Flash Access violation flag"]
pub type ACCVIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `WAIT` reader - Wait flag for segment write"]
pub type WAIT_R = crate::BitReader<bool>;
#[doc = "Field `WAIT` writer - Wait flag for segment write"]
pub type WAIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock bit: 1 - Flash is locked (read only)"]
pub type LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `EMEX` reader - Flash Emergency Exit"]
pub type EMEX_R = crate::BitReader<bool>;
#[doc = "Field `EMEX` writer - Flash Emergency Exit"]
pub type EMEX_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `LOCKA` reader - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_R = crate::BitReader<bool>;
#[doc = "Field `LOCKA` writer - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
pub type LOCKA_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `FAIL` reader - Last Program or Erase failed"]
pub type FAIL_R = crate::BitReader<bool>;
#[doc = "Field `FAIL` writer - Last Program or Erase failed"]
pub type FAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, FCTL3_SPEC, bool, O>;
#[doc = "Field `FWKEY` reader - FCTL3 Password"]
pub type FWKEY_R = crate::FieldReader<u8, FWKEYR_A>;
#[doc = "FCTL3 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYR_A {
    #[doc = "150: Value always read from the FCTL3 Password register"]
    PASSWORD = 150,
}
impl From<FWKEYR_A> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYR_A) -> Self {
        variant as _
    }
}
impl FWKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FWKEYR_A> {
        match self.bits {
            150 => Some(FWKEYR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == FWKEYR_A::PASSWORD
    }
}
#[doc = "FCTL3 Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FWKEYW_AW {
    #[doc = "165: Value which must be written to the FCTL3 Password register"]
    PASSWORD = 165,
}
impl From<FWKEYW_AW> for u8 {
    #[inline(always)]
    fn from(variant: FWKEYW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `FWKEY` writer - FCTL3 Password"]
pub type FWKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, FCTL3_SPEC, u8, FWKEYW_AW, 8, O>;
impl<'a, const O: u8> FWKEY_W<'a, O> {
    #[doc = "Value which must be written to the FCTL3 Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(FWKEYW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&self) -> KEYV_R {
        KEYV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&self) -> ACCVIFG_R {
        ACCVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&self) -> EMEX_R {
        EMEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - FCTL3 Password"]
    #[inline(always)]
    pub fn fwkey(&self) -> FWKEY_R {
        FWKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash busy: 1"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - Flash Key violation flag"]
    #[inline(always)]
    pub fn keyv(&mut self) -> KEYV_W<1> {
        KEYV_W::new(self)
    }
    #[doc = "Bit 2 - Flash Access violation flag"]
    #[inline(always)]
    pub fn accvifg(&mut self) -> ACCVIFG_W<2> {
        ACCVIFG_W::new(self)
    }
    #[doc = "Bit 3 - Wait flag for segment write"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<3> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 4 - Lock bit: 1 - Flash is locked (read only)"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<4> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 5 - Flash Emergency Exit"]
    #[inline(always)]
    pub fn emex(&mut self) -> EMEX_W<5> {
        EMEX_W::new(self)
    }
    #[doc = "Bit 6 - Segment A Lock bit: read = 1 - Segment is locked (read only)"]
    #[inline(always)]
    pub fn locka(&mut self) -> LOCKA_W<6> {
        LOCKA_W::new(self)
    }
    #[doc = "Bit 7 - Last Program or Erase failed"]
    #[inline(always)]
    pub fn fail(&mut self) -> FAIL_W<7> {
        FAIL_W::new(self)
    }
    #[doc = "Bits 8:15 - FCTL3 Password"]
    #[inline(always)]
    pub fn fwkey(&mut self) -> FWKEY_W<8> {
        FWKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl3](index.html) module"]
pub struct FCTL3_SPEC;
impl crate::RegisterSpec for FCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctl3::R](R) reader structure"]
impl crate::Readable for FCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctl3::W](W) writer structure"]
impl crate::Writable for FCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTL3 to value 0"]
impl crate::Resettable for FCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
