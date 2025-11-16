#[doc = "Register `DAM` reader"]
pub type R = crate::R<DamSpec>;
#[doc = "Register `DAM` writer"]
pub type W = crate::W<DamSpec>;
#[doc = "D/A Operation Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dace0 {
    #[doc = "0: D/A conversion disabled for channel 0"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 0."]
    _1 = 1,
}
impl From<Dace0> for bool {
    #[inline(always)]
    fn from(variant: Dace0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACE0` reader - D/A Operation Enable 0"]
pub type Dace0R = crate::BitReader<Dace0>;
impl Dace0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dace0 {
        match self.bits {
            false => Dace0::_0,
            true => Dace0::_1,
        }
    }
    #[doc = "D/A conversion disabled for channel 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dace0::_0
    }
    #[doc = "D/A conversion enabled for channel 0."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dace0::_1
    }
}
#[doc = "Field `DACE0` writer - D/A Operation Enable 0"]
pub type Dace0W<'a, REG> = crate::BitWriter<'a, REG, Dace0>;
impl<'a, REG> Dace0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "D/A conversion disabled for channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dace0::_0)
    }
    #[doc = "D/A conversion enabled for channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dace0::_1)
    }
}
#[doc = "D/A Operation Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dace1 {
    #[doc = "0: D/A conversion disabled for channel 1"]
    _0 = 0,
    #[doc = "1: D/A conversion enabled for channel 1"]
    _1 = 1,
}
impl From<Dace1> for bool {
    #[inline(always)]
    fn from(variant: Dace1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACE1` reader - D/A Operation Enable 1"]
pub type Dace1R = crate::BitReader<Dace1>;
impl Dace1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dace1 {
        match self.bits {
            false => Dace1::_0,
            true => Dace1::_1,
        }
    }
    #[doc = "D/A conversion disabled for channel 1"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dace1::_0
    }
    #[doc = "D/A conversion enabled for channel 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dace1::_1
    }
}
#[doc = "Field `DACE1` writer - D/A Operation Enable 1"]
pub type Dace1W<'a, REG> = crate::BitWriter<'a, REG, Dace1>;
impl<'a, REG> Dace1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "D/A conversion disabled for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dace1::_0)
    }
    #[doc = "D/A conversion enabled for channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dace1::_1)
    }
}
impl R {
    #[doc = "Bit 4 - D/A Operation Enable 0"]
    #[inline(always)]
    pub fn dace0(&self) -> Dace0R {
        Dace0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D/A Operation Enable 1"]
    #[inline(always)]
    pub fn dace1(&self) -> Dace1R {
        Dace1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAM")
            .field("dace1", &self.dace1())
            .field("dace0", &self.dace0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - D/A Operation Enable 0"]
    #[inline(always)]
    pub fn dace0(&mut self) -> Dace0W<DamSpec> {
        Dace0W::new(self, 4)
    }
    #[doc = "Bit 5 - D/A Operation Enable 1"]
    #[inline(always)]
    pub fn dace1(&mut self) -> Dace1W<DamSpec> {
        Dace1W::new(self, 5)
    }
}
#[doc = "D/A Converter Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DamSpec;
impl crate::RegisterSpec for DamSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dam::R`](R) reader structure"]
impl crate::Readable for DamSpec {}
#[doc = "`write(|w| ..)` method takes [`dam::W`](W) writer structure"]
impl crate::Writable for DamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAM to value 0"]
impl crate::Resettable for DamSpec {}
