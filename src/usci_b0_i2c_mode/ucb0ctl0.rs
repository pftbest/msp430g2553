#[doc = "Register `UCB0CTL0` reader"]
pub struct R(crate::R<UCB0CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0CTL0` writer"]
pub struct W(crate::W<UCB0CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0CTL0_SPEC>;
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
impl From<crate::W<UCB0CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_R = crate::BitReader<bool>;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0CTL0_SPEC, bool, O>;
#[doc = "Field `UCMODE` reader - Sync. Mode: USCI Mode 1"]
pub type UCMODE_R = crate::FieldReader<u8, UCMODE_A>;
#[doc = "Sync. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Field `UCMODE` writer - Sync. Mode: USCI Mode 1"]
pub type UCMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, UCB0CTL0_SPEC, u8, UCMODE_A, 2, O>;
impl<'a, const O: u8> UCMODE_W<'a, O> {
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
}
#[doc = "Field `UCMST` reader - Sync. Mode: Master Select"]
pub type UCMST_R = crate::BitReader<bool>;
#[doc = "Field `UCMST` writer - Sync. Mode: Master Select"]
pub type UCMST_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0CTL0_SPEC, bool, O>;
#[doc = "Field `UCMM` reader - Multi-Master Environment"]
pub type UCMM_R = crate::BitReader<bool>;
#[doc = "Field `UCMM` writer - Multi-Master Environment"]
pub type UCMM_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0CTL0_SPEC, bool, O>;
#[doc = "Field `UCSLA10` reader - 10-bit Slave Address Mode"]
pub type UCSLA10_R = crate::BitReader<bool>;
#[doc = "Field `UCSLA10` writer - 10-bit Slave Address Mode"]
pub type UCSLA10_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0CTL0_SPEC, bool, O>;
#[doc = "Field `UCA10` reader - 10-bit Address Mode"]
pub type UCA10_R = crate::BitReader<bool>;
#[doc = "Field `UCA10` writer - 10-bit Address Mode"]
pub type UCA10_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, UCB0CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&self) -> UCMST_R {
        UCMST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&self) -> UCMM_R {
        UCMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&self) -> UCSLA10_R {
        UCSLA10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&self) -> UCA10_R {
        UCA10_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W<0> {
        UCSYNC_W::new(self)
    }
    #[doc = "Bits 1:2 - Sync. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W<1> {
        UCMODE_W::new(self)
    }
    #[doc = "Bit 3 - Sync. Mode: Master Select"]
    #[inline(always)]
    pub fn ucmst(&mut self) -> UCMST_W<3> {
        UCMST_W::new(self)
    }
    #[doc = "Bit 5 - Multi-Master Environment"]
    #[inline(always)]
    pub fn ucmm(&mut self) -> UCMM_W<5> {
        UCMM_W::new(self)
    }
    #[doc = "Bit 6 - 10-bit Slave Address Mode"]
    #[inline(always)]
    pub fn ucsla10(&mut self) -> UCSLA10_W<6> {
        UCSLA10_W::new(self)
    }
    #[doc = "Bit 7 - 10-bit Address Mode"]
    #[inline(always)]
    pub fn uca10(&mut self) -> UCA10_W<7> {
        UCA10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI B0 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0ctl0](index.html) module"]
pub struct UCB0CTL0_SPEC;
impl crate::RegisterSpec for UCB0CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ucb0ctl0::R](R) reader structure"]
impl crate::Readable for UCB0CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0ctl0::W](W) writer structure"]
impl crate::Writable for UCB0CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0CTL0 to value 0"]
impl crate::Resettable for UCB0CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
