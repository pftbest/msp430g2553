#[doc = "Reader of register UCA0MCTL"]
pub type R = crate::R<u8, super::UCA0MCTL>;
#[doc = "Writer for register UCA0MCTL"]
pub type W = crate::W<u8, super::UCA0MCTL>;
#[doc = "Register UCA0MCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0MCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCOS16`"]
pub type UCOS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCOS16`"]
pub struct UCOS16_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOS16_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "USCI Second Stage Modulation Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS_A {
    #[doc = "0: USCI Second Stage Modulation: 0"]
    UCBRS_0,
    #[doc = "1: USCI Second Stage Modulation: 1"]
    UCBRS_1,
    #[doc = "2: USCI Second Stage Modulation: 2"]
    UCBRS_2,
    #[doc = "3: USCI Second Stage Modulation: 3"]
    UCBRS_3,
    #[doc = "4: USCI Second Stage Modulation: 4"]
    UCBRS_4,
    #[doc = "5: USCI Second Stage Modulation: 5"]
    UCBRS_5,
    #[doc = "6: USCI Second Stage Modulation: 6"]
    UCBRS_6,
    #[doc = "7: USCI Second Stage Modulation: 7"]
    UCBRS_7,
}
impl From<UCBRS_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRS_A) -> Self {
        match variant {
            UCBRS_A::UCBRS_0 => 0,
            UCBRS_A::UCBRS_1 => 1,
            UCBRS_A::UCBRS_2 => 2,
            UCBRS_A::UCBRS_3 => 3,
            UCBRS_A::UCBRS_4 => 4,
            UCBRS_A::UCBRS_5 => 5,
            UCBRS_A::UCBRS_6 => 6,
            UCBRS_A::UCBRS_7 => 7,
        }
    }
}
#[doc = "Reader of field `UCBRS`"]
pub type UCBRS_R = crate::R<u8, UCBRS_A>;
impl UCBRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRS_A {
        match self.bits {
            0 => UCBRS_A::UCBRS_0,
            1 => UCBRS_A::UCBRS_1,
            2 => UCBRS_A::UCBRS_2,
            3 => UCBRS_A::UCBRS_3,
            4 => UCBRS_A::UCBRS_4,
            5 => UCBRS_A::UCBRS_5,
            6 => UCBRS_A::UCBRS_6,
            7 => UCBRS_A::UCBRS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCBRS_0`"]
    #[inline(always)]
    pub fn is_ucbrs_0(&self) -> bool {
        *self == UCBRS_A::UCBRS_0
    }
    #[doc = "Checks if the value of the field is `UCBRS_1`"]
    #[inline(always)]
    pub fn is_ucbrs_1(&self) -> bool {
        *self == UCBRS_A::UCBRS_1
    }
    #[doc = "Checks if the value of the field is `UCBRS_2`"]
    #[inline(always)]
    pub fn is_ucbrs_2(&self) -> bool {
        *self == UCBRS_A::UCBRS_2
    }
    #[doc = "Checks if the value of the field is `UCBRS_3`"]
    #[inline(always)]
    pub fn is_ucbrs_3(&self) -> bool {
        *self == UCBRS_A::UCBRS_3
    }
    #[doc = "Checks if the value of the field is `UCBRS_4`"]
    #[inline(always)]
    pub fn is_ucbrs_4(&self) -> bool {
        *self == UCBRS_A::UCBRS_4
    }
    #[doc = "Checks if the value of the field is `UCBRS_5`"]
    #[inline(always)]
    pub fn is_ucbrs_5(&self) -> bool {
        *self == UCBRS_A::UCBRS_5
    }
    #[doc = "Checks if the value of the field is `UCBRS_6`"]
    #[inline(always)]
    pub fn is_ucbrs_6(&self) -> bool {
        *self == UCBRS_A::UCBRS_6
    }
    #[doc = "Checks if the value of the field is `UCBRS_7`"]
    #[inline(always)]
    pub fn is_ucbrs_7(&self) -> bool {
        *self == UCBRS_A::UCBRS_7
    }
}
#[doc = "Write proxy for field `UCBRS`"]
pub struct UCBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBRS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USCI Second Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrs_0(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_0)
    }
    #[doc = "USCI Second Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrs_1(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_1)
    }
    #[doc = "USCI Second Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrs_2(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_2)
    }
    #[doc = "USCI Second Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrs_3(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_3)
    }
    #[doc = "USCI Second Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrs_4(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_4)
    }
    #[doc = "USCI Second Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrs_5(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_5)
    }
    #[doc = "USCI Second Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrs_6(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_6)
    }
    #[doc = "USCI Second Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrs_7(self) -> &'a mut W {
        self.variant(UCBRS_A::UCBRS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
#[doc = "USCI First Stage Modulation Select 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRF_A {
    #[doc = "0: USCI First Stage Modulation: 0"]
    UCBRF_0,
    #[doc = "1: USCI First Stage Modulation: 1"]
    UCBRF_1,
    #[doc = "2: USCI First Stage Modulation: 2"]
    UCBRF_2,
    #[doc = "3: USCI First Stage Modulation: 3"]
    UCBRF_3,
    #[doc = "4: USCI First Stage Modulation: 4"]
    UCBRF_4,
    #[doc = "5: USCI First Stage Modulation: 5"]
    UCBRF_5,
    #[doc = "6: USCI First Stage Modulation: 6"]
    UCBRF_6,
    #[doc = "7: USCI First Stage Modulation: 7"]
    UCBRF_7,
    #[doc = "8: USCI First Stage Modulation: 8"]
    UCBRF_8,
    #[doc = "9: USCI First Stage Modulation: 9"]
    UCBRF_9,
    #[doc = "10: USCI First Stage Modulation: A"]
    UCBRF_10,
    #[doc = "11: USCI First Stage Modulation: B"]
    UCBRF_11,
    #[doc = "12: USCI First Stage Modulation: C"]
    UCBRF_12,
    #[doc = "13: USCI First Stage Modulation: D"]
    UCBRF_13,
    #[doc = "14: USCI First Stage Modulation: E"]
    UCBRF_14,
    #[doc = "15: USCI First Stage Modulation: F"]
    UCBRF_15,
}
impl From<UCBRF_A> for u8 {
    #[inline(always)]
    fn from(variant: UCBRF_A) -> Self {
        match variant {
            UCBRF_A::UCBRF_0 => 0,
            UCBRF_A::UCBRF_1 => 1,
            UCBRF_A::UCBRF_2 => 2,
            UCBRF_A::UCBRF_3 => 3,
            UCBRF_A::UCBRF_4 => 4,
            UCBRF_A::UCBRF_5 => 5,
            UCBRF_A::UCBRF_6 => 6,
            UCBRF_A::UCBRF_7 => 7,
            UCBRF_A::UCBRF_8 => 8,
            UCBRF_A::UCBRF_9 => 9,
            UCBRF_A::UCBRF_10 => 10,
            UCBRF_A::UCBRF_11 => 11,
            UCBRF_A::UCBRF_12 => 12,
            UCBRF_A::UCBRF_13 => 13,
            UCBRF_A::UCBRF_14 => 14,
            UCBRF_A::UCBRF_15 => 15,
        }
    }
}
#[doc = "Reader of field `UCBRF`"]
pub type UCBRF_R = crate::R<u8, UCBRF_A>;
impl UCBRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRF_A {
        match self.bits {
            0 => UCBRF_A::UCBRF_0,
            1 => UCBRF_A::UCBRF_1,
            2 => UCBRF_A::UCBRF_2,
            3 => UCBRF_A::UCBRF_3,
            4 => UCBRF_A::UCBRF_4,
            5 => UCBRF_A::UCBRF_5,
            6 => UCBRF_A::UCBRF_6,
            7 => UCBRF_A::UCBRF_7,
            8 => UCBRF_A::UCBRF_8,
            9 => UCBRF_A::UCBRF_9,
            10 => UCBRF_A::UCBRF_10,
            11 => UCBRF_A::UCBRF_11,
            12 => UCBRF_A::UCBRF_12,
            13 => UCBRF_A::UCBRF_13,
            14 => UCBRF_A::UCBRF_14,
            15 => UCBRF_A::UCBRF_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCBRF_0`"]
    #[inline(always)]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == UCBRF_A::UCBRF_0
    }
    #[doc = "Checks if the value of the field is `UCBRF_1`"]
    #[inline(always)]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == UCBRF_A::UCBRF_1
    }
    #[doc = "Checks if the value of the field is `UCBRF_2`"]
    #[inline(always)]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == UCBRF_A::UCBRF_2
    }
    #[doc = "Checks if the value of the field is `UCBRF_3`"]
    #[inline(always)]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == UCBRF_A::UCBRF_3
    }
    #[doc = "Checks if the value of the field is `UCBRF_4`"]
    #[inline(always)]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == UCBRF_A::UCBRF_4
    }
    #[doc = "Checks if the value of the field is `UCBRF_5`"]
    #[inline(always)]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == UCBRF_A::UCBRF_5
    }
    #[doc = "Checks if the value of the field is `UCBRF_6`"]
    #[inline(always)]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == UCBRF_A::UCBRF_6
    }
    #[doc = "Checks if the value of the field is `UCBRF_7`"]
    #[inline(always)]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == UCBRF_A::UCBRF_7
    }
    #[doc = "Checks if the value of the field is `UCBRF_8`"]
    #[inline(always)]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == UCBRF_A::UCBRF_8
    }
    #[doc = "Checks if the value of the field is `UCBRF_9`"]
    #[inline(always)]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == UCBRF_A::UCBRF_9
    }
    #[doc = "Checks if the value of the field is `UCBRF_10`"]
    #[inline(always)]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == UCBRF_A::UCBRF_10
    }
    #[doc = "Checks if the value of the field is `UCBRF_11`"]
    #[inline(always)]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == UCBRF_A::UCBRF_11
    }
    #[doc = "Checks if the value of the field is `UCBRF_12`"]
    #[inline(always)]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == UCBRF_A::UCBRF_12
    }
    #[doc = "Checks if the value of the field is `UCBRF_13`"]
    #[inline(always)]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == UCBRF_A::UCBRF_13
    }
    #[doc = "Checks if the value of the field is `UCBRF_14`"]
    #[inline(always)]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == UCBRF_A::UCBRF_14
    }
    #[doc = "Checks if the value of the field is `UCBRF_15`"]
    #[inline(always)]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == UCBRF_A::UCBRF_15
    }
}
#[doc = "Write proxy for field `UCBRF`"]
pub struct UCBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBRF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline(always)]
    pub fn ucbrf_0(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline(always)]
    pub fn ucbrf_1(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline(always)]
    pub fn ucbrf_2(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline(always)]
    pub fn ucbrf_3(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline(always)]
    pub fn ucbrf_4(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline(always)]
    pub fn ucbrf_5(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline(always)]
    pub fn ucbrf_6(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline(always)]
    pub fn ucbrf_7(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline(always)]
    pub fn ucbrf_8(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline(always)]
    pub fn ucbrf_9(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline(always)]
    pub fn ucbrf_10(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline(always)]
    pub fn ucbrf_11(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline(always)]
    pub fn ucbrf_12(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline(always)]
    pub fn ucbrf_13(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline(always)]
    pub fn ucbrf_14(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline(always)]
    pub fn ucbrf_15(self) -> &'a mut W {
        self.variant(UCBRF_A::UCBRF_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W { w: self }
    }
    #[doc = "Bits 1:3 - USCI Second Stage Modulation Select 2"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UCBRS_W {
        UCBRS_W { w: self }
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W { w: self }
    }
}
