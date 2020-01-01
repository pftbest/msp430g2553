#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    _reserved1: [u8; 7usize],
    #[doc = "0x08 - Port 3 Input"]
    pub p3in: P3IN,
    #[doc = "0x09 - Port 3 Output"]
    pub p3out: P3OUT,
    #[doc = "0x0a - Port 3 Direction"]
    pub p3dir: P3DIR,
    #[doc = "0x0b - Port 3 Selection"]
    pub p3sel: P3SEL,
    _reserved5: [u8; 39usize],
    #[doc = "0x33 - Port 3 Selection 2"]
    pub p3sel2: P3SEL2,
}
#[doc = "Port 3 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3ren](p3ren) module"]
pub type P3REN = crate::Reg<u8, _P3REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3REN;
#[doc = "`read()` method returns [p3ren::R](p3ren::R) reader structure"]
impl crate::Readable for P3REN {}
#[doc = "`write(|w| ..)` method takes [p3ren::W](p3ren::W) writer structure"]
impl crate::Writable for P3REN {}
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "Port 3 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3in](p3in) module"]
pub type P3IN = crate::Reg<u8, _P3IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3IN;
#[doc = "`read()` method returns [p3in::R](p3in::R) reader structure"]
impl crate::Readable for P3IN {}
#[doc = "`write(|w| ..)` method takes [p3in::W](p3in::W) writer structure"]
impl crate::Writable for P3IN {}
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "Port 3 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3out](p3out) module"]
pub type P3OUT = crate::Reg<u8, _P3OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3OUT;
#[doc = "`read()` method returns [p3out::R](p3out::R) reader structure"]
impl crate::Readable for P3OUT {}
#[doc = "`write(|w| ..)` method takes [p3out::W](p3out::W) writer structure"]
impl crate::Writable for P3OUT {}
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "Port 3 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3dir](p3dir) module"]
pub type P3DIR = crate::Reg<u8, _P3DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3DIR;
#[doc = "`read()` method returns [p3dir::R](p3dir::R) reader structure"]
impl crate::Readable for P3DIR {}
#[doc = "`write(|w| ..)` method takes [p3dir::W](p3dir::W) writer structure"]
impl crate::Writable for P3DIR {}
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "Port 3 Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel](p3sel) module"]
pub type P3SEL = crate::Reg<u8, _P3SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL;
#[doc = "`read()` method returns [p3sel::R](p3sel::R) reader structure"]
impl crate::Readable for P3SEL {}
#[doc = "`write(|w| ..)` method takes [p3sel::W](p3sel::W) writer structure"]
impl crate::Writable for P3SEL {}
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "Port 3 Selection 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3sel2](p3sel2) module"]
pub type P3SEL2 = crate::Reg<u8, _P3SEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P3SEL2;
#[doc = "`read()` method returns [p3sel2::R](p3sel2::R) reader structure"]
impl crate::Readable for P3SEL2 {}
#[doc = "`write(|w| ..)` method takes [p3sel2::W](p3sel2::W) writer structure"]
impl crate::Writable for P3SEL2 {}
#[doc = "Port 3 Selection 2"]
pub mod p3sel2;
