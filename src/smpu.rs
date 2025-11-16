#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    smpuctl: Smpuctl,
    _reserved1: [u8; 0x0e],
    smpumbiu: Smpumbiu,
    _reserved2: [u8; 0x02],
    smpufbiu: Smpufbiu,
    _reserved3: [u8; 0x02],
    smpusram0: Smpusram0,
    _reserved4: [u8; 0x06],
    smpupbiu: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Slave MPU Control Register"]
    #[inline(always)]
    pub const fn smpuctl(&self) -> &Smpuctl {
        &self.smpuctl
    }
    #[doc = "0x10 - Access Control Register for MBIU"]
    #[inline(always)]
    pub const fn smpumbiu(&self) -> &Smpumbiu {
        &self.smpumbiu
    }
    #[doc = "0x14 - Access Control Register for FBIU"]
    #[inline(always)]
    pub const fn smpufbiu(&self) -> &Smpufbiu {
        &self.smpufbiu
    }
    #[doc = "0x18 - Access Control Register for SRAM0"]
    #[inline(always)]
    pub const fn smpusram0(&self) -> &Smpusram0 {
        &self.smpusram0
    }
    #[doc = "0x20..0x26 - Access Control Register for P%sBIU"]
    #[inline(always)]
    pub const fn smpupbiu(&self, n: usize) -> &Smpupbiu {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x26 - Access Control Register for P%sBIU"]
    #[inline(always)]
    pub fn smpupbiu_iter(&self) -> impl Iterator<Item = &Smpupbiu> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x20 - Access Control Register for P0BIU"]
    #[inline(always)]
    pub const fn smpup0biu(&self) -> &Smpupbiu {
        self.smpupbiu(0)
    }
    #[doc = "0x24 - Access Control Register for P2BIU"]
    #[inline(always)]
    pub const fn smpup2biu(&self) -> &Smpupbiu {
        self.smpupbiu(1)
    }
    #[doc = "0x28 - Access Control Register for P6BIU"]
    #[inline(always)]
    pub const fn smpup6biu(&self) -> &Smpupbiu {
        self.smpupbiu(2)
    }
}
#[doc = "SMPUCTL (rw) register accessor: Slave MPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpuctl`] module"]
#[doc(alias = "SMPUCTL")]
pub type Smpuctl = crate::Reg<smpuctl::SmpuctlSpec>;
#[doc = "Slave MPU Control Register"]
pub mod smpuctl;
#[doc = "SMPUMBIU (rw) register accessor: Access Control Register for MBIU\n\nYou can [`read`](crate::Reg::read) this register and get [`smpumbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpumbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpumbiu`] module"]
#[doc(alias = "SMPUMBIU")]
pub type Smpumbiu = crate::Reg<smpumbiu::SmpumbiuSpec>;
#[doc = "Access Control Register for MBIU"]
pub mod smpumbiu;
#[doc = "SMPUFBIU (rw) register accessor: Access Control Register for FBIU\n\nYou can [`read`](crate::Reg::read) this register and get [`smpufbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpufbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpufbiu`] module"]
#[doc(alias = "SMPUFBIU")]
pub type Smpufbiu = crate::Reg<smpufbiu::SmpufbiuSpec>;
#[doc = "Access Control Register for FBIU"]
pub mod smpufbiu;
#[doc = "SMPUSRAM0 (rw) register accessor: Access Control Register for SRAM0\n\nYou can [`read`](crate::Reg::read) this register and get [`smpusram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpusram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpusram0`] module"]
#[doc(alias = "SMPUSRAM0")]
pub type Smpusram0 = crate::Reg<smpusram0::Smpusram0Spec>;
#[doc = "Access Control Register for SRAM0"]
pub mod smpusram0;
#[doc = "SMPUPBIU (rw) register accessor: Access Control Register for P%sBIU\n\nYou can [`read`](crate::Reg::read) this register and get [`smpupbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpupbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpupbiu`] module"]
#[doc(alias = "SMPUPBIU")]
pub type Smpupbiu = crate::Reg<smpupbiu::SmpupbiuSpec>;
#[doc = "Access Control Register for P%sBIU"]
pub mod smpupbiu;
