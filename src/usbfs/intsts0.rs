#[doc = "Register `INTSTS0` reader"]
pub type R = crate::R<Intsts0Spec>;
#[doc = "Register `INTSTS0` writer"]
pub type W = crate::W<Intsts0Spec>;
#[doc = "Control Transfer Stage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsq {
    #[doc = "0: Idle or setup stage"]
    _000 = 0,
    #[doc = "1: Control read data stage"]
    _001 = 1,
    #[doc = "2: Control read status stage"]
    _010 = 2,
    #[doc = "3: Control write data stage"]
    _011 = 3,
    #[doc = "4: Control write status stage"]
    _100 = 4,
    #[doc = "5: Control write (no data) status stage"]
    _101 = 5,
    #[doc = "6: Control transfer sequence error"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Ctsq> for u8 {
    #[inline(always)]
    fn from(variant: Ctsq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsq {
    type Ux = u8;
}
impl crate::IsEnum for Ctsq {}
#[doc = "Field `CTSQ` reader - Control Transfer Stage"]
pub type CtsqR = crate::FieldReader<Ctsq>;
impl CtsqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsq {
        match self.bits {
            0 => Ctsq::_000,
            1 => Ctsq::_001,
            2 => Ctsq::_010,
            3 => Ctsq::_011,
            4 => Ctsq::_100,
            5 => Ctsq::_101,
            6 => Ctsq::_110,
            7 => Ctsq::Others,
            _ => unreachable!(),
        }
    }
    #[doc = "Idle or setup stage"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Ctsq::_000
    }
    #[doc = "Control read data stage"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Ctsq::_001
    }
    #[doc = "Control read status stage"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Ctsq::_010
    }
    #[doc = "Control write data stage"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Ctsq::_011
    }
    #[doc = "Control write status stage"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Ctsq::_100
    }
    #[doc = "Control write (no data) status stage"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Ctsq::_101
    }
    #[doc = "Control transfer sequence error"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Ctsq::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        *self == Ctsq::Others
    }
}
#[doc = "USB Request Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Valid {
    #[doc = "0: Setup packet is not received"]
    _0 = 0,
    #[doc = "1: Setup packet is received"]
    _1 = 1,
}
impl From<Valid> for bool {
    #[inline(always)]
    fn from(variant: Valid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - USB Request Reception"]
pub type ValidR = crate::BitReader<Valid>;
impl ValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Valid {
        match self.bits {
            false => Valid::_0,
            true => Valid::_1,
        }
    }
    #[doc = "Setup packet is not received"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Valid::_0
    }
    #[doc = "Setup packet is received"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Valid::_1
    }
}
#[doc = "Field `VALID` writer - USB Request Reception"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG, Valid>;
impl<'a, REG> ValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setup packet is not received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::_0)
    }
    #[doc = "Setup packet is received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Valid::_1)
    }
}
#[doc = "Device State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvsq {
    #[doc = "0: Powered state"]
    _000 = 0,
    #[doc = "1: Default state"]
    _001 = 1,
    #[doc = "2: Address state"]
    _010 = 2,
    #[doc = "3: Configured state"]
    _011 = 3,
    #[doc = "4: Suspended state"]
    Others = 4,
}
impl From<Dvsq> for u8 {
    #[inline(always)]
    fn from(variant: Dvsq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvsq {
    type Ux = u8;
}
impl crate::IsEnum for Dvsq {}
#[doc = "Field `DVSQ` reader - Device State"]
pub type DvsqR = crate::FieldReader<Dvsq>;
impl DvsqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dvsq {
        match self.bits {
            0 => Dvsq::_000,
            1 => Dvsq::_001,
            2 => Dvsq::_010,
            3 => Dvsq::_011,
            _ => Dvsq::Others,
        }
    }
    #[doc = "Powered state"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Dvsq::_000
    }
    #[doc = "Default state"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Dvsq::_001
    }
    #[doc = "Address state"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Dvsq::_010
    }
    #[doc = "Configured state"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Dvsq::_011
    }
    #[doc = "Suspended state"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Dvsq::Others)
    }
}
#[doc = "VBUS Input Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbsts {
    #[doc = "0: USB_VBUS pin is low."]
    _0 = 0,
    #[doc = "1: USB_VBUS pin is high."]
    _1 = 1,
}
impl From<Vbsts> for bool {
    #[inline(always)]
    fn from(variant: Vbsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBSTS` reader - VBUS Input Status"]
pub type VbstsR = crate::BitReader<Vbsts>;
impl VbstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbsts {
        match self.bits {
            false => Vbsts::_0,
            true => Vbsts::_1,
        }
    }
    #[doc = "USB_VBUS pin is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbsts::_0
    }
    #[doc = "USB_VBUS pin is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbsts::_1
    }
}
#[doc = "Buffer Ready Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brdy {
    #[doc = "0: BRDY interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BRDY interrupts are generated."]
    _1 = 1,
}
impl From<Brdy> for bool {
    #[inline(always)]
    fn from(variant: Brdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRDY` reader - Buffer Ready Interrupt Status"]
pub type BrdyR = crate::BitReader<Brdy>;
impl BrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brdy {
        match self.bits {
            false => Brdy::_0,
            true => Brdy::_1,
        }
    }
    #[doc = "BRDY interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brdy::_0
    }
    #[doc = "BRDY interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brdy::_1
    }
}
#[doc = "Buffer Not Ready Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nrdy {
    #[doc = "0: NRDY interrupts are not generated."]
    _0 = 0,
    #[doc = "1: NRDY interrupts are generated."]
    _1 = 1,
}
impl From<Nrdy> for bool {
    #[inline(always)]
    fn from(variant: Nrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRDY` reader - Buffer Not Ready Interrupt Status"]
pub type NrdyR = crate::BitReader<Nrdy>;
impl NrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nrdy {
        match self.bits {
            false => Nrdy::_0,
            true => Nrdy::_1,
        }
    }
    #[doc = "NRDY interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nrdy::_0
    }
    #[doc = "NRDY interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nrdy::_1
    }
}
#[doc = "Buffer Empty Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bemp {
    #[doc = "0: BEMP interrupts are not generated."]
    _0 = 0,
    #[doc = "1: BEMP interrupts are generated."]
    _1 = 1,
}
impl From<Bemp> for bool {
    #[inline(always)]
    fn from(variant: Bemp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEMP` reader - Buffer Empty Interrupt Status"]
pub type BempR = crate::BitReader<Bemp>;
impl BempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bemp {
        match self.bits {
            false => Bemp::_0,
            true => Bemp::_1,
        }
    }
    #[doc = "BEMP interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bemp::_0
    }
    #[doc = "BEMP interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bemp::_1
    }
}
#[doc = "Control Transfer Stage Transition Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctrt {
    #[doc = "0: Control transfer stage transition interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Control transfer stage transition interrupts are generated."]
    _1 = 1,
}
impl From<Ctrt> for bool {
    #[inline(always)]
    fn from(variant: Ctrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRT` reader - Control Transfer Stage Transition Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type CtrtR = crate::BitReader<Ctrt>;
impl CtrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctrt {
        match self.bits {
            false => Ctrt::_0,
            true => Ctrt::_1,
        }
    }
    #[doc = "Control transfer stage transition interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctrt::_0
    }
    #[doc = "Control transfer stage transition interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctrt::_1
    }
}
#[doc = "Field `CTRT` writer - Control Transfer Stage Transition Interrupt Status"]
pub type CtrtW<'a, REG> = crate::BitWriter0C<'a, REG, Ctrt>;
impl<'a, REG> CtrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control transfer stage transition interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrt::_0)
    }
    #[doc = "Control transfer stage transition interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctrt::_1)
    }
}
#[doc = "Device State Transition Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dvst {
    #[doc = "0: Device state transition interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Device state transition interrupts are generated."]
    _1 = 1,
}
impl From<Dvst> for bool {
    #[inline(always)]
    fn from(variant: Dvst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVST` reader - Device State Transition Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type DvstR = crate::BitReader<Dvst>;
impl DvstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dvst {
        match self.bits {
            false => Dvst::_0,
            true => Dvst::_1,
        }
    }
    #[doc = "Device state transition interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dvst::_0
    }
    #[doc = "Device state transition interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dvst::_1
    }
}
#[doc = "Field `DVST` writer - Device State Transition Interrupt Status"]
pub type DvstW<'a, REG> = crate::BitWriter0C<'a, REG, Dvst>;
impl<'a, REG> DvstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device state transition interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dvst::_0)
    }
    #[doc = "Device state transition interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvst::_1)
    }
}
#[doc = "Frame Number Refresh Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sofr {
    #[doc = "0: SOF interrupts are not generated."]
    _0 = 0,
    #[doc = "1: SOF interrupts are generated."]
    _1 = 1,
}
impl From<Sofr> for bool {
    #[inline(always)]
    fn from(variant: Sofr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFR` reader - Frame Number Refresh Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SofrR = crate::BitReader<Sofr>;
impl SofrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sofr {
        match self.bits {
            false => Sofr::_0,
            true => Sofr::_1,
        }
    }
    #[doc = "SOF interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sofr::_0
    }
    #[doc = "SOF interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sofr::_1
    }
}
#[doc = "Field `SOFR` writer - Frame Number Refresh Interrupt Status"]
pub type SofrW<'a, REG> = crate::BitWriter0C<'a, REG, Sofr>;
impl<'a, REG> SofrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sofr::_0)
    }
    #[doc = "SOF interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sofr::_1)
    }
}
#[doc = "Resume Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resm {
    #[doc = "0: Resume interrupts are not generated."]
    _0 = 0,
    #[doc = "1: Resume interrupts are generated."]
    _1 = 1,
}
impl From<Resm> for bool {
    #[inline(always)]
    fn from(variant: Resm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESM` reader - Resume Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type ResmR = crate::BitReader<Resm>;
impl ResmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resm {
        match self.bits {
            false => Resm::_0,
            true => Resm::_1,
        }
    }
    #[doc = "Resume interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Resm::_0
    }
    #[doc = "Resume interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Resm::_1
    }
}
#[doc = "Field `RESM` writer - Resume Interrupt Status"]
pub type ResmW<'a, REG> = crate::BitWriter0C<'a, REG, Resm>;
impl<'a, REG> ResmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Resm::_0)
    }
    #[doc = "Resume interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Resm::_1)
    }
}
#[doc = "VBUS Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbint {
    #[doc = "0: VBUS interrupts are not generated."]
    _0 = 0,
    #[doc = "1: VBUS interrupts are generated."]
    _1 = 1,
}
impl From<Vbint> for bool {
    #[inline(always)]
    fn from(variant: Vbint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBINT` reader - VBUS Interrupt Status\n\n<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type VbintR = crate::BitReader<Vbint>;
impl VbintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbint {
        match self.bits {
            false => Vbint::_0,
            true => Vbint::_1,
        }
    }
    #[doc = "VBUS interrupts are not generated."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbint::_0
    }
    #[doc = "VBUS interrupts are generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbint::_1
    }
}
#[doc = "Field `VBINT` writer - VBUS Interrupt Status"]
pub type VbintW<'a, REG> = crate::BitWriter0C<'a, REG, Vbint>;
impl<'a, REG> VbintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBUS interrupts are not generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbint::_0)
    }
    #[doc = "VBUS interrupts are generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbint::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Control Transfer Stage"]
    #[inline(always)]
    pub fn ctsq(&self) -> CtsqR {
        CtsqR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - USB Request Reception"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Device State"]
    #[inline(always)]
    pub fn dvsq(&self) -> DvsqR {
        DvsqR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - VBUS Input Status"]
    #[inline(always)]
    pub fn vbsts(&self) -> VbstsR {
        VbstsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffer Ready Interrupt Status"]
    #[inline(always)]
    pub fn brdy(&self) -> BrdyR {
        BrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer Not Ready Interrupt Status"]
    #[inline(always)]
    pub fn nrdy(&self) -> NrdyR {
        NrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Empty Interrupt Status"]
    #[inline(always)]
    pub fn bemp(&self) -> BempR {
        BempR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    pub fn ctrt(&self) -> CtrtR {
        CtrtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Status"]
    #[inline(always)]
    pub fn dvst(&self) -> DvstR {
        DvstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    pub fn sofr(&self) -> SofrR {
        SofrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resume Interrupt Status"]
    #[inline(always)]
    pub fn resm(&self) -> ResmR {
        ResmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VBUS Interrupt Status"]
    #[inline(always)]
    pub fn vbint(&self) -> VbintR {
        VbintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS0")
            .field("bemp", &self.bemp())
            .field("nrdy", &self.nrdy())
            .field("brdy", &self.brdy())
            .field("vbsts", &self.vbsts())
            .field("dvsq", &self.dvsq())
            .field("valid", &self.valid())
            .field("ctsq", &self.ctsq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - USB Request Reception"]
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<Intsts0Spec> {
        ValidW::new(self, 3)
    }
    #[doc = "Bit 11 - Control Transfer Stage Transition Interrupt Status"]
    #[inline(always)]
    pub fn ctrt(&mut self) -> CtrtW<Intsts0Spec> {
        CtrtW::new(self, 11)
    }
    #[doc = "Bit 12 - Device State Transition Interrupt Status"]
    #[inline(always)]
    pub fn dvst(&mut self) -> DvstW<Intsts0Spec> {
        DvstW::new(self, 12)
    }
    #[doc = "Bit 13 - Frame Number Refresh Interrupt Status"]
    #[inline(always)]
    pub fn sofr(&mut self) -> SofrW<Intsts0Spec> {
        SofrW::new(self, 13)
    }
    #[doc = "Bit 14 - Resume Interrupt Status"]
    #[inline(always)]
    pub fn resm(&mut self) -> ResmW<Intsts0Spec> {
        ResmW::new(self, 14)
    }
    #[doc = "Bit 15 - VBUS Interrupt Status"]
    #[inline(always)]
    pub fn vbint(&mut self) -> VbintW<Intsts0Spec> {
        VbintW::new(self, 15)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intsts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intsts0Spec;
impl crate::RegisterSpec for Intsts0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intsts0::R`](R) reader structure"]
impl crate::Readable for Intsts0Spec {}
#[doc = "`write(|w| ..)` method takes [`intsts0::W`](W) writer structure"]
impl crate::Writable for Intsts0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xf800;
}
#[doc = "`reset()` method sets INTSTS0 to value 0"]
impl crate::Resettable for Intsts0Spec {}
