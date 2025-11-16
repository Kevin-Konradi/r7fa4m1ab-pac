#[doc = "Register `LVCMPCR` reader"]
pub type R = crate::R<LvcmpcrSpec>;
#[doc = "Register `LVCMPCR` writer"]
pub type W = crate::W<LvcmpcrSpec>;
#[doc = "Voltage Detection 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1e {
    #[doc = "0: Voltage detection 1 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 1 circuit enabled"]
    _1 = 1,
}
impl From<Lvd1e> for bool {
    #[inline(always)]
    fn from(variant: Lvd1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1E` reader - Voltage Detection 1 Enable"]
pub type Lvd1eR = crate::BitReader<Lvd1e>;
impl Lvd1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1e {
        match self.bits {
            false => Lvd1e::_0,
            true => Lvd1e::_1,
        }
    }
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1e::_0
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1e::_1
    }
}
#[doc = "Field `LVD1E` writer - Voltage Detection 1 Enable"]
pub type Lvd1eW<'a, REG> = crate::BitWriter<'a, REG, Lvd1e>;
impl<'a, REG> Lvd1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage detection 1 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1e::_0)
    }
    #[doc = "Voltage detection 1 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1e::_1)
    }
}
#[doc = "Voltage Detection 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2e {
    #[doc = "0: Voltage detection 2 circuit disabled"]
    _0 = 0,
    #[doc = "1: Voltage detection 2 circuit enabled"]
    _1 = 1,
}
impl From<Lvd2e> for bool {
    #[inline(always)]
    fn from(variant: Lvd2e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2E` reader - Voltage Detection 2 Enable"]
pub type Lvd2eR = crate::BitReader<Lvd2e>;
impl Lvd2eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2e {
        match self.bits {
            false => Lvd2e::_0,
            true => Lvd2e::_1,
        }
    }
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2e::_0
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2e::_1
    }
}
#[doc = "Field `LVD2E` writer - Voltage Detection 2 Enable"]
pub type Lvd2eW<'a, REG> = crate::BitWriter<'a, REG, Lvd2e>;
impl<'a, REG> Lvd2eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Voltage detection 2 circuit disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2e::_0)
    }
    #[doc = "Voltage detection 2 circuit enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2e::_1)
    }
}
impl R {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(&self) -> Lvd1eR {
        Lvd1eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(&self) -> Lvd2eR {
        Lvd2eR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVCMPCR")
            .field("lvd2e", &self.lvd2e())
            .field("lvd1e", &self.lvd1e())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(&mut self) -> Lvd1eW<LvcmpcrSpec> {
        Lvd1eW::new(self, 5)
    }
    #[doc = "Bit 6 - Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(&mut self) -> Lvd2eW<LvcmpcrSpec> {
        Lvd2eW::new(self, 6)
    }
}
#[doc = "Voltage Monitor Circuit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvcmpcrSpec;
impl crate::RegisterSpec for LvcmpcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvcmpcr::R`](R) reader structure"]
impl crate::Readable for LvcmpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lvcmpcr::W`](W) writer structure"]
impl crate::Writable for LvcmpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LVCMPCR to value 0"]
impl crate::Resettable for LvcmpcrSpec {}
