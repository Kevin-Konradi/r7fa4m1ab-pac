#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syscfg: Syscfg,
    _reserved1: [u8; 0x02],
    syssts0: Syssts0,
    _reserved2: [u8; 0x02],
    dvstctr0: Dvstctr0,
    _reserved3: [u8; 0x0a],
    _reserved_3_cfifo: [u8; 0x02],
    _reserved4: [u8; 0x02],
    _reserved_4_d: [u8; 0x02],
    _reserved5: [u8; 0x02],
    _reserved_5_d: [u8; 0x02],
    _reserved6: [u8; 0x02],
    cfifosel: Cfifosel,
    cfifoctr: Cfifoctr,
    _reserved8: [u8; 0x04],
    d0fifosel: D0fifosel,
    d0fifoctr: D0fifoctr,
    d1fifosel: D1fifosel,
    d1fifoctr: D1fifoctr,
    intenb0: Intenb0,
    intenb1: Intenb1,
    _reserved14: [u8; 0x02],
    brdyenb: Brdyenb,
    nrdyenb: Nrdyenb,
    bempenb: Bempenb,
    sofcfg: Sofcfg,
    _reserved18: [u8; 0x02],
    intsts0: Intsts0,
    intsts1: Intsts1,
    _reserved20: [u8; 0x02],
    brdysts: Brdysts,
    nrdysts: Nrdysts,
    bempsts: Bempsts,
    frmnum: Frmnum,
    _reserved24: [u8; 0x06],
    usbreq: Usbreq,
    usbval: Usbval,
    usbindx: Usbindx,
    usbleng: Usbleng,
    dcpcfg: Dcpcfg,
    dcpmaxp: Dcpmaxp,
    dcpctr: Dcpctr,
    _reserved31: [u8; 0x02],
    pipesel: Pipesel,
    _reserved32: [u8; 0x02],
    pipecfg: Pipecfg,
    _reserved33: [u8; 0x02],
    pipemaxp: Pipemaxp,
    pipeperi: Pipeperi,
    pipectr: [Pipectr; 5],
    pipe6ctr: Pipe6ctr,
    pipe7ctr: Pipe6ctr,
    pipe8ctr: Pipe6ctr,
    pipe9ctr: Pipe6ctr,
    _reserved40: [u8; 0x0e],
    pipetre: (),
    _reserved41: [u8; 0x02],
    pipetrn: (),
    _reserved42: [u8; 0x1e],
    usbbcctrl0: Usbbcctrl0,
    _reserved43: [u8; 0x1a],
    usbmc: Usbmc,
    _reserved44: [u8; 0x02],
    devadd: [Devadd; 6],
}
impl RegisterBlock {
    #[doc = "0x00 - System Configuration Control Register"]
    #[inline(always)]
    pub const fn syscfg(&self) -> &Syscfg {
        &self.syscfg
    }
    #[doc = "0x04 - System Configuration Status Register 0"]
    #[inline(always)]
    pub const fn syssts0(&self) -> &Syssts0 {
        &self.syssts0
    }
    #[doc = "0x08 - Device State Control Register 0"]
    #[inline(always)]
    pub const fn dvstctr0(&self) -> &Dvstctr0 {
        &self.dvstctr0
    }
    #[doc = "0x14 - CFIFO Port Register L"]
    #[inline(always)]
    pub const fn cfifol(&self) -> &Cfifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - CFIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &Cfifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register L"]
    #[inline(always)]
    pub const fn d0fifol(&self) -> &D0fifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - D0FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifo(&self) -> &D0fifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register L"]
    #[inline(always)]
    pub const fn d1fifol(&self) -> &D1fifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - D1FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(&self) -> &D1fifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - CFIFO Port Select Register"]
    #[inline(always)]
    pub const fn cfifosel(&self) -> &Cfifosel {
        &self.cfifosel
    }
    #[doc = "0x22 - CFIFO Port Control Register"]
    #[inline(always)]
    pub const fn cfifoctr(&self) -> &Cfifoctr {
        &self.cfifoctr
    }
    #[doc = "0x28 - D0FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d0fifosel(&self) -> &D0fifosel {
        &self.d0fifosel
    }
    #[doc = "0x2a - D0FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d0fifoctr(&self) -> &D0fifoctr {
        &self.d0fifoctr
    }
    #[doc = "0x2c - D1FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d1fifosel(&self) -> &D1fifosel {
        &self.d1fifosel
    }
    #[doc = "0x2e - D1FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d1fifoctr(&self) -> &D1fifoctr {
        &self.d1fifoctr
    }
    #[doc = "0x30 - Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn intenb0(&self) -> &Intenb0 {
        &self.intenb0
    }
    #[doc = "0x32 - Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn intenb1(&self) -> &Intenb1 {
        &self.intenb1
    }
    #[doc = "0x36 - BRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn brdyenb(&self) -> &Brdyenb {
        &self.brdyenb
    }
    #[doc = "0x38 - NRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nrdyenb(&self) -> &Nrdyenb {
        &self.nrdyenb
    }
    #[doc = "0x3a - BEMP Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bempenb(&self) -> &Bempenb {
        &self.bempenb
    }
    #[doc = "0x3c - SOF Output Configuration Register"]
    #[inline(always)]
    pub const fn sofcfg(&self) -> &Sofcfg {
        &self.sofcfg
    }
    #[doc = "0x40 - Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn intsts0(&self) -> &Intsts0 {
        &self.intsts0
    }
    #[doc = "0x42 - Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn intsts1(&self) -> &Intsts1 {
        &self.intsts1
    }
    #[doc = "0x46 - BRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn brdysts(&self) -> &Brdysts {
        &self.brdysts
    }
    #[doc = "0x48 - NRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn nrdysts(&self) -> &Nrdysts {
        &self.nrdysts
    }
    #[doc = "0x4a - BEMP Interrupt Status Register"]
    #[inline(always)]
    pub const fn bempsts(&self) -> &Bempsts {
        &self.bempsts
    }
    #[doc = "0x4c - Frame Number Register"]
    #[inline(always)]
    pub const fn frmnum(&self) -> &Frmnum {
        &self.frmnum
    }
    #[doc = "0x54 - USB Request Type Register"]
    #[inline(always)]
    pub const fn usbreq(&self) -> &Usbreq {
        &self.usbreq
    }
    #[doc = "0x56 - USB Request Value Register"]
    #[inline(always)]
    pub const fn usbval(&self) -> &Usbval {
        &self.usbval
    }
    #[doc = "0x58 - USB Request Index Register"]
    #[inline(always)]
    pub const fn usbindx(&self) -> &Usbindx {
        &self.usbindx
    }
    #[doc = "0x5a - USB Request Length Register"]
    #[inline(always)]
    pub const fn usbleng(&self) -> &Usbleng {
        &self.usbleng
    }
    #[doc = "0x5c - DCP Configuration Register"]
    #[inline(always)]
    pub const fn dcpcfg(&self) -> &Dcpcfg {
        &self.dcpcfg
    }
    #[doc = "0x5e - DCP Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn dcpmaxp(&self) -> &Dcpmaxp {
        &self.dcpmaxp
    }
    #[doc = "0x60 - DCP Control Register"]
    #[inline(always)]
    pub const fn dcpctr(&self) -> &Dcpctr {
        &self.dcpctr
    }
    #[doc = "0x64 - Pipe Window Select Register"]
    #[inline(always)]
    pub const fn pipesel(&self) -> &Pipesel {
        &self.pipesel
    }
    #[doc = "0x68 - Pipe Configuration Register"]
    #[inline(always)]
    pub const fn pipecfg(&self) -> &Pipecfg {
        &self.pipecfg
    }
    #[doc = "0x6c - Pipe Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn pipemaxp(&self) -> &Pipemaxp {
        &self.pipemaxp
    }
    #[doc = "0x6e - Pipe Cycle Control Register"]
    #[inline(always)]
    pub const fn pipeperi(&self) -> &Pipeperi {
        &self.pipeperi
    }
    #[doc = "0x70..0x7a - Pipe %s Control Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1CTR` register.</div>"]
    #[inline(always)]
    pub const fn pipectr(&self, n: usize) -> &Pipectr {
        &self.pipectr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x7a - Pipe %s Control Register"]
    #[inline(always)]
    pub fn pipectr_iter(&self) -> impl Iterator<Item = &Pipectr> {
        self.pipectr.iter()
    }
    #[doc = "0x70 - Pipe 1 Control Register"]
    #[inline(always)]
    pub const fn pipe1ctr(&self) -> &Pipectr {
        self.pipectr(0)
    }
    #[doc = "0x72 - Pipe 2 Control Register"]
    #[inline(always)]
    pub const fn pipe2ctr(&self) -> &Pipectr {
        self.pipectr(1)
    }
    #[doc = "0x74 - Pipe 3 Control Register"]
    #[inline(always)]
    pub const fn pipe3ctr(&self) -> &Pipectr {
        self.pipectr(2)
    }
    #[doc = "0x76 - Pipe 4 Control Register"]
    #[inline(always)]
    pub const fn pipe4ctr(&self) -> &Pipectr {
        self.pipectr(3)
    }
    #[doc = "0x78 - Pipe 5 Control Register"]
    #[inline(always)]
    pub const fn pipe5ctr(&self) -> &Pipectr {
        self.pipectr(4)
    }
    #[doc = "0x7a - Pipe 6 Control Register"]
    #[inline(always)]
    pub const fn pipe6ctr(&self) -> &Pipe6ctr {
        &self.pipe6ctr
    }
    #[doc = "0x7a - Pipe 7 Control Register"]
    #[inline(always)]
    pub const fn pipe7ctr(&self) -> &Pipe6ctr {
        &self.pipe7ctr
    }
    #[doc = "0x7a - Pipe 8 Control Register"]
    #[inline(always)]
    pub const fn pipe8ctr(&self) -> &Pipe6ctr {
        &self.pipe8ctr
    }
    #[doc = "0x7a - Pipe 9 Control Register"]
    #[inline(always)]
    pub const fn pipe9ctr(&self) -> &Pipe6ctr {
        &self.pipe9ctr
    }
    #[doc = "0x90..0x9a - Pipe %s Transaction Counter Enable Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRE` register.</div>"]
    #[inline(always)]
    pub const fn pipetre(&self, n: usize) -> &Pipetre {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0x9a - Pipe %s Transaction Counter Enable Register"]
    #[inline(always)]
    pub fn pipetre_iter(&self) -> impl Iterator<Item = &Pipetre> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x90 - Pipe 1 Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipe1tre(&self) -> &Pipetre {
        self.pipetre(0)
    }
    #[doc = "0x94 - Pipe 2 Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipe2tre(&self) -> &Pipetre {
        self.pipetre(1)
    }
    #[doc = "0x98 - Pipe 3 Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipe3tre(&self) -> &Pipetre {
        self.pipetre(2)
    }
    #[doc = "0x9c - Pipe 4 Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipe4tre(&self) -> &Pipetre {
        self.pipetre(3)
    }
    #[doc = "0xa0 - Pipe 5 Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipe5tre(&self) -> &Pipetre {
        self.pipetre(4)
    }
    #[doc = "0x92..0x9c - Pipe %s Transaction Counter Register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRN` register.</div>"]
    #[inline(always)]
    pub const fn pipetrn(&self, n: usize) -> &Pipetrn {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(146)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x92..0x9c - Pipe %s Transaction Counter Register"]
    #[inline(always)]
    pub fn pipetrn_iter(&self) -> impl Iterator<Item = &Pipetrn> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(146)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x92 - Pipe 1 Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipe1trn(&self) -> &Pipetrn {
        self.pipetrn(0)
    }
    #[doc = "0x96 - Pipe 2 Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipe2trn(&self) -> &Pipetrn {
        self.pipetrn(1)
    }
    #[doc = "0x9a - Pipe 3 Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipe3trn(&self) -> &Pipetrn {
        self.pipetrn(2)
    }
    #[doc = "0x9e - Pipe 4 Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipe4trn(&self) -> &Pipetrn {
        self.pipetrn(3)
    }
    #[doc = "0xa2 - Pipe 5 Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipe5trn(&self) -> &Pipetrn {
        self.pipetrn(4)
    }
    #[doc = "0xb0 - BC Control Register 0"]
    #[inline(always)]
    pub const fn usbbcctrl0(&self) -> &Usbbcctrl0 {
        &self.usbbcctrl0
    }
    #[doc = "0xcc - USB Module Control Register"]
    #[inline(always)]
    pub const fn usbmc(&self) -> &Usbmc {
        &self.usbmc
    }
    #[doc = "0xd0..0xdc - Device Address %s Configuration Register"]
    #[inline(always)]
    pub const fn devadd(&self, n: usize) -> &Devadd {
        &self.devadd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd0..0xdc - Device Address %s Configuration Register"]
    #[inline(always)]
    pub fn devadd_iter(&self) -> impl Iterator<Item = &Devadd> {
        self.devadd.iter()
    }
}
#[doc = "SYSCFG (rw) register accessor: System Configuration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg`] module"]
#[doc(alias = "SYSCFG")]
pub type Syscfg = crate::Reg<syscfg::SyscfgSpec>;
#[doc = "System Configuration Control Register"]
pub mod syscfg;
#[doc = "SYSSTS0 (r) register accessor: System Configuration Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syssts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syssts0`] module"]
#[doc(alias = "SYSSTS0")]
pub type Syssts0 = crate::Reg<syssts0::Syssts0Spec>;
#[doc = "System Configuration Status Register 0"]
pub mod syssts0;
#[doc = "DVSTCTR0 (rw) register accessor: Device State Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvstctr0`] module"]
#[doc(alias = "DVSTCTR0")]
pub type Dvstctr0 = crate::Reg<dvstctr0::Dvstctr0Spec>;
#[doc = "Device State Control Register 0"]
pub mod dvstctr0;
#[doc = "CFIFO (rw) register accessor: CFIFO Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifo`] module"]
#[doc(alias = "CFIFO")]
pub type Cfifo = crate::Reg<cfifo::CfifoSpec>;
#[doc = "CFIFO Port Register"]
pub mod cfifo;
#[doc = "CFIFOL (rw) register accessor: CFIFO Port Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifol`] module"]
#[doc(alias = "CFIFOL")]
pub type Cfifol = crate::Reg<cfifol::CfifolSpec>;
#[doc = "CFIFO Port Register L"]
pub mod cfifol;
#[doc = "D0FIFO (rw) register accessor: D0FIFO Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifo`] module"]
#[doc(alias = "D0FIFO")]
pub type D0fifo = crate::Reg<d0fifo::D0fifoSpec>;
#[doc = "D0FIFO Port Register"]
pub mod d0fifo;
#[doc = "D0FIFOL (rw) register accessor: D0FIFO Port Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`d0fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifol`] module"]
#[doc(alias = "D0FIFOL")]
pub type D0fifol = crate::Reg<d0fifol::D0fifolSpec>;
#[doc = "D0FIFO Port Register L"]
pub mod d0fifol;
#[doc = "D1FIFO (rw) register accessor: D1FIFO Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifo`] module"]
#[doc(alias = "D1FIFO")]
pub type D1fifo = crate::Reg<d1fifo::D1fifoSpec>;
#[doc = "D1FIFO Port Register"]
pub mod d1fifo;
#[doc = "D1FIFOL (rw) register accessor: D1FIFO Port Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`d1fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifol`] module"]
#[doc(alias = "D1FIFOL")]
pub type D1fifol = crate::Reg<d1fifol::D1fifolSpec>;
#[doc = "D1FIFO Port Register L"]
pub mod d1fifol;
#[doc = "CFIFOSEL (rw) register accessor: CFIFO Port Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifosel`] module"]
#[doc(alias = "CFIFOSEL")]
pub type Cfifosel = crate::Reg<cfifosel::CfifoselSpec>;
#[doc = "CFIFO Port Select Register"]
pub mod cfifosel;
#[doc = "CFIFOCTR (rw) register accessor: CFIFO Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfifoctr`] module"]
#[doc(alias = "CFIFOCTR")]
pub type Cfifoctr = crate::Reg<cfifoctr::CfifoctrSpec>;
#[doc = "CFIFO Port Control Register"]
pub mod cfifoctr;
#[doc = "D0FIFOSEL (rw) register accessor: D0FIFO Port Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifosel`] module"]
#[doc(alias = "D0FIFOSEL")]
pub type D0fifosel = crate::Reg<d0fifosel::D0fifoselSpec>;
#[doc = "D0FIFO Port Select Register"]
pub mod d0fifosel;
#[doc = "D0FIFOCTR (rw) register accessor: D0FIFO Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d0fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d0fifoctr`] module"]
#[doc(alias = "D0FIFOCTR")]
pub type D0fifoctr = crate::Reg<d0fifoctr::D0fifoctrSpec>;
#[doc = "D0FIFO Port Control Register"]
pub mod d0fifoctr;
#[doc = "D1FIFOSEL (rw) register accessor: D1FIFO Port Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifosel`] module"]
#[doc(alias = "D1FIFOSEL")]
pub type D1fifosel = crate::Reg<d1fifosel::D1fifoselSpec>;
#[doc = "D1FIFO Port Select Register"]
pub mod d1fifosel;
#[doc = "D1FIFOCTR (rw) register accessor: D1FIFO Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1fifoctr`] module"]
#[doc(alias = "D1FIFOCTR")]
pub type D1fifoctr = crate::Reg<d1fifoctr::D1fifoctrSpec>;
#[doc = "D1FIFO Port Control Register"]
pub mod d1fifoctr;
#[doc = "INTENB0 (rw) register accessor: Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intenb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenb0`] module"]
#[doc(alias = "INTENB0")]
pub type Intenb0 = crate::Reg<intenb0::Intenb0Spec>;
#[doc = "Interrupt Enable Register 0"]
pub mod intenb0;
#[doc = "INTENB1 (rw) register accessor: Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intenb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenb1`] module"]
#[doc(alias = "INTENB1")]
pub type Intenb1 = crate::Reg<intenb1::Intenb1Spec>;
#[doc = "Interrupt Enable Register 1"]
pub mod intenb1;
#[doc = "BRDYENB (rw) register accessor: BRDY Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdyenb`] module"]
#[doc(alias = "BRDYENB")]
pub type Brdyenb = crate::Reg<brdyenb::BrdyenbSpec>;
#[doc = "BRDY Interrupt Enable Register"]
pub mod brdyenb;
#[doc = "NRDYENB (rw) register accessor: NRDY Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrdyenb`] module"]
#[doc(alias = "NRDYENB")]
pub type Nrdyenb = crate::Reg<nrdyenb::NrdyenbSpec>;
#[doc = "NRDY Interrupt Enable Register"]
pub mod nrdyenb;
#[doc = "BEMPENB (rw) register accessor: BEMP Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bempenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bempenb`] module"]
#[doc(alias = "BEMPENB")]
pub type Bempenb = crate::Reg<bempenb::BempenbSpec>;
#[doc = "BEMP Interrupt Enable Register"]
pub mod bempenb;
#[doc = "SOFCFG (rw) register accessor: SOF Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sofcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sofcfg`] module"]
#[doc(alias = "SOFCFG")]
pub type Sofcfg = crate::Reg<sofcfg::SofcfgSpec>;
#[doc = "SOF Output Configuration Register"]
pub mod sofcfg;
#[doc = "INTSTS0 (rw) register accessor: Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts0`] module"]
#[doc(alias = "INTSTS0")]
pub type Intsts0 = crate::Reg<intsts0::Intsts0Spec>;
#[doc = "Interrupt Status Register 0"]
pub mod intsts0;
#[doc = "INTSTS1 (rw) register accessor: Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intsts1`] module"]
#[doc(alias = "INTSTS1")]
pub type Intsts1 = crate::Reg<intsts1::Intsts1Spec>;
#[doc = "Interrupt Status Register 1"]
pub mod intsts1;
#[doc = "BRDYSTS (rw) register accessor: BRDY Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brdysts`] module"]
#[doc(alias = "BRDYSTS")]
pub type Brdysts = crate::Reg<brdysts::BrdystsSpec>;
#[doc = "BRDY Interrupt Status Register"]
pub mod brdysts;
#[doc = "NRDYSTS (rw) register accessor: NRDY Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nrdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrdysts`] module"]
#[doc(alias = "NRDYSTS")]
pub type Nrdysts = crate::Reg<nrdysts::NrdystsSpec>;
#[doc = "NRDY Interrupt Status Register"]
pub mod nrdysts;
#[doc = "BEMPSTS (rw) register accessor: BEMP Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bempsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bempsts`] module"]
#[doc(alias = "BEMPSTS")]
pub type Bempsts = crate::Reg<bempsts::BempstsSpec>;
#[doc = "BEMP Interrupt Status Register"]
pub mod bempsts;
#[doc = "FRMNUM (rw) register accessor: Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frmnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frmnum`] module"]
#[doc(alias = "FRMNUM")]
pub type Frmnum = crate::Reg<frmnum::FrmnumSpec>;
#[doc = "Frame Number Register"]
pub mod frmnum;
#[doc = "USBREQ (rw) register accessor: USB Request Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbreq`] module"]
#[doc(alias = "USBREQ")]
pub type Usbreq = crate::Reg<usbreq::UsbreqSpec>;
#[doc = "USB Request Type Register"]
pub mod usbreq;
#[doc = "USBVAL (rw) register accessor: USB Request Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbval`] module"]
#[doc(alias = "USBVAL")]
pub type Usbval = crate::Reg<usbval::UsbvalSpec>;
#[doc = "USB Request Value Register"]
pub mod usbval;
#[doc = "USBINDX (rw) register accessor: USB Request Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbindx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbindx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbindx`] module"]
#[doc(alias = "USBINDX")]
pub type Usbindx = crate::Reg<usbindx::UsbindxSpec>;
#[doc = "USB Request Index Register"]
pub mod usbindx;
#[doc = "USBLENG (rw) register accessor: USB Request Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbleng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbleng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbleng`] module"]
#[doc(alias = "USBLENG")]
pub type Usbleng = crate::Reg<usbleng::UsblengSpec>;
#[doc = "USB Request Length Register"]
pub mod usbleng;
#[doc = "DCPCFG (rw) register accessor: DCP Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpcfg`] module"]
#[doc(alias = "DCPCFG")]
pub type Dcpcfg = crate::Reg<dcpcfg::DcpcfgSpec>;
#[doc = "DCP Configuration Register"]
pub mod dcpcfg;
#[doc = "DCPMAXP (rw) register accessor: DCP Maximum Packet Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpmaxp`] module"]
#[doc(alias = "DCPMAXP")]
pub type Dcpmaxp = crate::Reg<dcpmaxp::DcpmaxpSpec>;
#[doc = "DCP Maximum Packet Size Register"]
pub mod dcpmaxp;
#[doc = "DCPCTR (rw) register accessor: DCP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcpctr`] module"]
#[doc(alias = "DCPCTR")]
pub type Dcpctr = crate::Reg<dcpctr::DcpctrSpec>;
#[doc = "DCP Control Register"]
pub mod dcpctr;
#[doc = "PIPESEL (rw) register accessor: Pipe Window Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipesel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipesel`] module"]
#[doc(alias = "PIPESEL")]
pub type Pipesel = crate::Reg<pipesel::PipeselSpec>;
#[doc = "Pipe Window Select Register"]
pub mod pipesel;
#[doc = "PIPECFG (rw) register accessor: Pipe Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipecfg`] module"]
#[doc(alias = "PIPECFG")]
pub type Pipecfg = crate::Reg<pipecfg::PipecfgSpec>;
#[doc = "Pipe Configuration Register"]
pub mod pipecfg;
#[doc = "PIPEMAXP (rw) register accessor: Pipe Maximum Packet Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipemaxp`] module"]
#[doc(alias = "PIPEMAXP")]
pub type Pipemaxp = crate::Reg<pipemaxp::PipemaxpSpec>;
#[doc = "Pipe Maximum Packet Size Register"]
pub mod pipemaxp;
#[doc = "PIPEPERI (rw) register accessor: Pipe Cycle Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipeperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipeperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipeperi`] module"]
#[doc(alias = "PIPEPERI")]
pub type Pipeperi = crate::Reg<pipeperi::PipeperiSpec>;
#[doc = "Pipe Cycle Control Register"]
pub mod pipeperi;
#[doc = "PIPECTR (rw) register accessor: Pipe %s Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipectr`] module"]
#[doc(alias = "PIPECTR")]
pub type Pipectr = crate::Reg<pipectr::PipectrSpec>;
#[doc = "Pipe %s Control Register"]
pub mod pipectr;
pub use Pipectr as Pipe6ctr;
pub use pipectr as pipe6ctr;
#[doc = "PIPETRE (rw) register accessor: Pipe %s Transaction Counter Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipetre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipetre`] module"]
#[doc(alias = "PIPETRE")]
pub type Pipetre = crate::Reg<pipetre::PipetreSpec>;
#[doc = "Pipe %s Transaction Counter Enable Register"]
pub mod pipetre;
#[doc = "PIPETRN (rw) register accessor: Pipe %s Transaction Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pipetrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pipetrn`] module"]
#[doc(alias = "PIPETRN")]
pub type Pipetrn = crate::Reg<pipetrn::PipetrnSpec>;
#[doc = "Pipe %s Transaction Counter Register"]
pub mod pipetrn;
#[doc = "DEVADD (rw) register accessor: Device Address %s Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devadd`] module"]
#[doc(alias = "DEVADD")]
pub type Devadd = crate::Reg<devadd::DevaddSpec>;
#[doc = "Device Address %s Configuration Register"]
pub mod devadd;
#[doc = "USBMC (rw) register accessor: USB Module Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbmc`] module"]
#[doc(alias = "USBMC")]
pub type Usbmc = crate::Reg<usbmc::UsbmcSpec>;
#[doc = "USB Module Control Register"]
pub mod usbmc;
#[doc = "USBBCCTRL0 (rw) register accessor: BC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usbbcctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbbcctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbbcctrl0`] module"]
#[doc(alias = "USBBCCTRL0")]
pub type Usbbcctrl0 = crate::Reg<usbbcctrl0::Usbbcctrl0Spec>;
#[doc = "BC Control Register 0"]
pub mod usbbcctrl0;
