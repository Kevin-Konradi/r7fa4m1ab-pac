#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    busmcnt: (),
    _reserved1: [u8; 0x0100],
    busscntfli: Busscntfli,
    _reserved2: [u8; 0x06],
    busscnt: (),
    _reserved3: [u8; 0x0c],
    busscntp0b: Busscntp0b,
    _reserved4: [u8; 0x02],
    busscntp2b: Busscntp0b,
    _reserved5: [u8; 0x02],
    busscntp3b: Busscntp0b,
    _reserved6: [u8; 0x02],
    busscntp4b: Busscntp0b,
    _reserved7: [u8; 0x06],
    busscntp6b: Busscntp6b,
    _reserved8: [u8; 0x06],
    busscntfbu: Busscntfbu,
    _reserved9: [u8; 0x06ce],
    buserradd: (),
    _reserved10: [u8; 0x04],
    buserrstat: (),
}
impl RegisterBlock {
    #[doc = "0x1000..0x1008 - Master Bus Control Register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `BUSMCNTM4I` register.</div>"]
    #[inline(always)]
    pub const fn busmcnt(&self, n: usize) -> &Busmcnt {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1008 - Master Bus Control Register %s"]
    #[inline(always)]
    pub fn busmcnt_iter(&self) -> impl Iterator<Item = &Busmcnt> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1000 - Master Bus Control Register M4I"]
    #[inline(always)]
    pub const fn busmcntm4i(&self) -> &Busmcnt {
        self.busmcnt(0)
    }
    #[doc = "0x1004 - Master Bus Control Register M4D"]
    #[inline(always)]
    pub const fn busmcntm4d(&self) -> &Busmcnt {
        self.busmcnt(1)
    }
    #[doc = "0x1008 - Master Bus Control Register SYS"]
    #[inline(always)]
    pub const fn busmcntsys(&self) -> &Busmcnt {
        self.busmcnt(2)
    }
    #[doc = "0x100c - Master Bus Control Register DMA"]
    #[inline(always)]
    pub const fn busmcntdma(&self) -> &Busmcnt {
        self.busmcnt(3)
    }
    #[doc = "0x1100 - Slave Bus Control Register FLI"]
    #[inline(always)]
    pub const fn busscntfli(&self) -> &Busscntfli {
        &self.busscntfli
    }
    #[doc = "0x1108 - Slave Bus Control Register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `BUSSCNTMBIU` register.</div>"]
    #[inline(always)]
    pub const fn busscnt(&self, n: usize) -> &Busscnt {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1108 - Slave Bus Control Register %s"]
    #[inline(always)]
    pub fn busscnt_iter(&self) -> impl Iterator<Item = &Busscnt> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x1108 - Slave Bus Control Register MBIU"]
    #[inline(always)]
    pub const fn busscntmbiu(&self) -> &Busscnt {
        self.busscnt(0)
    }
    #[doc = "0x110c - Slave Bus Control Register RAM0"]
    #[inline(always)]
    pub const fn busscntram0(&self) -> &Busscnt {
        self.busscnt(1)
    }
    #[doc = "0x1114 - Slave Bus Control Register P0B"]
    #[inline(always)]
    pub const fn busscntp0b(&self) -> &Busscntp0b {
        &self.busscntp0b
    }
    #[doc = "0x1114 - Slave Bus Control Register P2B"]
    #[inline(always)]
    pub const fn busscntp2b(&self) -> &Busscntp0b {
        &self.busscntp2b
    }
    #[doc = "0x1114 - Slave Bus Control Register P3B"]
    #[inline(always)]
    pub const fn busscntp3b(&self) -> &Busscntp0b {
        &self.busscntp3b
    }
    #[doc = "0x1114 - Slave Bus Control Register P4B"]
    #[inline(always)]
    pub const fn busscntp4b(&self) -> &Busscntp0b {
        &self.busscntp4b
    }
    #[doc = "0x1128 - Slave Bus Control Register P6B"]
    #[inline(always)]
    pub const fn busscntp6b(&self) -> &Busscntp6b {
        &self.busscntp6b
    }
    #[doc = "0x1130 - Slave Bus Control Register FBU"]
    #[inline(always)]
    pub const fn busscntfbu(&self) -> &Busscntfbu {
        &self.busscntfbu
    }
    #[doc = "0x1800..0x1810 - Bus Error Address Register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRADD` register.</div>"]
    #[inline(always)]
    pub const fn buserradd(&self, n: usize) -> &Buserradd {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6144)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x1810 - Bus Error Address Register %s"]
    #[inline(always)]
    pub fn buserradd_iter(&self) -> impl Iterator<Item = &Buserradd> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6144)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x1800 - Bus Error Address Register 1"]
    #[inline(always)]
    pub const fn bus1erradd(&self) -> &Buserradd {
        self.buserradd(0)
    }
    #[doc = "0x1810 - Bus Error Address Register 2"]
    #[inline(always)]
    pub const fn bus2erradd(&self) -> &Buserradd {
        self.buserradd(1)
    }
    #[doc = "0x1820 - Bus Error Address Register 3"]
    #[inline(always)]
    pub const fn bus3erradd(&self) -> &Buserradd {
        self.buserradd(2)
    }
    #[doc = "0x1830 - Bus Error Address Register 4"]
    #[inline(always)]
    pub const fn bus4erradd(&self) -> &Buserradd {
        self.buserradd(3)
    }
    #[doc = "0x1804 - Bus Error Status Register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRSTAT` register.</div>"]
    #[inline(always)]
    pub const fn buserrstat(&self, n: usize) -> &Buserrstat {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6148)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1804 - Bus Error Status Register %s"]
    #[inline(always)]
    pub fn buserrstat_iter(&self) -> impl Iterator<Item = &Buserrstat> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6148)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x1804 - Bus Error Status Register 1"]
    #[inline(always)]
    pub const fn bus1errstat(&self) -> &Buserrstat {
        self.buserrstat(0)
    }
    #[doc = "0x1814 - Bus Error Status Register 2"]
    #[inline(always)]
    pub const fn bus2errstat(&self) -> &Buserrstat {
        self.buserrstat(1)
    }
    #[doc = "0x1824 - Bus Error Status Register 3"]
    #[inline(always)]
    pub const fn bus3errstat(&self) -> &Buserrstat {
        self.buserrstat(2)
    }
    #[doc = "0x1834 - Bus Error Status Register 4"]
    #[inline(always)]
    pub const fn bus4errstat(&self) -> &Buserrstat {
        self.buserrstat(3)
    }
}
#[doc = "BUSMCNT (rw) register accessor: Master Bus Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busmcnt`] module"]
#[doc(alias = "BUSMCNT")]
pub type Busmcnt = crate::Reg<busmcnt::BusmcntSpec>;
#[doc = "Master Bus Control Register %s"]
pub mod busmcnt;
#[doc = "BUSSCNTFLI (rw) register accessor: Slave Bus Control Register FLI\n\nYou can [`read`](crate::Reg::read) this register and get [`busscntfli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntfli`] module"]
#[doc(alias = "BUSSCNTFLI")]
pub type Busscntfli = crate::Reg<busscntfli::BusscntfliSpec>;
#[doc = "Slave Bus Control Register FLI"]
pub mod busscntfli;
#[doc = "BUSSCNT (rw) register accessor: Slave Bus Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`busscnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscnt`] module"]
#[doc(alias = "BUSSCNT")]
pub type Busscnt = crate::Reg<busscnt::BusscntSpec>;
#[doc = "Slave Bus Control Register %s"]
pub mod busscnt;
pub use Busscnt as Busscntp0b;
pub use busscnt as busscntp0b;
#[doc = "BUSSCNTP6B (rw) register accessor: Slave Bus Control Register P6B\n\nYou can [`read`](crate::Reg::read) this register and get [`busscntp6b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntp6b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntp6b`] module"]
#[doc(alias = "BUSSCNTP6B")]
pub type Busscntp6b = crate::Reg<busscntp6b::Busscntp6bSpec>;
#[doc = "Slave Bus Control Register P6B"]
pub mod busscntp6b;
#[doc = "BUSSCNTFBU (rw) register accessor: Slave Bus Control Register FBU\n\nYou can [`read`](crate::Reg::read) this register and get [`busscntfbu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfbu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busscntfbu`] module"]
#[doc(alias = "BUSSCNTFBU")]
pub type Busscntfbu = crate::Reg<busscntfbu::BusscntfbuSpec>;
#[doc = "Slave Bus Control Register FBU"]
pub mod busscntfbu;
#[doc = "BUSERRADD (r) register accessor: Bus Error Address Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`buserradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserradd`] module"]
#[doc(alias = "BUSERRADD")]
pub type Buserradd = crate::Reg<buserradd::BuserraddSpec>;
#[doc = "Bus Error Address Register %s"]
pub mod buserradd;
#[doc = "BUSERRSTAT (r) register accessor: Bus Error Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`buserrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserrstat`] module"]
#[doc(alias = "BUSERRSTAT")]
pub type Buserrstat = crate::Reg<buserrstat::BuserrstatSpec>;
#[doc = "Bus Error Status Register %s"]
pub mod buserrstat;
