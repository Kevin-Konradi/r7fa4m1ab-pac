#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcsr: Adcsr,
    _reserved1: [u8; 0x02],
    adansa0: Adansa0,
    adansa1: Adansa1,
    adads0: Adads0,
    adads1: Adads1,
    adadc: Adadc,
    _reserved6: [u8; 0x01],
    adcer: Adcer,
    adstrgr: Adstrgr,
    adexicr: Adexicr,
    adansb0: Adansb0,
    adansb1: Adansb1,
    addbldr: Addbldr,
    adtsdr: Adtsdr,
    adocdr: Adocdr,
    adrd: Adrd,
    addr: [Addr; 15],
    _reserved16: [u8; 0x02],
    addr16: Addr16,
    addr17: Addr16,
    addr18: Addr16,
    addr19: Addr16,
    addr20: Addr16,
    addr21: Addr16,
    addr22: Addr16,
    addr23: Addr16,
    addr24: Addr16,
    addr25: Addr16,
    _reserved26: [u8; 0x26],
    addiscr: Addiscr,
    _reserved27: [u8; 0x05],
    adgspcr: Adgspcr,
    _reserved28: [u8; 0x02],
    addbldra: Addbldra,
    addbldrb: Addbldrb,
    _reserved30: [u8; 0x02],
    adhvrefcnt: Adhvrefcnt,
    _reserved31: [u8; 0x01],
    adwinmon: Adwinmon,
    _reserved32: [u8; 0x03],
    adcmpcr: Adcmpcr,
    adcmpanser: Adcmpanser,
    adcmpler: Adcmpler,
    adcmpansr0: Adcmpansr0,
    adcmpansr1: Adcmpansr1,
    adcmplr0: Adcmplr0,
    adcmplr1: Adcmplr1,
    adcmpdr0: Adcmpdr0,
    adcmpdr1: Adcmpdr1,
    adcmpsr0: Adcmpsr0,
    adcmpsr1: Adcmpsr1,
    adcmpser: Adcmpser,
    _reserved44: [u8; 0x01],
    adcmpbnsr: Adcmpbnsr,
    _reserved45: [u8; 0x01],
    adwinllb: Adwinllb,
    adwinulb: Adwinulb,
    adcmpbsr: Adcmpbsr,
    _reserved48: [u8; 0x30],
    adsstrl: Adsstrl,
    adsstrt: Adsstrt,
    adsstro: Adsstro,
    adsstr: [Adsstr; 15],
}
impl RegisterBlock {
    #[doc = "0x00 - A/D Control Register"]
    #[inline(always)]
    pub const fn adcsr(&self) -> &Adcsr {
        &self.adcsr
    }
    #[doc = "0x04 - A/D Channel Select Register A0"]
    #[inline(always)]
    pub const fn adansa0(&self) -> &Adansa0 {
        &self.adansa0
    }
    #[doc = "0x06 - A/D Channel Select Register A1"]
    #[inline(always)]
    pub const fn adansa1(&self) -> &Adansa1 {
        &self.adansa1
    }
    #[doc = "0x08 - A/D-Converted Value Addition/Average Channel Select Register 0"]
    #[inline(always)]
    pub const fn adads0(&self) -> &Adads0 {
        &self.adads0
    }
    #[doc = "0x0a - A/D-Converted Value Addition/Average Channel Select Register 1"]
    #[inline(always)]
    pub const fn adads1(&self) -> &Adads1 {
        &self.adads1
    }
    #[doc = "0x0c - A/D-Converted Value Addition/Average Count Select Register"]
    #[inline(always)]
    pub const fn adadc(&self) -> &Adadc {
        &self.adadc
    }
    #[doc = "0x0e - A/D Control Extended Register"]
    #[inline(always)]
    pub const fn adcer(&self) -> &Adcer {
        &self.adcer
    }
    #[doc = "0x10 - A/D Conversion Start Trigger Select Register"]
    #[inline(always)]
    pub const fn adstrgr(&self) -> &Adstrgr {
        &self.adstrgr
    }
    #[doc = "0x12 - A/D Conversion Extended Input Control Register"]
    #[inline(always)]
    pub const fn adexicr(&self) -> &Adexicr {
        &self.adexicr
    }
    #[doc = "0x14 - A/D Channel Select Register B0"]
    #[inline(always)]
    pub const fn adansb0(&self) -> &Adansb0 {
        &self.adansb0
    }
    #[doc = "0x16 - A/D Channel Select Register B1"]
    #[inline(always)]
    pub const fn adansb1(&self) -> &Adansb1 {
        &self.adansb1
    }
    #[doc = "0x18 - A/D Data Duplication Register"]
    #[inline(always)]
    pub const fn addbldr(&self) -> &Addbldr {
        &self.addbldr
    }
    #[doc = "0x1a - A/D Temperature Sensor Data Register"]
    #[inline(always)]
    pub const fn adtsdr(&self) -> &Adtsdr {
        &self.adtsdr
    }
    #[doc = "0x1c - A/D Internal Reference Voltage Data Register"]
    #[inline(always)]
    pub const fn adocdr(&self) -> &Adocdr {
        &self.adocdr
    }
    #[doc = "0x1e - A/D Self-Diagnosis Data Register"]
    #[inline(always)]
    pub const fn adrd(&self) -> &Adrd {
        &self.adrd
    }
    #[doc = "0x20..0x3e - A/D Data Register %s"]
    #[inline(always)]
    pub const fn addr(&self, n: usize) -> &Addr {
        &self.addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x3e - A/D Data Register %s"]
    #[inline(always)]
    pub fn addr_iter(&self) -> impl Iterator<Item = &Addr> {
        self.addr.iter()
    }
    #[doc = "0x40 - A/D Data Register 16"]
    #[inline(always)]
    pub const fn addr16(&self) -> &Addr16 {
        &self.addr16
    }
    #[doc = "0x40 - A/D Data Register 17"]
    #[inline(always)]
    pub const fn addr17(&self) -> &Addr16 {
        &self.addr17
    }
    #[doc = "0x40 - A/D Data Register 18"]
    #[inline(always)]
    pub const fn addr18(&self) -> &Addr16 {
        &self.addr18
    }
    #[doc = "0x40 - A/D Data Register 19"]
    #[inline(always)]
    pub const fn addr19(&self) -> &Addr16 {
        &self.addr19
    }
    #[doc = "0x40 - A/D Data Register 20"]
    #[inline(always)]
    pub const fn addr20(&self) -> &Addr16 {
        &self.addr20
    }
    #[doc = "0x40 - A/D Data Register 21"]
    #[inline(always)]
    pub const fn addr21(&self) -> &Addr16 {
        &self.addr21
    }
    #[doc = "0x40 - A/D Data Register 22"]
    #[inline(always)]
    pub const fn addr22(&self) -> &Addr16 {
        &self.addr22
    }
    #[doc = "0x40 - A/D Data Register 23"]
    #[inline(always)]
    pub const fn addr23(&self) -> &Addr16 {
        &self.addr23
    }
    #[doc = "0x40 - A/D Data Register 24"]
    #[inline(always)]
    pub const fn addr24(&self) -> &Addr16 {
        &self.addr24
    }
    #[doc = "0x40 - A/D Data Register 25"]
    #[inline(always)]
    pub const fn addr25(&self) -> &Addr16 {
        &self.addr25
    }
    #[doc = "0x7a - A/D Disconnection Detection Control Register"]
    #[inline(always)]
    pub const fn addiscr(&self) -> &Addiscr {
        &self.addiscr
    }
    #[doc = "0x80 - A/D Group Scan Priority Control Register"]
    #[inline(always)]
    pub const fn adgspcr(&self) -> &Adgspcr {
        &self.adgspcr
    }
    #[doc = "0x84 - A/D Data Duplexing Register A"]
    #[inline(always)]
    pub const fn addbldra(&self) -> &Addbldra {
        &self.addbldra
    }
    #[doc = "0x86 - A/D Data Duplexing Register B"]
    #[inline(always)]
    pub const fn addbldrb(&self) -> &Addbldrb {
        &self.addbldrb
    }
    #[doc = "0x8a - A/D High-Potential/Low-Potential Reference Voltage Control Register"]
    #[inline(always)]
    pub const fn adhvrefcnt(&self) -> &Adhvrefcnt {
        &self.adhvrefcnt
    }
    #[doc = "0x8c - A/D Compare Function Window A/B Status Monitor Register"]
    #[inline(always)]
    pub const fn adwinmon(&self) -> &Adwinmon {
        &self.adwinmon
    }
    #[doc = "0x90 - A/D Compare Function Control Register"]
    #[inline(always)]
    pub const fn adcmpcr(&self) -> &Adcmpcr {
        &self.adcmpcr
    }
    #[doc = "0x92 - A/D Compare Function Window A Extended Input Select Register"]
    #[inline(always)]
    pub const fn adcmpanser(&self) -> &Adcmpanser {
        &self.adcmpanser
    }
    #[doc = "0x93 - A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
    #[inline(always)]
    pub const fn adcmpler(&self) -> &Adcmpler {
        &self.adcmpler
    }
    #[doc = "0x94 - A/D Compare Function Window A Channel Select Register 0"]
    #[inline(always)]
    pub const fn adcmpansr0(&self) -> &Adcmpansr0 {
        &self.adcmpansr0
    }
    #[doc = "0x96 - A/D Compare Function Window A Channel Select Register 1"]
    #[inline(always)]
    pub const fn adcmpansr1(&self) -> &Adcmpansr1 {
        &self.adcmpansr1
    }
    #[doc = "0x98 - A/D Compare Function Window A Comparison Condition Setting Register 0"]
    #[inline(always)]
    pub const fn adcmplr0(&self) -> &Adcmplr0 {
        &self.adcmplr0
    }
    #[doc = "0x9a - A/D Compare Function Window A Comparison Condition Setting Register 1"]
    #[inline(always)]
    pub const fn adcmplr1(&self) -> &Adcmplr1 {
        &self.adcmplr1
    }
    #[doc = "0x9c - A/D Compare Function Window A Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr0(&self) -> &Adcmpdr0 {
        &self.adcmpdr0
    }
    #[doc = "0x9e - A/D Compare Function Window A Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adcmpdr1(&self) -> &Adcmpdr1 {
        &self.adcmpdr1
    }
    #[doc = "0xa0 - A/D Compare Function Window A Channel Status Register 0"]
    #[inline(always)]
    pub const fn adcmpsr0(&self) -> &Adcmpsr0 {
        &self.adcmpsr0
    }
    #[doc = "0xa2 - A/D Compare Function Window A Channel Status Register 1"]
    #[inline(always)]
    pub const fn adcmpsr1(&self) -> &Adcmpsr1 {
        &self.adcmpsr1
    }
    #[doc = "0xa4 - A/D Compare Function Window A Extended Input Channel Status Register"]
    #[inline(always)]
    pub const fn adcmpser(&self) -> &Adcmpser {
        &self.adcmpser
    }
    #[doc = "0xa6 - A/D Compare Function Window B Channel Selection Register"]
    #[inline(always)]
    pub const fn adcmpbnsr(&self) -> &Adcmpbnsr {
        &self.adcmpbnsr
    }
    #[doc = "0xa8 - A/D Compare Function Window B Lower-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinllb(&self) -> &Adwinllb {
        &self.adwinllb
    }
    #[doc = "0xaa - A/D Compare Function Window B Upper-Side Level Setting Register"]
    #[inline(always)]
    pub const fn adwinulb(&self) -> &Adwinulb {
        &self.adwinulb
    }
    #[doc = "0xac - A/D Compare Function Window B Status Register"]
    #[inline(always)]
    pub const fn adcmpbsr(&self) -> &Adcmpbsr {
        &self.adcmpbsr
    }
    #[doc = "0xdd - A/D Sampling State Register L"]
    #[inline(always)]
    pub const fn adsstrl(&self) -> &Adsstrl {
        &self.adsstrl
    }
    #[doc = "0xde - A/D Sampling State Register T"]
    #[inline(always)]
    pub const fn adsstrt(&self) -> &Adsstrt {
        &self.adsstrt
    }
    #[doc = "0xdf - A/D Sampling State Register O"]
    #[inline(always)]
    pub const fn adsstro(&self) -> &Adsstro {
        &self.adsstro
    }
    #[doc = "0xe0..0xef - A/D Sampling State Register %s"]
    #[inline(always)]
    pub const fn adsstr(&self, n: usize) -> &Adsstr {
        &self.adsstr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0xef - A/D Sampling State Register %s"]
    #[inline(always)]
    pub fn adsstr_iter(&self) -> impl Iterator<Item = &Adsstr> {
        self.adsstr.iter()
    }
}
#[doc = "ADCSR (rw) register accessor: A/D Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsr`] module"]
#[doc(alias = "ADCSR")]
pub type Adcsr = crate::Reg<adcsr::AdcsrSpec>;
#[doc = "A/D Control Register"]
pub mod adcsr;
#[doc = "ADANSA0 (rw) register accessor: A/D Channel Select Register A0\n\nYou can [`read`](crate::Reg::read) this register and get [`adansa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansa0`] module"]
#[doc(alias = "ADANSA0")]
pub type Adansa0 = crate::Reg<adansa0::Adansa0Spec>;
#[doc = "A/D Channel Select Register A0"]
pub mod adansa0;
#[doc = "ADANSA1 (rw) register accessor: A/D Channel Select Register A1\n\nYou can [`read`](crate::Reg::read) this register and get [`adansa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansa1`] module"]
#[doc(alias = "ADANSA1")]
pub type Adansa1 = crate::Reg<adansa1::Adansa1Spec>;
#[doc = "A/D Channel Select Register A1"]
pub mod adansa1;
#[doc = "ADADS0 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adads0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adads0`] module"]
#[doc(alias = "ADADS0")]
pub type Adads0 = crate::Reg<adads0::Adads0Spec>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
pub mod adads0;
#[doc = "ADADS1 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adads1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adads1`] module"]
#[doc(alias = "ADADS1")]
pub type Adads1 = crate::Reg<adads1::Adads1Spec>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
pub mod adads1;
#[doc = "ADADC (rw) register accessor: A/D-Converted Value Addition/Average Count Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adadc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adadc`] module"]
#[doc(alias = "ADADC")]
pub type Adadc = crate::Reg<adadc::AdadcSpec>;
#[doc = "A/D-Converted Value Addition/Average Count Select Register"]
pub mod adadc;
#[doc = "ADCER (rw) register accessor: A/D Control Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcer`] module"]
#[doc(alias = "ADCER")]
pub type Adcer = crate::Reg<adcer::AdcerSpec>;
#[doc = "A/D Control Extended Register"]
pub mod adcer;
#[doc = "ADSTRGR (rw) register accessor: A/D Conversion Start Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adstrgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adstrgr`] module"]
#[doc(alias = "ADSTRGR")]
pub type Adstrgr = crate::Reg<adstrgr::AdstrgrSpec>;
#[doc = "A/D Conversion Start Trigger Select Register"]
pub mod adstrgr;
#[doc = "ADEXICR (rw) register accessor: A/D Conversion Extended Input Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adexicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adexicr`] module"]
#[doc(alias = "ADEXICR")]
pub type Adexicr = crate::Reg<adexicr::AdexicrSpec>;
#[doc = "A/D Conversion Extended Input Control Register"]
pub mod adexicr;
#[doc = "ADANSB0 (rw) register accessor: A/D Channel Select Register B0\n\nYou can [`read`](crate::Reg::read) this register and get [`adansb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansb0`] module"]
#[doc(alias = "ADANSB0")]
pub type Adansb0 = crate::Reg<adansb0::Adansb0Spec>;
#[doc = "A/D Channel Select Register B0"]
pub mod adansb0;
#[doc = "ADANSB1 (rw) register accessor: A/D Channel Select Register B1\n\nYou can [`read`](crate::Reg::read) this register and get [`adansb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adansb1`] module"]
#[doc(alias = "ADANSB1")]
pub type Adansb1 = crate::Reg<adansb1::Adansb1Spec>;
#[doc = "A/D Channel Select Register B1"]
pub mod adansb1;
#[doc = "ADDBLDR (r) register accessor: A/D Data Duplication Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldr`] module"]
#[doc(alias = "ADDBLDR")]
pub type Addbldr = crate::Reg<addbldr::AddbldrSpec>;
#[doc = "A/D Data Duplication Register"]
pub mod addbldr;
#[doc = "ADTSDR (r) register accessor: A/D Temperature Sensor Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adtsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adtsdr`] module"]
#[doc(alias = "ADTSDR")]
pub type Adtsdr = crate::Reg<adtsdr::AdtsdrSpec>;
#[doc = "A/D Temperature Sensor Data Register"]
pub mod adtsdr;
#[doc = "ADOCDR (r) register accessor: A/D Internal Reference Voltage Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adocdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adocdr`] module"]
#[doc(alias = "ADOCDR")]
pub type Adocdr = crate::Reg<adocdr::AdocdrSpec>;
#[doc = "A/D Internal Reference Voltage Data Register"]
pub mod adocdr;
#[doc = "ADRD (r) register accessor: A/D Self-Diagnosis Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adrd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adrd`] module"]
#[doc(alias = "ADRD")]
pub type Adrd = crate::Reg<adrd::AdrdSpec>;
#[doc = "A/D Self-Diagnosis Data Register"]
pub mod adrd;
#[doc = "ADDR (r) register accessor: A/D Data Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "A/D Data Register %s"]
pub mod addr;
pub use Addr as Addr16;
pub use addr as addr16;
#[doc = "ADDISCR (rw) register accessor: A/D Disconnection Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addiscr`] module"]
#[doc(alias = "ADDISCR")]
pub type Addiscr = crate::Reg<addiscr::AddiscrSpec>;
#[doc = "A/D Disconnection Detection Control Register"]
pub mod addiscr;
#[doc = "ADGSPCR (rw) register accessor: A/D Group Scan Priority Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adgspcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adgspcr`] module"]
#[doc(alias = "ADGSPCR")]
pub type Adgspcr = crate::Reg<adgspcr::AdgspcrSpec>;
#[doc = "A/D Group Scan Priority Control Register"]
pub mod adgspcr;
#[doc = "ADDBLDRA (r) register accessor: A/D Data Duplexing Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldra`] module"]
#[doc(alias = "ADDBLDRA")]
pub type Addbldra = crate::Reg<addbldra::AddbldraSpec>;
#[doc = "A/D Data Duplexing Register A"]
pub mod addbldra;
#[doc = "ADDBLDRB (r) register accessor: A/D Data Duplexing Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`addbldrb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addbldrb`] module"]
#[doc(alias = "ADDBLDRB")]
pub type Addbldrb = crate::Reg<addbldrb::AddbldrbSpec>;
#[doc = "A/D Data Duplexing Register B"]
pub mod addbldrb;
#[doc = "ADHVREFCNT (rw) register accessor: A/D High-Potential/Low-Potential Reference Voltage Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adhvrefcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adhvrefcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adhvrefcnt`] module"]
#[doc(alias = "ADHVREFCNT")]
pub type Adhvrefcnt = crate::Reg<adhvrefcnt::AdhvrefcntSpec>;
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
pub mod adhvrefcnt;
#[doc = "ADWINMON (r) register accessor: A/D Compare Function Window A/B Status Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinmon`] module"]
#[doc(alias = "ADWINMON")]
pub type Adwinmon = crate::Reg<adwinmon::AdwinmonSpec>;
#[doc = "A/D Compare Function Window A/B Status Monitor Register"]
pub mod adwinmon;
#[doc = "ADCMPCR (rw) register accessor: A/D Compare Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpcr`] module"]
#[doc(alias = "ADCMPCR")]
pub type Adcmpcr = crate::Reg<adcmpcr::AdcmpcrSpec>;
#[doc = "A/D Compare Function Control Register"]
pub mod adcmpcr;
#[doc = "ADCMPANSER (rw) register accessor: A/D Compare Function Window A Extended Input Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpanser`] module"]
#[doc(alias = "ADCMPANSER")]
pub type Adcmpanser = crate::Reg<adcmpanser::AdcmpanserSpec>;
#[doc = "A/D Compare Function Window A Extended Input Select Register"]
pub mod adcmpanser;
#[doc = "ADCMPLER (rw) register accessor: A/D Compare Function Window A Extended Input Comparison Condition Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpler`] module"]
#[doc(alias = "ADCMPLER")]
pub type Adcmpler = crate::Reg<adcmpler::AdcmplerSpec>;
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
pub mod adcmpler;
#[doc = "ADCMPANSR0 (rw) register accessor: A/D Compare Function Window A Channel Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpansr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpansr0`] module"]
#[doc(alias = "ADCMPANSR0")]
pub type Adcmpansr0 = crate::Reg<adcmpansr0::Adcmpansr0Spec>;
#[doc = "A/D Compare Function Window A Channel Select Register 0"]
pub mod adcmpansr0;
#[doc = "ADCMPANSR1 (rw) register accessor: A/D Compare Function Window A Channel Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpansr1`] module"]
#[doc(alias = "ADCMPANSR1")]
pub type Adcmpansr1 = crate::Reg<adcmpansr1::Adcmpansr1Spec>;
#[doc = "A/D Compare Function Window A Channel Select Register 1"]
pub mod adcmpansr1;
#[doc = "ADCMPLR0 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmplr0`] module"]
#[doc(alias = "ADCMPLR0")]
pub type Adcmplr0 = crate::Reg<adcmplr0::Adcmplr0Spec>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
pub mod adcmplr0;
#[doc = "ADCMPLR1 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmplr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmplr1`] module"]
#[doc(alias = "ADCMPLR1")]
pub type Adcmplr1 = crate::Reg<adcmplr1::Adcmplr1Spec>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
pub mod adcmplr1;
#[doc = "ADCMPDR0 (rw) register accessor: A/D Compare Function Window A Lower-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpdr0`] module"]
#[doc(alias = "ADCMPDR0")]
pub type Adcmpdr0 = crate::Reg<adcmpdr0::Adcmpdr0Spec>;
#[doc = "A/D Compare Function Window A Lower-Side Level Setting Register"]
pub mod adcmpdr0;
#[doc = "ADCMPDR1 (rw) register accessor: A/D Compare Function Window A Upper-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpdr1`] module"]
#[doc(alias = "ADCMPDR1")]
pub type Adcmpdr1 = crate::Reg<adcmpdr1::Adcmpdr1Spec>;
#[doc = "A/D Compare Function Window A Upper-Side Level Setting Register"]
pub mod adcmpdr1;
#[doc = "ADCMPSR0 (rw) register accessor: A/D Compare Function Window A Channel Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpsr0`] module"]
#[doc(alias = "ADCMPSR0")]
pub type Adcmpsr0 = crate::Reg<adcmpsr0::Adcmpsr0Spec>;
#[doc = "A/D Compare Function Window A Channel Status Register 0"]
pub mod adcmpsr0;
#[doc = "ADCMPSR1 (rw) register accessor: A/D Compare Function Window A Channel Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpsr1`] module"]
#[doc(alias = "ADCMPSR1")]
pub type Adcmpsr1 = crate::Reg<adcmpsr1::Adcmpsr1Spec>;
#[doc = "A/D Compare Function Window A Channel Status Register 1"]
pub mod adcmpsr1;
#[doc = "ADCMPSER (rw) register accessor: A/D Compare Function Window A Extended Input Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpser`] module"]
#[doc(alias = "ADCMPSER")]
pub type Adcmpser = crate::Reg<adcmpser::AdcmpserSpec>;
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
pub mod adcmpser;
#[doc = "ADCMPBNSR (rw) register accessor: A/D Compare Function Window B Channel Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpbnsr`] module"]
#[doc(alias = "ADCMPBNSR")]
pub type Adcmpbnsr = crate::Reg<adcmpbnsr::AdcmpbnsrSpec>;
#[doc = "A/D Compare Function Window B Channel Selection Register"]
pub mod adcmpbnsr;
#[doc = "ADWINLLB (rw) register accessor: A/D Compare Function Window B Lower-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinllb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinllb`] module"]
#[doc(alias = "ADWINLLB")]
pub type Adwinllb = crate::Reg<adwinllb::AdwinllbSpec>;
#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register"]
pub mod adwinllb;
#[doc = "ADWINULB (rw) register accessor: A/D Compare Function Window B Upper-Side Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adwinulb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adwinulb`] module"]
#[doc(alias = "ADWINULB")]
pub type Adwinulb = crate::Reg<adwinulb::AdwinulbSpec>;
#[doc = "A/D Compare Function Window B Upper-Side Level Setting Register"]
pub mod adwinulb;
#[doc = "ADCMPBSR (rw) register accessor: A/D Compare Function Window B Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpbsr`] module"]
#[doc(alias = "ADCMPBSR")]
pub type Adcmpbsr = crate::Reg<adcmpbsr::AdcmpbsrSpec>;
#[doc = "A/D Compare Function Window B Status Register"]
pub mod adcmpbsr;
#[doc = "ADSSTRL (rw) register accessor: A/D Sampling State Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstrl`] module"]
#[doc(alias = "ADSSTRL")]
pub type Adsstrl = crate::Reg<adsstrl::AdsstrlSpec>;
#[doc = "A/D Sampling State Register L"]
pub mod adsstrl;
#[doc = "ADSSTRT (rw) register accessor: A/D Sampling State Register T\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstrt`] module"]
#[doc(alias = "ADSSTRT")]
pub type Adsstrt = crate::Reg<adsstrt::AdsstrtSpec>;
#[doc = "A/D Sampling State Register T"]
pub mod adsstrt;
#[doc = "ADSSTRO (rw) register accessor: A/D Sampling State Register O\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstro`] module"]
#[doc(alias = "ADSSTRO")]
pub type Adsstro = crate::Reg<adsstro::AdsstroSpec>;
#[doc = "A/D Sampling State Register O"]
pub mod adsstro;
#[doc = "ADSSTR (rw) register accessor: A/D Sampling State Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`adsstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsstr`] module"]
#[doc(alias = "ADSSTR")]
pub type Adsstr = crate::Reg<adsstr::AdsstrSpec>;
#[doc = "A/D Sampling State Register %s"]
pub mod adsstr;
