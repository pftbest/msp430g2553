#[doc = "Register `BCSCTL1` reader"]
pub struct R(crate::R<BCSCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCSCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCSCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCSCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCSCTL1` writer"]
pub struct W(crate::W<BCSCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCSCTL1_SPEC>;
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
impl From<crate::W<BCSCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCSCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCSCTL1` reader - Basic Clock System Control 1 register"]
pub type BCSCTL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCSCTL1` writer - Basic Clock System Control 1 register"]
pub type BCSCTL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, BCSCTL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSEL` reader - Range Select Bit 0"]
pub type RSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSEL` writer - Range Select Bit 0"]
pub type RSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, BCSCTL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIVA` reader - ACLK Divider 0"]
pub type DIVA_R = crate::FieldReader<u8, DIVA_A>;
#[doc = "ACLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: ACLK Divider 0: /1"]
    DIVA_0 = 0,
    #[doc = "1: ACLK Divider 1: /2"]
    DIVA_1 = 1,
    #[doc = "2: ACLK Divider 2: /4"]
    DIVA_2 = 2,
    #[doc = "3: ACLK Divider 3: /8"]
    DIVA_3 = 3,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::DIVA_0,
            1 => DIVA_A::DIVA_1,
            2 => DIVA_A::DIVA_2,
            3 => DIVA_A::DIVA_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVA_0`"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "Checks if the value of the field is `DIVA_1`"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "Checks if the value of the field is `DIVA_2`"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "Checks if the value of the field is `DIVA_3`"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
}
#[doc = "Field `DIVA` writer - ACLK Divider 0"]
pub type DIVA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, BCSCTL1_SPEC, u8, DIVA_A, 2, O>;
impl<'a, const O: u8> DIVA_W<'a, O> {
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_3)
    }
}
#[doc = "Field `XTS` reader - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XTS_R = crate::BitReader<bool>;
#[doc = "Field `XTS` writer - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XTS_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, BCSCTL1_SPEC, bool, O>;
#[doc = "Field `XT2OFF` reader - Enable XT2CLK"]
pub type XT2OFF_R = crate::BitReader<bool>;
#[doc = "Field `XT2OFF` writer - Enable XT2CLK"]
pub type XT2OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, BCSCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Basic Clock System Control 1 register"]
    #[inline(always)]
    pub fn bcsctl1(&self) -> BCSCTL1_R {
        BCSCTL1_R::new(self.bits)
    }
    #[doc = "Bits 0:3 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel(&self) -> RSEL_R {
        RSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Basic Clock System Control 1 register"]
    #[inline(always)]
    pub fn bcsctl1(&mut self) -> BCSCTL1_W<0> {
        BCSCTL1_W::new(self)
    }
    #[doc = "Bits 0:3 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel(&mut self) -> RSEL_W<0> {
        RSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W<4> {
        DIVA_W::new(self)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&mut self) -> XTS_W<6> {
        XTS_W::new(self)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> XT2OFF_W<7> {
        XT2OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic Clock System Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl1](index.html) module"]
pub struct BCSCTL1_SPEC;
impl crate::RegisterSpec for BCSCTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcsctl1::R](R) reader structure"]
impl crate::Readable for BCSCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcsctl1::W](W) writer structure"]
impl crate::Writable for BCSCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCSCTL1 to value 0"]
impl crate::Resettable for BCSCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
