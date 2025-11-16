#[doc = "Register `ICSER` reader"]
pub type R = crate::R<IcserSpec>;
#[doc = "Register `ICSER` writer"]
pub type W = crate::W<IcserSpec>;
#[doc = "Slave Address Register 0 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar0e {
    #[doc = "0: Slave address in SARL0 and SARU0 is disabled."]
    _0 = 0,
    #[doc = "1: Slave address in SARL0 and SARU0 is enabled."]
    _1 = 1,
}
impl From<Sar0e> for bool {
    #[inline(always)]
    fn from(variant: Sar0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAR0E` reader - Slave Address Register 0 Enable"]
pub type Sar0eR = crate::BitReader<Sar0e>;
impl Sar0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sar0e {
        match self.bits {
            false => Sar0e::_0,
            true => Sar0e::_1,
        }
    }
    #[doc = "Slave address in SARL0 and SARU0 is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar0e::_0
    }
    #[doc = "Slave address in SARL0 and SARU0 is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar0e::_1
    }
}
#[doc = "Field `SAR0E` writer - Slave Address Register 0 Enable"]
pub type Sar0eW<'a, REG> = crate::BitWriter<'a, REG, Sar0e>;
impl<'a, REG> Sar0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address in SARL0 and SARU0 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar0e::_0)
    }
    #[doc = "Slave address in SARL0 and SARU0 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar0e::_1)
    }
}
#[doc = "Slave Address Register 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar1e {
    #[doc = "0: Slave address in SARL1 and SARU1 is disabled."]
    _0 = 0,
    #[doc = "1: Slave address in SARL1 and SARU1 is enabled."]
    _1 = 1,
}
impl From<Sar1e> for bool {
    #[inline(always)]
    fn from(variant: Sar1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAR1E` reader - Slave Address Register 1 Enable"]
pub type Sar1eR = crate::BitReader<Sar1e>;
impl Sar1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sar1e {
        match self.bits {
            false => Sar1e::_0,
            true => Sar1e::_1,
        }
    }
    #[doc = "Slave address in SARL1 and SARU1 is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar1e::_0
    }
    #[doc = "Slave address in SARL1 and SARU1 is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar1e::_1
    }
}
#[doc = "Field `SAR1E` writer - Slave Address Register 1 Enable"]
pub type Sar1eW<'a, REG> = crate::BitWriter<'a, REG, Sar1e>;
impl<'a, REG> Sar1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address in SARL1 and SARU1 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar1e::_0)
    }
    #[doc = "Slave address in SARL1 and SARU1 is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar1e::_1)
    }
}
#[doc = "Slave Address Register 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sar2e {
    #[doc = "0: Slave address in SARL2 and SARU2 is disabled."]
    _0 = 0,
    #[doc = "1: Slave address in SARL2 and SARU2 is enabled"]
    _1 = 1,
}
impl From<Sar2e> for bool {
    #[inline(always)]
    fn from(variant: Sar2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAR2E` reader - Slave Address Register 2 Enable"]
pub type Sar2eR = crate::BitReader<Sar2e>;
impl Sar2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sar2e {
        match self.bits {
            false => Sar2e::_0,
            true => Sar2e::_1,
        }
    }
    #[doc = "Slave address in SARL2 and SARU2 is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sar2e::_0
    }
    #[doc = "Slave address in SARL2 and SARU2 is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sar2e::_1
    }
}
#[doc = "Field `SAR2E` writer - Slave Address Register 2 Enable"]
pub type Sar2eW<'a, REG> = crate::BitWriter<'a, REG, Sar2e>;
impl<'a, REG> Sar2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave address in SARL2 and SARU2 is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sar2e::_0)
    }
    #[doc = "Slave address in SARL2 and SARU2 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sar2e::_1)
    }
}
#[doc = "General Call Address Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcae {
    #[doc = "0: General call address detection is disabled."]
    _0 = 0,
    #[doc = "1: General call address detection is enabled."]
    _1 = 1,
}
impl From<Gcae> for bool {
    #[inline(always)]
    fn from(variant: Gcae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAE` reader - General Call Address Enable"]
pub type GcaeR = crate::BitReader<Gcae>;
impl GcaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcae {
        match self.bits {
            false => Gcae::_0,
            true => Gcae::_1,
        }
    }
    #[doc = "General call address detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Gcae::_0
    }
    #[doc = "General call address detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Gcae::_1
    }
}
#[doc = "Field `GCAE` writer - General Call Address Enable"]
pub type GcaeW<'a, REG> = crate::BitWriter<'a, REG, Gcae>;
impl<'a, REG> GcaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call address detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcae::_0)
    }
    #[doc = "General call address detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcae::_1)
    }
}
#[doc = "Device-ID Address Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dide {
    #[doc = "0: Device-ID address detection is disabled."]
    _0 = 0,
    #[doc = "1: Device-ID address detection is enabled."]
    _1 = 1,
}
impl From<Dide> for bool {
    #[inline(always)]
    fn from(variant: Dide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIDE` reader - Device-ID Address Detection Enable"]
pub type DideR = crate::BitReader<Dide>;
impl DideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dide {
        match self.bits {
            false => Dide::_0,
            true => Dide::_1,
        }
    }
    #[doc = "Device-ID address detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dide::_0
    }
    #[doc = "Device-ID address detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dide::_1
    }
}
#[doc = "Field `DIDE` writer - Device-ID Address Detection Enable"]
pub type DideW<'a, REG> = crate::BitWriter<'a, REG, Dide>;
impl<'a, REG> DideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device-ID address detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dide::_0)
    }
    #[doc = "Device-ID address detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dide::_1)
    }
}
#[doc = "Host Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hoae {
    #[doc = "0: Host address detection is disabled."]
    _0 = 0,
    #[doc = "1: Host address detection is enabled."]
    _1 = 1,
}
impl From<Hoae> for bool {
    #[inline(always)]
    fn from(variant: Hoae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOAE` reader - Host Address Enable"]
pub type HoaeR = crate::BitReader<Hoae>;
impl HoaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hoae {
        match self.bits {
            false => Hoae::_0,
            true => Hoae::_1,
        }
    }
    #[doc = "Host address detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hoae::_0
    }
    #[doc = "Host address detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hoae::_1
    }
}
#[doc = "Field `HOAE` writer - Host Address Enable"]
pub type HoaeW<'a, REG> = crate::BitWriter<'a, REG, Hoae>;
impl<'a, REG> HoaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host address detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hoae::_0)
    }
    #[doc = "Host address detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hoae::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address Register 0 Enable"]
    #[inline(always)]
    pub fn sar0e(&self) -> Sar0eR {
        Sar0eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Address Register 1 Enable"]
    #[inline(always)]
    pub fn sar1e(&self) -> Sar1eR {
        Sar1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Address Register 2 Enable"]
    #[inline(always)]
    pub fn sar2e(&self) -> Sar2eR {
        Sar2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(&self) -> GcaeR {
        GcaeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Enable"]
    #[inline(always)]
    pub fn dide(&self) -> DideR {
        DideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Address Enable"]
    #[inline(always)]
    pub fn hoae(&self) -> HoaeR {
        HoaeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSER")
            .field("hoae", &self.hoae())
            .field("dide", &self.dide())
            .field("gcae", &self.gcae())
            .field("sar2e", &self.sar2e())
            .field("sar1e", &self.sar1e())
            .field("sar0e", &self.sar0e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address Register 0 Enable"]
    #[inline(always)]
    pub fn sar0e(&mut self) -> Sar0eW<IcserSpec> {
        Sar0eW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Address Register 1 Enable"]
    #[inline(always)]
    pub fn sar1e(&mut self) -> Sar1eW<IcserSpec> {
        Sar1eW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Address Register 2 Enable"]
    #[inline(always)]
    pub fn sar2e(&mut self) -> Sar2eW<IcserSpec> {
        Sar2eW::new(self, 2)
    }
    #[doc = "Bit 3 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(&mut self) -> GcaeW<IcserSpec> {
        GcaeW::new(self, 3)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Enable"]
    #[inline(always)]
    pub fn dide(&mut self) -> DideW<IcserSpec> {
        DideW::new(self, 5)
    }
    #[doc = "Bit 7 - Host Address Enable"]
    #[inline(always)]
    pub fn hoae(&mut self) -> HoaeW<IcserSpec> {
        HoaeW::new(self, 7)
    }
}
#[doc = "I2C Bus Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcserSpec;
impl crate::RegisterSpec for IcserSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icser::R`](R) reader structure"]
impl crate::Readable for IcserSpec {}
#[doc = "`write(|w| ..)` method takes [`icser::W`](W) writer structure"]
impl crate::Writable for IcserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICSER to value 0x09"]
impl crate::Resettable for IcserSpec {
    const RESET_VALUE: u8 = 0x09;
}
