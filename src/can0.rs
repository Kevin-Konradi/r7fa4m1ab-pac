#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    mb_id: (),
    _reserved1: [u8; 0x04],
    mb_dl: (),
    _reserved2: [u8; 0x02],
    mb_d0: (),
    _reserved3: [u8; 0x01],
    mb_d1: (),
    _reserved4: [u8; 0x01],
    mb_d2: (),
    _reserved5: [u8; 0x01],
    mb_d3: (),
    _reserved6: [u8; 0x01],
    mb_d4: (),
    _reserved7: [u8; 0x01],
    mb_d5: (),
    _reserved8: [u8; 0x01],
    mb_d6: (),
    _reserved9: [u8; 0x01],
    mb_d7: (),
    _reserved10: [u8; 0x01],
    mb_ts: (),
    _reserved11: [u8; 0x01f2],
    mkr: [Mkr; 8],
    fidcr: [Fidcr; 2],
    mkivlr: Mkivlr,
    _reserved_14_mier: [u8; 0x04],
    _reserved15: [u8; 0x03f0],
    _reserved_15_mctl: [u8; 0x20],
    ctlr: Ctlr,
    str: Str,
    bcr: Bcr,
    rfcr: Rfcr,
    rfpcr: Rfpcr,
    tfcr: Tfcr,
    tfpcr: Tfpcr,
    eier: Eier,
    eifr: Eifr,
    recr: Recr,
    tecr: Tecr,
    ecsr: Ecsr,
    cssr: Cssr,
    mssr: Mssr,
    msmr: Msmr,
    tsr: Tsr,
    afsr: Afsr,
    tcr: Tcr,
}
impl RegisterBlock {
    #[doc = "0x200..0x280 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_id(&self, n: usize) -> &MbId {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x280 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_id_iter(&self) -> impl Iterator<Item = &MbId> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x200 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_id(&self) -> &MbId {
        self.mb_id(0)
    }
    #[doc = "0x210 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_id(&self) -> &MbId {
        self.mb_id(1)
    }
    #[doc = "0x220 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_id(&self) -> &MbId {
        self.mb_id(2)
    }
    #[doc = "0x230 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_id(&self) -> &MbId {
        self.mb_id(3)
    }
    #[doc = "0x240 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_id(&self) -> &MbId {
        self.mb_id(4)
    }
    #[doc = "0x250 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_id(&self) -> &MbId {
        self.mb_id(5)
    }
    #[doc = "0x260 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_id(&self) -> &MbId {
        self.mb_id(6)
    }
    #[doc = "0x270 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_id(&self) -> &MbId {
        self.mb_id(7)
    }
    #[doc = "0x280 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_id(&self) -> &MbId {
        self.mb_id(8)
    }
    #[doc = "0x290 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_id(&self) -> &MbId {
        self.mb_id(9)
    }
    #[doc = "0x2a0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_id(&self) -> &MbId {
        self.mb_id(10)
    }
    #[doc = "0x2b0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_id(&self) -> &MbId {
        self.mb_id(11)
    }
    #[doc = "0x2c0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_id(&self) -> &MbId {
        self.mb_id(12)
    }
    #[doc = "0x2d0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_id(&self) -> &MbId {
        self.mb_id(13)
    }
    #[doc = "0x2e0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_id(&self) -> &MbId {
        self.mb_id(14)
    }
    #[doc = "0x2f0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_id(&self) -> &MbId {
        self.mb_id(15)
    }
    #[doc = "0x300 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_id(&self) -> &MbId {
        self.mb_id(16)
    }
    #[doc = "0x310 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_id(&self) -> &MbId {
        self.mb_id(17)
    }
    #[doc = "0x320 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_id(&self) -> &MbId {
        self.mb_id(18)
    }
    #[doc = "0x330 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_id(&self) -> &MbId {
        self.mb_id(19)
    }
    #[doc = "0x340 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_id(&self) -> &MbId {
        self.mb_id(20)
    }
    #[doc = "0x350 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_id(&self) -> &MbId {
        self.mb_id(21)
    }
    #[doc = "0x360 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_id(&self) -> &MbId {
        self.mb_id(22)
    }
    #[doc = "0x370 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_id(&self) -> &MbId {
        self.mb_id(23)
    }
    #[doc = "0x380 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_id(&self) -> &MbId {
        self.mb_id(24)
    }
    #[doc = "0x390 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_id(&self) -> &MbId {
        self.mb_id(25)
    }
    #[doc = "0x3a0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_id(&self) -> &MbId {
        self.mb_id(26)
    }
    #[doc = "0x3b0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_id(&self) -> &MbId {
        self.mb_id(27)
    }
    #[doc = "0x3c0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_id(&self) -> &MbId {
        self.mb_id(28)
    }
    #[doc = "0x3d0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_id(&self) -> &MbId {
        self.mb_id(29)
    }
    #[doc = "0x3e0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_id(&self) -> &MbId {
        self.mb_id(30)
    }
    #[doc = "0x3f0 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_id(&self) -> &MbId {
        self.mb_id(31)
    }
    #[doc = "0x204..0x244 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_dl(&self, n: usize) -> &MbDl {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x204..0x244 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_dl_iter(&self) -> impl Iterator<Item = &MbDl> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x204 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_dl(&self) -> &MbDl {
        self.mb_dl(0)
    }
    #[doc = "0x214 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_dl(&self) -> &MbDl {
        self.mb_dl(1)
    }
    #[doc = "0x224 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_dl(&self) -> &MbDl {
        self.mb_dl(2)
    }
    #[doc = "0x234 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_dl(&self) -> &MbDl {
        self.mb_dl(3)
    }
    #[doc = "0x244 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_dl(&self) -> &MbDl {
        self.mb_dl(4)
    }
    #[doc = "0x254 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_dl(&self) -> &MbDl {
        self.mb_dl(5)
    }
    #[doc = "0x264 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_dl(&self) -> &MbDl {
        self.mb_dl(6)
    }
    #[doc = "0x274 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_dl(&self) -> &MbDl {
        self.mb_dl(7)
    }
    #[doc = "0x284 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_dl(&self) -> &MbDl {
        self.mb_dl(8)
    }
    #[doc = "0x294 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_dl(&self) -> &MbDl {
        self.mb_dl(9)
    }
    #[doc = "0x2a4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_dl(&self) -> &MbDl {
        self.mb_dl(10)
    }
    #[doc = "0x2b4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_dl(&self) -> &MbDl {
        self.mb_dl(11)
    }
    #[doc = "0x2c4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_dl(&self) -> &MbDl {
        self.mb_dl(12)
    }
    #[doc = "0x2d4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_dl(&self) -> &MbDl {
        self.mb_dl(13)
    }
    #[doc = "0x2e4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_dl(&self) -> &MbDl {
        self.mb_dl(14)
    }
    #[doc = "0x2f4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_dl(&self) -> &MbDl {
        self.mb_dl(15)
    }
    #[doc = "0x304 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_dl(&self) -> &MbDl {
        self.mb_dl(16)
    }
    #[doc = "0x314 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_dl(&self) -> &MbDl {
        self.mb_dl(17)
    }
    #[doc = "0x324 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_dl(&self) -> &MbDl {
        self.mb_dl(18)
    }
    #[doc = "0x334 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_dl(&self) -> &MbDl {
        self.mb_dl(19)
    }
    #[doc = "0x344 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_dl(&self) -> &MbDl {
        self.mb_dl(20)
    }
    #[doc = "0x354 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_dl(&self) -> &MbDl {
        self.mb_dl(21)
    }
    #[doc = "0x364 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_dl(&self) -> &MbDl {
        self.mb_dl(22)
    }
    #[doc = "0x374 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_dl(&self) -> &MbDl {
        self.mb_dl(23)
    }
    #[doc = "0x384 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_dl(&self) -> &MbDl {
        self.mb_dl(24)
    }
    #[doc = "0x394 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_dl(&self) -> &MbDl {
        self.mb_dl(25)
    }
    #[doc = "0x3a4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_dl(&self) -> &MbDl {
        self.mb_dl(26)
    }
    #[doc = "0x3b4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_dl(&self) -> &MbDl {
        self.mb_dl(27)
    }
    #[doc = "0x3c4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_dl(&self) -> &MbDl {
        self.mb_dl(28)
    }
    #[doc = "0x3d4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_dl(&self) -> &MbDl {
        self.mb_dl(29)
    }
    #[doc = "0x3e4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_dl(&self) -> &MbDl {
        self.mb_dl(30)
    }
    #[doc = "0x3f4 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_dl(&self) -> &MbDl {
        self.mb_dl(31)
    }
    #[doc = "0x206..0x226 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d0(&self, n: usize) -> &MbD0 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(518)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x206..0x226 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d0_iter(&self) -> impl Iterator<Item = &MbD0> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(518)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x206 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d0(&self) -> &MbD0 {
        self.mb_d0(0)
    }
    #[doc = "0x216 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d0(&self) -> &MbD0 {
        self.mb_d0(1)
    }
    #[doc = "0x226 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d0(&self) -> &MbD0 {
        self.mb_d0(2)
    }
    #[doc = "0x236 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d0(&self) -> &MbD0 {
        self.mb_d0(3)
    }
    #[doc = "0x246 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d0(&self) -> &MbD0 {
        self.mb_d0(4)
    }
    #[doc = "0x256 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d0(&self) -> &MbD0 {
        self.mb_d0(5)
    }
    #[doc = "0x266 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d0(&self) -> &MbD0 {
        self.mb_d0(6)
    }
    #[doc = "0x276 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d0(&self) -> &MbD0 {
        self.mb_d0(7)
    }
    #[doc = "0x286 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d0(&self) -> &MbD0 {
        self.mb_d0(8)
    }
    #[doc = "0x296 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d0(&self) -> &MbD0 {
        self.mb_d0(9)
    }
    #[doc = "0x2a6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d0(&self) -> &MbD0 {
        self.mb_d0(10)
    }
    #[doc = "0x2b6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d0(&self) -> &MbD0 {
        self.mb_d0(11)
    }
    #[doc = "0x2c6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d0(&self) -> &MbD0 {
        self.mb_d0(12)
    }
    #[doc = "0x2d6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d0(&self) -> &MbD0 {
        self.mb_d0(13)
    }
    #[doc = "0x2e6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d0(&self) -> &MbD0 {
        self.mb_d0(14)
    }
    #[doc = "0x2f6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d0(&self) -> &MbD0 {
        self.mb_d0(15)
    }
    #[doc = "0x306 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d0(&self) -> &MbD0 {
        self.mb_d0(16)
    }
    #[doc = "0x316 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d0(&self) -> &MbD0 {
        self.mb_d0(17)
    }
    #[doc = "0x326 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d0(&self) -> &MbD0 {
        self.mb_d0(18)
    }
    #[doc = "0x336 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d0(&self) -> &MbD0 {
        self.mb_d0(19)
    }
    #[doc = "0x346 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d0(&self) -> &MbD0 {
        self.mb_d0(20)
    }
    #[doc = "0x356 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d0(&self) -> &MbD0 {
        self.mb_d0(21)
    }
    #[doc = "0x366 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d0(&self) -> &MbD0 {
        self.mb_d0(22)
    }
    #[doc = "0x376 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d0(&self) -> &MbD0 {
        self.mb_d0(23)
    }
    #[doc = "0x386 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d0(&self) -> &MbD0 {
        self.mb_d0(24)
    }
    #[doc = "0x396 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d0(&self) -> &MbD0 {
        self.mb_d0(25)
    }
    #[doc = "0x3a6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d0(&self) -> &MbD0 {
        self.mb_d0(26)
    }
    #[doc = "0x3b6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d0(&self) -> &MbD0 {
        self.mb_d0(27)
    }
    #[doc = "0x3c6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d0(&self) -> &MbD0 {
        self.mb_d0(28)
    }
    #[doc = "0x3d6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d0(&self) -> &MbD0 {
        self.mb_d0(29)
    }
    #[doc = "0x3e6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d0(&self) -> &MbD0 {
        self.mb_d0(30)
    }
    #[doc = "0x3f6 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d0(&self) -> &MbD0 {
        self.mb_d0(31)
    }
    #[doc = "0x207..0x227 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d1(&self, n: usize) -> &MbD1 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(519)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x207..0x227 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d1_iter(&self) -> impl Iterator<Item = &MbD1> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(519)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x207 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d1(&self) -> &MbD1 {
        self.mb_d1(0)
    }
    #[doc = "0x217 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d1(&self) -> &MbD1 {
        self.mb_d1(1)
    }
    #[doc = "0x227 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d1(&self) -> &MbD1 {
        self.mb_d1(2)
    }
    #[doc = "0x237 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d1(&self) -> &MbD1 {
        self.mb_d1(3)
    }
    #[doc = "0x247 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d1(&self) -> &MbD1 {
        self.mb_d1(4)
    }
    #[doc = "0x257 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d1(&self) -> &MbD1 {
        self.mb_d1(5)
    }
    #[doc = "0x267 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d1(&self) -> &MbD1 {
        self.mb_d1(6)
    }
    #[doc = "0x277 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d1(&self) -> &MbD1 {
        self.mb_d1(7)
    }
    #[doc = "0x287 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d1(&self) -> &MbD1 {
        self.mb_d1(8)
    }
    #[doc = "0x297 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d1(&self) -> &MbD1 {
        self.mb_d1(9)
    }
    #[doc = "0x2a7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d1(&self) -> &MbD1 {
        self.mb_d1(10)
    }
    #[doc = "0x2b7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d1(&self) -> &MbD1 {
        self.mb_d1(11)
    }
    #[doc = "0x2c7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d1(&self) -> &MbD1 {
        self.mb_d1(12)
    }
    #[doc = "0x2d7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d1(&self) -> &MbD1 {
        self.mb_d1(13)
    }
    #[doc = "0x2e7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d1(&self) -> &MbD1 {
        self.mb_d1(14)
    }
    #[doc = "0x2f7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d1(&self) -> &MbD1 {
        self.mb_d1(15)
    }
    #[doc = "0x307 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d1(&self) -> &MbD1 {
        self.mb_d1(16)
    }
    #[doc = "0x317 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d1(&self) -> &MbD1 {
        self.mb_d1(17)
    }
    #[doc = "0x327 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d1(&self) -> &MbD1 {
        self.mb_d1(18)
    }
    #[doc = "0x337 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d1(&self) -> &MbD1 {
        self.mb_d1(19)
    }
    #[doc = "0x347 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d1(&self) -> &MbD1 {
        self.mb_d1(20)
    }
    #[doc = "0x357 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d1(&self) -> &MbD1 {
        self.mb_d1(21)
    }
    #[doc = "0x367 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d1(&self) -> &MbD1 {
        self.mb_d1(22)
    }
    #[doc = "0x377 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d1(&self) -> &MbD1 {
        self.mb_d1(23)
    }
    #[doc = "0x387 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d1(&self) -> &MbD1 {
        self.mb_d1(24)
    }
    #[doc = "0x397 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d1(&self) -> &MbD1 {
        self.mb_d1(25)
    }
    #[doc = "0x3a7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d1(&self) -> &MbD1 {
        self.mb_d1(26)
    }
    #[doc = "0x3b7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d1(&self) -> &MbD1 {
        self.mb_d1(27)
    }
    #[doc = "0x3c7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d1(&self) -> &MbD1 {
        self.mb_d1(28)
    }
    #[doc = "0x3d7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d1(&self) -> &MbD1 {
        self.mb_d1(29)
    }
    #[doc = "0x3e7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d1(&self) -> &MbD1 {
        self.mb_d1(30)
    }
    #[doc = "0x3f7 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d1(&self) -> &MbD1 {
        self.mb_d1(31)
    }
    #[doc = "0x208..0x228 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d2(&self, n: usize) -> &MbD2 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x208..0x228 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d2_iter(&self) -> impl Iterator<Item = &MbD2> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x208 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d2(&self) -> &MbD2 {
        self.mb_d2(0)
    }
    #[doc = "0x218 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d2(&self) -> &MbD2 {
        self.mb_d2(1)
    }
    #[doc = "0x228 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d2(&self) -> &MbD2 {
        self.mb_d2(2)
    }
    #[doc = "0x238 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d2(&self) -> &MbD2 {
        self.mb_d2(3)
    }
    #[doc = "0x248 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d2(&self) -> &MbD2 {
        self.mb_d2(4)
    }
    #[doc = "0x258 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d2(&self) -> &MbD2 {
        self.mb_d2(5)
    }
    #[doc = "0x268 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d2(&self) -> &MbD2 {
        self.mb_d2(6)
    }
    #[doc = "0x278 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d2(&self) -> &MbD2 {
        self.mb_d2(7)
    }
    #[doc = "0x288 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d2(&self) -> &MbD2 {
        self.mb_d2(8)
    }
    #[doc = "0x298 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d2(&self) -> &MbD2 {
        self.mb_d2(9)
    }
    #[doc = "0x2a8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d2(&self) -> &MbD2 {
        self.mb_d2(10)
    }
    #[doc = "0x2b8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d2(&self) -> &MbD2 {
        self.mb_d2(11)
    }
    #[doc = "0x2c8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d2(&self) -> &MbD2 {
        self.mb_d2(12)
    }
    #[doc = "0x2d8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d2(&self) -> &MbD2 {
        self.mb_d2(13)
    }
    #[doc = "0x2e8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d2(&self) -> &MbD2 {
        self.mb_d2(14)
    }
    #[doc = "0x2f8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d2(&self) -> &MbD2 {
        self.mb_d2(15)
    }
    #[doc = "0x308 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d2(&self) -> &MbD2 {
        self.mb_d2(16)
    }
    #[doc = "0x318 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d2(&self) -> &MbD2 {
        self.mb_d2(17)
    }
    #[doc = "0x328 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d2(&self) -> &MbD2 {
        self.mb_d2(18)
    }
    #[doc = "0x338 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d2(&self) -> &MbD2 {
        self.mb_d2(19)
    }
    #[doc = "0x348 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d2(&self) -> &MbD2 {
        self.mb_d2(20)
    }
    #[doc = "0x358 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d2(&self) -> &MbD2 {
        self.mb_d2(21)
    }
    #[doc = "0x368 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d2(&self) -> &MbD2 {
        self.mb_d2(22)
    }
    #[doc = "0x378 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d2(&self) -> &MbD2 {
        self.mb_d2(23)
    }
    #[doc = "0x388 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d2(&self) -> &MbD2 {
        self.mb_d2(24)
    }
    #[doc = "0x398 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d2(&self) -> &MbD2 {
        self.mb_d2(25)
    }
    #[doc = "0x3a8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d2(&self) -> &MbD2 {
        self.mb_d2(26)
    }
    #[doc = "0x3b8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d2(&self) -> &MbD2 {
        self.mb_d2(27)
    }
    #[doc = "0x3c8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d2(&self) -> &MbD2 {
        self.mb_d2(28)
    }
    #[doc = "0x3d8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d2(&self) -> &MbD2 {
        self.mb_d2(29)
    }
    #[doc = "0x3e8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d2(&self) -> &MbD2 {
        self.mb_d2(30)
    }
    #[doc = "0x3f8 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d2(&self) -> &MbD2 {
        self.mb_d2(31)
    }
    #[doc = "0x209..0x229 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d3(&self, n: usize) -> &MbD3 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(521)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x209..0x229 - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d3_iter(&self) -> impl Iterator<Item = &MbD3> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(521)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x209 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d3(&self) -> &MbD3 {
        self.mb_d3(0)
    }
    #[doc = "0x219 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d3(&self) -> &MbD3 {
        self.mb_d3(1)
    }
    #[doc = "0x229 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d3(&self) -> &MbD3 {
        self.mb_d3(2)
    }
    #[doc = "0x239 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d3(&self) -> &MbD3 {
        self.mb_d3(3)
    }
    #[doc = "0x249 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d3(&self) -> &MbD3 {
        self.mb_d3(4)
    }
    #[doc = "0x259 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d3(&self) -> &MbD3 {
        self.mb_d3(5)
    }
    #[doc = "0x269 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d3(&self) -> &MbD3 {
        self.mb_d3(6)
    }
    #[doc = "0x279 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d3(&self) -> &MbD3 {
        self.mb_d3(7)
    }
    #[doc = "0x289 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d3(&self) -> &MbD3 {
        self.mb_d3(8)
    }
    #[doc = "0x299 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d3(&self) -> &MbD3 {
        self.mb_d3(9)
    }
    #[doc = "0x2a9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d3(&self) -> &MbD3 {
        self.mb_d3(10)
    }
    #[doc = "0x2b9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d3(&self) -> &MbD3 {
        self.mb_d3(11)
    }
    #[doc = "0x2c9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d3(&self) -> &MbD3 {
        self.mb_d3(12)
    }
    #[doc = "0x2d9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d3(&self) -> &MbD3 {
        self.mb_d3(13)
    }
    #[doc = "0x2e9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d3(&self) -> &MbD3 {
        self.mb_d3(14)
    }
    #[doc = "0x2f9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d3(&self) -> &MbD3 {
        self.mb_d3(15)
    }
    #[doc = "0x309 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d3(&self) -> &MbD3 {
        self.mb_d3(16)
    }
    #[doc = "0x319 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d3(&self) -> &MbD3 {
        self.mb_d3(17)
    }
    #[doc = "0x329 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d3(&self) -> &MbD3 {
        self.mb_d3(18)
    }
    #[doc = "0x339 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d3(&self) -> &MbD3 {
        self.mb_d3(19)
    }
    #[doc = "0x349 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d3(&self) -> &MbD3 {
        self.mb_d3(20)
    }
    #[doc = "0x359 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d3(&self) -> &MbD3 {
        self.mb_d3(21)
    }
    #[doc = "0x369 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d3(&self) -> &MbD3 {
        self.mb_d3(22)
    }
    #[doc = "0x379 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d3(&self) -> &MbD3 {
        self.mb_d3(23)
    }
    #[doc = "0x389 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d3(&self) -> &MbD3 {
        self.mb_d3(24)
    }
    #[doc = "0x399 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d3(&self) -> &MbD3 {
        self.mb_d3(25)
    }
    #[doc = "0x3a9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d3(&self) -> &MbD3 {
        self.mb_d3(26)
    }
    #[doc = "0x3b9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d3(&self) -> &MbD3 {
        self.mb_d3(27)
    }
    #[doc = "0x3c9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d3(&self) -> &MbD3 {
        self.mb_d3(28)
    }
    #[doc = "0x3d9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d3(&self) -> &MbD3 {
        self.mb_d3(29)
    }
    #[doc = "0x3e9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d3(&self) -> &MbD3 {
        self.mb_d3(30)
    }
    #[doc = "0x3f9 - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d3(&self) -> &MbD3 {
        self.mb_d3(31)
    }
    #[doc = "0x20a..0x22a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d4(&self, n: usize) -> &MbD4 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(522)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20a..0x22a - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d4_iter(&self) -> impl Iterator<Item = &MbD4> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(522)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x20a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d4(&self) -> &MbD4 {
        self.mb_d4(0)
    }
    #[doc = "0x21a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d4(&self) -> &MbD4 {
        self.mb_d4(1)
    }
    #[doc = "0x22a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d4(&self) -> &MbD4 {
        self.mb_d4(2)
    }
    #[doc = "0x23a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d4(&self) -> &MbD4 {
        self.mb_d4(3)
    }
    #[doc = "0x24a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d4(&self) -> &MbD4 {
        self.mb_d4(4)
    }
    #[doc = "0x25a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d4(&self) -> &MbD4 {
        self.mb_d4(5)
    }
    #[doc = "0x26a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d4(&self) -> &MbD4 {
        self.mb_d4(6)
    }
    #[doc = "0x27a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d4(&self) -> &MbD4 {
        self.mb_d4(7)
    }
    #[doc = "0x28a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d4(&self) -> &MbD4 {
        self.mb_d4(8)
    }
    #[doc = "0x29a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d4(&self) -> &MbD4 {
        self.mb_d4(9)
    }
    #[doc = "0x2aa - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d4(&self) -> &MbD4 {
        self.mb_d4(10)
    }
    #[doc = "0x2ba - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d4(&self) -> &MbD4 {
        self.mb_d4(11)
    }
    #[doc = "0x2ca - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d4(&self) -> &MbD4 {
        self.mb_d4(12)
    }
    #[doc = "0x2da - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d4(&self) -> &MbD4 {
        self.mb_d4(13)
    }
    #[doc = "0x2ea - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d4(&self) -> &MbD4 {
        self.mb_d4(14)
    }
    #[doc = "0x2fa - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d4(&self) -> &MbD4 {
        self.mb_d4(15)
    }
    #[doc = "0x30a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d4(&self) -> &MbD4 {
        self.mb_d4(16)
    }
    #[doc = "0x31a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d4(&self) -> &MbD4 {
        self.mb_d4(17)
    }
    #[doc = "0x32a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d4(&self) -> &MbD4 {
        self.mb_d4(18)
    }
    #[doc = "0x33a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d4(&self) -> &MbD4 {
        self.mb_d4(19)
    }
    #[doc = "0x34a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d4(&self) -> &MbD4 {
        self.mb_d4(20)
    }
    #[doc = "0x35a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d4(&self) -> &MbD4 {
        self.mb_d4(21)
    }
    #[doc = "0x36a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d4(&self) -> &MbD4 {
        self.mb_d4(22)
    }
    #[doc = "0x37a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d4(&self) -> &MbD4 {
        self.mb_d4(23)
    }
    #[doc = "0x38a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d4(&self) -> &MbD4 {
        self.mb_d4(24)
    }
    #[doc = "0x39a - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d4(&self) -> &MbD4 {
        self.mb_d4(25)
    }
    #[doc = "0x3aa - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d4(&self) -> &MbD4 {
        self.mb_d4(26)
    }
    #[doc = "0x3ba - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d4(&self) -> &MbD4 {
        self.mb_d4(27)
    }
    #[doc = "0x3ca - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d4(&self) -> &MbD4 {
        self.mb_d4(28)
    }
    #[doc = "0x3da - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d4(&self) -> &MbD4 {
        self.mb_d4(29)
    }
    #[doc = "0x3ea - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d4(&self) -> &MbD4 {
        self.mb_d4(30)
    }
    #[doc = "0x3fa - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d4(&self) -> &MbD4 {
        self.mb_d4(31)
    }
    #[doc = "0x20b..0x22b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d5(&self, n: usize) -> &MbD5 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(523)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20b..0x22b - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d5_iter(&self) -> impl Iterator<Item = &MbD5> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(523)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x20b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d5(&self) -> &MbD5 {
        self.mb_d5(0)
    }
    #[doc = "0x21b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d5(&self) -> &MbD5 {
        self.mb_d5(1)
    }
    #[doc = "0x22b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d5(&self) -> &MbD5 {
        self.mb_d5(2)
    }
    #[doc = "0x23b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d5(&self) -> &MbD5 {
        self.mb_d5(3)
    }
    #[doc = "0x24b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d5(&self) -> &MbD5 {
        self.mb_d5(4)
    }
    #[doc = "0x25b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d5(&self) -> &MbD5 {
        self.mb_d5(5)
    }
    #[doc = "0x26b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d5(&self) -> &MbD5 {
        self.mb_d5(6)
    }
    #[doc = "0x27b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d5(&self) -> &MbD5 {
        self.mb_d5(7)
    }
    #[doc = "0x28b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d5(&self) -> &MbD5 {
        self.mb_d5(8)
    }
    #[doc = "0x29b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d5(&self) -> &MbD5 {
        self.mb_d5(9)
    }
    #[doc = "0x2ab - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d5(&self) -> &MbD5 {
        self.mb_d5(10)
    }
    #[doc = "0x2bb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d5(&self) -> &MbD5 {
        self.mb_d5(11)
    }
    #[doc = "0x2cb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d5(&self) -> &MbD5 {
        self.mb_d5(12)
    }
    #[doc = "0x2db - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d5(&self) -> &MbD5 {
        self.mb_d5(13)
    }
    #[doc = "0x2eb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d5(&self) -> &MbD5 {
        self.mb_d5(14)
    }
    #[doc = "0x2fb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d5(&self) -> &MbD5 {
        self.mb_d5(15)
    }
    #[doc = "0x30b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d5(&self) -> &MbD5 {
        self.mb_d5(16)
    }
    #[doc = "0x31b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d5(&self) -> &MbD5 {
        self.mb_d5(17)
    }
    #[doc = "0x32b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d5(&self) -> &MbD5 {
        self.mb_d5(18)
    }
    #[doc = "0x33b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d5(&self) -> &MbD5 {
        self.mb_d5(19)
    }
    #[doc = "0x34b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d5(&self) -> &MbD5 {
        self.mb_d5(20)
    }
    #[doc = "0x35b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d5(&self) -> &MbD5 {
        self.mb_d5(21)
    }
    #[doc = "0x36b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d5(&self) -> &MbD5 {
        self.mb_d5(22)
    }
    #[doc = "0x37b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d5(&self) -> &MbD5 {
        self.mb_d5(23)
    }
    #[doc = "0x38b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d5(&self) -> &MbD5 {
        self.mb_d5(24)
    }
    #[doc = "0x39b - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d5(&self) -> &MbD5 {
        self.mb_d5(25)
    }
    #[doc = "0x3ab - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d5(&self) -> &MbD5 {
        self.mb_d5(26)
    }
    #[doc = "0x3bb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d5(&self) -> &MbD5 {
        self.mb_d5(27)
    }
    #[doc = "0x3cb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d5(&self) -> &MbD5 {
        self.mb_d5(28)
    }
    #[doc = "0x3db - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d5(&self) -> &MbD5 {
        self.mb_d5(29)
    }
    #[doc = "0x3eb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d5(&self) -> &MbD5 {
        self.mb_d5(30)
    }
    #[doc = "0x3fb - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d5(&self) -> &MbD5 {
        self.mb_d5(31)
    }
    #[doc = "0x20c..0x22c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d6(&self, n: usize) -> &MbD6 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(524)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20c..0x22c - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d6_iter(&self) -> impl Iterator<Item = &MbD6> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(524)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x20c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d6(&self) -> &MbD6 {
        self.mb_d6(0)
    }
    #[doc = "0x21c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d6(&self) -> &MbD6 {
        self.mb_d6(1)
    }
    #[doc = "0x22c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d6(&self) -> &MbD6 {
        self.mb_d6(2)
    }
    #[doc = "0x23c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d6(&self) -> &MbD6 {
        self.mb_d6(3)
    }
    #[doc = "0x24c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d6(&self) -> &MbD6 {
        self.mb_d6(4)
    }
    #[doc = "0x25c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d6(&self) -> &MbD6 {
        self.mb_d6(5)
    }
    #[doc = "0x26c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d6(&self) -> &MbD6 {
        self.mb_d6(6)
    }
    #[doc = "0x27c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d6(&self) -> &MbD6 {
        self.mb_d6(7)
    }
    #[doc = "0x28c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d6(&self) -> &MbD6 {
        self.mb_d6(8)
    }
    #[doc = "0x29c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d6(&self) -> &MbD6 {
        self.mb_d6(9)
    }
    #[doc = "0x2ac - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d6(&self) -> &MbD6 {
        self.mb_d6(10)
    }
    #[doc = "0x2bc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d6(&self) -> &MbD6 {
        self.mb_d6(11)
    }
    #[doc = "0x2cc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d6(&self) -> &MbD6 {
        self.mb_d6(12)
    }
    #[doc = "0x2dc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d6(&self) -> &MbD6 {
        self.mb_d6(13)
    }
    #[doc = "0x2ec - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d6(&self) -> &MbD6 {
        self.mb_d6(14)
    }
    #[doc = "0x2fc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d6(&self) -> &MbD6 {
        self.mb_d6(15)
    }
    #[doc = "0x30c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d6(&self) -> &MbD6 {
        self.mb_d6(16)
    }
    #[doc = "0x31c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d6(&self) -> &MbD6 {
        self.mb_d6(17)
    }
    #[doc = "0x32c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d6(&self) -> &MbD6 {
        self.mb_d6(18)
    }
    #[doc = "0x33c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d6(&self) -> &MbD6 {
        self.mb_d6(19)
    }
    #[doc = "0x34c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d6(&self) -> &MbD6 {
        self.mb_d6(20)
    }
    #[doc = "0x35c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d6(&self) -> &MbD6 {
        self.mb_d6(21)
    }
    #[doc = "0x36c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d6(&self) -> &MbD6 {
        self.mb_d6(22)
    }
    #[doc = "0x37c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d6(&self) -> &MbD6 {
        self.mb_d6(23)
    }
    #[doc = "0x38c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d6(&self) -> &MbD6 {
        self.mb_d6(24)
    }
    #[doc = "0x39c - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d6(&self) -> &MbD6 {
        self.mb_d6(25)
    }
    #[doc = "0x3ac - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d6(&self) -> &MbD6 {
        self.mb_d6(26)
    }
    #[doc = "0x3bc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d6(&self) -> &MbD6 {
        self.mb_d6(27)
    }
    #[doc = "0x3cc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d6(&self) -> &MbD6 {
        self.mb_d6(28)
    }
    #[doc = "0x3dc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d6(&self) -> &MbD6 {
        self.mb_d6(29)
    }
    #[doc = "0x3ec - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d6(&self) -> &MbD6 {
        self.mb_d6(30)
    }
    #[doc = "0x3fc - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d6(&self) -> &MbD6 {
        self.mb_d6(31)
    }
    #[doc = "0x20d..0x22d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_d7(&self, n: usize) -> &MbD7 {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(525)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20d..0x22d - Mailbox Register"]
    #[inline(always)]
    pub fn mb_d7_iter(&self) -> impl Iterator<Item = &MbD7> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(525)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x20d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_d7(&self) -> &MbD7 {
        self.mb_d7(0)
    }
    #[doc = "0x21d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_d7(&self) -> &MbD7 {
        self.mb_d7(1)
    }
    #[doc = "0x22d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_d7(&self) -> &MbD7 {
        self.mb_d7(2)
    }
    #[doc = "0x23d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_d7(&self) -> &MbD7 {
        self.mb_d7(3)
    }
    #[doc = "0x24d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_d7(&self) -> &MbD7 {
        self.mb_d7(4)
    }
    #[doc = "0x25d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_d7(&self) -> &MbD7 {
        self.mb_d7(5)
    }
    #[doc = "0x26d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_d7(&self) -> &MbD7 {
        self.mb_d7(6)
    }
    #[doc = "0x27d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_d7(&self) -> &MbD7 {
        self.mb_d7(7)
    }
    #[doc = "0x28d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_d7(&self) -> &MbD7 {
        self.mb_d7(8)
    }
    #[doc = "0x29d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_d7(&self) -> &MbD7 {
        self.mb_d7(9)
    }
    #[doc = "0x2ad - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_d7(&self) -> &MbD7 {
        self.mb_d7(10)
    }
    #[doc = "0x2bd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_d7(&self) -> &MbD7 {
        self.mb_d7(11)
    }
    #[doc = "0x2cd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_d7(&self) -> &MbD7 {
        self.mb_d7(12)
    }
    #[doc = "0x2dd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_d7(&self) -> &MbD7 {
        self.mb_d7(13)
    }
    #[doc = "0x2ed - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_d7(&self) -> &MbD7 {
        self.mb_d7(14)
    }
    #[doc = "0x2fd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_d7(&self) -> &MbD7 {
        self.mb_d7(15)
    }
    #[doc = "0x30d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_d7(&self) -> &MbD7 {
        self.mb_d7(16)
    }
    #[doc = "0x31d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_d7(&self) -> &MbD7 {
        self.mb_d7(17)
    }
    #[doc = "0x32d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_d7(&self) -> &MbD7 {
        self.mb_d7(18)
    }
    #[doc = "0x33d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_d7(&self) -> &MbD7 {
        self.mb_d7(19)
    }
    #[doc = "0x34d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_d7(&self) -> &MbD7 {
        self.mb_d7(20)
    }
    #[doc = "0x35d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_d7(&self) -> &MbD7 {
        self.mb_d7(21)
    }
    #[doc = "0x36d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_d7(&self) -> &MbD7 {
        self.mb_d7(22)
    }
    #[doc = "0x37d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_d7(&self) -> &MbD7 {
        self.mb_d7(23)
    }
    #[doc = "0x38d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_d7(&self) -> &MbD7 {
        self.mb_d7(24)
    }
    #[doc = "0x39d - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_d7(&self) -> &MbD7 {
        self.mb_d7(25)
    }
    #[doc = "0x3ad - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_d7(&self) -> &MbD7 {
        self.mb_d7(26)
    }
    #[doc = "0x3bd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_d7(&self) -> &MbD7 {
        self.mb_d7(27)
    }
    #[doc = "0x3cd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_d7(&self) -> &MbD7 {
        self.mb_d7(28)
    }
    #[doc = "0x3dd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_d7(&self) -> &MbD7 {
        self.mb_d7(29)
    }
    #[doc = "0x3ed - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_d7(&self) -> &MbD7 {
        self.mb_d7(30)
    }
    #[doc = "0x3fd - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_d7(&self) -> &MbD7 {
        self.mb_d7(31)
    }
    #[doc = "0x20e..0x24e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb_ts(&self, n: usize) -> &MbTs {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(526)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20e..0x24e - Mailbox Register"]
    #[inline(always)]
    pub fn mb_ts_iter(&self) -> impl Iterator<Item = &MbTs> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(526)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x20e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb0_ts(&self) -> &MbTs {
        self.mb_ts(0)
    }
    #[doc = "0x21e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb1_ts(&self) -> &MbTs {
        self.mb_ts(1)
    }
    #[doc = "0x22e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb2_ts(&self) -> &MbTs {
        self.mb_ts(2)
    }
    #[doc = "0x23e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb3_ts(&self) -> &MbTs {
        self.mb_ts(3)
    }
    #[doc = "0x24e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb4_ts(&self) -> &MbTs {
        self.mb_ts(4)
    }
    #[doc = "0x25e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb5_ts(&self) -> &MbTs {
        self.mb_ts(5)
    }
    #[doc = "0x26e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb6_ts(&self) -> &MbTs {
        self.mb_ts(6)
    }
    #[doc = "0x27e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb7_ts(&self) -> &MbTs {
        self.mb_ts(7)
    }
    #[doc = "0x28e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb8_ts(&self) -> &MbTs {
        self.mb_ts(8)
    }
    #[doc = "0x29e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb9_ts(&self) -> &MbTs {
        self.mb_ts(9)
    }
    #[doc = "0x2ae - Mailbox Register"]
    #[inline(always)]
    pub const fn mb10_ts(&self) -> &MbTs {
        self.mb_ts(10)
    }
    #[doc = "0x2be - Mailbox Register"]
    #[inline(always)]
    pub const fn mb11_ts(&self) -> &MbTs {
        self.mb_ts(11)
    }
    #[doc = "0x2ce - Mailbox Register"]
    #[inline(always)]
    pub const fn mb12_ts(&self) -> &MbTs {
        self.mb_ts(12)
    }
    #[doc = "0x2de - Mailbox Register"]
    #[inline(always)]
    pub const fn mb13_ts(&self) -> &MbTs {
        self.mb_ts(13)
    }
    #[doc = "0x2ee - Mailbox Register"]
    #[inline(always)]
    pub const fn mb14_ts(&self) -> &MbTs {
        self.mb_ts(14)
    }
    #[doc = "0x2fe - Mailbox Register"]
    #[inline(always)]
    pub const fn mb15_ts(&self) -> &MbTs {
        self.mb_ts(15)
    }
    #[doc = "0x30e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb16_ts(&self) -> &MbTs {
        self.mb_ts(16)
    }
    #[doc = "0x31e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb17_ts(&self) -> &MbTs {
        self.mb_ts(17)
    }
    #[doc = "0x32e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb18_ts(&self) -> &MbTs {
        self.mb_ts(18)
    }
    #[doc = "0x33e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb19_ts(&self) -> &MbTs {
        self.mb_ts(19)
    }
    #[doc = "0x34e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb20_ts(&self) -> &MbTs {
        self.mb_ts(20)
    }
    #[doc = "0x35e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb21_ts(&self) -> &MbTs {
        self.mb_ts(21)
    }
    #[doc = "0x36e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb22_ts(&self) -> &MbTs {
        self.mb_ts(22)
    }
    #[doc = "0x37e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb23_ts(&self) -> &MbTs {
        self.mb_ts(23)
    }
    #[doc = "0x38e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb24_ts(&self) -> &MbTs {
        self.mb_ts(24)
    }
    #[doc = "0x39e - Mailbox Register"]
    #[inline(always)]
    pub const fn mb25_ts(&self) -> &MbTs {
        self.mb_ts(25)
    }
    #[doc = "0x3ae - Mailbox Register"]
    #[inline(always)]
    pub const fn mb26_ts(&self) -> &MbTs {
        self.mb_ts(26)
    }
    #[doc = "0x3be - Mailbox Register"]
    #[inline(always)]
    pub const fn mb27_ts(&self) -> &MbTs {
        self.mb_ts(27)
    }
    #[doc = "0x3ce - Mailbox Register"]
    #[inline(always)]
    pub const fn mb28_ts(&self) -> &MbTs {
        self.mb_ts(28)
    }
    #[doc = "0x3de - Mailbox Register"]
    #[inline(always)]
    pub const fn mb29_ts(&self) -> &MbTs {
        self.mb_ts(29)
    }
    #[doc = "0x3ee - Mailbox Register"]
    #[inline(always)]
    pub const fn mb30_ts(&self) -> &MbTs {
        self.mb_ts(30)
    }
    #[doc = "0x3fe - Mailbox Register"]
    #[inline(always)]
    pub const fn mb31_ts(&self) -> &MbTs {
        self.mb_ts(31)
    }
    #[doc = "0x400..0x420 - Mask Register"]
    #[inline(always)]
    pub const fn mkr(&self, n: usize) -> &Mkr {
        &self.mkr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - Mask Register"]
    #[inline(always)]
    pub fn mkr_iter(&self) -> impl Iterator<Item = &Mkr> {
        self.mkr.iter()
    }
    #[doc = "0x420..0x428 - FIFO Received ID Compare Registers"]
    #[inline(always)]
    pub const fn fidcr(&self, n: usize) -> &Fidcr {
        &self.fidcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x420..0x428 - FIFO Received ID Compare Registers"]
    #[inline(always)]
    pub fn fidcr_iter(&self) -> impl Iterator<Item = &Fidcr> {
        self.fidcr.iter()
    }
    #[doc = "0x428 - Mask Invalid Register"]
    #[inline(always)]
    pub const fn mkivlr(&self) -> &Mkivlr {
        &self.mkivlr
    }
    #[doc = "0x42c - Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
    #[inline(always)]
    pub const fn mier_fifo(&self) -> &MierFifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1068).cast() }
    }
    #[doc = "0x42c - Mailbox Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mier(&self) -> &Mier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1068).cast() }
    }
    #[doc = "0x820..0x840 - Message Control Register for Receive"]
    #[inline(always)]
    pub const fn mctl_rx(&self, n: usize) -> &MctlRx {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2080)
                .add(n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x820..0x840 - Message Control Register for Receive"]
    #[inline(always)]
    pub fn mctl_rx_iter(&self) -> impl Iterator<Item = &MctlRx> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2080)
                .add(n)
                .cast()
        })
    }
    #[doc = "0x820..0x840 - Message Control Register for Transmit"]
    #[inline(always)]
    pub const fn mctl_tx(&self, n: usize) -> &MctlTx {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2080)
                .add(n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x820..0x840 - Message Control Register for Transmit"]
    #[inline(always)]
    pub fn mctl_tx_iter(&self) -> impl Iterator<Item = &MctlTx> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2080)
                .add(n)
                .cast()
        })
    }
    #[doc = "0x840 - Control Register"]
    #[inline(always)]
    pub const fn ctlr(&self) -> &Ctlr {
        &self.ctlr
    }
    #[doc = "0x842 - Status Register"]
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    #[doc = "0x844 - Bit Configuration Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0x848 - Receive FIFO Control Register"]
    #[inline(always)]
    pub const fn rfcr(&self) -> &Rfcr {
        &self.rfcr
    }
    #[doc = "0x849 - Receive FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn rfpcr(&self) -> &Rfpcr {
        &self.rfpcr
    }
    #[doc = "0x84a - Transmit FIFO Control Register"]
    #[inline(always)]
    pub const fn tfcr(&self) -> &Tfcr {
        &self.tfcr
    }
    #[doc = "0x84b - Transmit FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn tfpcr(&self) -> &Tfpcr {
        &self.tfpcr
    }
    #[doc = "0x84c - Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn eier(&self) -> &Eier {
        &self.eier
    }
    #[doc = "0x84d - Error Interrupt Factor Judge Register"]
    #[inline(always)]
    pub const fn eifr(&self) -> &Eifr {
        &self.eifr
    }
    #[doc = "0x84e - Receive Error Count Register"]
    #[inline(always)]
    pub const fn recr(&self) -> &Recr {
        &self.recr
    }
    #[doc = "0x84f - Transmit Error Count Register"]
    #[inline(always)]
    pub const fn tecr(&self) -> &Tecr {
        &self.tecr
    }
    #[doc = "0x850 - Error Code Store Register"]
    #[inline(always)]
    pub const fn ecsr(&self) -> &Ecsr {
        &self.ecsr
    }
    #[doc = "0x851 - Channel Search Support Register"]
    #[inline(always)]
    pub const fn cssr(&self) -> &Cssr {
        &self.cssr
    }
    #[doc = "0x852 - Mailbox Search Status Register"]
    #[inline(always)]
    pub const fn mssr(&self) -> &Mssr {
        &self.mssr
    }
    #[doc = "0x853 - Mailbox Search Mode Register"]
    #[inline(always)]
    pub const fn msmr(&self) -> &Msmr {
        &self.msmr
    }
    #[doc = "0x854 - Time Stamp Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &Tsr {
        &self.tsr
    }
    #[doc = "0x856 - Acceptance Filter Support Register"]
    #[inline(always)]
    pub const fn afsr(&self) -> &Afsr {
        &self.afsr
    }
    #[doc = "0x858 - Test Control Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
}
#[doc = "MB_ID (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_id`] module"]
#[doc(alias = "MB_ID")]
pub type MbId = crate::Reg<mb_id::MbIdSpec>;
#[doc = "Mailbox Register"]
pub mod mb_id;
#[doc = "MB_DL (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_dl`] module"]
#[doc(alias = "MB_DL")]
pub type MbDl = crate::Reg<mb_dl::MbDlSpec>;
#[doc = "Mailbox Register"]
pub mod mb_dl;
#[doc = "MB_D0 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d0`] module"]
#[doc(alias = "MB_D0")]
pub type MbD0 = crate::Reg<mb_d0::MbD0Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d0;
#[doc = "MB_D1 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d1`] module"]
#[doc(alias = "MB_D1")]
pub type MbD1 = crate::Reg<mb_d1::MbD1Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d1;
#[doc = "MB_D2 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d2`] module"]
#[doc(alias = "MB_D2")]
pub type MbD2 = crate::Reg<mb_d2::MbD2Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d2;
#[doc = "MB_D3 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d3`] module"]
#[doc(alias = "MB_D3")]
pub type MbD3 = crate::Reg<mb_d3::MbD3Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d3;
#[doc = "MB_D4 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d4`] module"]
#[doc(alias = "MB_D4")]
pub type MbD4 = crate::Reg<mb_d4::MbD4Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d4;
#[doc = "MB_D5 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d5`] module"]
#[doc(alias = "MB_D5")]
pub type MbD5 = crate::Reg<mb_d5::MbD5Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d5;
#[doc = "MB_D6 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d6`] module"]
#[doc(alias = "MB_D6")]
pub type MbD6 = crate::Reg<mb_d6::MbD6Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d6;
#[doc = "MB_D7 (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_d7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_d7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_d7`] module"]
#[doc(alias = "MB_D7")]
pub type MbD7 = crate::Reg<mb_d7::MbD7Spec>;
#[doc = "Mailbox Register"]
pub mod mb_d7;
#[doc = "MB_TS (rw) register accessor: Mailbox Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb_ts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb_ts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb_ts`] module"]
#[doc(alias = "MB_TS")]
pub type MbTs = crate::Reg<mb_ts::MbTsSpec>;
#[doc = "Mailbox Register"]
pub mod mb_ts;
#[doc = "MKR (rw) register accessor: Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mkr`] module"]
#[doc(alias = "MKR")]
pub type Mkr = crate::Reg<mkr::MkrSpec>;
#[doc = "Mask Register"]
pub mod mkr;
#[doc = "FIDCR (rw) register accessor: FIFO Received ID Compare Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fidcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fidcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fidcr`] module"]
#[doc(alias = "FIDCR")]
pub type Fidcr = crate::Reg<fidcr::FidcrSpec>;
#[doc = "FIFO Received ID Compare Registers"]
pub mod fidcr;
#[doc = "MKIVLR (rw) register accessor: Mask Invalid Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mkivlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkivlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mkivlr`] module"]
#[doc(alias = "MKIVLR")]
pub type Mkivlr = crate::Reg<mkivlr::MkivlrSpec>;
#[doc = "Mask Invalid Register"]
pub mod mkivlr;
#[doc = "MIER (rw) register accessor: Mailbox Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mier`] module"]
#[doc(alias = "MIER")]
pub type Mier = crate::Reg<mier::MierSpec>;
#[doc = "Mailbox Interrupt Enable Register"]
pub mod mier;
#[doc = "MIER_FIFO (rw) register accessor: Mailbox Interrupt Enable Register for FIFO Mailbox Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mier_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mier_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mier_fifo`] module"]
#[doc(alias = "MIER_FIFO")]
pub type MierFifo = crate::Reg<mier_fifo::MierFifoSpec>;
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
pub mod mier_fifo;
#[doc = "MCTL_TX (rw) register accessor: Message Control Register for Transmit\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctl_tx`] module"]
#[doc(alias = "MCTL_TX")]
pub type MctlTx = crate::Reg<mctl_tx::MctlTxSpec>;
#[doc = "Message Control Register for Transmit"]
pub mod mctl_tx;
#[doc = "MCTL_RX (rw) register accessor: Message Control Register for Receive\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctl_rx`] module"]
#[doc(alias = "MCTL_RX")]
pub type MctlRx = crate::Reg<mctl_rx::MctlRxSpec>;
#[doc = "Message Control Register for Receive"]
pub mod mctl_rx;
#[doc = "CTLR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctlr`] module"]
#[doc(alias = "CTLR")]
pub type Ctlr = crate::Reg<ctlr::CtlrSpec>;
#[doc = "Control Register"]
pub mod ctlr;
#[doc = "STR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`] module"]
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
#[doc = "Status Register"]
pub mod str;
#[doc = "BCR (rw) register accessor: Bit Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Bit Configuration Register"]
pub mod bcr;
#[doc = "RFCR (rw) register accessor: Receive FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcr`] module"]
#[doc(alias = "RFCR")]
pub type Rfcr = crate::Reg<rfcr::RfcrSpec>;
#[doc = "Receive FIFO Control Register"]
pub mod rfcr;
#[doc = "RFPCR (w) register accessor: Receive FIFO Pointer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfpcr`] module"]
#[doc(alias = "RFPCR")]
pub type Rfpcr = crate::Reg<rfpcr::RfpcrSpec>;
#[doc = "Receive FIFO Pointer Control Register"]
pub mod rfpcr;
#[doc = "TFCR (rw) register accessor: Transmit FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfcr`] module"]
#[doc(alias = "TFCR")]
pub type Tfcr = crate::Reg<tfcr::TfcrSpec>;
#[doc = "Transmit FIFO Control Register"]
pub mod tfcr;
#[doc = "TFPCR (w) register accessor: Transmit FIFO Pointer Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfpcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfpcr`] module"]
#[doc(alias = "TFPCR")]
pub type Tfpcr = crate::Reg<tfpcr::TfpcrSpec>;
#[doc = "Transmit FIFO Pointer Control Register"]
pub mod tfpcr;
#[doc = "EIER (rw) register accessor: Error Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eier`] module"]
#[doc(alias = "EIER")]
pub type Eier = crate::Reg<eier::EierSpec>;
#[doc = "Error Interrupt Enable Register"]
pub mod eier;
#[doc = "EIFR (rw) register accessor: Error Interrupt Factor Judge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eifr`] module"]
#[doc(alias = "EIFR")]
pub type Eifr = crate::Reg<eifr::EifrSpec>;
#[doc = "Error Interrupt Factor Judge Register"]
pub mod eifr;
#[doc = "RECR (r) register accessor: Receive Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`recr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recr`] module"]
#[doc(alias = "RECR")]
pub type Recr = crate::Reg<recr::RecrSpec>;
#[doc = "Receive Error Count Register"]
pub mod recr;
#[doc = "TECR (r) register accessor: Transmit Error Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tecr`] module"]
#[doc(alias = "TECR")]
pub type Tecr = crate::Reg<tecr::TecrSpec>;
#[doc = "Transmit Error Count Register"]
pub mod tecr;
#[doc = "ECSR (rw) register accessor: Error Code Store Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecsr`] module"]
#[doc(alias = "ECSR")]
pub type Ecsr = crate::Reg<ecsr::EcsrSpec>;
#[doc = "Error Code Store Register"]
pub mod ecsr;
#[doc = "CSSR (rw) register accessor: Channel Search Support Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cssr`] module"]
#[doc(alias = "CSSR")]
pub type Cssr = crate::Reg<cssr::CssrSpec>;
#[doc = "Channel Search Support Register"]
pub mod cssr;
#[doc = "MSSR (r) register accessor: Mailbox Search Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mssr`] module"]
#[doc(alias = "MSSR")]
pub type Mssr = crate::Reg<mssr::MssrSpec>;
#[doc = "Mailbox Search Status Register"]
pub mod mssr;
#[doc = "MSMR (rw) register accessor: Mailbox Search Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msmr`] module"]
#[doc(alias = "MSMR")]
pub type Msmr = crate::Reg<msmr::MsmrSpec>;
#[doc = "Mailbox Search Mode Register"]
pub mod msmr;
#[doc = "TSR (r) register accessor: Time Stamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr`] module"]
#[doc(alias = "TSR")]
pub type Tsr = crate::Reg<tsr::TsrSpec>;
#[doc = "Time Stamp Register"]
pub mod tsr;
#[doc = "AFSR (rw) register accessor: Acceptance Filter Support Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afsr`] module"]
#[doc(alias = "AFSR")]
pub type Afsr = crate::Reg<afsr::AfsrSpec>;
#[doc = "Acceptance Filter Support Register"]
pub mod afsr;
#[doc = "TCR (rw) register accessor: Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Test Control Register"]
pub mod tcr;
