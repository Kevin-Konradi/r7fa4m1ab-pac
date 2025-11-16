#[doc = "Register `SMPUSRAM0` reader"]
pub type R = crate::R<Smpusram0Spec>;
#[doc = "Register `SMPUSRAM0` writer"]
pub type W = crate::W<Smpusram0Spec>;
#[doc = "CPU Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpcpu {
    #[doc = "0: CPU read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU read of memory protection enabled."]
    _1 = 1,
}
impl From<Rpcpu> for bool {
    #[inline(always)]
    fn from(variant: Rpcpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPCPU` reader - CPU Read protection"]
pub type RpcpuR = crate::BitReader<Rpcpu>;
impl RpcpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpcpu {
        match self.bits {
            false => Rpcpu::_0,
            true => Rpcpu::_1,
        }
    }
    #[doc = "CPU read of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpcpu::_0
    }
    #[doc = "CPU read of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpcpu::_1
    }
}
#[doc = "Field `RPCPU` writer - CPU Read protection"]
pub type RpcpuW<'a, REG> = crate::BitWriter<'a, REG, Rpcpu>;
impl<'a, REG> RpcpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpcpu::_0)
    }
    #[doc = "CPU read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpcpu::_1)
    }
}
#[doc = "CPU Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpcpu {
    #[doc = "0: CPU write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: CPU write of memory protection enabled."]
    _1 = 1,
}
impl From<Wpcpu> for bool {
    #[inline(always)]
    fn from(variant: Wpcpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPCPU` reader - CPU Write protection"]
pub type WpcpuR = crate::BitReader<Wpcpu>;
impl WpcpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpcpu {
        match self.bits {
            false => Wpcpu::_0,
            true => Wpcpu::_1,
        }
    }
    #[doc = "CPU write of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wpcpu::_0
    }
    #[doc = "CPU write of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wpcpu::_1
    }
}
#[doc = "Field `WPCPU` writer - CPU Write protection"]
pub type WpcpuW<'a, REG> = crate::BitWriter<'a, REG, Wpcpu>;
impl<'a, REG> WpcpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wpcpu::_0)
    }
    #[doc = "CPU write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpcpu::_1)
    }
}
#[doc = "Master Group A Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpgrpa {
    #[doc = "0: Master group A read of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A read of memory protection enabled."]
    _1 = 1,
}
impl From<Rpgrpa> for bool {
    #[inline(always)]
    fn from(variant: Rpgrpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPGRPA` reader - Master Group A Read protection"]
pub type RpgrpaR = crate::BitReader<Rpgrpa>;
impl RpgrpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpgrpa {
        match self.bits {
            false => Rpgrpa::_0,
            true => Rpgrpa::_1,
        }
    }
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpgrpa::_0
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpgrpa::_1
    }
}
#[doc = "Field `RPGRPA` writer - Master Group A Read protection"]
pub type RpgrpaW<'a, REG> = crate::BitWriter<'a, REG, Rpgrpa>;
impl<'a, REG> RpgrpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A read of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpgrpa::_0)
    }
    #[doc = "Master group A read of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpgrpa::_1)
    }
}
#[doc = "Master Group A Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpgrpa {
    #[doc = "0: Master group A write of memory protection disabled."]
    _0 = 0,
    #[doc = "1: Master group A write of memory protection enabled."]
    _1 = 1,
}
impl From<Wpgrpa> for bool {
    #[inline(always)]
    fn from(variant: Wpgrpa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPGRPA` reader - Master Group A Write protection"]
pub type WpgrpaR = crate::BitReader<Wpgrpa>;
impl WpgrpaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpgrpa {
        match self.bits {
            false => Wpgrpa::_0,
            true => Wpgrpa::_1,
        }
    }
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wpgrpa::_0
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wpgrpa::_1
    }
}
#[doc = "Field `WPGRPA` writer - Master Group A Write protection"]
pub type WpgrpaW<'a, REG> = crate::BitWriter<'a, REG, Wpgrpa>;
impl<'a, REG> WpgrpaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master group A write of memory protection disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wpgrpa::_0)
    }
    #[doc = "Master group A write of memory protection enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpgrpa::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(&self) -> RpcpuR {
        RpcpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(&self) -> WpcpuR {
        WpcpuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&self) -> RpgrpaR {
        RpgrpaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&self) -> WpgrpaR {
        WpgrpaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPUSRAM0")
            .field("wpgrpa", &self.wpgrpa())
            .field("rpgrpa", &self.rpgrpa())
            .field("wpcpu", &self.wpcpu())
            .field("rpcpu", &self.rpcpu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CPU Read protection"]
    #[inline(always)]
    pub fn rpcpu(&mut self) -> RpcpuW<Smpusram0Spec> {
        RpcpuW::new(self, 0)
    }
    #[doc = "Bit 1 - CPU Write protection"]
    #[inline(always)]
    pub fn wpcpu(&mut self) -> WpcpuW<Smpusram0Spec> {
        WpcpuW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Group A Read protection"]
    #[inline(always)]
    pub fn rpgrpa(&mut self) -> RpgrpaW<Smpusram0Spec> {
        RpgrpaW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Group A Write protection"]
    #[inline(always)]
    pub fn wpgrpa(&mut self) -> WpgrpaW<Smpusram0Spec> {
        WpgrpaW::new(self, 3)
    }
}
#[doc = "Access Control Register for SRAM0\n\nYou can [`read`](crate::Reg::read) this register and get [`smpusram0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpusram0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Smpusram0Spec;
impl crate::RegisterSpec for Smpusram0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smpusram0::R`](R) reader structure"]
impl crate::Readable for Smpusram0Spec {}
#[doc = "`write(|w| ..)` method takes [`smpusram0::W`](W) writer structure"]
impl crate::Writable for Smpusram0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPUSRAM0 to value 0"]
impl crate::Resettable for Smpusram0Spec {}
