#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_p000: [u8; 0x04],
    _reserved_1_p001: [u8; 0x04],
    _reserved_2_p002: [u8; 0x04],
    _reserved_3_p003: [u8; 0x04],
    _reserved_4_p004: [u8; 0x04],
    _reserved_5_p005: [u8; 0x04],
    _reserved_6_p006: [u8; 0x04],
    _reserved_7_p007: [u8; 0x04],
    _reserved_8_p008: [u8; 0x04],
    _reserved9: [u8; 0x04],
    _reserved_9_p010: [u8; 0x04],
    _reserved_10_p011: [u8; 0x04],
    _reserved_11_p012: [u8; 0x04],
    _reserved_12_p013: [u8; 0x04],
    _reserved_13_p014: [u8; 0x04],
    _reserved_14_p015: [u8; 0x04],
    _reserved_15_p100: [u8; 0x04],
    _reserved_16_p101: [u8; 0x04],
    _reserved_17_p102: [u8; 0x04],
    _reserved_18_p103: [u8; 0x04],
    _reserved_19_p104: [u8; 0x04],
    _reserved_20_p105: [u8; 0x04],
    _reserved_21_p106: [u8; 0x04],
    _reserved_22_p107: [u8; 0x04],
    _reserved_23_p108: [u8; 0x04],
    _reserved_24_p109: [u8; 0x04],
    _reserved_25_p110: [u8; 0x04],
    _reserved_26_p111: [u8; 0x04],
    _reserved_27_p112: [u8; 0x04],
    _reserved_28_p113: [u8; 0x04],
    _reserved_29_p114: [u8; 0x04],
    _reserved_30_p115: [u8; 0x04],
    _reserved_31_p200: [u8; 0x04],
    _reserved_32_p201: [u8; 0x04],
    _reserved_33_p202: [u8; 0x04],
    _reserved_34_p203: [u8; 0x04],
    _reserved_35_p204: [u8; 0x04],
    _reserved_36_p205: [u8; 0x04],
    _reserved_37_p206: [u8; 0x04],
    _reserved38: [u8; 0x14],
    _reserved_38_p212: [u8; 0x04],
    _reserved_39_p213: [u8; 0x04],
    _reserved_40_p214: [u8; 0x04],
    _reserved_41_p215: [u8; 0x04],
    _reserved_42_p300: [u8; 0x04],
    _reserved_43_p301: [u8; 0x04],
    _reserved_44_p302: [u8; 0x04],
    _reserved_45_p303: [u8; 0x04],
    _reserved_46_p304: [u8; 0x04],
    _reserved_47_p305: [u8; 0x04],
    _reserved_48_p306: [u8; 0x04],
    _reserved_49_p307: [u8; 0x04],
    _reserved50: [u8; 0x20],
    _reserved_50_p400: [u8; 0x04],
    _reserved_51_p401: [u8; 0x04],
    _reserved_52_p402: [u8; 0x04],
    _reserved_53_p403: [u8; 0x04],
    _reserved_54_p404: [u8; 0x04],
    _reserved_55_p405: [u8; 0x04],
    _reserved_56_p406: [u8; 0x04],
    _reserved_57_p407: [u8; 0x04],
    _reserved_58_p408: [u8; 0x04],
    _reserved_59_p409: [u8; 0x04],
    _reserved_60_p410: [u8; 0x04],
    _reserved_61_p411: [u8; 0x04],
    _reserved_62_p412: [u8; 0x04],
    _reserved_63_p413: [u8; 0x04],
    _reserved_64_p414: [u8; 0x04],
    _reserved_65_p415: [u8; 0x04],
    _reserved_66_p500: [u8; 0x04],
    _reserved_67_p501: [u8; 0x04],
    _reserved_68_p502: [u8; 0x04],
    _reserved_69_p503: [u8; 0x04],
    _reserved_70_p504: [u8; 0x04],
    _reserved_71_p505: [u8; 0x04],
    _reserved72: [u8; 0x28],
    _reserved_72_p600: [u8; 0x04],
    _reserved_73_p601: [u8; 0x04],
    _reserved_74_p602: [u8; 0x04],
    _reserved_75_p603: [u8; 0x04],
    _reserved76: [u8; 0x10],
    _reserved_76_p608: [u8; 0x04],
    _reserved_77_p609: [u8; 0x04],
    _reserved_78_p610: [u8; 0x04],
    _reserved79: [u8; 0x34],
    _reserved_79_p708: [u8; 0x04],
    _reserved80: [u8; 0x3c],
    _reserved_80_p808: [u8; 0x04],
    _reserved_81_p809: [u8; 0x04],
    _reserved82: [u8; 0x50],
    _reserved_82_p914: [u8; 0x04],
    _reserved_83_p915: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P000pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(&self) -> &P000pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_by(&self) -> &P000pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(3).cast() }
    }
    #[doc = "0x04 - P001 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P001 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P001 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p001pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P002 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P002 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P002 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p002pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P003 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P003 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P003 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p003pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P004 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P004 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P004 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p004pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P005 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P005 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P005 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p005pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P006 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P006 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P006 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p006pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P007 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P007 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P007 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p007pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x04 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P001pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x06 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_ha(&self) -> &P001pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x07 - P008 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p008pfs_by(&self) -> &P001pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(7).cast() }
    }
    #[doc = "0x28 - P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x28 - P011 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P011 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P011 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p011pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x28 - P012 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P012 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P012 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p012pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x28 - P013 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P013 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P013 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p013pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x28 - P014 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P014 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P014 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p014pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x28 - P015 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P010pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2a - P015 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_ha(&self) -> &P010pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - P015 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p015pfs_by(&self) -> &P010pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x40 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P101 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P101 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P101 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p101pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P102 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P102 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P102 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p102pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P103 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P103 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P103 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p103pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P104 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P104 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P104 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p104pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P105 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P105 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P105 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p105pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P106 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P106 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P106 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p106pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x40 - P107 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P100pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x42 - P107 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_ha(&self) -> &P100pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - P107 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p107pfs_by(&self) -> &P100pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x60 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P108pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x62 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(&self) -> &P108pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(98).cast() }
    }
    #[doc = "0x63 - P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_by(&self) -> &P108pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(99).cast() }
    }
    #[doc = "0x64 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P109pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x66 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(&self) -> &P109pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(102).cast() }
    }
    #[doc = "0x67 - P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_by(&self) -> &P109pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(103).cast() }
    }
    #[doc = "0x68 - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P110pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(104).cast() }
    }
    #[doc = "0x6a - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(&self) -> &P110pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(106).cast() }
    }
    #[doc = "0x6b - P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_by(&self) -> &P110pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(107).cast() }
    }
    #[doc = "0x6c - P111 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P111pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6e - P111 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_ha(&self) -> &P111pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    #[doc = "0x6f - P111 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p111pfs_by(&self) -> &P111pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x6c - P112 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P111pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6e - P112 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_ha(&self) -> &P111pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    #[doc = "0x6f - P112 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p112pfs_by(&self) -> &P111pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x6c - P113 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs(&self) -> &P111pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6e - P113 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs_ha(&self) -> &P111pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    #[doc = "0x6f - P113 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p113pfs_by(&self) -> &P111pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x6c - P114 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs(&self) -> &P111pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6e - P114 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs_ha(&self) -> &P111pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    #[doc = "0x6f - P114 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p114pfs_by(&self) -> &P111pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x6c - P115 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs(&self) -> &P111pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6e - P115 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs_ha(&self) -> &P111pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(110).cast() }
    }
    #[doc = "0x6f - P115 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p115pfs_by(&self) -> &P111pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(111).cast() }
    }
    #[doc = "0x80 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x82 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(&self) -> &P200pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(130).cast() }
    }
    #[doc = "0x83 - P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_by(&self) -> &P200pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(131).cast() }
    }
    #[doc = "0x84 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x86 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(&self) -> &P201pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(134).cast() }
    }
    #[doc = "0x87 - P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_by(&self) -> &P201pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(135).cast() }
    }
    #[doc = "0x88 - P202 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs(&self) -> &P202pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8a - P202 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs_ha(&self) -> &P202pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - P202 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p202pfs_by(&self) -> &P202pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0x88 - P203 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs(&self) -> &P202pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8a - P203 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs_ha(&self) -> &P202pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - P203 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p203pfs_by(&self) -> &P202pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0x88 - P204 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P202pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8a - P204 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_ha(&self) -> &P202pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - P204 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p204pfs_by(&self) -> &P202pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0x88 - P205 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P202pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8a - P205 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_ha(&self) -> &P202pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - P205 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p205pfs_by(&self) -> &P202pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0x88 - P206 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P202pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8a - P206 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_ha(&self) -> &P202pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(138).cast() }
    }
    #[doc = "0x8b - P206 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p206pfs_by(&self) -> &P202pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(139).cast() }
    }
    #[doc = "0xb0 - P212 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P212pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb2 - P212 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_ha(&self) -> &P212pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    #[doc = "0xb3 - P212 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p212pfs_by(&self) -> &P212pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    #[doc = "0xb0 - P213 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P212pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb2 - P213 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_ha(&self) -> &P212pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    #[doc = "0xb3 - P213 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p213pfs_by(&self) -> &P212pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    #[doc = "0xb0 - P214 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P212pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb2 - P214 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_ha(&self) -> &P212pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    #[doc = "0xb3 - P214 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p214pfs_by(&self) -> &P212pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    #[doc = "0xb0 - P215 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs(&self) -> &P212pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb2 - P215 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs_ha(&self) -> &P212pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(178).cast() }
    }
    #[doc = "0xb3 - P215 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p215pfs_by(&self) -> &P212pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(179).cast() }
    }
    #[doc = "0xc0 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc2 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(&self) -> &P300pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(194).cast() }
    }
    #[doc = "0xc3 - P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_by(&self) -> &P300pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(195).cast() }
    }
    #[doc = "0xc4 - P301 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P301 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P301 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p301pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P302 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P302 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P302 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p302pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P303 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P303 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P303 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p303pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P304 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P304 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P304 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p304pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P305 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P305 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P305 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p305pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P306 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P306 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P306 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p306pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0xc4 - P307 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs(&self) -> &P301pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc6 - P307 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs_ha(&self) -> &P301pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(198).cast() }
    }
    #[doc = "0xc7 - P307 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p307pfs_by(&self) -> &P301pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(199).cast() }
    }
    #[doc = "0x100 - P400 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P400 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P400 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p400pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P401 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p401pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P401 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p401pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P401 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p401pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P402 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p402pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P402 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p402pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P402 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p402pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P403 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p403pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P403 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p403pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P403 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p403pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P404 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p404pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P404 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p404pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P404 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p404pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P405 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p405pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P405 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p405pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P405 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p405pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P406 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p406pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P406 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p406pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P406 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p406pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x100 - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs(&self) -> &P400pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x102 - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_ha(&self) -> &P400pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(258).cast() }
    }
    #[doc = "0x103 - P407 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p407pfs_by(&self) -> &P400pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(259).cast() }
    }
    #[doc = "0x120 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P408pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x122 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_ha(&self) -> &P408pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(290).cast() }
    }
    #[doc = "0x123 - P408 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p408pfs_by(&self) -> &P408pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(291).cast() }
    }
    #[doc = "0x124 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P409pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x126 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_ha(&self) -> &P409pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(294).cast() }
    }
    #[doc = "0x127 - P409 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p409pfs_by(&self) -> &P409pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(295).cast() }
    }
    #[doc = "0x128 - P410 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P410 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P410 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p410pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x128 - P411 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P411 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P411 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p411pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x128 - P412 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P412 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P412 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p412pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x128 - P413 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P413 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P413 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p413pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x128 - P414 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P414 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P414 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p414pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x128 - P415 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs(&self) -> &P410pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12a - P415 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs_ha(&self) -> &P410pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(298).cast() }
    }
    #[doc = "0x12b - P415 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p415pfs_by(&self) -> &P410pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x140 - P500 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P500 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P500 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p500pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x140 - P501 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p501pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P501 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p501pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P501 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p501pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x140 - P502 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p502pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P502 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p502pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P502 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p502pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x140 - P503 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p503pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P503 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p503pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P503 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p503pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x140 - P504 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p504pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P504 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p504pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P504 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p504pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x140 - P505 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p505pfs(&self) -> &P500pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x142 - P505 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p505pfs_ha(&self) -> &P500pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(322).cast() }
    }
    #[doc = "0x143 - P505 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p505pfs_by(&self) -> &P500pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(323).cast() }
    }
    #[doc = "0x180 - P600 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs(&self) -> &P600pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x182 - P600 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs_ha(&self) -> &P600pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    #[doc = "0x183 - P600 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p600pfs_by(&self) -> &P600pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    #[doc = "0x180 - P601 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p601pfs(&self) -> &P600pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x182 - P601 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p601pfs_ha(&self) -> &P600pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    #[doc = "0x183 - P601 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p601pfs_by(&self) -> &P600pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    #[doc = "0x180 - P602 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p602pfs(&self) -> &P600pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x182 - P602 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p602pfs_ha(&self) -> &P600pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    #[doc = "0x183 - P602 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p602pfs_by(&self) -> &P600pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    #[doc = "0x180 - P603 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p603pfs(&self) -> &P600pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x182 - P603 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p603pfs_ha(&self) -> &P600pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(386).cast() }
    }
    #[doc = "0x183 - P603 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p603pfs_by(&self) -> &P600pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(387).cast() }
    }
    #[doc = "0x1a0 - P608 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P608pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a2 - P608 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs_ha(&self) -> &P608pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(418).cast() }
    }
    #[doc = "0x1a3 - P608 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p608pfs_by(&self) -> &P608pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(419).cast() }
    }
    #[doc = "0x1a0 - P609 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P608pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a2 - P609 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs_ha(&self) -> &P608pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(418).cast() }
    }
    #[doc = "0x1a3 - P609 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p609pfs_by(&self) -> &P608pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(419).cast() }
    }
    #[doc = "0x1a8 - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs(&self) -> &P610pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1aa - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_ha(&self) -> &P610pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(426).cast() }
    }
    #[doc = "0x1ab - P610 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p610pfs_by(&self) -> &P610pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(427).cast() }
    }
    #[doc = "0x1e0 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P708pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e2 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_ha(&self) -> &P708pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(482).cast() }
    }
    #[doc = "0x1e3 - P708 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p708pfs_by(&self) -> &P708pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(483).cast() }
    }
    #[doc = "0x220 - P808 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p808pfs(&self) -> &P808pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x222 - P808 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p808pfs_ha(&self) -> &P808pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(546).cast() }
    }
    #[doc = "0x223 - P808 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p808pfs_by(&self) -> &P808pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(547).cast() }
    }
    #[doc = "0x220 - P809 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p809pfs(&self) -> &P808pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x222 - P809 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p809pfs_ha(&self) -> &P808pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(546).cast() }
    }
    #[doc = "0x223 - P809 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p809pfs_by(&self) -> &P808pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(547).cast() }
    }
    #[doc = "0x278 - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs(&self) -> &P914pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27a - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_ha(&self) -> &P914pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(634).cast() }
    }
    #[doc = "0x27b - P914 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p914pfs_by(&self) -> &P914pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(635).cast() }
    }
    #[doc = "0x278 - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs(&self) -> &P914pfs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27a - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_ha(&self) -> &P914pfsHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(634).cast() }
    }
    #[doc = "0x27b - P915 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p915pfs_by(&self) -> &P914pfsBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(635).cast() }
    }
}
#[doc = "P000PFS (rw) register accessor: P00%s Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p000pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p000pfs`] module"]
#[doc(alias = "P000PFS")]
pub type P000pfs = crate::Reg<p000pfs::P000pfsSpec>;
#[doc = "P00%s Pin Function Control Register"]
pub mod p000pfs;
#[doc = "P000PFS_HA (rw) register accessor: P00%s Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p000pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p000pfs_ha`] module"]
#[doc(alias = "P000PFS_HA")]
pub type P000pfsHa = crate::Reg<p000pfs_ha::P000pfsHaSpec>;
#[doc = "P00%s Pin Function Control Register"]
pub mod p000pfs_ha;
#[doc = "P000PFS_BY (rw) register accessor: P00%s Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p000pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p000pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p000pfs_by`] module"]
#[doc(alias = "P000PFS_BY")]
pub type P000pfsBy = crate::Reg<p000pfs_by::P000pfsBySpec>;
#[doc = "P00%s Pin Function Control Register"]
pub mod p000pfs_by;
pub use P000pfs as P001pfs;
pub use P000pfs as P010pfs;
pub use P000pfs as P100pfs;
pub use P000pfsBy as P001pfsBy;
pub use P000pfsBy as P010pfsBy;
pub use P000pfsBy as P100pfsBy;
pub use P000pfsHa as P001pfsHa;
pub use P000pfsHa as P010pfsHa;
pub use P000pfsHa as P100pfsHa;
pub use p000pfs as p001pfs;
pub use p000pfs as p010pfs;
pub use p000pfs as p100pfs;
pub use p000pfs_by as p001pfs_by;
pub use p000pfs_by as p010pfs_by;
pub use p000pfs_by as p100pfs_by;
pub use p000pfs_ha as p001pfs_ha;
pub use p000pfs_ha as p010pfs_ha;
pub use p000pfs_ha as p100pfs_ha;
#[doc = "P108PFS (rw) register accessor: P108 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p108pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p108pfs`] module"]
#[doc(alias = "P108PFS")]
pub type P108pfs = crate::Reg<p108pfs::P108pfsSpec>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs;
#[doc = "P108PFS_HA (rw) register accessor: P108 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p108pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p108pfs_ha`] module"]
#[doc(alias = "P108PFS_HA")]
pub type P108pfsHa = crate::Reg<p108pfs_ha::P108pfsHaSpec>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs_ha;
#[doc = "P108PFS_BY (rw) register accessor: P108 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p108pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p108pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p108pfs_by`] module"]
#[doc(alias = "P108PFS_BY")]
pub type P108pfsBy = crate::Reg<p108pfs_by::P108pfsBySpec>;
#[doc = "P108 Pin Function Control Register"]
pub mod p108pfs_by;
#[doc = "P109PFS (rw) register accessor: P109 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p109pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p109pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p109pfs`] module"]
#[doc(alias = "P109PFS")]
pub type P109pfs = crate::Reg<p109pfs::P109pfsSpec>;
#[doc = "P109 Pin Function Control Register"]
pub mod p109pfs;
#[doc = "P109PFS_HA (rw) register accessor: P109 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p109pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p109pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p109pfs_ha`] module"]
#[doc(alias = "P109PFS_HA")]
pub type P109pfsHa = crate::Reg<p109pfs_ha::P109pfsHaSpec>;
#[doc = "P109 Pin Function Control Register"]
pub mod p109pfs_ha;
pub use P000pfs as P111pfs;
pub use P000pfs as P200pfs;
pub use P000pfsBy as P109pfsBy;
pub use P000pfsBy as P111pfsBy;
pub use P000pfsBy as P200pfsBy;
pub use P000pfsHa as P111pfsHa;
pub use P000pfsHa as P200pfsHa;
pub use P108pfs as P110pfs;
pub use P108pfsBy as P110pfsBy;
pub use P108pfsHa as P110pfsHa;
pub use p000pfs as p111pfs;
pub use p000pfs as p200pfs;
pub use p000pfs_by as p109pfs_by;
pub use p000pfs_by as p111pfs_by;
pub use p000pfs_by as p200pfs_by;
pub use p000pfs_ha as p111pfs_ha;
pub use p000pfs_ha as p200pfs_ha;
pub use p108pfs as p110pfs;
pub use p108pfs_by as p110pfs_by;
pub use p108pfs_ha as p110pfs_ha;
#[doc = "P201PFS (rw) register accessor: P201 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p201pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p201pfs`] module"]
#[doc(alias = "P201PFS")]
pub type P201pfs = crate::Reg<p201pfs::P201pfsSpec>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs;
#[doc = "P201PFS_HA (rw) register accessor: P201 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p201pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p201pfs_ha`] module"]
#[doc(alias = "P201PFS_HA")]
pub type P201pfsHa = crate::Reg<p201pfs_ha::P201pfsHaSpec>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs_ha;
#[doc = "P201PFS_BY (rw) register accessor: P201 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p201pfs_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p201pfs_by`] module"]
#[doc(alias = "P201PFS_BY")]
pub type P201pfsBy = crate::Reg<p201pfs_by::P201pfsBySpec>;
#[doc = "P201 Pin Function Control Register"]
pub mod p201pfs_by;
pub use P000pfs as P202pfs;
pub use P000pfs as P212pfs;
pub use P000pfs as P301pfs;
pub use P000pfs as P400pfs;
pub use P000pfsBy as P202pfsBy;
pub use P000pfsBy as P212pfsBy;
pub use P000pfsBy as P301pfsBy;
pub use P000pfsBy as P400pfsBy;
pub use P000pfsHa as P202pfsHa;
pub use P000pfsHa as P212pfsHa;
pub use P000pfsHa as P301pfsHa;
pub use P000pfsHa as P400pfsHa;
pub use P108pfs as P300pfs;
pub use P108pfsBy as P300pfsBy;
pub use P108pfsHa as P300pfsHa;
pub use p000pfs as p202pfs;
pub use p000pfs as p212pfs;
pub use p000pfs as p301pfs;
pub use p000pfs as p400pfs;
pub use p000pfs_by as p202pfs_by;
pub use p000pfs_by as p212pfs_by;
pub use p000pfs_by as p301pfs_by;
pub use p000pfs_by as p400pfs_by;
pub use p000pfs_ha as p202pfs_ha;
pub use p000pfs_ha as p212pfs_ha;
pub use p000pfs_ha as p301pfs_ha;
pub use p000pfs_ha as p400pfs_ha;
pub use p108pfs as p300pfs;
pub use p108pfs_by as p300pfs_by;
pub use p108pfs_ha as p300pfs_ha;
#[doc = "P408PFS (rw) register accessor: P408 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p408pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p408pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p408pfs`] module"]
#[doc(alias = "P408PFS")]
pub type P408pfs = crate::Reg<p408pfs::P408pfsSpec>;
#[doc = "P408 Pin Function Control Register"]
pub mod p408pfs;
#[doc = "P408PFS_HA (rw) register accessor: P408 Pin Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p408pfs_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p408pfs_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p408pfs_ha`] module"]
#[doc(alias = "P408PFS_HA")]
pub type P408pfsHa = crate::Reg<p408pfs_ha::P408pfsHaSpec>;
#[doc = "P408 Pin Function Control Register"]
pub mod p408pfs_ha;
pub use P000pfs as P409pfs;
pub use P000pfs as P410pfs;
pub use P000pfs as P500pfs;
pub use P000pfs as P600pfs;
pub use P000pfs as P608pfs;
pub use P000pfs as P610pfs;
pub use P000pfs as P708pfs;
pub use P000pfs as P808pfs;
pub use P000pfsBy as P408pfsBy;
pub use P000pfsBy as P409pfsBy;
pub use P000pfsBy as P410pfsBy;
pub use P000pfsBy as P500pfsBy;
pub use P000pfsBy as P600pfsBy;
pub use P000pfsBy as P608pfsBy;
pub use P000pfsBy as P610pfsBy;
pub use P000pfsBy as P708pfsBy;
pub use P000pfsBy as P808pfsBy;
pub use P000pfsHa as P409pfsHa;
pub use P000pfsHa as P410pfsHa;
pub use P000pfsHa as P500pfsHa;
pub use P000pfsHa as P600pfsHa;
pub use P000pfsHa as P608pfsHa;
pub use P000pfsHa as P610pfsHa;
pub use P000pfsHa as P708pfsHa;
pub use P000pfsHa as P808pfsHa;
pub use P109pfs as P914pfs;
pub use P109pfsBy as P914pfsBy;
pub use P109pfsHa as P914pfsHa;
pub use p000pfs as p409pfs;
pub use p000pfs as p410pfs;
pub use p000pfs as p500pfs;
pub use p000pfs as p600pfs;
pub use p000pfs as p608pfs;
pub use p000pfs as p610pfs;
pub use p000pfs as p708pfs;
pub use p000pfs as p808pfs;
pub use p000pfs_by as p408pfs_by;
pub use p000pfs_by as p409pfs_by;
pub use p000pfs_by as p410pfs_by;
pub use p000pfs_by as p500pfs_by;
pub use p000pfs_by as p600pfs_by;
pub use p000pfs_by as p608pfs_by;
pub use p000pfs_by as p610pfs_by;
pub use p000pfs_by as p708pfs_by;
pub use p000pfs_by as p808pfs_by;
pub use p000pfs_ha as p409pfs_ha;
pub use p000pfs_ha as p410pfs_ha;
pub use p000pfs_ha as p500pfs_ha;
pub use p000pfs_ha as p600pfs_ha;
pub use p000pfs_ha as p608pfs_ha;
pub use p000pfs_ha as p610pfs_ha;
pub use p000pfs_ha as p708pfs_ha;
pub use p000pfs_ha as p808pfs_ha;
pub use p109pfs as p914pfs;
pub use p109pfs_by as p914pfs_by;
pub use p109pfs_ha as p914pfs_ha;
