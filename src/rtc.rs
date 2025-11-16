#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r64cnt: R64cnt,
    _reserved1: [u8; 0x01],
    _reserved_1_bcnt0: [u8; 0x01],
    _reserved2: [u8; 0x01],
    _reserved_2_bcnt1: [u8; 0x01],
    _reserved3: [u8; 0x01],
    _reserved_3_bcnt2: [u8; 0x01],
    _reserved4: [u8; 0x01],
    _reserved_4_bcnt3: [u8; 0x01],
    _reserved5: [u8; 0x01],
    rdaycnt: Rdaycnt,
    _reserved6: [u8; 0x01],
    rmoncnt: Rmoncnt,
    _reserved7: [u8; 0x01],
    ryrcnt: Ryrcnt,
    _reserved_8_rsecar: [u8; 0x01],
    _reserved9: [u8; 0x01],
    _reserved_9_rminar: [u8; 0x01],
    _reserved10: [u8; 0x01],
    _reserved_10_rhrar: [u8; 0x01],
    _reserved11: [u8; 0x01],
    _reserved_11_rwkar: [u8; 0x01],
    _reserved12: [u8; 0x01],
    _reserved_12_rdayar: [u8; 0x01],
    _reserved13: [u8; 0x01],
    _reserved_13_rmonar: [u8; 0x01],
    _reserved14: [u8; 0x01],
    _reserved_14_ryrar: [u8; 0x02],
    _reserved_15_ryraren: [u8; 0x01],
    _reserved16: [u8; 0x03],
    rcr1: Rcr1,
    _reserved17: [u8; 0x01],
    rcr2: Rcr2,
    _reserved18: [u8; 0x03],
    rcr4: Rcr4,
    _reserved19: [u8; 0x01],
    rfrh: Rfrh,
    rfrl: Rfrl,
    radj: Radj,
    _reserved22: [u8; 0x11],
    rtccr: (),
    _reserved23: [u8; 0x12],
    bcnt0cp: (),
    rseccp: (),
    _reserved25: [u8; 0x02],
    bcnt1cp: (),
    rmincp: (),
    _reserved27: [u8; 0x02],
    bcnt2cp: (),
    rhrcp: (),
    _reserved29: [u8; 0x04],
    bcnt3cp: (),
    rdaycp: (),
    _reserved31: [u8; 0x02],
    rmoncp: (),
}
impl RegisterBlock {
    #[doc = "0x00 - 64-Hz Counter"]
    #[inline(always)]
    pub const fn r64cnt(&self) -> &R64cnt {
        &self.r64cnt
    }
    #[doc = "0x02 - Binary Counter 0"]
    #[inline(always)]
    pub const fn bcnt0(&self) -> &Bcnt0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Second Counter"]
    #[inline(always)]
    pub const fn rseccnt(&self) -> &Rseccnt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x04 - Binary Counter 1"]
    #[inline(always)]
    pub const fn bcnt1(&self) -> &Bcnt1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Minute Counter"]
    #[inline(always)]
    pub const fn rmincnt(&self) -> &Rmincnt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - Binary Counter 2"]
    #[inline(always)]
    pub const fn bcnt2(&self) -> &Bcnt2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - Hour Counter"]
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &Rhrcnt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - Binary Counter 3"]
    #[inline(always)]
    pub const fn bcnt3(&self) -> &Bcnt3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Day-of-Week Counter"]
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &Rwkcnt {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - Day Counter"]
    #[inline(always)]
    pub const fn rdaycnt(&self) -> &Rdaycnt {
        &self.rdaycnt
    }
    #[doc = "0x0c - Month Counter"]
    #[inline(always)]
    pub const fn rmoncnt(&self) -> &Rmoncnt {
        &self.rmoncnt
    }
    #[doc = "0x0e - Year Counter"]
    #[inline(always)]
    pub const fn ryrcnt(&self) -> &Ryrcnt {
        &self.ryrcnt
    }
    #[doc = "0x10 - Binary Counter 0 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> &Bcnt0ar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Second Alarm Register"]
    #[inline(always)]
    pub const fn rsecar(&self) -> &Rsecar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x12 - Binary Counter 1 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> &Bcnt1ar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - Minute Alarm Register"]
    #[inline(always)]
    pub const fn rminar(&self) -> &Rminar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x14 - Binary Counter 2 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> &Bcnt2ar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Hour Alarm Register"]
    #[inline(always)]
    pub const fn rhrar(&self) -> &Rhrar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x16 - Binary Counter 3 Alarm Register"]
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> &Bcnt3ar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x16 - Day-of-Week Alarm Register"]
    #[inline(always)]
    pub const fn rwkar(&self) -> &Rwkar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x18 - Binary Counter 0 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt0aer(&self) -> &Bcnt0aer {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Date Alarm Register"]
    #[inline(always)]
    pub const fn rdayar(&self) -> &Rdayar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1a - Binary Counter 1 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt1aer(&self) -> &Bcnt1aer {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - Month Alarm Register"]
    #[inline(always)]
    pub const fn rmonar(&self) -> &Rmonar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - Binary Counter 2 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt2aer(&self) -> &Bcnt2aer {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Year Alarm Register"]
    #[inline(always)]
    pub const fn ryrar(&self) -> &Ryrar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1e - Binary Counter 3 Alarm Enable Register"]
    #[inline(always)]
    pub const fn bcnt3aer(&self) -> &Bcnt3aer {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x1e - Year Alarm Enable Register"]
    #[inline(always)]
    pub const fn ryraren(&self) -> &Ryraren {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    #[doc = "0x22 - RTC Control Register 1"]
    #[inline(always)]
    pub const fn rcr1(&self) -> &Rcr1 {
        &self.rcr1
    }
    #[doc = "0x24 - RTC Control Register 2"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &Rcr2 {
        &self.rcr2
    }
    #[doc = "0x28 - RTC Control Register 4"]
    #[inline(always)]
    pub const fn rcr4(&self) -> &Rcr4 {
        &self.rcr4
    }
    #[doc = "0x2a - Frequency Register H"]
    #[inline(always)]
    pub const fn rfrh(&self) -> &Rfrh {
        &self.rfrh
    }
    #[doc = "0x2c - Frequency Register L"]
    #[inline(always)]
    pub const fn rfrl(&self) -> &Rfrl {
        &self.rfrl
    }
    #[doc = "0x2e - Time Error Adjustment Register"]
    #[inline(always)]
    pub const fn radj(&self) -> &Radj {
        &self.radj
    }
    #[doc = "0x40 - Time Capture Control Register %s"]
    #[inline(always)]
    pub const fn rtccr(&self, n: usize) -> &Rtccr {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(2 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40 - Time Capture Control Register %s"]
    #[inline(always)]
    pub fn rtccr_iter(&self) -> impl Iterator<Item = &Rtccr> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(2 * n)
                .cast()
        })
    }
    #[doc = "0x52 - BCNT0 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt0cp(&self, n: usize) -> &Bcnt0cp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52 - BCNT0 Capture Register %s"]
    #[inline(always)]
    pub fn bcnt0cp_iter(&self) -> impl Iterator<Item = &Bcnt0cp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x52 - Second Capture Register %s"]
    #[inline(always)]
    pub const fn rseccp(&self, n: usize) -> &Rseccp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52 - Second Capture Register %s"]
    #[inline(always)]
    pub fn rseccp_iter(&self) -> impl Iterator<Item = &Rseccp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x54 - BCNT1 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt1cp(&self, n: usize) -> &Bcnt1cp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54 - BCNT1 Capture Register %s"]
    #[inline(always)]
    pub fn bcnt1cp_iter(&self) -> impl Iterator<Item = &Bcnt1cp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x54 - Minute Capture Register %s"]
    #[inline(always)]
    pub const fn rmincp(&self, n: usize) -> &Rmincp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54 - Minute Capture Register %s"]
    #[inline(always)]
    pub fn rmincp_iter(&self) -> impl Iterator<Item = &Rmincp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x56 - BCNT2 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt2cp(&self, n: usize) -> &Bcnt2cp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x56 - BCNT2 Capture Register %s"]
    #[inline(always)]
    pub fn bcnt2cp_iter(&self) -> impl Iterator<Item = &Bcnt2cp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x56 - Hour Capture Register %s"]
    #[inline(always)]
    pub const fn rhrcp(&self, n: usize) -> &Rhrcp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x56 - Hour Capture Register %s"]
    #[inline(always)]
    pub fn rhrcp_iter(&self) -> impl Iterator<Item = &Rhrcp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x5a - BCNT3 Capture Register %s"]
    #[inline(always)]
    pub const fn bcnt3cp(&self, n: usize) -> &Bcnt3cp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5a - BCNT3 Capture Register %s"]
    #[inline(always)]
    pub fn bcnt3cp_iter(&self) -> impl Iterator<Item = &Bcnt3cp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x5a - Date Capture Register %s"]
    #[inline(always)]
    pub const fn rdaycp(&self, n: usize) -> &Rdaycp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5a - Date Capture Register %s"]
    #[inline(always)]
    pub fn rdaycp_iter(&self) -> impl Iterator<Item = &Rdaycp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x5c - Month Capture Register %s"]
    #[inline(always)]
    pub const fn rmoncp(&self, n: usize) -> &Rmoncp {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c - Month Capture Register %s"]
    #[inline(always)]
    pub fn rmoncp_iter(&self) -> impl Iterator<Item = &Rmoncp> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "R64CNT (r) register accessor: 64-Hz Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`r64cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r64cnt`] module"]
#[doc(alias = "R64CNT")]
pub type R64cnt = crate::Reg<r64cnt::R64cntSpec>;
#[doc = "64-Hz Counter"]
pub mod r64cnt;
#[doc = "RSECCNT (rw) register accessor: Second Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rseccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rseccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rseccnt`] module"]
#[doc(alias = "RSECCNT")]
pub type Rseccnt = crate::Reg<rseccnt::RseccntSpec>;
#[doc = "Second Counter"]
pub mod rseccnt;
#[doc = "BCNT0 (rw) register accessor: Binary Counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0`] module"]
#[doc(alias = "BCNT0")]
pub type Bcnt0 = crate::Reg<bcnt0::Bcnt0Spec>;
#[doc = "Binary Counter 0"]
pub mod bcnt0;
#[doc = "RMINCNT (rw) register accessor: Minute Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rmincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmincnt`] module"]
#[doc(alias = "RMINCNT")]
pub type Rmincnt = crate::Reg<rmincnt::RmincntSpec>;
#[doc = "Minute Counter"]
pub mod rmincnt;
#[doc = "BCNT1 (rw) register accessor: Binary Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1`] module"]
#[doc(alias = "BCNT1")]
pub type Bcnt1 = crate::Reg<bcnt1::Bcnt1Spec>;
#[doc = "Binary Counter 1"]
pub mod bcnt1;
#[doc = "RHRCNT (rw) register accessor: Hour Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrcnt`] module"]
#[doc(alias = "RHRCNT")]
pub type Rhrcnt = crate::Reg<rhrcnt::RhrcntSpec>;
#[doc = "Hour Counter"]
pub mod rhrcnt;
#[doc = "BCNT2 (rw) register accessor: Binary Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2`] module"]
#[doc(alias = "BCNT2")]
pub type Bcnt2 = crate::Reg<bcnt2::Bcnt2Spec>;
#[doc = "Binary Counter 2"]
pub mod bcnt2;
#[doc = "RWKCNT (rw) register accessor: Day-of-Week Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rwkcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwkcnt`] module"]
#[doc(alias = "RWKCNT")]
pub type Rwkcnt = crate::Reg<rwkcnt::RwkcntSpec>;
#[doc = "Day-of-Week Counter"]
pub mod rwkcnt;
#[doc = "BCNT3 (rw) register accessor: Binary Counter 3\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3`] module"]
#[doc(alias = "BCNT3")]
pub type Bcnt3 = crate::Reg<bcnt3::Bcnt3Spec>;
#[doc = "Binary Counter 3"]
pub mod bcnt3;
#[doc = "RDAYCNT (rw) register accessor: Day Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdaycnt`] module"]
#[doc(alias = "RDAYCNT")]
pub type Rdaycnt = crate::Reg<rdaycnt::RdaycntSpec>;
#[doc = "Day Counter"]
pub mod rdaycnt;
#[doc = "RMONCNT (rw) register accessor: Month Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmoncnt`] module"]
#[doc(alias = "RMONCNT")]
pub type Rmoncnt = crate::Reg<rmoncnt::RmoncntSpec>;
#[doc = "Month Counter"]
pub mod rmoncnt;
#[doc = "RYRCNT (rw) register accessor: Year Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ryrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryrcnt`] module"]
#[doc(alias = "RYRCNT")]
pub type Ryrcnt = crate::Reg<ryrcnt::RyrcntSpec>;
#[doc = "Year Counter"]
pub mod ryrcnt;
#[doc = "RSECAR (rw) register accessor: Second Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsecar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsecar`] module"]
#[doc(alias = "RSECAR")]
pub type Rsecar = crate::Reg<rsecar::RsecarSpec>;
#[doc = "Second Alarm Register"]
pub mod rsecar;
#[doc = "BCNT0AR (rw) register accessor: Binary Counter 0 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0ar`] module"]
#[doc(alias = "BCNT0AR")]
pub type Bcnt0ar = crate::Reg<bcnt0ar::Bcnt0arSpec>;
#[doc = "Binary Counter 0 Alarm Register"]
pub mod bcnt0ar;
#[doc = "RMINAR (rw) register accessor: Minute Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rminar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rminar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rminar`] module"]
#[doc(alias = "RMINAR")]
pub type Rminar = crate::Reg<rminar::RminarSpec>;
#[doc = "Minute Alarm Register"]
pub mod rminar;
#[doc = "BCNT1AR (rw) register accessor: Binary Counter 1 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1ar`] module"]
#[doc(alias = "BCNT1AR")]
pub type Bcnt1ar = crate::Reg<bcnt1ar::Bcnt1arSpec>;
#[doc = "Binary Counter 1 Alarm Register"]
pub mod bcnt1ar;
#[doc = "RHRAR (rw) register accessor: Hour Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrar`] module"]
#[doc(alias = "RHRAR")]
pub type Rhrar = crate::Reg<rhrar::RhrarSpec>;
#[doc = "Hour Alarm Register"]
pub mod rhrar;
#[doc = "BCNT2AR (rw) register accessor: Binary Counter 2 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2ar`] module"]
#[doc(alias = "BCNT2AR")]
pub type Bcnt2ar = crate::Reg<bcnt2ar::Bcnt2arSpec>;
#[doc = "Binary Counter 2 Alarm Register"]
pub mod bcnt2ar;
#[doc = "RWKAR (rw) register accessor: Day-of-Week Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rwkar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwkar`] module"]
#[doc(alias = "RWKAR")]
pub type Rwkar = crate::Reg<rwkar::RwkarSpec>;
#[doc = "Day-of-Week Alarm Register"]
pub mod rwkar;
#[doc = "BCNT3AR (rw) register accessor: Binary Counter 3 Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3ar`] module"]
#[doc(alias = "BCNT3AR")]
pub type Bcnt3ar = crate::Reg<bcnt3ar::Bcnt3arSpec>;
#[doc = "Binary Counter 3 Alarm Register"]
pub mod bcnt3ar;
#[doc = "RDAYAR (rw) register accessor: Date Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdayar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdayar`] module"]
#[doc(alias = "RDAYAR")]
pub type Rdayar = crate::Reg<rdayar::RdayarSpec>;
#[doc = "Date Alarm Register"]
pub mod rdayar;
#[doc = "BCNT0AER (rw) register accessor: Binary Counter 0 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0aer`] module"]
#[doc(alias = "BCNT0AER")]
pub type Bcnt0aer = crate::Reg<bcnt0aer::Bcnt0aerSpec>;
#[doc = "Binary Counter 0 Alarm Enable Register"]
pub mod bcnt0aer;
#[doc = "RMONAR (rw) register accessor: Month Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmonar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmonar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmonar`] module"]
#[doc(alias = "RMONAR")]
pub type Rmonar = crate::Reg<rmonar::RmonarSpec>;
#[doc = "Month Alarm Register"]
pub mod rmonar;
#[doc = "BCNT1AER (rw) register accessor: Binary Counter 1 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1aer`] module"]
#[doc(alias = "BCNT1AER")]
pub type Bcnt1aer = crate::Reg<bcnt1aer::Bcnt1aerSpec>;
#[doc = "Binary Counter 1 Alarm Enable Register"]
pub mod bcnt1aer;
#[doc = "RYRAR (rw) register accessor: Year Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ryrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryrar`] module"]
#[doc(alias = "RYRAR")]
pub type Ryrar = crate::Reg<ryrar::RyrarSpec>;
#[doc = "Year Alarm Register"]
pub mod ryrar;
#[doc = "BCNT2AER (rw) register accessor: Binary Counter 2 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2aer`] module"]
#[doc(alias = "BCNT2AER")]
pub type Bcnt2aer = crate::Reg<bcnt2aer::Bcnt2aerSpec>;
#[doc = "Binary Counter 2 Alarm Enable Register"]
pub mod bcnt2aer;
#[doc = "RYRAREN (rw) register accessor: Year Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ryraren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryraren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ryraren`] module"]
#[doc(alias = "RYRAREN")]
pub type Ryraren = crate::Reg<ryraren::RyrarenSpec>;
#[doc = "Year Alarm Enable Register"]
pub mod ryraren;
#[doc = "BCNT3AER (rw) register accessor: Binary Counter 3 Alarm Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3aer`] module"]
#[doc(alias = "BCNT3AER")]
pub type Bcnt3aer = crate::Reg<bcnt3aer::Bcnt3aerSpec>;
#[doc = "Binary Counter 3 Alarm Enable Register"]
pub mod bcnt3aer;
#[doc = "RCR1 (rw) register accessor: RTC Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr1`] module"]
#[doc(alias = "RCR1")]
pub type Rcr1 = crate::Reg<rcr1::Rcr1Spec>;
#[doc = "RTC Control Register 1"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: RTC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr2`] module"]
#[doc(alias = "RCR2")]
pub type Rcr2 = crate::Reg<rcr2::Rcr2Spec>;
#[doc = "RTC Control Register 2"]
pub mod rcr2;
#[doc = "RCR4 (rw) register accessor: RTC Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr4`] module"]
#[doc(alias = "RCR4")]
pub type Rcr4 = crate::Reg<rcr4::Rcr4Spec>;
#[doc = "RTC Control Register 4"]
pub mod rcr4;
#[doc = "RFRH (rw) register accessor: Frequency Register H\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrh`] module"]
#[doc(alias = "RFRH")]
pub type Rfrh = crate::Reg<rfrh::RfrhSpec>;
#[doc = "Frequency Register H"]
pub mod rfrh;
#[doc = "RFRL (rw) register accessor: Frequency Register L\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfrl`] module"]
#[doc(alias = "RFRL")]
pub type Rfrl = crate::Reg<rfrl::RfrlSpec>;
#[doc = "Frequency Register L"]
pub mod rfrl;
#[doc = "RADJ (rw) register accessor: Time Error Adjustment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`radj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radj`] module"]
#[doc(alias = "RADJ")]
pub type Radj = crate::Reg<radj::RadjSpec>;
#[doc = "Time Error Adjustment Register"]
pub mod radj;
#[doc = "RTCCR (rw) register accessor: Time Capture Control Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`] module"]
#[doc(alias = "RTCCR")]
pub type Rtccr = crate::Reg<rtccr::RtccrSpec>;
#[doc = "Time Capture Control Register %s"]
pub mod rtccr;
#[doc = "RSECCP (r) register accessor: Second Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rseccp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rseccp`] module"]
#[doc(alias = "RSECCP")]
pub type Rseccp = crate::Reg<rseccp::RseccpSpec>;
#[doc = "Second Capture Register %s"]
pub mod rseccp;
#[doc = "BCNT0CP (r) register accessor: BCNT0 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt0cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt0cp`] module"]
#[doc(alias = "BCNT0CP")]
pub type Bcnt0cp = crate::Reg<bcnt0cp::Bcnt0cpSpec>;
#[doc = "BCNT0 Capture Register %s"]
pub mod bcnt0cp;
#[doc = "RMINCP (r) register accessor: Minute Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rmincp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmincp`] module"]
#[doc(alias = "RMINCP")]
pub type Rmincp = crate::Reg<rmincp::RmincpSpec>;
#[doc = "Minute Capture Register %s"]
pub mod rmincp;
#[doc = "BCNT1CP (r) register accessor: BCNT1 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt1cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt1cp`] module"]
#[doc(alias = "BCNT1CP")]
pub type Bcnt1cp = crate::Reg<bcnt1cp::Bcnt1cpSpec>;
#[doc = "BCNT1 Capture Register %s"]
pub mod bcnt1cp;
#[doc = "RHRCP (r) register accessor: Hour Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rhrcp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhrcp`] module"]
#[doc(alias = "RHRCP")]
pub type Rhrcp = crate::Reg<rhrcp::RhrcpSpec>;
#[doc = "Hour Capture Register %s"]
pub mod rhrcp;
#[doc = "BCNT2CP (r) register accessor: BCNT2 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt2cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt2cp`] module"]
#[doc(alias = "BCNT2CP")]
pub type Bcnt2cp = crate::Reg<bcnt2cp::Bcnt2cpSpec>;
#[doc = "BCNT2 Capture Register %s"]
pub mod bcnt2cp;
#[doc = "RDAYCP (r) register accessor: Date Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rdaycp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdaycp`] module"]
#[doc(alias = "RDAYCP")]
pub type Rdaycp = crate::Reg<rdaycp::RdaycpSpec>;
#[doc = "Date Capture Register %s"]
pub mod rdaycp;
#[doc = "BCNT3CP (r) register accessor: BCNT3 Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt3cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt3cp`] module"]
#[doc(alias = "BCNT3CP")]
pub type Bcnt3cp = crate::Reg<bcnt3cp::Bcnt3cpSpec>;
#[doc = "BCNT3 Capture Register %s"]
pub mod bcnt3cp;
#[doc = "RMONCP (r) register accessor: Month Capture Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`rmoncp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmoncp`] module"]
#[doc(alias = "RMONCP")]
pub type Rmoncp = crate::Reg<rmoncp::RmoncpSpec>;
#[doc = "Month Capture Register %s"]
pub mod rmoncp;
