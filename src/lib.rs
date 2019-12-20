#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430G2553 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn TRAPINT();
    fn PORT1();
    fn PORT2();
    fn ADC10();
    fn USCIAB0TX();
    fn USCIAB0RX();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn WDT();
    fn COMPARATORA();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn NMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 15] = [
    Vector { _handler: TRAPINT },
    Vector { _reserved: 0 },
    Vector { _handler: PORT1 },
    Vector { _handler: PORT2 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC10 },
    Vector {
        _handler: USCIAB0TX,
    },
    Vector {
        _handler: USCIAB0RX,
    },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: WDT },
    Vector {
        _handler: COMPARATORA,
    },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector { _handler: NMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - 0xFFE0 TRAPINT"]
    TRAPINT,
    #[doc = "2 - 0xFFE4 Port 1"]
    PORT1,
    #[doc = "3 - 0xFFE6 Port 2"]
    PORT2,
    #[doc = "5 - 0xFFEA ADC10"]
    ADC10,
    #[doc = "6 - 0xFFEC USCI A0/B0 Transmit"]
    USCIAB0TX,
    #[doc = "7 - 0xFFEE USCI A0/B0 Receive"]
    USCIAB0RX,
    #[doc = "8 - 0xFFF0 Timer0)A CC1, TA0"]
    TIMER0_A1,
    #[doc = "9 - 0xFFF2 Timer0_A CC0"]
    TIMER0_A0,
    #[doc = "10 - 0xFFF4 Watchdog Timer"]
    WDT,
    #[doc = "11 - 0xFFF6 Comparator A"]
    COMPARATORA,
    #[doc = "12 - 0xFFF8 Timer1_A CC1-4, TA1"]
    TIMER1_A1,
    #[doc = "13 - 0xFFFA Timer1_A CC0"]
    TIMER1_A0,
    #[doc = "14 - 0xFFFC Non-maskable"]
    NMI,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::TRAPINT => 0,
            Interrupt::PORT1 => 2,
            Interrupt::PORT2 => 3,
            Interrupt::ADC10 => 5,
            Interrupt::USCIAB0TX => 6,
            Interrupt::USCIAB0RX => 7,
            Interrupt::TIMER0_A1 => 8,
            Interrupt::TIMER0_A0 => 9,
            Interrupt::WDT => 10,
            Interrupt::COMPARATORA => 11,
            Interrupt::TIMER1_A1 => 12,
            Interrupt::TIMER1_A0 => 13,
            Interrupt::NMI => 14,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x0128 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        0x5c as *const _
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Comparator A"]
pub struct COMPARATOR_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMPARATOR_A {}
impl COMPARATOR_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comparator_a::RegisterBlock {
        0x58 as *const _
    }
}
impl Deref for COMPARATOR_A {
    type Target = comparator_a::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMPARATOR_A::ptr() }
    }
}
#[doc = "Comparator A"]
pub mod comparator_a;
#[doc = "Timer1_A3"]
pub struct TIMER1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_A3 {}
impl TIMER1_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1_a3::RegisterBlock {
        0x011e as *const _
    }
}
impl Deref for TIMER1_A3 {
    type Target = timer1_a3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1_A3::ptr() }
    }
}
#[doc = "Timer1_A3"]
pub mod timer1_a3;
#[doc = "Timer0_A3"]
pub struct TIMER0_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_A3 {}
impl TIMER0_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0_a3::RegisterBlock {
        0x012e as *const _
    }
}
impl Deref for TIMER0_A3 {
    type Target = timer0_a3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0_A3::ptr() }
    }
}
#[doc = "Timer0_A3"]
pub mod timer0_a3;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        0x68 as *const _
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "TLV Calibration Data"]
pub struct TLV_CALIBRATION_DATA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TLV_CALIBRATION_DATA {}
impl TLV_CALIBRATION_DATA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tlv_calibration_data::RegisterBlock {
        0x10c0 as *const _
    }
}
impl Deref for TLV_CALIBRATION_DATA {
    type Target = tlv_calibration_data::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TLV_CALIBRATION_DATA::ptr() }
    }
}
#[doc = "TLV Calibration Data"]
pub mod tlv_calibration_data;
#[doc = "ADC10"]
pub struct ADC10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC10 {}
impl ADC10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc10::RegisterBlock {
        0x48 as *const _
    }
}
impl Deref for ADC10 {
    type Target = adc10::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC10::ptr() }
    }
}
#[doc = "ADC10"]
pub mod adc10;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        0x68 as *const _
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        0x60 as *const _
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "Special Function"]
pub struct SPECIAL_FUNCTION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPECIAL_FUNCTION {}
impl SPECIAL_FUNCTION {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const special_function::RegisterBlock {
        0 as *const _
    }
}
impl Deref for SPECIAL_FUNCTION {
    type Target = special_function::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPECIAL_FUNCTION::ptr() }
    }
}
#[doc = "Special Function"]
pub mod special_function;
#[doc = "Port 3/4"]
pub struct PORT_3_4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3_4 {}
impl PORT_3_4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_3_4::RegisterBlock {
        0x10 as *const _
    }
}
impl Deref for PORT_3_4 {
    type Target = port_3_4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_3_4::ptr() }
    }
}
#[doc = "Port 3/4"]
pub mod port_3_4;
#[doc = "Calibration Data"]
pub struct CALIBRATION_DATA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CALIBRATION_DATA {}
impl CALIBRATION_DATA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const calibration_data::RegisterBlock {
        0x10f8 as *const _
    }
}
impl Deref for CALIBRATION_DATA {
    type Target = calibration_data::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CALIBRATION_DATA::ptr() }
    }
}
#[doc = "Calibration Data"]
pub mod calibration_data;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        0x20 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "System Clock"]
pub struct SYSTEM_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM_CLOCK {}
impl SYSTEM_CLOCK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_clock::RegisterBlock {
        0x52 as *const _
    }
}
impl Deref for SYSTEM_CLOCK {
    type Target = system_clock::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEM_CLOCK::ptr() }
    }
}
#[doc = "System Clock"]
pub mod system_clock;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "COMPARATOR_A"]
    pub COMPARATOR_A: COMPARATOR_A,
    #[doc = "TIMER1_A3"]
    pub TIMER1_A3: TIMER1_A3,
    #[doc = "TIMER0_A3"]
    pub TIMER0_A3: TIMER0_A3,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "TLV_CALIBRATION_DATA"]
    pub TLV_CALIBRATION_DATA: TLV_CALIBRATION_DATA,
    #[doc = "ADC10"]
    pub ADC10: ADC10,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "SPECIAL_FUNCTION"]
    pub SPECIAL_FUNCTION: SPECIAL_FUNCTION,
    #[doc = "PORT_3_4"]
    pub PORT_3_4: PORT_3_4,
    #[doc = "CALIBRATION_DATA"]
    pub CALIBRATION_DATA: CALIBRATION_DATA,
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "SYSTEM_CLOCK"]
    pub SYSTEM_CLOCK: SYSTEM_CLOCK,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FLASH: FLASH {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            COMPARATOR_A: COMPARATOR_A {
                _marker: PhantomData,
            },
            TIMER1_A3: TIMER1_A3 {
                _marker: PhantomData,
            },
            TIMER0_A3: TIMER0_A3 {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            TLV_CALIBRATION_DATA: TLV_CALIBRATION_DATA {
                _marker: PhantomData,
            },
            ADC10: ADC10 {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            SPECIAL_FUNCTION: SPECIAL_FUNCTION {
                _marker: PhantomData,
            },
            PORT_3_4: PORT_3_4 {
                _marker: PhantomData,
            },
            CALIBRATION_DATA: CALIBRATION_DATA {
                _marker: PhantomData,
            },
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            SYSTEM_CLOCK: SYSTEM_CLOCK {
                _marker: PhantomData,
            },
        }
    }
}
