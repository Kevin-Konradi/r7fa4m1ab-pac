#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crccr0: Crccr0,
    crccr1: Crccr1,
    _reserved2: [u8; 0x02],
    _reserved_2_crcdir: [u8; 0x04],
    _reserved_3_crcdor: [u8; 0x04],
    crcsar: Crcsar,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Control Register0"]
    #[inline(always)]
    pub const fn crccr0(&self) -> &Crccr0 {
        &self.crccr0
    }
    #[doc = "0x01 - CRC Control Register1"]
    #[inline(always)]
    pub const fn crccr1(&self) -> &Crccr1 {
        &self.crccr1
    }
    #[doc = "0x04 - CRC Data Input Register (byte access)"]
    #[inline(always)]
    pub const fn crcdir_by(&self) -> &CrcdirBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - CRC Data Input Register"]
    #[inline(always)]
    pub const fn crcdir(&self) -> &Crcdir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register(byte access)"]
    #[inline(always)]
    pub const fn crcdor_by(&self) -> &CrcdorBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register (halfword access)"]
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> &CrcdorHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - CRC Data Output Register"]
    #[inline(always)]
    pub const fn crcdor(&self) -> &Crcdor {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Snoop Address Register"]
    #[inline(always)]
    pub const fn crcsar(&self) -> &Crcsar {
        &self.crcsar
    }
}
#[doc = "CRCCR0 (rw) register accessor: CRC Control Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr0`] module"]
#[doc(alias = "CRCCR0")]
pub type Crccr0 = crate::Reg<crccr0::Crccr0Spec>;
#[doc = "CRC Control Register0"]
pub mod crccr0;
#[doc = "CRCCR1 (rw) register accessor: CRC Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr1`] module"]
#[doc(alias = "CRCCR1")]
pub type Crccr1 = crate::Reg<crccr1::Crccr1Spec>;
#[doc = "CRC Control Register1"]
pub mod crccr1;
#[doc = "CRCDIR (rw) register accessor: CRC Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdir`] module"]
#[doc(alias = "CRCDIR")]
pub type Crcdir = crate::Reg<crcdir::CrcdirSpec>;
#[doc = "CRC Data Input Register"]
pub mod crcdir;
#[doc = "CRCDIR_BY (rw) register accessor: CRC Data Input Register (byte access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdir_by`] module"]
#[doc(alias = "CRCDIR_BY")]
pub type CrcdirBy = crate::Reg<crcdir_by::CrcdirBySpec>;
#[doc = "CRC Data Input Register (byte access)"]
pub mod crcdir_by;
#[doc = "CRCDOR (rw) register accessor: CRC Data Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor`] module"]
#[doc(alias = "CRCDOR")]
pub type Crcdor = crate::Reg<crcdor::CrcdorSpec>;
#[doc = "CRC Data Output Register"]
pub mod crcdor;
#[doc = "CRCDOR_HA (rw) register accessor: CRC Data Output Register (halfword access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor_ha`] module"]
#[doc(alias = "CRCDOR_HA")]
pub type CrcdorHa = crate::Reg<crcdor_ha::CrcdorHaSpec>;
#[doc = "CRC Data Output Register (halfword access)"]
pub mod crcdor_ha;
#[doc = "CRCDOR_BY (rw) register accessor: CRC Data Output Register(byte access)\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdor_by`] module"]
#[doc(alias = "CRCDOR_BY")]
pub type CrcdorBy = crate::Reg<crcdor_by::CrcdorBySpec>;
#[doc = "CRC Data Output Register(byte access)"]
pub mod crcdor_by;
#[doc = "CRCSAR (rw) register accessor: Snoop Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsar`] module"]
#[doc(alias = "CRCSAR")]
pub type Crcsar = crate::Reg<crcsar::CrcsarSpec>;
#[doc = "Snoop Address Register"]
pub mod crcsar;
