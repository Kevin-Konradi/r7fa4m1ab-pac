#[doc = "Register `SYSCFG` reader"]
pub type R = crate::R<SyscfgSpec>;
#[doc = "Register `SYSCFG` writer"]
pub type W = crate::W<SyscfgSpec>;
#[doc = "USB Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbe {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Usbe> for bool {
    #[inline(always)]
    fn from(variant: Usbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBE` reader - USB Operation Enable"]
pub type UsbeR = crate::BitReader<Usbe>;
impl UsbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbe {
        match self.bits {
            false => Usbe::_0,
            true => Usbe::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbe::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbe::_1
    }
}
#[doc = "Field `USBE` writer - USB Operation Enable"]
pub type UsbeW<'a, REG> = crate::BitWriter<'a, REG, Usbe>;
impl<'a, REG> UsbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbe::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbe::_1)
    }
}
#[doc = "D- Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmrpu {
    #[doc = "0: Line pull-up disabled"]
    _0 = 0,
    #[doc = "1: Line pull-up enabled."]
    _1 = 1,
}
impl From<Dmrpu> for bool {
    #[inline(always)]
    fn from(variant: Dmrpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMRPU` reader - D- Line Resistor Control"]
pub type DmrpuR = crate::BitReader<Dmrpu>;
impl DmrpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmrpu {
        match self.bits {
            false => Dmrpu::_0,
            true => Dmrpu::_1,
        }
    }
    #[doc = "Line pull-up disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dmrpu::_0
    }
    #[doc = "Line pull-up enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dmrpu::_1
    }
}
#[doc = "Field `DMRPU` writer - D- Line Resistor Control"]
pub type DmrpuW<'a, REG> = crate::BitWriter<'a, REG, Dmrpu>;
impl<'a, REG> DmrpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Line pull-up disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmrpu::_0)
    }
    #[doc = "Line pull-up enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmrpu::_1)
    }
}
#[doc = "D+ Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dprpu {
    #[doc = "0: Line pull-down disabled"]
    _0 = 0,
    #[doc = "1: Line pull-down enabled."]
    _1 = 1,
}
impl From<Dprpu> for bool {
    #[inline(always)]
    fn from(variant: Dprpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPRPU` reader - D+ Line Resistor Control"]
pub type DprpuR = crate::BitReader<Dprpu>;
impl DprpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dprpu {
        match self.bits {
            false => Dprpu::_0,
            true => Dprpu::_1,
        }
    }
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dprpu::_0
    }
    #[doc = "Line pull-down enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dprpu::_1
    }
}
#[doc = "Field `DPRPU` writer - D+ Line Resistor Control"]
pub type DprpuW<'a, REG> = crate::BitWriter<'a, REG, Dprpu>;
impl<'a, REG> DprpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dprpu::_0)
    }
    #[doc = "Line pull-down enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dprpu::_1)
    }
}
#[doc = "D+/D- Line Resistor Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drpd {
    #[doc = "0: Line pull-down disabled"]
    _0 = 0,
    #[doc = "1: Line pull-down enabled."]
    _1 = 1,
}
impl From<Drpd> for bool {
    #[inline(always)]
    fn from(variant: Drpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRPD` reader - D+/D- Line Resistor Control"]
pub type DrpdR = crate::BitReader<Drpd>;
impl DrpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drpd {
        match self.bits {
            false => Drpd::_0,
            true => Drpd::_1,
        }
    }
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drpd::_0
    }
    #[doc = "Line pull-down enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drpd::_1
    }
}
#[doc = "Field `DRPD` writer - D+/D- Line Resistor Control"]
pub type DrpdW<'a, REG> = crate::BitWriter<'a, REG, Drpd>;
impl<'a, REG> DrpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Line pull-down disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd::_0)
    }
    #[doc = "Line pull-down enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drpd::_1)
    }
}
#[doc = "Controller Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcfm {
    #[doc = "0: Device controller selected"]
    _0 = 0,
    #[doc = "1: Host controller selected."]
    _1 = 1,
}
impl From<Dcfm> for bool {
    #[inline(always)]
    fn from(variant: Dcfm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCFM` reader - Controller Function Select"]
pub type DcfmR = crate::BitReader<Dcfm>;
impl DcfmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcfm {
        match self.bits {
            false => Dcfm::_0,
            true => Dcfm::_1,
        }
    }
    #[doc = "Device controller selected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcfm::_0
    }
    #[doc = "Host controller selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcfm::_1
    }
}
#[doc = "Field `DCFM` writer - Controller Function Select"]
pub type DcfmW<'a, REG> = crate::BitWriter<'a, REG, Dcfm>;
impl<'a, REG> DcfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device controller selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_0)
    }
    #[doc = "Host controller selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcfm::_1)
    }
}
#[doc = "CNEN Single End Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnen {
    #[doc = "0: Single end receiver disabled"]
    _0 = 0,
    #[doc = "1: Single end receiver enabled"]
    _1 = 1,
}
impl From<Cnen> for bool {
    #[inline(always)]
    fn from(variant: Cnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNEN` reader - CNEN Single End Receiver Enable"]
pub type CnenR = crate::BitReader<Cnen>;
impl CnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnen {
        match self.bits {
            false => Cnen::_0,
            true => Cnen::_1,
        }
    }
    #[doc = "Single end receiver disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cnen::_0
    }
    #[doc = "Single end receiver enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cnen::_1
    }
}
#[doc = "Field `CNEN` writer - CNEN Single End Receiver Enable"]
pub type CnenW<'a, REG> = crate::BitWriter<'a, REG, Cnen>;
impl<'a, REG> CnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single end receiver disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_0)
    }
    #[doc = "Single end receiver enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnen::_1)
    }
}
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scke {
    #[doc = "0: Clock supply to the USBFS stopped"]
    _0 = 0,
    #[doc = "1: Clock supply to the USBFS enabled."]
    _1 = 1,
}
impl From<Scke> for bool {
    #[inline(always)]
    fn from(variant: Scke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKE` reader - USB Clock Enable"]
pub type SckeR = crate::BitReader<Scke>;
impl SckeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scke {
        match self.bits {
            false => Scke::_0,
            true => Scke::_1,
        }
    }
    #[doc = "Clock supply to the USBFS stopped"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scke::_0
    }
    #[doc = "Clock supply to the USBFS enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scke::_1
    }
}
#[doc = "Field `SCKE` writer - USB Clock Enable"]
pub type SckeW<'a, REG> = crate::BitWriter<'a, REG, Scke>;
impl<'a, REG> SckeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock supply to the USBFS stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Scke::_0)
    }
    #[doc = "Clock supply to the USBFS enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Scke::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> UsbeR {
        UsbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - D- Line Resistor Control"]
    #[inline(always)]
    pub fn dmrpu(&self) -> DmrpuR {
        DmrpuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - D+ Line Resistor Control"]
    #[inline(always)]
    pub fn dprpu(&self) -> DprpuR {
        DprpuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D+/D- Line Resistor Control"]
    #[inline(always)]
    pub fn drpd(&self) -> DrpdR {
        DrpdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controller Function Select"]
    #[inline(always)]
    pub fn dcfm(&self) -> DcfmR {
        DcfmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CNEN Single End Receiver Enable"]
    #[inline(always)]
    pub fn cnen(&self) -> CnenR {
        CnenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - USB Clock Enable"]
    #[inline(always)]
    pub fn scke(&self) -> SckeR {
        SckeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG")
            .field("scke", &self.scke())
            .field("cnen", &self.cnen())
            .field("dcfm", &self.dcfm())
            .field("drpd", &self.drpd())
            .field("dprpu", &self.dprpu())
            .field("dmrpu", &self.dmrpu())
            .field("usbe", &self.usbe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB Operation Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> UsbeW<SyscfgSpec> {
        UsbeW::new(self, 0)
    }
    #[doc = "Bit 3 - D- Line Resistor Control"]
    #[inline(always)]
    pub fn dmrpu(&mut self) -> DmrpuW<SyscfgSpec> {
        DmrpuW::new(self, 3)
    }
    #[doc = "Bit 4 - D+ Line Resistor Control"]
    #[inline(always)]
    pub fn dprpu(&mut self) -> DprpuW<SyscfgSpec> {
        DprpuW::new(self, 4)
    }
    #[doc = "Bit 5 - D+/D- Line Resistor Control"]
    #[inline(always)]
    pub fn drpd(&mut self) -> DrpdW<SyscfgSpec> {
        DrpdW::new(self, 5)
    }
    #[doc = "Bit 6 - Controller Function Select"]
    #[inline(always)]
    pub fn dcfm(&mut self) -> DcfmW<SyscfgSpec> {
        DcfmW::new(self, 6)
    }
    #[doc = "Bit 8 - CNEN Single End Receiver Enable"]
    #[inline(always)]
    pub fn cnen(&mut self) -> CnenW<SyscfgSpec> {
        CnenW::new(self, 8)
    }
    #[doc = "Bit 10 - USB Clock Enable"]
    #[inline(always)]
    pub fn scke(&mut self) -> SckeW<SyscfgSpec> {
        SckeW::new(self, 10)
    }
}
#[doc = "System Configuration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgSpec;
impl crate::RegisterSpec for SyscfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg::R`](R) reader structure"]
impl crate::Readable for SyscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg::W`](W) writer structure"]
impl crate::Writable for SyscfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG to value 0"]
impl crate::Resettable for SyscfgSpec {}
