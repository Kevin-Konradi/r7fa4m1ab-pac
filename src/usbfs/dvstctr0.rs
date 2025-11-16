#[doc = "Register `DVSTCTR0` reader"]
pub type R = crate::R<Dvstctr0Spec>;
#[doc = "Register `DVSTCTR0` writer"]
pub type W = crate::W<Dvstctr0Spec>;
#[doc = "USB Bus Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rhst {
    #[doc = "0: Communication speed not determined"]
    _000 = 0,
    #[doc = "1: Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    _001 = 1,
    #[doc = "2: Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    _010 = 2,
    #[doc = "3: Setting prohibited"]
    _011 = 3,
    #[doc = "4: USB bus reset in progress(When the host controller function is selected)"]
    Others = 4,
}
impl From<Rhst> for u8 {
    #[inline(always)]
    fn from(variant: Rhst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rhst {
    type Ux = u8;
}
impl crate::IsEnum for Rhst {}
#[doc = "Field `RHST` reader - USB Bus Reset Status"]
pub type RhstR = crate::FieldReader<Rhst>;
impl RhstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rhst {
        match self.bits {
            0 => Rhst::_000,
            1 => Rhst::_001,
            2 => Rhst::_010,
            3 => Rhst::_011,
            _ => Rhst::Others,
        }
    }
    #[doc = "Communication speed not determined"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rhst::_000
    }
    #[doc = "Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rhst::_001
    }
    #[doc = "Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rhst::_010
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rhst::_011
    }
    #[doc = "USB bus reset in progress(When the host controller function is selected)"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Rhst::Others)
    }
}
#[doc = "USB Bus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uact {
    #[doc = "0: Downstream port is disabled (SOF transmission is disabled)."]
    _0 = 0,
    #[doc = "1: Downstream port is enabled (SOF transmission is enabled)."]
    _1 = 1,
}
impl From<Uact> for bool {
    #[inline(always)]
    fn from(variant: Uact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UACT` reader - USB Bus Enable"]
pub type UactR = crate::BitReader<Uact>;
impl UactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uact {
        match self.bits {
            false => Uact::_0,
            true => Uact::_1,
        }
    }
    #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Uact::_0
    }
    #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Uact::_1
    }
}
#[doc = "Field `UACT` writer - USB Bus Enable"]
pub type UactW<'a, REG> = crate::BitWriter<'a, REG, Uact>;
impl<'a, REG> UactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Uact::_0)
    }
    #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Uact::_1)
    }
}
#[doc = "Resume Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resume {
    #[doc = "0: Resume signal is not output."]
    _0 = 0,
    #[doc = "1: Resume signal is output."]
    _1 = 1,
}
impl From<Resume> for bool {
    #[inline(always)]
    fn from(variant: Resume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUME` reader - Resume Output"]
pub type ResumeR = crate::BitReader<Resume>;
impl ResumeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resume {
        match self.bits {
            false => Resume::_0,
            true => Resume::_1,
        }
    }
    #[doc = "Resume signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Resume::_0
    }
    #[doc = "Resume signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Resume::_1
    }
}
#[doc = "Field `RESUME` writer - Resume Output"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG, Resume>;
impl<'a, REG> ResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resume signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::_0)
    }
    #[doc = "Resume signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Resume::_1)
    }
}
#[doc = "USB Bus Reset Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrst {
    #[doc = "0: USB bus reset signal is not output."]
    _0 = 0,
    #[doc = "1: USB bus reset signal is output."]
    _1 = 1,
}
impl From<Usbrst> for bool {
    #[inline(always)]
    fn from(variant: Usbrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRST` reader - USB Bus Reset Output"]
pub type UsbrstR = crate::BitReader<Usbrst>;
impl UsbrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrst {
        match self.bits {
            false => Usbrst::_0,
            true => Usbrst::_1,
        }
    }
    #[doc = "USB bus reset signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbrst::_0
    }
    #[doc = "USB bus reset signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbrst::_1
    }
}
#[doc = "Field `USBRST` writer - USB Bus Reset Output"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG, Usbrst>;
impl<'a, REG> UsbrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB bus reset signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::_0)
    }
    #[doc = "USB bus reset signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrst::_1)
    }
}
#[doc = "Wakeup Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwupe {
    #[doc = "0: Downstream port wakeup is disabled."]
    _0 = 0,
    #[doc = "1: Downstream port wakeup is enabled."]
    _1 = 1,
}
impl From<Rwupe> for bool {
    #[inline(always)]
    fn from(variant: Rwupe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUPE` reader - Wakeup Detection Enable"]
pub type RwupeR = crate::BitReader<Rwupe>;
impl RwupeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwupe {
        match self.bits {
            false => Rwupe::_0,
            true => Rwupe::_1,
        }
    }
    #[doc = "Downstream port wakeup is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rwupe::_0
    }
    #[doc = "Downstream port wakeup is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rwupe::_1
    }
}
#[doc = "Field `RWUPE` writer - Wakeup Detection Enable"]
pub type RwupeW<'a, REG> = crate::BitWriter<'a, REG, Rwupe>;
impl<'a, REG> RwupeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Downstream port wakeup is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwupe::_0)
    }
    #[doc = "Downstream port wakeup is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwupe::_1)
    }
}
#[doc = "Wakeup Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkup {
    #[doc = "0: Remote wakeup signal is not output."]
    _0 = 0,
    #[doc = "1: Remote wakeup signal is output."]
    _1 = 1,
}
impl From<Wkup> for bool {
    #[inline(always)]
    fn from(variant: Wkup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` reader - Wakeup Output"]
pub type WkupR = crate::BitReader<Wkup>;
impl WkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkup {
        match self.bits {
            false => Wkup::_0,
            true => Wkup::_1,
        }
    }
    #[doc = "Remote wakeup signal is not output."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wkup::_0
    }
    #[doc = "Remote wakeup signal is output."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wkup::_1
    }
}
#[doc = "Field `WKUP` writer - Wakeup Output"]
pub type WkupW<'a, REG> = crate::BitWriter<'a, REG, Wkup>;
impl<'a, REG> WkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Remote wakeup signal is not output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkup::_0)
    }
    #[doc = "Remote wakeup signal is output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkup::_1)
    }
}
#[doc = "USB_VBUSEN Output Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbusen {
    #[doc = "0: External USB_VBUSEN pin outputs low"]
    _0 = 0,
    #[doc = "1: External USB_VBUSEN pin outputs high"]
    _1 = 1,
}
impl From<Vbusen> for bool {
    #[inline(always)]
    fn from(variant: Vbusen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSEN` reader - USB_VBUSEN Output Pin Control"]
pub type VbusenR = crate::BitReader<Vbusen>;
impl VbusenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbusen {
        match self.bits {
            false => Vbusen::_0,
            true => Vbusen::_1,
        }
    }
    #[doc = "External USB_VBUSEN pin outputs low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbusen::_0
    }
    #[doc = "External USB_VBUSEN pin outputs high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbusen::_1
    }
}
#[doc = "Field `VBUSEN` writer - USB_VBUSEN Output Pin Control"]
pub type VbusenW<'a, REG> = crate::BitWriter<'a, REG, Vbusen>;
impl<'a, REG> VbusenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External USB_VBUSEN pin outputs low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusen::_0)
    }
    #[doc = "External USB_VBUSEN pin outputs high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusen::_1)
    }
}
#[doc = "USB_EXICEN Output Pin Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exicen {
    #[doc = "0: External USB_EXICEN pin outputs low"]
    _0 = 0,
    #[doc = "1: External USB_EXICEN pin outputs high"]
    _1 = 1,
}
impl From<Exicen> for bool {
    #[inline(always)]
    fn from(variant: Exicen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXICEN` reader - USB_EXICEN Output Pin Control"]
pub type ExicenR = crate::BitReader<Exicen>;
impl ExicenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exicen {
        match self.bits {
            false => Exicen::_0,
            true => Exicen::_1,
        }
    }
    #[doc = "External USB_EXICEN pin outputs low"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exicen::_0
    }
    #[doc = "External USB_EXICEN pin outputs high"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exicen::_1
    }
}
#[doc = "Field `EXICEN` writer - USB_EXICEN Output Pin Control"]
pub type ExicenW<'a, REG> = crate::BitWriter<'a, REG, Exicen>;
impl<'a, REG> ExicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External USB_EXICEN pin outputs low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exicen::_0)
    }
    #[doc = "External USB_EXICEN pin outputs high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exicen::_1)
    }
}
#[doc = "Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hnpbtoa {
    #[doc = "0: Normal Operation"]
    _0 = 0,
    #[doc = "1: Switching from device B to device A is enabled"]
    _1 = 1,
}
impl From<Hnpbtoa> for bool {
    #[inline(always)]
    fn from(variant: Hnpbtoa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPBTOA` reader - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
pub type HnpbtoaR = crate::BitReader<Hnpbtoa>;
impl HnpbtoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hnpbtoa {
        match self.bits {
            false => Hnpbtoa::_0,
            true => Hnpbtoa::_1,
        }
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hnpbtoa::_0
    }
    #[doc = "Switching from device B to device A is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hnpbtoa::_1
    }
}
#[doc = "Field `HNPBTOA` writer - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
pub type HnpbtoaW<'a, REG> = crate::BitWriter<'a, REG, Hnpbtoa>;
impl<'a, REG> HnpbtoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpbtoa::_0)
    }
    #[doc = "Switching from device B to device A is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpbtoa::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Bus Reset Status"]
    #[inline(always)]
    pub fn rhst(&self) -> RhstR {
        RhstR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - USB Bus Enable"]
    #[inline(always)]
    pub fn uact(&self) -> UactR {
        UactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resume Output"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Bus Reset Output"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Detection Enable"]
    #[inline(always)]
    pub fn rwupe(&self) -> RwupeR {
        RwupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup Output"]
    #[inline(always)]
    pub fn wkup(&self) -> WkupR {
        WkupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub fn vbusen(&self) -> VbusenR {
        VbusenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB_EXICEN Output Pin Control"]
    #[inline(always)]
    pub fn exicen(&self) -> ExicenR {
        ExicenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub fn hnpbtoa(&self) -> HnpbtoaR {
        HnpbtoaR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVSTCTR0")
            .field("hnpbtoa", &self.hnpbtoa())
            .field("exicen", &self.exicen())
            .field("vbusen", &self.vbusen())
            .field("wkup", &self.wkup())
            .field("rwupe", &self.rwupe())
            .field("usbrst", &self.usbrst())
            .field("resume", &self.resume())
            .field("uact", &self.uact())
            .field("rhst", &self.rhst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - USB Bus Enable"]
    #[inline(always)]
    pub fn uact(&mut self) -> UactW<Dvstctr0Spec> {
        UactW::new(self, 4)
    }
    #[doc = "Bit 5 - Resume Output"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<Dvstctr0Spec> {
        ResumeW::new(self, 5)
    }
    #[doc = "Bit 6 - USB Bus Reset Output"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> UsbrstW<Dvstctr0Spec> {
        UsbrstW::new(self, 6)
    }
    #[doc = "Bit 7 - Wakeup Detection Enable"]
    #[inline(always)]
    pub fn rwupe(&mut self) -> RwupeW<Dvstctr0Spec> {
        RwupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Wakeup Output"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WkupW<Dvstctr0Spec> {
        WkupW::new(self, 8)
    }
    #[doc = "Bit 9 - USB_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub fn vbusen(&mut self) -> VbusenW<Dvstctr0Spec> {
        VbusenW::new(self, 9)
    }
    #[doc = "Bit 10 - USB_EXICEN Output Pin Control"]
    #[inline(always)]
    pub fn exicen(&mut self) -> ExicenW<Dvstctr0Spec> {
        ExicenW::new(self, 10)
    }
    #[doc = "Bit 11 - Host Negotiation Protocol (HNP) Control This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub fn hnpbtoa(&mut self) -> HnpbtoaW<Dvstctr0Spec> {
        HnpbtoaW::new(self, 11)
    }
}
#[doc = "Device State Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvstctr0Spec;
impl crate::RegisterSpec for Dvstctr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dvstctr0::R`](R) reader structure"]
impl crate::Readable for Dvstctr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dvstctr0::W`](W) writer structure"]
impl crate::Writable for Dvstctr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVSTCTR0 to value 0"]
impl crate::Resettable for Dvstctr0Spec {}
