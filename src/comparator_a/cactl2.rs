#[doc = "Reader of register CACTL2"]
pub type R = crate::R<u8, super::CACTL2>;
#[doc = "Writer for register CACTL2"]
pub type W = crate::W<u8, super::CACTL2>;
#[doc = "Register CACTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CACTL2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAOUT`"]
pub type CAOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAOUT`"]
pub struct CAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAOUT_W<'a> {
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
#[doc = "Reader of field `CAF`"]
pub type CAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAF`"]
pub struct CAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAF_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CASHORT`"]
pub type CASHORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CASHORT`"]
pub struct CASHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CASHORT_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Comp. A +Terminal Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2CA_A {
    #[doc = "0: No + or - connection"]
    NONE_NONE = 0,
    #[doc = "2: No connection to +, connect - to CA1"]
    NONE_CA1 = 2,
    #[doc = "4: No connection to +, connect - to CA2"]
    NONE_CA2 = 4,
    #[doc = "6: No connection to +, connect - to CA3"]
    NONE_CA3 = 6,
    #[doc = "8: No connection to +, connect - to CA4"]
    NONE_CA4 = 8,
    #[doc = "10: No connection to +, connect - to CA5"]
    NONE_CA5 = 10,
    #[doc = "12: No connection to +, connect - to CA6"]
    NONE_CA6 = 12,
    #[doc = "14: No connection to +, connect - to CA7"]
    NONE_CA7 = 14,
    #[doc = "1: Connect + to CA0, no connection to -"]
    CA0_NONE = 1,
    #[doc = "3: Connect + to CA0, connect - to CA1"]
    CA0_CA1 = 3,
    #[doc = "5: Connect + to CA0, connect - to CA2"]
    CA0_CA2 = 5,
    #[doc = "7: Connect + to CA0, connect - to CA3"]
    CA0_CA3 = 7,
    #[doc = "9: Connect + to CA0, connect - to CA4"]
    CA0_CA4 = 9,
    #[doc = "11: Connect + to CA0, connect - to CA5"]
    CA0_CA5 = 11,
    #[doc = "13: Connect + to CA0, connect - to CA6"]
    CA0_CA6 = 13,
    #[doc = "15: Connect + to CA0, connect - to CA7"]
    CA0_CA7 = 15,
    #[doc = "16: Connect + to CA1, no connection to -"]
    CA1_NONE = 16,
    #[doc = "18: Connect + to CA1, connect - to CA1"]
    CA1_CA1 = 18,
    #[doc = "20: Connect + to CA1, connect - to CA2"]
    CA1_CA2 = 20,
    #[doc = "22: Connect + to CA1, connect - to CA3"]
    CA1_CA3 = 22,
    #[doc = "24: Connect + to CA1, connect - to CA4"]
    CA1_CA4 = 24,
    #[doc = "26: Connect + to CA1, connect - to CA5"]
    CA1_CA5 = 26,
    #[doc = "28: Connect + to CA1, connect - to CA6"]
    CA1_CA6 = 28,
    #[doc = "30: Connect + to CA1, connect - to CA7"]
    CA1_CA7 = 30,
    #[doc = "17: Connect + to CA2, no connection to -"]
    CA2_NONE = 17,
    #[doc = "19: Connect + to CA2, connect - to CA1"]
    CA2_CA1 = 19,
    #[doc = "21: Connect + to CA2, connect - to CA2"]
    CA2_CA2 = 21,
    #[doc = "23: Connect + to CA2, connect - to CA3"]
    CA2_CA3 = 23,
    #[doc = "25: Connect + to CA2, connect - to CA4"]
    CA2_CA4 = 25,
    #[doc = "27: Connect + to CA2, connect - to CA5"]
    CA2_CA5 = 27,
    #[doc = "29: Connect + to CA2, connect - to CA6"]
    CA2_CA6 = 29,
    #[doc = "31: Connect + to CA2, connect - to CA7"]
    CA2_CA7 = 31,
}
impl From<P2CA_A> for u8 {
    #[inline(always)]
    fn from(variant: P2CA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2CA`"]
pub type P2CA_R = crate::R<u8, P2CA_A>;
impl P2CA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2CA_A {
        match self.bits {
            0 => P2CA_A::NONE_NONE,
            2 => P2CA_A::NONE_CA1,
            4 => P2CA_A::NONE_CA2,
            6 => P2CA_A::NONE_CA3,
            8 => P2CA_A::NONE_CA4,
            10 => P2CA_A::NONE_CA5,
            12 => P2CA_A::NONE_CA6,
            14 => P2CA_A::NONE_CA7,
            1 => P2CA_A::CA0_NONE,
            3 => P2CA_A::CA0_CA1,
            5 => P2CA_A::CA0_CA2,
            7 => P2CA_A::CA0_CA3,
            9 => P2CA_A::CA0_CA4,
            11 => P2CA_A::CA0_CA5,
            13 => P2CA_A::CA0_CA6,
            15 => P2CA_A::CA0_CA7,
            16 => P2CA_A::CA1_NONE,
            18 => P2CA_A::CA1_CA1,
            20 => P2CA_A::CA1_CA2,
            22 => P2CA_A::CA1_CA3,
            24 => P2CA_A::CA1_CA4,
            26 => P2CA_A::CA1_CA5,
            28 => P2CA_A::CA1_CA6,
            30 => P2CA_A::CA1_CA7,
            17 => P2CA_A::CA2_NONE,
            19 => P2CA_A::CA2_CA1,
            21 => P2CA_A::CA2_CA2,
            23 => P2CA_A::CA2_CA3,
            25 => P2CA_A::CA2_CA4,
            27 => P2CA_A::CA2_CA5,
            29 => P2CA_A::CA2_CA6,
            31 => P2CA_A::CA2_CA7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE_NONE`"]
    #[inline(always)]
    pub fn is_none_none(&self) -> bool {
        *self == P2CA_A::NONE_NONE
    }
    #[doc = "Checks if the value of the field is `NONE_CA1`"]
    #[inline(always)]
    pub fn is_none_ca1(&self) -> bool {
        *self == P2CA_A::NONE_CA1
    }
    #[doc = "Checks if the value of the field is `NONE_CA2`"]
    #[inline(always)]
    pub fn is_none_ca2(&self) -> bool {
        *self == P2CA_A::NONE_CA2
    }
    #[doc = "Checks if the value of the field is `NONE_CA3`"]
    #[inline(always)]
    pub fn is_none_ca3(&self) -> bool {
        *self == P2CA_A::NONE_CA3
    }
    #[doc = "Checks if the value of the field is `NONE_CA4`"]
    #[inline(always)]
    pub fn is_none_ca4(&self) -> bool {
        *self == P2CA_A::NONE_CA4
    }
    #[doc = "Checks if the value of the field is `NONE_CA5`"]
    #[inline(always)]
    pub fn is_none_ca5(&self) -> bool {
        *self == P2CA_A::NONE_CA5
    }
    #[doc = "Checks if the value of the field is `NONE_CA6`"]
    #[inline(always)]
    pub fn is_none_ca6(&self) -> bool {
        *self == P2CA_A::NONE_CA6
    }
    #[doc = "Checks if the value of the field is `NONE_CA7`"]
    #[inline(always)]
    pub fn is_none_ca7(&self) -> bool {
        *self == P2CA_A::NONE_CA7
    }
    #[doc = "Checks if the value of the field is `CA0_NONE`"]
    #[inline(always)]
    pub fn is_ca0_none(&self) -> bool {
        *self == P2CA_A::CA0_NONE
    }
    #[doc = "Checks if the value of the field is `CA0_CA1`"]
    #[inline(always)]
    pub fn is_ca0_ca1(&self) -> bool {
        *self == P2CA_A::CA0_CA1
    }
    #[doc = "Checks if the value of the field is `CA0_CA2`"]
    #[inline(always)]
    pub fn is_ca0_ca2(&self) -> bool {
        *self == P2CA_A::CA0_CA2
    }
    #[doc = "Checks if the value of the field is `CA0_CA3`"]
    #[inline(always)]
    pub fn is_ca0_ca3(&self) -> bool {
        *self == P2CA_A::CA0_CA3
    }
    #[doc = "Checks if the value of the field is `CA0_CA4`"]
    #[inline(always)]
    pub fn is_ca0_ca4(&self) -> bool {
        *self == P2CA_A::CA0_CA4
    }
    #[doc = "Checks if the value of the field is `CA0_CA5`"]
    #[inline(always)]
    pub fn is_ca0_ca5(&self) -> bool {
        *self == P2CA_A::CA0_CA5
    }
    #[doc = "Checks if the value of the field is `CA0_CA6`"]
    #[inline(always)]
    pub fn is_ca0_ca6(&self) -> bool {
        *self == P2CA_A::CA0_CA6
    }
    #[doc = "Checks if the value of the field is `CA0_CA7`"]
    #[inline(always)]
    pub fn is_ca0_ca7(&self) -> bool {
        *self == P2CA_A::CA0_CA7
    }
    #[doc = "Checks if the value of the field is `CA1_NONE`"]
    #[inline(always)]
    pub fn is_ca1_none(&self) -> bool {
        *self == P2CA_A::CA1_NONE
    }
    #[doc = "Checks if the value of the field is `CA1_CA1`"]
    #[inline(always)]
    pub fn is_ca1_ca1(&self) -> bool {
        *self == P2CA_A::CA1_CA1
    }
    #[doc = "Checks if the value of the field is `CA1_CA2`"]
    #[inline(always)]
    pub fn is_ca1_ca2(&self) -> bool {
        *self == P2CA_A::CA1_CA2
    }
    #[doc = "Checks if the value of the field is `CA1_CA3`"]
    #[inline(always)]
    pub fn is_ca1_ca3(&self) -> bool {
        *self == P2CA_A::CA1_CA3
    }
    #[doc = "Checks if the value of the field is `CA1_CA4`"]
    #[inline(always)]
    pub fn is_ca1_ca4(&self) -> bool {
        *self == P2CA_A::CA1_CA4
    }
    #[doc = "Checks if the value of the field is `CA1_CA5`"]
    #[inline(always)]
    pub fn is_ca1_ca5(&self) -> bool {
        *self == P2CA_A::CA1_CA5
    }
    #[doc = "Checks if the value of the field is `CA1_CA6`"]
    #[inline(always)]
    pub fn is_ca1_ca6(&self) -> bool {
        *self == P2CA_A::CA1_CA6
    }
    #[doc = "Checks if the value of the field is `CA1_CA7`"]
    #[inline(always)]
    pub fn is_ca1_ca7(&self) -> bool {
        *self == P2CA_A::CA1_CA7
    }
    #[doc = "Checks if the value of the field is `CA2_NONE`"]
    #[inline(always)]
    pub fn is_ca2_none(&self) -> bool {
        *self == P2CA_A::CA2_NONE
    }
    #[doc = "Checks if the value of the field is `CA2_CA1`"]
    #[inline(always)]
    pub fn is_ca2_ca1(&self) -> bool {
        *self == P2CA_A::CA2_CA1
    }
    #[doc = "Checks if the value of the field is `CA2_CA2`"]
    #[inline(always)]
    pub fn is_ca2_ca2(&self) -> bool {
        *self == P2CA_A::CA2_CA2
    }
    #[doc = "Checks if the value of the field is `CA2_CA3`"]
    #[inline(always)]
    pub fn is_ca2_ca3(&self) -> bool {
        *self == P2CA_A::CA2_CA3
    }
    #[doc = "Checks if the value of the field is `CA2_CA4`"]
    #[inline(always)]
    pub fn is_ca2_ca4(&self) -> bool {
        *self == P2CA_A::CA2_CA4
    }
    #[doc = "Checks if the value of the field is `CA2_CA5`"]
    #[inline(always)]
    pub fn is_ca2_ca5(&self) -> bool {
        *self == P2CA_A::CA2_CA5
    }
    #[doc = "Checks if the value of the field is `CA2_CA6`"]
    #[inline(always)]
    pub fn is_ca2_ca6(&self) -> bool {
        *self == P2CA_A::CA2_CA6
    }
    #[doc = "Checks if the value of the field is `CA2_CA7`"]
    #[inline(always)]
    pub fn is_ca2_ca7(&self) -> bool {
        *self == P2CA_A::CA2_CA7
    }
}
#[doc = "Write proxy for field `P2CA`"]
pub struct P2CA_W<'a> {
    w: &'a mut W,
}
impl<'a> P2CA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2CA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No + or - connection"]
    #[inline(always)]
    pub fn none_none(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_NONE)
    }
    #[doc = "No connection to +, connect - to CA1"]
    #[inline(always)]
    pub fn none_ca1(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA1)
    }
    #[doc = "No connection to +, connect - to CA2"]
    #[inline(always)]
    pub fn none_ca2(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA2)
    }
    #[doc = "No connection to +, connect - to CA3"]
    #[inline(always)]
    pub fn none_ca3(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA3)
    }
    #[doc = "No connection to +, connect - to CA4"]
    #[inline(always)]
    pub fn none_ca4(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA4)
    }
    #[doc = "No connection to +, connect - to CA5"]
    #[inline(always)]
    pub fn none_ca5(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA5)
    }
    #[doc = "No connection to +, connect - to CA6"]
    #[inline(always)]
    pub fn none_ca6(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA6)
    }
    #[doc = "No connection to +, connect - to CA7"]
    #[inline(always)]
    pub fn none_ca7(self) -> &'a mut W {
        self.variant(P2CA_A::NONE_CA7)
    }
    #[doc = "Connect + to CA0, no connection to -"]
    #[inline(always)]
    pub fn ca0_none(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_NONE)
    }
    #[doc = "Connect + to CA0, connect - to CA1"]
    #[inline(always)]
    pub fn ca0_ca1(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA1)
    }
    #[doc = "Connect + to CA0, connect - to CA2"]
    #[inline(always)]
    pub fn ca0_ca2(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA2)
    }
    #[doc = "Connect + to CA0, connect - to CA3"]
    #[inline(always)]
    pub fn ca0_ca3(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA3)
    }
    #[doc = "Connect + to CA0, connect - to CA4"]
    #[inline(always)]
    pub fn ca0_ca4(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA4)
    }
    #[doc = "Connect + to CA0, connect - to CA5"]
    #[inline(always)]
    pub fn ca0_ca5(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA5)
    }
    #[doc = "Connect + to CA0, connect - to CA6"]
    #[inline(always)]
    pub fn ca0_ca6(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA6)
    }
    #[doc = "Connect + to CA0, connect - to CA7"]
    #[inline(always)]
    pub fn ca0_ca7(self) -> &'a mut W {
        self.variant(P2CA_A::CA0_CA7)
    }
    #[doc = "Connect + to CA1, no connection to -"]
    #[inline(always)]
    pub fn ca1_none(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_NONE)
    }
    #[doc = "Connect + to CA1, connect - to CA1"]
    #[inline(always)]
    pub fn ca1_ca1(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA1)
    }
    #[doc = "Connect + to CA1, connect - to CA2"]
    #[inline(always)]
    pub fn ca1_ca2(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA2)
    }
    #[doc = "Connect + to CA1, connect - to CA3"]
    #[inline(always)]
    pub fn ca1_ca3(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA3)
    }
    #[doc = "Connect + to CA1, connect - to CA4"]
    #[inline(always)]
    pub fn ca1_ca4(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA4)
    }
    #[doc = "Connect + to CA1, connect - to CA5"]
    #[inline(always)]
    pub fn ca1_ca5(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA5)
    }
    #[doc = "Connect + to CA1, connect - to CA6"]
    #[inline(always)]
    pub fn ca1_ca6(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA6)
    }
    #[doc = "Connect + to CA1, connect - to CA7"]
    #[inline(always)]
    pub fn ca1_ca7(self) -> &'a mut W {
        self.variant(P2CA_A::CA1_CA7)
    }
    #[doc = "Connect + to CA2, no connection to -"]
    #[inline(always)]
    pub fn ca2_none(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_NONE)
    }
    #[doc = "Connect + to CA2, connect - to CA1"]
    #[inline(always)]
    pub fn ca2_ca1(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA1)
    }
    #[doc = "Connect + to CA2, connect - to CA2"]
    #[inline(always)]
    pub fn ca2_ca2(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA2)
    }
    #[doc = "Connect + to CA2, connect - to CA3"]
    #[inline(always)]
    pub fn ca2_ca3(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA3)
    }
    #[doc = "Connect + to CA2, connect - to CA4"]
    #[inline(always)]
    pub fn ca2_ca4(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA4)
    }
    #[doc = "Connect + to CA2, connect - to CA5"]
    #[inline(always)]
    pub fn ca2_ca5(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA5)
    }
    #[doc = "Connect + to CA2, connect - to CA6"]
    #[inline(always)]
    pub fn ca2_ca6(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA6)
    }
    #[doc = "Connect + to CA2, connect - to CA7"]
    #[inline(always)]
    pub fn ca2_ca7(self) -> &'a mut W {
        self.variant(P2CA_A::CA2_CA7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x1f << 2)) | (((value as u8) & 0x1f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CAOUT_R {
        CAOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CASHORT_R {
        CASHORT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&self) -> P2CA_R {
        P2CA_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&mut self) -> CAOUT_W {
        CAOUT_W { w: self }
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W {
        CAF_W { w: self }
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&mut self) -> CASHORT_W {
        CASHORT_W { w: self }
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&mut self) -> P2CA_W {
        P2CA_W { w: self }
    }
}
