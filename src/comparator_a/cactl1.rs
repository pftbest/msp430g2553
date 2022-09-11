#[doc = "Register `CACTL1` reader"]
pub struct R(crate::R<CACTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACTL1` writer"]
pub struct W(crate::W<CACTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACTL1_SPEC>;
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
impl From<crate::W<CACTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAIFG` reader - Comp. A Interrupt Flag"]
pub type CAIFG_R = crate::BitReader<bool>;
#[doc = "Field `CAIFG` writer - Comp. A Interrupt Flag"]
pub type CAIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
#[doc = "Field `CAIE` reader - Comp. A Interrupt Enable"]
pub type CAIE_R = crate::BitReader<bool>;
#[doc = "Field `CAIE` writer - Comp. A Interrupt Enable"]
pub type CAIE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
#[doc = "Field `CAIES` reader - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CAIES_R = crate::BitReader<bool>;
#[doc = "Field `CAIES` writer - Comp. A Int. Edge Select: 0:rising / 1:falling"]
pub type CAIES_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
#[doc = "Field `CAON` reader - Comp. A enable"]
pub type CAON_R = crate::BitReader<bool>;
#[doc = "Field `CAON` writer - Comp. A enable"]
pub type CAON_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
#[doc = "Field `CAREF` reader - Comp. A Internal Reference Select 0"]
pub type CAREF_R = crate::FieldReader<u8, CAREF_A>;
#[doc = "Comp. A Internal Reference Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAREF_A {
    #[doc = "0: Comp. A Int. Ref. Select 0 : Off"]
    CAREF_0 = 0,
    #[doc = "1: Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    CAREF_1 = 1,
    #[doc = "2: Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    CAREF_2 = 2,
    #[doc = "3: Comp. A Int. Ref. Select 3 : Vt"]
    CAREF_3 = 3,
}
impl From<CAREF_A> for u8 {
    #[inline(always)]
    fn from(variant: CAREF_A) -> Self {
        variant as _
    }
}
impl CAREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAREF_A {
        match self.bits {
            0 => CAREF_A::CAREF_0,
            1 => CAREF_A::CAREF_1,
            2 => CAREF_A::CAREF_2,
            3 => CAREF_A::CAREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAREF_0`"]
    #[inline(always)]
    pub fn is_caref_0(&self) -> bool {
        *self == CAREF_A::CAREF_0
    }
    #[doc = "Checks if the value of the field is `CAREF_1`"]
    #[inline(always)]
    pub fn is_caref_1(&self) -> bool {
        *self == CAREF_A::CAREF_1
    }
    #[doc = "Checks if the value of the field is `CAREF_2`"]
    #[inline(always)]
    pub fn is_caref_2(&self) -> bool {
        *self == CAREF_A::CAREF_2
    }
    #[doc = "Checks if the value of the field is `CAREF_3`"]
    #[inline(always)]
    pub fn is_caref_3(&self) -> bool {
        *self == CAREF_A::CAREF_3
    }
}
#[doc = "Field `CAREF` writer - Comp. A Internal Reference Select 0"]
pub type CAREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CACTL1_SPEC, u8, CAREF_A, 2, O>;
impl<'a, const O: u8> CAREF_W<'a, O> {
    #[doc = "Comp. A Int. Ref. Select 0 : Off"]
    #[inline(always)]
    pub fn caref_0(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_0)
    }
    #[doc = "Comp. A Int. Ref. Select 1 : 0.25*Vcc"]
    #[inline(always)]
    pub fn caref_1(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_1)
    }
    #[doc = "Comp. A Int. Ref. Select 2 : 0.5*Vcc"]
    #[inline(always)]
    pub fn caref_2(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_2)
    }
    #[doc = "Comp. A Int. Ref. Select 3 : Vt"]
    #[inline(always)]
    pub fn caref_3(self) -> &'a mut W {
        self.variant(CAREF_A::CAREF_3)
    }
}
#[doc = "Field `CARSEL` reader - Comp. A Internal Reference Enable"]
pub type CARSEL_R = crate::BitReader<bool>;
#[doc = "Field `CARSEL` writer - Comp. A Internal Reference Enable"]
pub type CARSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
#[doc = "Field `CAEX` reader - Comp. A Exchange Inputs"]
pub type CAEX_R = crate::BitReader<bool>;
#[doc = "Field `CAEX` writer - Comp. A Exchange Inputs"]
pub type CAEX_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CACTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&self) -> CAIFG_R {
        CAIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&self) -> CAIE_R {
        CAIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&self) -> CAIES_R {
        CAIES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&self) -> CAON_R {
        CAON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&self) -> CAREF_R {
        CAREF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&self) -> CARSEL_R {
        CARSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&self) -> CAEX_R {
        CAEX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Interrupt Flag"]
    #[inline(always)]
    pub fn caifg(&mut self) -> CAIFG_W<0> {
        CAIFG_W::new(self)
    }
    #[doc = "Bit 1 - Comp. A Interrupt Enable"]
    #[inline(always)]
    pub fn caie(&mut self) -> CAIE_W<1> {
        CAIE_W::new(self)
    }
    #[doc = "Bit 2 - Comp. A Int. Edge Select: 0:rising / 1:falling"]
    #[inline(always)]
    pub fn caies(&mut self) -> CAIES_W<2> {
        CAIES_W::new(self)
    }
    #[doc = "Bit 3 - Comp. A enable"]
    #[inline(always)]
    pub fn caon(&mut self) -> CAON_W<3> {
        CAON_W::new(self)
    }
    #[doc = "Bits 4:5 - Comp. A Internal Reference Select 0"]
    #[inline(always)]
    pub fn caref(&mut self) -> CAREF_W<4> {
        CAREF_W::new(self)
    }
    #[doc = "Bit 6 - Comp. A Internal Reference Enable"]
    #[inline(always)]
    pub fn carsel(&mut self) -> CARSEL_W<6> {
        CARSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comp. A Exchange Inputs"]
    #[inline(always)]
    pub fn caex(&mut self) -> CAEX_W<7> {
        CAEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator A Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cactl1](index.html) module"]
pub struct CACTL1_SPEC;
impl crate::RegisterSpec for CACTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cactl1::R](R) reader structure"]
impl crate::Readable for CACTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cactl1::W](W) writer structure"]
impl crate::Writable for CACTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACTL1 to value 0"]
impl crate::Resettable for CACTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
