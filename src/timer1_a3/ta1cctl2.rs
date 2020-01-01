#[doc = "Reader of register TA1CCTL2"]
pub type R = crate::R<u16, super::TA1CCTL2>;
#[doc = "Writer for register TA1CCTL2"]
pub type W = crate::W<u16, super::TA1CCTL2>;
#[doc = "Register TA1CCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TA1CCTL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCIFG`"]
pub type CCIFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCIFG`"]
pub struct CCIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIFG_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COV`"]
pub type COV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COV`"]
pub struct COV_W<'a> {
    w: &'a mut W,
}
impl<'a> COV_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OUT`"]
pub type OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT`"]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CCI`"]
pub type CCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCI`"]
pub struct CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Output mode 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: PWM output mode: 0 - output only"]
    OUTMOD_0 = 0,
    #[doc = "1: PWM output mode: 1 - set"]
    OUTMOD_1 = 1,
    #[doc = "2: PWM output mode: 2 - PWM toggle/reset"]
    OUTMOD_2 = 2,
    #[doc = "3: PWM output mode: 3 - PWM set/reset"]
    OUTMOD_3 = 3,
    #[doc = "4: PWM output mode: 4 - toggle"]
    OUTMOD_4 = 4,
    #[doc = "5: PWM output mode: 5 - Reset"]
    OUTMOD_5 = 5,
    #[doc = "6: PWM output mode: 6 - PWM toggle/set"]
    OUTMOD_6 = 6,
    #[doc = "7: PWM output mode: 7 - PWM reset/set"]
    OUTMOD_7 = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTMOD`"]
pub type OUTMOD_R = crate::R<u8, OUTMOD_A>;
impl OUTMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::OUTMOD_0,
            1 => OUTMOD_A::OUTMOD_1,
            2 => OUTMOD_A::OUTMOD_2,
            3 => OUTMOD_A::OUTMOD_3,
            4 => OUTMOD_A::OUTMOD_4,
            5 => OUTMOD_A::OUTMOD_5,
            6 => OUTMOD_A::OUTMOD_6,
            7 => OUTMOD_A::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMOD_0`"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_0
    }
    #[doc = "Checks if the value of the field is `OUTMOD_1`"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_1
    }
    #[doc = "Checks if the value of the field is `OUTMOD_2`"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_2
    }
    #[doc = "Checks if the value of the field is `OUTMOD_3`"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_3
    }
    #[doc = "Checks if the value of the field is `OUTMOD_4`"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_4
    }
    #[doc = "Checks if the value of the field is `OUTMOD_5`"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_5
    }
    #[doc = "Checks if the value of the field is `OUTMOD_6`"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_6
    }
    #[doc = "Checks if the value of the field is `OUTMOD_7`"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_7
    }
}
#[doc = "Write proxy for field `OUTMOD`"]
pub struct OUTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PWM output mode: 0 - output only"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_0)
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_1)
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_2)
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_3)
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_4)
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_5)
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_6)
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP`"]
pub struct CAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCCI`"]
pub type SCCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCCI`"]
pub struct SCCI_W<'a> {
    w: &'a mut W,
}
impl<'a> SCCI_W<'a> {
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
            (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
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
            (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Capture input select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCIS_A {
    #[doc = "0: Capture input select: 0 - CCIxA"]
    CCIS_0 = 0,
    #[doc = "1: Capture input select: 1 - CCIxB"]
    CCIS_1 = 1,
    #[doc = "2: Capture input select: 2 - GND"]
    CCIS_2 = 2,
    #[doc = "3: Capture input select: 3 - Vcc"]
    CCIS_3 = 3,
}
impl From<CCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CCIS`"]
pub type CCIS_R = crate::R<u8, CCIS_A>;
impl CCIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIS_A {
        match self.bits {
            0 => CCIS_A::CCIS_0,
            1 => CCIS_A::CCIS_1,
            2 => CCIS_A::CCIS_2,
            3 => CCIS_A::CCIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCIS_0`"]
    #[inline(always)]
    pub fn is_ccis_0(&self) -> bool {
        *self == CCIS_A::CCIS_0
    }
    #[doc = "Checks if the value of the field is `CCIS_1`"]
    #[inline(always)]
    pub fn is_ccis_1(&self) -> bool {
        *self == CCIS_A::CCIS_1
    }
    #[doc = "Checks if the value of the field is `CCIS_2`"]
    #[inline(always)]
    pub fn is_ccis_2(&self) -> bool {
        *self == CCIS_A::CCIS_2
    }
    #[doc = "Checks if the value of the field is `CCIS_3`"]
    #[inline(always)]
    pub fn is_ccis_3(&self) -> bool {
        *self == CCIS_A::CCIS_3
    }
}
#[doc = "Write proxy for field `CCIS`"]
pub struct CCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline(always)]
    pub fn ccis_0(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_0)
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline(always)]
    pub fn ccis_1(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_1)
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline(always)]
    pub fn ccis_2(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_2)
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline(always)]
    pub fn ccis_3(self) -> &'a mut W {
        self.variant(CCIS_A::CCIS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 12)) | (((value as u16) & 0x03) << 12);
        self.w
    }
}
#[doc = "Capture mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Capture mode: 0 - disabled"]
    CM_0 = 0,
    #[doc = "1: Capture mode: 1 - pos. edge"]
    CM_1 = 1,
    #[doc = "2: Capture mode: 1 - neg. edge"]
    CM_2 = 2,
    #[doc = "3: Capture mode: 1 - both edges"]
    CM_3 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::CM_0,
            1 => CM_A::CM_1,
            2 => CM_A::CM_2,
            3 => CM_A::CM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_1`"]
    #[inline(always)]
    pub fn is_cm_1(&self) -> bool {
        *self == CM_A::CM_1
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Capture mode: 0 - disabled"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline(always)]
    pub fn cm_1(self) -> &'a mut W {
        self.variant(CM_A::CM_1)
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CM_A::CM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline(always)]
    pub fn scci(&self) -> SCCI_R {
        SCCI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    pub fn ccis(&self) -> CCIS_R {
        CCIS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CCIFG_W {
        CCIFG_W { w: self }
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline(always)]
    pub fn cov(&mut self) -> COV_W {
        COV_W { w: self }
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline(always)]
    pub fn cci(&mut self) -> CCI_W {
        CCI_W { w: self }
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OUTMOD_W {
        OUTMOD_W { w: self }
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline(always)]
    pub fn cap(&mut self) -> CAP_W {
        CAP_W { w: self }
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline(always)]
    pub fn scci(&mut self) -> SCCI_W {
        SCCI_W { w: self }
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CCIS_W {
        CCIS_W { w: self }
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
