#[doc = "Register `SYSSTS0` reader"]
pub type R = crate::R<Syssts0Spec>;
#[doc = "USB Data Line Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lnst {
    #[doc = "0: SE0"]
    _00 = 0,
    #[doc = "1: K-State (FS) / J-State(LS)"]
    _01 = 1,
    #[doc = "2: J-State(FS) / K-State(LS)"]
    _10 = 2,
    #[doc = "3: SE1"]
    _11 = 3,
}
impl From<Lnst> for u8 {
    #[inline(always)]
    fn from(variant: Lnst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lnst {
    type Ux = u8;
}
impl crate::IsEnum for Lnst {}
#[doc = "Field `LNST` reader - USB Data Line Status Monitor"]
pub type LnstR = crate::FieldReader<Lnst>;
impl LnstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lnst {
        match self.bits {
            0 => Lnst::_00,
            1 => Lnst::_01,
            2 => Lnst::_10,
            3 => Lnst::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "SE0"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Lnst::_00
    }
    #[doc = "K-State (FS) / J-State(LS)"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Lnst::_01
    }
    #[doc = "J-State(FS) / K-State(LS)"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Lnst::_10
    }
    #[doc = "SE1"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Lnst::_11
    }
}
#[doc = "External ID0 Input Pin Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idmon {
    #[doc = "0: USB0_ID pin is low"]
    _0 = 0,
    #[doc = "1: USB0_ID pin is high"]
    _1 = 1,
}
impl From<Idmon> for bool {
    #[inline(always)]
    fn from(variant: Idmon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDMON` reader - External ID0 Input Pin Monitor"]
pub type IdmonR = crate::BitReader<Idmon>;
impl IdmonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idmon {
        match self.bits {
            false => Idmon::_0,
            true => Idmon::_1,
        }
    }
    #[doc = "USB0_ID pin is low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idmon::_0
    }
    #[doc = "USB0_ID pin is high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idmon::_1
    }
}
#[doc = "USB Host Sequencer Status Monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Htact {
    #[doc = "0: Host sequencer completely stopped"]
    _0 = 0,
    #[doc = "1: Host sequencer not completely stopped."]
    _1 = 1,
}
impl From<Htact> for bool {
    #[inline(always)]
    fn from(variant: Htact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTACT` reader - USB Host Sequencer Status Monitor"]
pub type HtactR = crate::BitReader<Htact>;
impl HtactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Htact {
        match self.bits {
            false => Htact::_0,
            true => Htact::_1,
        }
    }
    #[doc = "Host sequencer completely stopped"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Htact::_0
    }
    #[doc = "Host sequencer not completely stopped."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Htact::_1
    }
}
#[doc = "Field `OVCMON` reader - External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\] bit indicates the status of the USBHS_OVRCURB pin."]
pub type OvcmonR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - USB Data Line Status Monitor"]
    #[inline(always)]
    pub fn lnst(&self) -> LnstR {
        LnstR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External ID0 Input Pin Monitor"]
    #[inline(always)]
    pub fn idmon(&self) -> IdmonR {
        IdmonR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Host Sequencer Status Monitor"]
    #[inline(always)]
    pub fn htact(&self) -> HtactR {
        HtactR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 14:15 - External USB0_OVRCURA/ USB0_OVRCURB Input Pin Monitor The OCVMON\\[1\\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\] bit indicates the status of the USBHS_OVRCURB pin."]
    #[inline(always)]
    pub fn ovcmon(&self) -> OvcmonR {
        OvcmonR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSSTS0")
            .field("ovcmon", &self.ovcmon())
            .field("htact", &self.htact())
            .field("idmon", &self.idmon())
            .field("lnst", &self.lnst())
            .finish()
    }
}
#[doc = "System Configuration Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`syssts0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syssts0Spec;
impl crate::RegisterSpec for Syssts0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syssts0::R`](R) reader structure"]
impl crate::Readable for Syssts0Spec {}
#[doc = "`reset()` method sets SYSSTS0 to value 0"]
impl crate::Resettable for Syssts0Spec {}
