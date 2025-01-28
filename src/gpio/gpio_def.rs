use super::Gpio;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $Rec:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, [$($A:literal),*] $(, $MODE:ty)?),)+
    ]) => {
        #[doc=concat!("Port ", $port_id)]
        pub mod $gpiox {
            use crate::pac::$GPIOX;
            use crate::rcc::{rec, ResetEnable};

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl crate::gpio::GpioExt for $GPIOX {
                type Parts = Parts;
                type Rec = rec::$Rec;

                fn split(self, prec: rec::$Rec) -> Parts {
                    prec.enable().reset();

                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }

                fn split_without_reset(self, prec: rec::$Rec) -> Parts {
                    prec.enable();

                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            #[doc=concat!("Common type for GPIO", $port_id, " related pins")]
            pub type $PXn<MODE> = crate::gpio::PartiallyErasedPin<$port_id, MODE>;

            $(
                #[doc=concat!("P", $port_id, $i, " pin")]
                pub type $PXi<MODE = crate::gpio::Analog> = crate::gpio::Pin<$port_id, $i, MODE>;

                $(
                    impl<MODE> crate::gpio::marker::IntoAf<$A> for $PXi<MODE> { }
                )*
            )+

        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}

#[cfg(feature = "gpio-h503")]
pub use h503::*;

#[cfg(feature = "gpio-h503")]
mod h503 {
    use super::Gpio;

    gpio!(GPIOA, gpioa, Gpioa, PA, 'A', PAn, [
        PA0: (pa0, 0, [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        PA1: (pa1, 1, [1, 4, 5, 6, 7, 8, 9, 11, 14, 15]),
        PA2: (pa2, 2, [1, 2, 3, 4, 6, 7, 8, 14, 15]),
        PA3: (pa3, 3, [1, 3, 4, 5, 6, 7, 8, 13, 14, 15]),
        PA4: (pa4, 4, [1, 3, 4, 5, 6, 7, 8, 10, 13, 14, 15]),
        PA5: (pa5, 5, [1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15]),
        PA6: (pa6, 6, [1, 2, 5, 15]),
        PA7: (pa7, 7, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15]),
        PA8: (pa8, 8, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        PA9: (pa9, 9, [0, 1, 3, 4, 5, 7, 10, 13, 15]),
        PA10: (pa10, 10, [1, 3, 4, 7, 15]),
        PA11: (pa11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15]),
        PA12: (pa12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15]),
        PA13: (pa13, 13, [0, 1, 2, 7, 8, 9, 12, 14, 15], crate::gpio::Debugger),
        PA14: (pa14, 14, [0, 1, 2, 3, 4, 7, 8, 9, 14, 15], crate::gpio::Debugger),
        PA15: (pa15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15], crate::gpio::Debugger),
    ]);

    gpio!(GPIOB, gpiob, Gpiob, PB, 'B', PBn, [
        PB0: (pb0, 0, [1, 2, 4, 9, 14, 15]),
        PB1: (pb1, 1, [1, 2, 3, 5, 6, 9, 12, 14, 15]),
        PB2: (pb2, 2, [0, 1, 2, 4, 5, 6, 7, 9, 14, 15]),
        PB3: (pb3, 3, [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15], crate::gpio::Debugger),
        PB4: (pb4, 4, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 15], crate::gpio::Debugger),
        PB5: (pb5, 5, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15]),
        PB6: (pb6, 6, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 15]),
        PB7: (pb7, 7, [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 13, 14, 15]),
        PB8: (pb8, 8, [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        PB10: (pb10, 10, [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15]),
        PB12: (pb12, 12, [1, 4, 5, 7, 8, 9, 15]),
        PB13: (pb13, 13, [1, 3, 4, 5, 7, 8, 9, 10, 11, 15]),
        PB14: (pb14, 14, [1, 2, 4, 5, 7, 8, 15]),
        PB15: (pb15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    ]);

    gpio!(GPIOC, gpioc, Gpioc, PC, 'C', PCn, [
        PC0: (pc0, 0, [1, 2, 5, 7, 15]),
        PC1: (pc1, 1, [0, 2, 4, 5, 7, 8, 14, 15]),
        PC2: (pc2, 2, [0, 4, 5, 7, 8, 15]),
        PC3: (pc3, 3, [0, 3, 4, 5, 7, 14, 15]),
        PC4: (pc4, 4, [1, 3, 4, 5, 7, 15]),
        PC5: (pc5, 5, [1, 5, 12, 14, 15]),
        PC6: (pc6, 6, [1, 2, 3, 4, 5, 8, 9, 13, 15]),
        PC7: (pc7, 7, [0, 1, 2, 3, 5, 6, 8, 9, 13, 15]),
        PC8: (pc8, 8, [0, 1, 2, 3, 4, 5, 8, 9, 13, 15]),
        PC9: (pc9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 9, 13, 15]),
        PC10: (pc10, 10, [1, 3, 5, 6, 7, 8, 9, 13, 15]),
        PC11: (pc11, 11, [1, 3, 4, 5, 6, 7, 8, 14, 15]),
        PC12: (pc12, 12, [0, 1, 2, 3, 6, 7, 8, 14, 15]),
        PC13: (pc13, 13, [15]),
        PC14: (pc14, 14, [15]),
        PC15: (pc15, 15, [15]),
    ]);

    gpio!(GPIOD, gpiod, Gpiod, PD, 'D', PDn, [
        PD2: (pd2, 2, [0, 1, 2, 6, 7, 9, 14, 15]),
    ]);

    gpio!(GPIOH, gpioh, Gpioh, PH, 'H', PHn, [
        PH0: (ph0, 0, [15]),
        PH1: (ph1, 1, [15]),
    ]);

    impl<const P: char> Gpio<P> {
        pub(crate) const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
            const {
                match P {
                    'A' => crate::pac::GPIOA::ptr(),
                    'B' => crate::pac::GPIOB::ptr() as _,
                    'C' => crate::pac::GPIOC::ptr() as _,
                    'D' => crate::pac::GPIOD::ptr() as _,
                    'H' => crate::pac::GPIOH::ptr() as _,
                    _ => panic!("Unknown GPIO port"),
                }
            }
        }
    }
}

#[cfg(feature = "gpio-h523_h533")]
pub use h523_h533::*;

#[cfg(feature = "gpio-h523_h533")]
mod h523_h533 {
    use super::Gpio;

    gpio!(GPIOA, gpioa, Gpioa, PA, 'A', PAn, [
        PA0: (pa0, 0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 14, 15]),
        PA1: (pa1, 1, [1, 2, 4, 5, 6, 7, 8, 9, 14, 15]),
        PA2: (pa2, 2, [1, 2, 3, 4, 5, 7, 15]),
        PA3: (pa3, 3, [1, 2, 3, 4, 5, 6, 7, 15]),
        PA4: (pa4, 4, [2, 3, 4, 5, 6, 7, 13, 15]),
        PA5: (pa5, 5, [1, 3, 5, 13, 14, 15]),
        PA6: (pa6, 6, [1, 2, 3, 5, 6, 13, 15]),
        PA7: (pa7, 7, [1, 2, 3, 5, 10, 13, 15]),
        PA8: (pa8, 8, [0, 1, 3, 4, 5, 6, 7, 9, 10, 12, 13, 15]),
        PA9: (pa9, 9, [1, 3, 4, 5, 7, 12, 13, 15]),
        PA10: (pa10, 10, [1, 3, 4, 6, 7, 9, 12, 13, 15]),
        PA11: (pa11, 11, [1, 3, 5, 6, 7, 9, 10, 15]),
        PA12: (pa12, 12, [1, 3, 5, 6, 7, 9, 10, 15]),
        PA13: (pa13, 13, [0, 15], crate::gpio::Debugger),
        PA14: (pa14, 14, [0, 15], crate::gpio::Debugger),
        PA15: (pa15, 15, [0, 1, 4, 5, 6, 7, 8, 9, 12, 13, 14, 15], crate::gpio::Debugger),
    ]);

    gpio!(GPIOB, gpiob, Gpiob, PB, 'B', PBn, [
        PB0: (pb0, 0, [1, 2, 3, 5, 6, 7, 8, 15]),
        PB1: (pb1, 1, [1, 2, 3, 4, 5, 6, 7, 15]),
        PB2: (pb2, 2, [0, 3, 4, 5, 6, 7, 9, 10, 12, 15]),
        PB3: (pb3, 3, [0, 1, 3, 4, 5, 6, 8, 9, 10, 14, 15], crate::gpio::Debugger),
        PB4: (pb4, 4, [0, 2, 3, 4, 5, 6, 7, 9, 10, 13, 15], crate::gpio::Debugger),
        PB5: (pb5, 5, [2, 3, 4, 5, 6, 7, 9, 10, 13, 14, 15]),
        PB6: (pb6, 6, [2, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15]),
        PB7: (pb7, 7, [2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 15]),
        PB8: (pb8, 8, [2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 15]),
        PB9: (pb9, 9, [2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 15]),
        PB10: (pb10, 10, [1, 2, 3, 4, 5, 7, 9, 15]),
        PB12: (pb12, 12, [1, 2, 3, 4, 5, 6, 7, 9, 14, 15]),
        PB13: (pb13, 13, [1, 2, 3, 4, 5, 7, 8, 9, 12, 14, 15]),
        PB14: (pb14, 14, [1, 2, 3, 4, 5, 7, 8, 15]),
        PB15: (pb15, 15, [0, 1, 2, 3, 4, 5, 6, 8, 10, 13, 14, 15]),
    ]);

    gpio!(GPIOC, gpioc, Gpioc, PC, 'C', PCn, [
        PC0: (pc0, 0, [6, 7, 9, 10, 15]),
        PC1: (pc1, 1, [0, 5, 6, 10, 15]),
        PC2: (pc2, 2, [0, 2, 5, 6, 9, 15]),
        PC3: (pc3, 3, [0, 3, 5, 6, 9, 15]),
        PC4: (pc4, 4, [1, 3, 5, 7, 15]),
        PC5: (pc5, 5, [1, 4, 6, 10, 15]),
        PC6: (pc6, 6, [2, 3, 5, 7, 8, 9, 10, 11, 12, 13, 15]),
        PC7: (pc7, 7, [0, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 15]), //TRGIO (AF0) was missing from autogenerated code
        PC8: (pc8, 8, [0, 2, 3, 7, 8, 9, 10, 11, 12, 13, 15]),
        PC9: (pc9, 9, [0, 2, 3, 4, 5, 8, 9, 11, 12, 13, 15]),
        PC10: (pc10, 10, [3, 6, 7, 8, 9, 12, 13, 15]),
        PC11: (pc11, 11, [3, 6, 7, 8, 9, 12, 13, 15]),
        PC12: (pc12, 12, [0, 2, 3, 6, 7, 8, 12, 13, 15]),
        PC13: (pc13, 13, [15]),
        PC14: (pc14, 14, [15]),
        PC15: (pc15, 15, [15]),
    ]);

    gpio!(GPIOD, gpiod, Gpiod, PD, 'D', PDn, [
        PD0: (pd0, 0, [3, 8, 9, 12, 15]),
        PD1: (pd1, 1, [8, 9, 12, 15]),
        PD2: (pd2, 2, [0, 2, 4, 8, 12, 13, 15]),
        PD3: (pd3, 3, [5, 7, 12, 13, 15]),
        PD4: (pd4, 4, [7, 10, 12, 15]),
        PD5: (pd5, 5, [1, 5, 7, 9, 10, 12, 15]),
        PD6: (pd6, 6, [3, 4, 5, 7, 10, 11, 12, 13, 15]),
        PD7: (pd7, 7, [3, 4, 5, 6, 7, 10, 11, 12, 15]),
        PD8: (pd8, 8, [7, 12, 15]),
        PD9: (pd9, 9, [7, 9, 12, 15]),
        PD10: (pd10, 10, [3, 7, 12, 15]),
        PD11: (pd11, 11, [3, 7, 8, 9, 12, 15]),
        PD12: (pd12, 12, [1, 2, 3, 5, 7, 8, 9, 12, 13, 15]),
        PD13: (pd13, 13, [1, 2, 3, 5, 9, 12, 13, 15]),
        PD14: (pd14, 14, [2, 12, 15]),
        PD15: (pd15, 15, [2, 12, 15]),
    ]);

    gpio!(GPIOE, gpioe, Gpioe, PE, 'E', PEn, [
        PE0: (pe0, 0, [1, 2, 3, 4, 6, 9, 12, 13, 15]),
        PE2: (pe2, 2, [0, 1, 5, 9, 12, 13, 15]),
        PE3: (pe3, 3, [0, 4, 12, 15]),
        PE4: (pe4, 4, [0, 4, 5, 12, 13, 15]),
        PE5: (pe5, 5, [0, 4, 5, 12, 13, 15]),
        PE6: (pe6, 6, [0, 1, 4, 5, 12, 13, 15]),
        PE7: (pe7, 7, [1, 10, 12, 15]),
        PE8: (pe8, 8, [1, 10, 12, 15]),
        PE9: (pe9, 9, [1, 10, 12, 15]),
        PE10: (pe10, 10, [1, 10, 12, 15]),
        PE11: (pe11, 11, [1, 4, 5, 6, 12, 15]),
        PE12: (pe12, 12, [1, 5, 12, 15]),
        PE13: (pe13, 13, [1, 5, 12, 15]),
        PE14: (pe14, 14, [1, 5, 12, 15]),
        PE15: (pe15, 15, [1, 3, 12, 15]),
    ]);

    gpio!(GPIOF, gpiof, Gpiof, PF, 'F', PFn, [
        PF0: (pf0, 0, [3, 4, 12, 15]),
        PF1: (pf1, 1, [3, 4, 12, 15]),
        PF2: (pf2, 2, [4, 12, 15]),
        PF3: (pf3, 3, [12, 15]),
        PF4: (pf4, 4, [12, 15]),
        PF5: (pf5, 5, [5, 12, 15]),
        PF6: (pf6, 6, [10, 15]),
        PF7: (pf7, 7, [10, 15]),
        PF8: (pf8, 8, [10, 15]),
        PF9: (pf9, 9, [10, 15]),
        PF10: (pf10, 10, [4, 9, 13, 15]),
        PF11: (pf11, 11, [9, 13, 15]),
        PF12: (pf12, 12, [12, 15]),
        PF13: (pf13, 13, [12, 15]),
        PF14: (pf14, 14, [12, 15]),
        PF15: (pf15, 15, [5, 12, 15]),
    ]);

    gpio!(GPIOG, gpiog, Gpiog, PG, 'G', PGn, [
        PG0: (pg0, 0, [12, 15]),
        PG1: (pg1, 1, [7, 12, 15]),
        PG2: (pg2, 2, [3, 12, 15]),
        PG3: (pg3, 3, [3, 12, 15]),
        PG4: (pg4, 4, [1, 12, 15]),
        PG5: (pg5, 5, [1, 12, 15]),
        PG6: (pg6, 6, [3, 5, 10, 11, 12, 13, 15]),
        PG7: (pg7, 7, [3, 7, 11, 12, 13, 15]),
        PG8: (pg8, 8, [3, 5, 7, 15]),
        PG9: (pg9, 9, [5, 7, 9, 11, 12, 13, 15]),
        PG10: (pg10, 10, [5, 11, 12, 13, 15]),
        PG11: (pg11, 11, [1, 5, 10, 13, 15]),
        PG12: (pg12, 12, [1, 4, 7, 10, 12, 13, 15]),
        PG13: (pg13, 13, [0, 1, 7, 12, 15]),
        PG14: (pg14, 14, [0, 1, 4, 7, 9, 12, 15]),
        PG15: (pg15, 15, [5, 7, 13, 15]),
    ]);

    gpio!(GPIOH, gpioh, Gpioh, PH, 'H', PHn, [
        PH0: (ph0, 0, [15]),
        PH1: (ph1, 1, [15]),
    ]);

    impl<const P: char> Gpio<P> {
        pub(crate) const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
            const {
                match P {
                    'A' => crate::pac::GPIOA::ptr(),
                    'B' => crate::pac::GPIOB::ptr() as _,
                    'C' => crate::pac::GPIOC::ptr() as _,
                    'D' => crate::pac::GPIOD::ptr() as _,
                    'E' => crate::pac::GPIOE::ptr() as _,
                    'F' => crate::pac::GPIOF::ptr() as _,
                    'G' => crate::pac::GPIOG::ptr() as _,
                    'H' => crate::pac::GPIOH::ptr() as _,
                    _ => panic!("Unknown GPIO port"),
                }
            }
        }
    }
}

#[cfg(feature = "gpio-h56x_h573")]
pub use h56x_h573::*;

#[cfg(feature = "gpio-h56x_h573")]
mod h56x_h573 {
    use super::Gpio;

    gpio!(GPIOA, gpioa, Gpioa, PA, 'A', PAn, [
        PA0: (pa0, 0, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15]),
        PA1: (pa1, 1, [1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 15]),
        PA2: (pa2, 2, [1, 2, 4, 5, 7, 8, 11, 15]),
        PA3: (pa3, 3, [1, 2, 3, 4, 5, 6, 7, 11, 15]),
        PA4: (pa4, 4, [2, 3, 5, 6, 7, 8, 13, 15]),
        PA5: (pa5, 5, [1, 3, 5, 8, 11, 13, 14, 15]),
        PA6: (pa6, 6, [1, 2, 3, 5, 6, 7, 8, 9, 13, 15]),
        PA7: (pa7, 7, [1, 2, 3, 5, 7, 8, 9, 10, 11, 12, 13, 15]),
        PA8: (pa8, 8, [0, 1, 3, 4, 5, 7, 10, 11, 12, 13, 15]),
        PA9: (pa9, 9, [1, 3, 4, 5, 7, 11, 12, 13, 15]),
        PA10: (pa10, 10, [1, 3, 4, 6, 7, 9, 12, 13, 15]),
        PA11: (pa11, 11, [1, 3, 5, 6, 7, 9, 10, 15]),
        PA12: (pa12, 12, [1, 3, 5, 6, 7, 8, 9, 10, 15]),
        PA13: (pa13, 13, [0, 15], crate::gpio::Debugger),
        PA14: (pa14, 14, [0, 15], crate::gpio::Debugger),
        PA15: (pa15, 15, [0, 1, 2, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15], crate::gpio::Debugger),
    ]);

    gpio!(GPIOB, gpiob, Gpiob, PB, 'B', PBn, [
        PB0: (pb0, 0, [1, 2, 3, 6, 7, 8, 11, 14, 15]),
        PB1: (pb1, 1, [1, 2, 3, 6, 11, 14, 15]),
        PB2: (pb2, 2, [0, 2, 3, 4, 5, 6, 7, 9, 10, 12, 13, 15]),
        PB3: (pb3, 3, [0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15], crate::gpio::Debugger),
        PB4: (pb4, 4, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 14, 15], crate::gpio::Debugger),
        PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15]),
        PB6: (pb6, 6, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15]),
        PB7: (pb7, 7, [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
        PB8: (pb8, 8, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
        PB9: (pb9, 9, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
        PB10: (pb10, 10, [1, 3, 4, 5, 7, 9, 11, 15]),
        PB11: (pb11, 11, [1, 3, 4, 5, 6, 7, 11, 12, 15]),
        PB12: (pb12, 12, [1, 3, 4, 5, 7, 9, 11, 14, 15]),
        PB13: (pb13, 13, [1, 2, 3, 4, 5, 7, 9, 12, 14, 15]),
        PB14: (pb14, 14, [1, 2, 3, 4, 5, 7, 8, 9, 14, 15]),
        PB15: (pb15, 15, [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15]),
    ]);

    gpio!(GPIOC, gpioc, Gpioc, PC, 'C', PCn, [
        PC0: (pc0, 0, [1, 6, 7, 8, 9, 10, 12, 15]),
        PC1: (pc1, 1, [0, 2, 5, 6, 7, 8, 9, 10, 11, 15]),
        PC2: (pc2, 2, [0, 1, 2, 5, 6, 9, 11, 12, 15]),
        PC3: (pc3, 3, [0, 2, 3, 5, 6, 9, 11, 12, 15]),
        PC4: (pc4, 4, [1, 2, 3, 5, 7, 11, 12, 15]),
        PC5: (pc5, 5, [1, 2, 4, 6, 7, 10, 11, 12, 15]),
        PC6: (pc6, 6, [2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
        PC7: (pc7, 7, [0, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
        PC8: (pc8, 8, [0, 2, 3, 7, 8, 9, 10, 11, 12, 13, 15]),
        PC9: (pc9, 9, [0, 2, 3, 4, 5, 8, 9, 11, 12, 13, 15]),
        PC10: (pc10, 10, [2, 6, 7, 8, 9, 11, 12, 13, 15]),
        PC11: (pc11, 11, [2, 6, 7, 8, 9, 12, 13, 15]),
        PC12: (pc12, 12, [0, 2, 5, 6, 7, 8, 12, 13, 15]),
        PC13: (pc13, 13, [15]),
        PC14: (pc14, 14, [15]),
        PC15: (pc15, 15, [15]),
    ]);

    gpio!(GPIOD, gpiod, Gpiod, PD, 'D', PDn, [
        PD0: (pd0, 0, [3, 8, 9, 11, 12, 15]),
        PD1: (pd1, 1, [8, 9, 12, 15]),
        PD2: (pd2, 2, [0, 2, 4, 8, 12, 13, 14, 15]),
        PD3: (pd3, 3, [5, 7, 12, 13, 15]),
        PD4: (pd4, 4, [7, 10, 12, 15]),
        PD5: (pd5, 5, [1, 5, 7, 9, 10, 12, 15]),
        PD6: (pd6, 6, [2, 5, 6, 7, 10, 11, 12, 13, 15]),
        PD7: (pd7, 7, [5, 7, 10, 11, 12, 14, 15]),
        PD8: (pd8, 8, [7, 9, 12, 15]),
        PD9: (pd9, 9, [7, 9, 12, 15]),
        PD10: (pd10, 10, [3, 7, 12, 15]),
        PD11: (pd11, 11, [2, 3, 4, 7, 8, 9, 10, 12, 15]),
        PD12: (pd12, 12, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15]),
        PD13: (pd13, 13, [1, 2, 3, 4, 5, 9, 10, 11, 12, 13, 14, 15]),
        PD14: (pd14, 14, [2, 8, 11, 12, 15]),
        PD15: (pd15, 15, [2, 8, 11, 12, 15]),
    ]);

    gpio!(GPIOE, gpioe, Gpioe, PE, 'E', PEn, [
        PE0: (pe0, 0, [1, 2, 3, 4, 6, 8, 9, 10, 12, 13, 15]),
        PE1: (pe1, 1, [1, 8, 9, 12, 13, 15]),
        PE2: (pe2, 2, [0, 1, 2, 5, 6, 7, 8, 9, 11, 12, 13, 15]),
        PE3: (pe3, 3, [0, 4, 6, 7, 12, 15]),
        PE4: (pe4, 4, [0, 2, 4, 5, 6, 12, 13, 15]),
        PE5: (pe5, 5, [0, 2, 4, 5, 6, 12, 13, 15]),
        PE6: (pe6, 6, [0, 1, 2, 4, 5, 6, 10, 12, 13, 15]),
        PE7: (pe7, 7, [1, 6, 7, 10, 12, 15]),
        PE8: (pe8, 8, [1, 6, 7, 10, 12, 15]),
        PE9: (pe9, 9, [1, 6, 7, 10, 12, 15]),
        PE10: (pe10, 10, [1, 6,10, 12,  7]),
        PE11: (pe11, 11, [1, 4, 5, 6, 10, 12, 15]),
        PE12: (pe12, 12, [1, 5, 10, 12, 15]),
        PE13: (pe13, 13, [1, 5, 10, 12, 15]),
        PE14: (pe14, 14, [1, 5, 10, 12, 15]),
        PE15: (pe15, 15, [1, 3, 7, 12, 15]),
    ]);

    gpio!(GPIOF, gpiof, Gpiof, PF, 'F', PFn, [
        PF0: (pf0, 0, [4, 12, 13, 15]),
        PF1: (pf1, 1, [4, 12, 13, 15]),
        PF2: (pf2, 2, [2, 3, 4, 6, 7, 12, 13, 15]),
        PF3: (pf3, 3, [2, 7, 12, 13, 15]),
        PF4: (pf4, 4, [2, 7, 12, 15]),
        PF5: (pf5, 5, [2, 4, 5, 6, 7, 12, 14, 15]),
        PF6: (pf6, 6, [1, 5, 6, 7, 10, 13, 15]),
        PF7: (pf7, 7, [1, 5, 6, 7, 10, 13, 15]),
        PF8: (pf8, 8, [1, 5, 6, 7, 9, 10, 13, 15]),
        PF9: (pf9, 9, [1, 5, 6, 7, 9, 10, 13, 15]),
        PF10: (pf10, 10, [1, 2, 4, 9, 13, 15]),
        PF11: (pf11, 11, [5, 9, 10, 12, 13, 14, 15]),
        PF12: (pf12, 12, [12, 14, 15]),
        PF13: (pf13, 13, [4, 12, 14, 15]),
        PF14: (pf14, 14, [12, 14, 15]),
        PF15: (pf15, 15, [4, 5, 12, 15]),
    ]);

    gpio!(GPIOG, gpiog, Gpiog, PG, 'G', PGn, [
        PG0: (pg0, 0, [11, 12, 14, 15]),
        PG1: (pg1, 1, [7, 11, 12, 15]),
        PG2: (pg2, 2, [3, 7, 12, 14, 15]),
        PG3: (pg3, 3, [3, 7, 12, 13, 15]),
        PG4: (pg4, 4, [1, 12, 14, 15]),
        PG5: (pg5, 5, [1, 12, 15]),
        PG6: (pg6, 6, [1, 3, 4, 5, 10, 11, 12, 13, 15]),
        PG7: (pg7, 7, [2, 3, 4, 6, 7, 11, 12, 13, 15]),
        PG8: (pg8, 8, [3, 5, 7, 11, 12, 15]),
        PG9: (pg9, 9, [5, 7, 9, 10, 11, 12, 13, 15]),
        PG10: (pg10, 10, [5, 10, 11, 12, 13, 15]),
        PG11: (pg11, 11, [1, 5, 6, 7, 10, 11, 13, 15]),
        PG12: (pg12, 12, [1, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15]),
        PG13: (pg13, 13, [0, 1, 5, 6, 7, 10, 11, 12, 13, 15]),
        PG14: (pg14, 14, [0, 1, 4, 5, 6, 7, 10, 11, 12, 13, 15]),
        PG15: (pg15, 15, [5, 6, 7, 12, 13, 15]),
    ]);

    gpio!(GPIOH, gpioh, Gpioh, PH, 'H', PHn, [
        PH0: (ph0, 0, [15]),
        PH1: (ph1, 1, [15]),
        PH2: (ph2, 2, [1, 9, 10, 11, 12, 15]),
        PH3: (ph3, 3, [9, 10, 11, 12, 15]),
        PH4: (ph4, 4, [4, 5, 7, 13, 15]),
        PH5: (ph5, 5, [4, 5, 7, 12, 15]),
        PH6: (ph6, 6, [1, 2, 3, 4, 5, 11, 12, 13, 15]),
        PH7: (ph7, 7, [1, 3, 4, 5, 11, 12, 13, 15]),
        PH8: (ph8, 8, [1, 2, 3, 4, 5, 13, 15]),
        PH9: (ph9, 9, [1, 2, 3, 4, 5, 13, 15]),
        PH10: (ph10, 10, [1, 2, 3, 4, 5, 13, 15]),
        PH11: (ph11, 11, [1, 2, 3, 4, 5, 13, 15]),
        PH12: (ph12, 12, [1, 2, 3, 4, 5, 10, 13, 15]),
        PH13: (ph13, 13, [1, 3, 7, 8, 9, 13, 15]),
        PH14: (ph14, 14, [3, 8, 9, 13, 15]),
        PH15: (ph15, 15, [3, 13, 15]),
    ]);

    gpio!(GPIOI, gpioi, Gpioi, PI, 'I', PIn, [
        PI0: (pi0, 0, [2, 5, 13, 15]),
        PI1: (pi1, 1, [3, 5, 13, 15]),
        PI2: (pi2, 2, [3, 5, 13, 15]),
        PI3: (pi3, 3, [3, 5, 13, 15]),
        PI4: (pi4, 4, [3, 7, 10, 13, 15]),
        PI5: (pi5, 5, [3, 10, 13, 15]),
        PI6: (pi6, 6, [3, 10, 13, 15]),
        PI7: (pi7, 7, [3, 10, 13, 15]),
        PI8: (pi8, 8, [15]),
        PI9: (pi9, 9, [8, 9, 15]),
        PI10: (pi10, 10, [9, 11, 13, 15]),
        PI11: (pi11, 11, [13, 15]),
    ]);

    impl<const P: char> Gpio<P> {
        pub(crate) const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
            const {
                match P {
                    'A' => crate::pac::GPIOA::ptr(),
                    'B' => crate::pac::GPIOB::ptr() as _,
                    'C' => crate::pac::GPIOC::ptr() as _,
                    'D' => crate::pac::GPIOD::ptr() as _,
                    'E' => crate::pac::GPIOE::ptr() as _,
                    'F' => crate::pac::GPIOF::ptr() as _,
                    'G' => crate::pac::GPIOG::ptr() as _,
                    'H' => crate::pac::GPIOH::ptr() as _,
                    'I' => crate::pac::GPIOI::ptr() as _,
                    _ => panic!("Unknown GPIO port"),
                }
            }
        }
    }
}
