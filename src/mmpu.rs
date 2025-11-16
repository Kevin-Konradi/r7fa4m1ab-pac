#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmpuctla: Mmpuctla,
    _reserved1: [u8; 0x0100],
    mmpupta: Mmpupta,
    _reserved2: [u8; 0xfc],
    mmpuaca: (),
    _reserved3: [u8; 0x04],
    mmpusa: (),
    _reserved4: [u8; 0x04],
    mmpuea: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Bus Master MPU Control Register A"]
    #[inline(always)]
    pub const fn mmpuctla(&self) -> &Mmpuctla {
        &self.mmpuctla
    }
    #[doc = "0x102 - Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(&self) -> &Mmpupta {
        &self.mmpupta
    }
    #[doc = "0x200..0x220 - Group A Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuaca(&self, n: usize) -> &Mmpuaca {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x220 - Group A Region %s Access Control Register"]
    #[inline(always)]
    pub fn mmpuaca_iter(&self) -> impl Iterator<Item = &Mmpuaca> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x204..0x244 - Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(&self, n: usize) -> &Mmpusa {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x204..0x244 - Group A Region %s Start Address Register"]
    #[inline(always)]
    pub fn mmpusa_iter(&self) -> impl Iterator<Item = &Mmpusa> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x208..0x248 - Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(&self, n: usize) -> &Mmpuea {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x208..0x248 - Group A Region %s End Address Register"]
    #[inline(always)]
    pub fn mmpuea_iter(&self) -> impl Iterator<Item = &Mmpuea> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "MMPUCTLA (rw) register accessor: Bus Master MPU Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuctla`] module"]
#[doc(alias = "MMPUCTLA")]
pub type Mmpuctla = crate::Reg<mmpuctla::MmpuctlaSpec>;
#[doc = "Bus Master MPU Control Register A"]
pub mod mmpuctla;
#[doc = "MMPUACA (rw) register accessor: Group A Region %s Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuaca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuaca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuaca`] module"]
#[doc(alias = "MMPUACA")]
pub type Mmpuaca = crate::Reg<mmpuaca::MmpuacaSpec>;
#[doc = "Group A Region %s Access Control Register"]
pub mod mmpuaca;
#[doc = "MMPUSA (rw) register accessor: Group A Region %s Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpusa`] module"]
#[doc(alias = "MMPUSA")]
pub type Mmpusa = crate::Reg<mmpusa::MmpusaSpec>;
#[doc = "Group A Region %s Start Address Register"]
pub mod mmpusa;
#[doc = "MMPUEA (rw) register accessor: Group A Region %s End Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpuea`] module"]
#[doc(alias = "MMPUEA")]
pub type Mmpuea = crate::Reg<mmpuea::MmpueaSpec>;
#[doc = "Group A Region %s End Address Register"]
pub mod mmpuea;
#[doc = "MMPUPTA (rw) register accessor: Group A Protection of Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmpupta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpupta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmpupta`] module"]
#[doc(alias = "MMPUPTA")]
pub type Mmpupta = crate::Reg<mmpupta::MmpuptaSpec>;
#[doc = "Group A Protection of Register"]
pub mod mmpupta;
