use core::marker;
#[doc = " Trait implemented by readable registers to enable the `read` method.\r"]
#[doc = "\r"]
#[doc = " Registers marked with `Writable` can be also `modify`'ed.\r"]
pub trait Readable {}
#[doc = " Trait implemented by writeable registers.\r"]
#[doc = "\r"]
#[doc = " This enables the  `write`, `write_with_zero` and `reset` methods.\r"]
#[doc = "\r"]
#[doc = " Registers marked with `Readable` can be also `modify`'ed.\r"]
pub trait Writable {}
#[doc = " Reset value of the register.\r"]
#[doc = "\r"]
#[doc = " This value is the initial value for the `write` method. It can also be directly written to the\r"]
#[doc = " register by using the `reset` method.\r"]
pub trait ResetValue {
    #[doc = " Raw register type (`u8`, `u16`, `u32`, ...).\r"]
    type Type;
    #[doc = " Reset value of the register.\r"]
    fn reset_value() -> Self::Type;
}
#[doc = " This structure provides volatile access to registers.\r"]
pub struct Reg<U, REG> {
    register: vcell::VolatileCell<U>,
    _marker: marker::PhantomData<REG>,
}
unsafe impl<U: Send, REG> Send for Reg<U, REG> {}
impl<U, REG> Reg<U, REG>
where
    Self: Readable,
    U: Copy,
{
    #[doc = " Reads the contents of a `Readable` register.\r"]
    #[doc = "\r"]
    #[doc = " You can read the raw contents of a register by using `bits`:\r"]
    #[doc = " ```ignore\r"]
    #[doc = " let bits = periph.reg.read().bits();\r"]
    #[doc = " ```\r"]
    #[doc = " or get the content of a particular field of a register:\r"]
    #[doc = " ```ignore\r"]
    #[doc = " let reader = periph.reg.read();\r"]
    #[doc = " let bits = reader.field1().bits();\r"]
    #[doc = " let flag = reader.field2().bit_is_set();\r"]
    #[doc = " ```\r"]
    #[inline(always)]
    pub fn read(&self) -> R<U, Self> {
        R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        }
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: ResetValue<Type = U> + Writable,
    U: Copy,
{
    #[doc = " Writes the reset value to `Writable` register.\r"]
    #[doc = "\r"]
    #[doc = " Resets the register to its initial state.\r"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: ResetValue<Type = U> + Writable,
    U: Copy,
{
    #[doc = " Writes bits to a `Writable` register.\r"]
    #[doc = "\r"]
    #[doc = " You can write raw bits into a register:\r"]
    #[doc = " ```ignore\r"]
    #[doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });\r"]
    #[doc = " ```\r"]
    #[doc = " or write only the fields you need:\r"]
    #[doc = " ```ignore\r"]
    #[doc = " periph.reg.write(|w| w\r"]
    #[doc = "     .field1().bits(newfield1bits)\r"]
    #[doc = "     .field2().set_bit()\r"]
    #[doc = "     .field3().variant(VARIANT)\r"]
    #[doc = " );\r"]
    #[doc = " ```\r"]
    #[doc = " In the latter case, other fields will be set to their reset value.\r"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
                _reg: marker::PhantomData,
            })
            .bits,
        );
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: Writable,
    U: Copy + Default,
{
    #[doc = " Writes 0 to a `Writable` register.\r"]
    #[doc = "\r"]
    #[doc = " Similar to `write`, but unused bits will contain 0.\r"]
    #[inline(always)]
    pub fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        self.register.set(
            f(&mut W {
                bits: U::default(),
                _reg: marker::PhantomData,
            })
            .bits,
        );
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: Readable + Writable,
    U: Copy,
{
    #[doc = " Modifies the contents of the register by reading and then writing it.\r"]
    #[doc = "\r"]
    #[doc = " E.g. to do a read-modify-write sequence to change parts of a register:\r"]
    #[doc = " ```ignore\r"]
    #[doc = " periph.reg.modify(|r, w| unsafe { w.bits(\r"]
    #[doc = "    r.bits() | 3\r"]
    #[doc = " ) });\r"]
    #[doc = " ```\r"]
    #[doc = " or\r"]
    #[doc = " ```ignore\r"]
    #[doc = " periph.reg.modify(|_, w| w\r"]
    #[doc = "     .field1().bits(newfield1bits)\r"]
    #[doc = "     .field2().set_bit()\r"]
    #[doc = "     .field3().variant(VARIANT)\r"]
    #[doc = " );\r"]
    #[doc = " ```\r"]
    #[doc = " Other fields will have the value they had before the call to `modify`.\r"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F:
            FnOnce(&R<U, Self>, &'w mut W<U, Self>) -> &'w mut W<U, Self>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &R {
                    bits,
                    _reg: marker::PhantomData,
                },
                &mut W {
                    bits,
                    _reg: marker::PhantomData,
                },
            )
            .bits,
        );
    }
}
#[doc = " Register/field reader.\r"]
#[doc = "\r"]
#[doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`\r"]
#[doc = " method.\r"]
pub struct R<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}
impl<U, T> R<U, T>
where
    U: Copy,
{
    #[doc = " Creates a new instance of the reader.\r"]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
    #[doc = " Reads raw bits from register/field.\r"]
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}
impl<U, T, FI> PartialEq<FI> for R<U, T>
where
    U: PartialEq,
    FI: Copy + Into<U>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&(*other).into())
    }
}
impl<FI> R<bool, FI> {
    #[doc = " Value of the field as raw bits.\r"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = " Returns `true` if the bit is clear (0).\r"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = " Returns `true` if the bit is set (1).\r"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = " Register writer.\r"]
#[doc = "\r"]
#[doc = " Used as an argument to the closures in the `write` and `modify` methods of the register.\r"]
pub struct W<U, REG> {
    #[doc = "Writable bits\r"]
    pub(crate) bits: U,
    _reg: marker::PhantomData<REG>,
}
impl<U, REG> W<U, REG> {
    #[doc = " Writes raw bits to the register.\r"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: U) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = " Used if enumerated values cover not the whole range.\r"]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant<U, T> {
    #[doc = " Expected variant.\r"]
    Val(T),
    #[doc = " Raw bits.\r"]
    Res(U),
}
