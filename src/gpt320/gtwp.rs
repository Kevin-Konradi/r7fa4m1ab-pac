#[doc = "Register `GTWP` reader"]
pub type R = crate::R<GtwpSpec>;
#[doc = "Register `GTWP` writer"]
pub type W = crate::W<GtwpSpec>;
#[doc = "Register Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wp {
    #[doc = "0: Write to the register is enabled"]
    _0 = 0,
    #[doc = "1: Write to the register is disabled"]
    _1 = 1,
}
impl From<Wp> for bool {
    #[inline(always)]
    fn from(variant: Wp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP` reader - Register Write Disable"]
pub type WpR = crate::BitReader<Wp>;
impl WpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wp {
        match self.bits {
            false => Wp::_0,
            true => Wp::_1,
        }
    }
    #[doc = "Write to the register is enabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wp::_0
    }
    #[doc = "Write to the register is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wp::_1
    }
}
#[doc = "Field `WP` writer - Register Write Disable"]
pub type WpW<'a, REG> = crate::BitWriter<'a, REG, Wp>;
impl<'a, REG> WpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write to the register is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_0)
    }
    #[doc = "Write to the register is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wp::_1)
    }
}
#[doc = "GTWP Key Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prkey {
    #[doc = "165: Written to these bits, the WP bits write is permitted."]
    _0xA5 = 165,
    #[doc = "0: The WP bits write is not permitted."]
    Others = 0,
}
impl From<Prkey> for u8 {
    #[inline(always)]
    fn from(variant: Prkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prkey {
    type Ux = u8;
}
impl crate::IsEnum for Prkey {}
#[doc = "Field `PRKEY` writer - GTWP Key Code"]
pub type PrkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Prkey, crate::Safe>;
impl<'a, REG> PrkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Written to these bits, the WP bits write is permitted."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(Prkey::_0xA5)
    }
    #[doc = "The WP bits write is not permitted."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Prkey::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GTWP").field("wp", &self.wp()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<GtwpSpec> {
        WpW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GTWP Key Code"]
    #[inline(always)]
    pub fn prkey(&mut self) -> PrkeyW<GtwpSpec> {
        PrkeyW::new(self, 8)
    }
}
#[doc = "General PWM Timer Write-Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtwpSpec;
impl crate::RegisterSpec for GtwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtwp::R`](R) reader structure"]
impl crate::Readable for GtwpSpec {}
#[doc = "`write(|w| ..)` method takes [`gtwp::W`](W) writer structure"]
impl crate::Writable for GtwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTWP to value 0"]
impl crate::Resettable for GtwpSpec {}
