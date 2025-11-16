#[doc = "Register `AMPC` reader"]
pub type R = crate::R<AmpcSpec>;
#[doc = "Register `AMPC` writer"]
pub type W = crate::W<AmpcSpec>;
#[doc = "Operation control of operational amplifier(UNIT0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampe0 {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    _1 = 1,
}
impl From<Ampe0> for bool {
    #[inline(always)]
    fn from(variant: Ampe0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPE0` reader - Operation control of operational amplifier(UNIT0)"]
pub type Ampe0R = crate::BitReader<Ampe0>;
impl Ampe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampe0 {
        match self.bits {
            false => Ampe0::_0,
            true => Ampe0::_1,
        }
    }
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampe0::_0
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampe0::_1
    }
}
#[doc = "Field `AMPE0` writer - Operation control of operational amplifier(UNIT0)"]
pub type Ampe0W<'a, REG> = crate::BitWriter<'a, REG, Ampe0>;
impl<'a, REG> Ampe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe0::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for AGT is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe0::_1)
    }
}
#[doc = "Operation control of operational amplifier(UNIT1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampe1 {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<Ampe1> for bool {
    #[inline(always)]
    fn from(variant: Ampe1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPE1` reader - Operation control of operational amplifier(UNIT1)"]
pub type Ampe1R = crate::BitReader<Ampe1>;
impl Ampe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampe1 {
        match self.bits {
            false => Ampe1::_0,
            true => Ampe1::_1,
        }
    }
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampe1::_0
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampe1::_1
    }
}
#[doc = "Field `AMPE1` writer - Operation control of operational amplifier(UNIT1)"]
pub type Ampe1W<'a, REG> = crate::BitWriter<'a, REG, Ampe1>;
impl<'a, REG> Ampe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe1::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe1::_1)
    }
}
#[doc = "Operation control of operational amplifier(UNIT2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampe2 {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<Ampe2> for bool {
    #[inline(always)]
    fn from(variant: Ampe2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPE2` reader - Operation control of operational amplifier(UNIT2)"]
pub type Ampe2R = crate::BitReader<Ampe2>;
impl Ampe2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampe2 {
        match self.bits {
            false => Ampe2::_0,
            true => Ampe2::_1,
        }
    }
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampe2::_0
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampe2::_1
    }
}
#[doc = "Field `AMPE2` writer - Operation control of operational amplifier(UNIT2)"]
pub type Ampe2W<'a, REG> = crate::BitWriter<'a, REG, Ampe2>;
impl<'a, REG> Ampe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe2::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe2::_1)
    }
}
#[doc = "Operation control of operational amplifier(UNIT3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ampe3 {
    #[doc = "0: Operation amplifier is stopped."]
    _0 = 0,
    #[doc = "1: Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    _1 = 1,
}
impl From<Ampe3> for bool {
    #[inline(always)]
    fn from(variant: Ampe3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMPE3` reader - Operation control of operational amplifier(UNIT3)"]
pub type Ampe3R = crate::BitReader<Ampe3>;
impl Ampe3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ampe3 {
        match self.bits {
            false => Ampe3::_0,
            true => Ampe3::_1,
        }
    }
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ampe3::_0
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ampe3::_1
    }
}
#[doc = "Field `AMPE3` writer - Operation control of operational amplifier(UNIT3)"]
pub type Ampe3W<'a, REG> = crate::BitWriter<'a, REG, Ampe3>;
impl<'a, REG> Ampe3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operation amplifier is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe3::_0)
    }
    #[doc = "Software trigger mode: Operation of operational amplifier is enabled Operation of the operational amplifier reference current circuit is also enabled regardless of the IREFE bit se An activation trigger mode or An activation and A/D trigger mode: Wait for An activation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ampe3::_1)
    }
}
#[doc = "Operation control of operational amplifier reference current circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefe {
    #[doc = "0: Operational amplifier reference current circuit is stopped."]
    _0 = 0,
    #[doc = "1: Operation of operational amplifier reference current circuit is enabled."]
    _1 = 1,
}
impl From<Irefe> for bool {
    #[inline(always)]
    fn from(variant: Irefe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFE` reader - Operation control of operational amplifier reference current circuit"]
pub type IrefeR = crate::BitReader<Irefe>;
impl IrefeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefe {
        match self.bits {
            false => Irefe::_0,
            true => Irefe::_1,
        }
    }
    #[doc = "Operational amplifier reference current circuit is stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Irefe::_0
    }
    #[doc = "Operation of operational amplifier reference current circuit is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Irefe::_1
    }
}
#[doc = "Field `IREFE` writer - Operation control of operational amplifier reference current circuit"]
pub type IrefeW<'a, REG> = crate::BitWriter<'a, REG, Irefe>;
impl<'a, REG> IrefeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operational amplifier reference current circuit is stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Irefe::_0)
    }
    #[doc = "Operation of operational amplifier reference current circuit is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Irefe::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    pub fn ampe0(&self) -> Ampe0R {
        Ampe0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    pub fn ampe1(&self) -> Ampe1R {
        Ampe1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    pub fn ampe2(&self) -> Ampe2R {
        Ampe2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    pub fn ampe3(&self) -> Ampe3R {
        Ampe3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    pub fn irefe(&self) -> IrefeR {
        IrefeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPC")
            .field("irefe", &self.irefe())
            .field("ampe3", &self.ampe3())
            .field("ampe2", &self.ampe2())
            .field("ampe1", &self.ampe1())
            .field("ampe0", &self.ampe0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Operation control of operational amplifier(UNIT0)"]
    #[inline(always)]
    pub fn ampe0(&mut self) -> Ampe0W<AmpcSpec> {
        Ampe0W::new(self, 0)
    }
    #[doc = "Bit 1 - Operation control of operational amplifier(UNIT1)"]
    #[inline(always)]
    pub fn ampe1(&mut self) -> Ampe1W<AmpcSpec> {
        Ampe1W::new(self, 1)
    }
    #[doc = "Bit 2 - Operation control of operational amplifier(UNIT2)"]
    #[inline(always)]
    pub fn ampe2(&mut self) -> Ampe2W<AmpcSpec> {
        Ampe2W::new(self, 2)
    }
    #[doc = "Bit 3 - Operation control of operational amplifier(UNIT3)"]
    #[inline(always)]
    pub fn ampe3(&mut self) -> Ampe3W<AmpcSpec> {
        Ampe3W::new(self, 3)
    }
    #[doc = "Bit 7 - Operation control of operational amplifier reference current circuit"]
    #[inline(always)]
    pub fn irefe(&mut self) -> IrefeW<AmpcSpec> {
        IrefeW::new(self, 7)
    }
}
#[doc = "Operational amplifier control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ampc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpcSpec;
impl crate::RegisterSpec for AmpcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ampc::R`](R) reader structure"]
impl crate::Readable for AmpcSpec {}
#[doc = "`write(|w| ..)` method takes [`ampc::W`](W) writer structure"]
impl crate::Writable for AmpcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMPC to value 0"]
impl crate::Resettable for AmpcSpec {}
