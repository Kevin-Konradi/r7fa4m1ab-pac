#[doc = "Register `MSTPCRA` reader"]
pub type R = crate::R<MstpcraSpec>;
#[doc = "Register `MSTPCRA` writer"]
pub type W = crate::W<MstpcraSpec>;
#[doc = "RAM0 Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa0 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpa0> for bool {
    #[inline(always)]
    fn from(variant: Mstpa0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPA0` reader - RAM0 Module Stop"]
pub type Mstpa0R = crate::BitReader<Mstpa0>;
impl Mstpa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa0 {
        match self.bits {
            false => Mstpa0::_0,
            true => Mstpa0::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa0::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa0::_1
    }
}
#[doc = "Field `MSTPA0` writer - RAM0 Module Stop"]
pub type Mstpa0W<'a, REG> = crate::BitWriter<'a, REG, Mstpa0>;
impl<'a, REG> Mstpa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa0::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa0::_1)
    }
}
#[doc = "ECCRAM Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa6 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpa6> for bool {
    #[inline(always)]
    fn from(variant: Mstpa6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPA6` reader - ECCRAM Module Stop"]
pub type Mstpa6R = crate::BitReader<Mstpa6>;
impl Mstpa6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa6 {
        match self.bits {
            false => Mstpa6::_0,
            true => Mstpa6::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa6::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa6::_1
    }
}
#[doc = "Field `MSTPA6` writer - ECCRAM Module Stop"]
pub type Mstpa6W<'a, REG> = crate::BitWriter<'a, REG, Mstpa6>;
impl<'a, REG> Mstpa6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa6::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa6::_1)
    }
}
#[doc = "DMA Controller/Data Transfer Controller Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa22 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpa22> for bool {
    #[inline(always)]
    fn from(variant: Mstpa22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPA22` reader - DMA Controller/Data Transfer Controller Module Stop"]
pub type Mstpa22R = crate::BitReader<Mstpa22>;
impl Mstpa22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa22 {
        match self.bits {
            false => Mstpa22::_0,
            true => Mstpa22::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa22::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa22::_1
    }
}
#[doc = "Field `MSTPA22` writer - DMA Controller/Data Transfer Controller Module Stop"]
pub type Mstpa22W<'a, REG> = crate::BitWriter<'a, REG, Mstpa22>;
impl<'a, REG> Mstpa22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(&self) -> Mstpa0R {
        Mstpa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(&self) -> Mstpa6R {
        Mstpa6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&self) -> Mstpa22R {
        Mstpa22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTPCRA")
            .field("mstpa22", &self.mstpa22())
            .field("mstpa6", &self.mstpa6())
            .field("mstpa0", &self.mstpa0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(&mut self) -> Mstpa0W<MstpcraSpec> {
        Mstpa0W::new(self, 0)
    }
    #[doc = "Bit 6 - ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(&mut self) -> Mstpa6W<MstpcraSpec> {
        Mstpa6W::new(self, 6)
    }
    #[doc = "Bit 22 - DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&mut self) -> Mstpa22W<MstpcraSpec> {
        Mstpa22W::new(self, 22)
    }
}
#[doc = "Module Stop Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcraSpec;
impl crate::RegisterSpec for MstpcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcra::R`](R) reader structure"]
impl crate::Readable for MstpcraSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcra::W`](W) writer structure"]
impl crate::Writable for MstpcraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSTPCRA to value 0xffbf_ffbe"]
impl crate::Resettable for MstpcraSpec {
    const RESET_VALUE: u32 = 0xffbf_ffbe;
}
