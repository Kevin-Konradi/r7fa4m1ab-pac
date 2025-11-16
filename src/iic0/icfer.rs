#[doc = "Register `ICFER` reader"]
pub type R = crate::R<IcferSpec>;
#[doc = "Register `ICFER` writer"]
pub type W = crate::W<IcferSpec>;
#[doc = "Timeout Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmoe {
    #[doc = "0: The timeout function is disabled."]
    _0 = 0,
    #[doc = "1: The timeout function is enabled."]
    _1 = 1,
}
impl From<Tmoe> for bool {
    #[inline(always)]
    fn from(variant: Tmoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMOE` reader - Timeout Function Enable"]
pub type TmoeR = crate::BitReader<Tmoe>;
impl TmoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmoe {
        match self.bits {
            false => Tmoe::_0,
            true => Tmoe::_1,
        }
    }
    #[doc = "The timeout function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmoe::_0
    }
    #[doc = "The timeout function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmoe::_1
    }
}
#[doc = "Field `TMOE` writer - Timeout Function Enable"]
pub type TmoeW<'a, REG> = crate::BitWriter<'a, REG, Tmoe>;
impl<'a, REG> TmoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timeout function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoe::_0)
    }
    #[doc = "The timeout function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmoe::_1)
    }
}
#[doc = "Master Arbitration-Lost Detection Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Male {
    #[doc = "0: Master arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Master arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<Male> for bool {
    #[inline(always)]
    fn from(variant: Male) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MALE` reader - Master Arbitration-Lost Detection Enable"]
pub type MaleR = crate::BitReader<Male>;
impl MaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Male {
        match self.bits {
            false => Male::_0,
            true => Male::_1,
        }
    }
    #[doc = "Master arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Male::_0
    }
    #[doc = "Master arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Male::_1
    }
}
#[doc = "Field `MALE` writer - Master Arbitration-Lost Detection Enable"]
pub type MaleW<'a, REG> = crate::BitWriter<'a, REG, Male>;
impl<'a, REG> MaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Male::_0)
    }
    #[doc = "Master arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Male::_1)
    }
}
#[doc = "NACK Transmission Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nale {
    #[doc = "0: NACK transmission arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: NACK transmission arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<Nale> for bool {
    #[inline(always)]
    fn from(variant: Nale) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NaleR = crate::BitReader<Nale>;
impl NaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nale {
        match self.bits {
            false => Nale::_0,
            true => Nale::_1,
        }
    }
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nale::_0
    }
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nale::_1
    }
}
#[doc = "Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NaleW<'a, REG> = crate::BitWriter<'a, REG, Nale>;
impl<'a, REG> NaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nale::_0)
    }
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nale::_1)
    }
}
#[doc = "Slave Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sale {
    #[doc = "0: Slave arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Slave arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<Sale> for bool {
    #[inline(always)]
    fn from(variant: Sale) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SALE` reader - Slave Arbitration-Lost Detection Enable"]
pub type SaleR = crate::BitReader<Sale>;
impl SaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sale {
        match self.bits {
            false => Sale::_0,
            true => Sale::_1,
        }
    }
    #[doc = "Slave arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sale::_0
    }
    #[doc = "Slave arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sale::_1
    }
}
#[doc = "Field `SALE` writer - Slave Arbitration-Lost Detection Enable"]
pub type SaleW<'a, REG> = crate::BitWriter<'a, REG, Sale>;
impl<'a, REG> SaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sale::_0)
    }
    #[doc = "Slave arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sale::_1)
    }
}
#[doc = "NACK Reception Transfer Suspension Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nacke {
    #[doc = "0: Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    _0 = 0,
    #[doc = "1: Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    _1 = 1,
}
impl From<Nacke> for bool {
    #[inline(always)]
    fn from(variant: Nacke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKE` reader - NACK Reception Transfer Suspension Enable"]
pub type NackeR = crate::BitReader<Nacke>;
impl NackeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nacke {
        match self.bits {
            false => Nacke::_0,
            true => Nacke::_1,
        }
    }
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nacke::_0
    }
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nacke::_1
    }
}
#[doc = "Field `NACKE` writer - NACK Reception Transfer Suspension Enable"]
pub type NackeW<'a, REG> = crate::BitWriter<'a, REG, Nacke>;
impl<'a, REG> NackeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nacke::_0)
    }
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nacke::_1)
    }
}
#[doc = "Digital Noise Filter Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfe {
    #[doc = "0: No digital noise filter circuit is used."]
    _0 = 0,
    #[doc = "1: A digital noise filter circuit is used."]
    _1 = 1,
}
impl From<Nfe> for bool {
    #[inline(always)]
    fn from(variant: Nfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFE` reader - Digital Noise Filter Circuit Enable"]
pub type NfeR = crate::BitReader<Nfe>;
impl NfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfe {
        match self.bits {
            false => Nfe::_0,
            true => Nfe::_1,
        }
    }
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfe::_0
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfe::_1
    }
}
#[doc = "Field `NFE` writer - Digital Noise Filter Circuit Enable"]
pub type NfeW<'a, REG> = crate::BitWriter<'a, REG, Nfe>;
impl<'a, REG> NfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfe::_0)
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfe::_1)
    }
}
#[doc = "SCL Synchronous Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scle {
    #[doc = "0: No SCL synchronous circuit is used."]
    _0 = 0,
    #[doc = "1: An SCL synchronous circuit is used."]
    _1 = 1,
}
impl From<Scle> for bool {
    #[inline(always)]
    fn from(variant: Scle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLE` reader - SCL Synchronous Circuit Enable"]
pub type ScleR = crate::BitReader<Scle>;
impl ScleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scle {
        match self.bits {
            false => Scle::_0,
            true => Scle::_1,
        }
    }
    #[doc = "No SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scle::_0
    }
    #[doc = "An SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scle::_1
    }
}
#[doc = "Field `SCLE` writer - SCL Synchronous Circuit Enable"]
pub type ScleW<'a, REG> = crate::BitWriter<'a, REG, Scle>;
impl<'a, REG> ScleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Scle::_0)
    }
    #[doc = "An SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Scle::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    pub fn tmoe(&self) -> TmoeR {
        TmoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(&self) -> MaleR {
        MaleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(&self) -> NaleR {
        NaleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(&self) -> SaleR {
        SaleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub fn nacke(&self) -> NackeR {
        NackeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn nfe(&self) -> NfeR {
        NfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scle(&self) -> ScleR {
        ScleR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICFER")
            .field("scle", &self.scle())
            .field("nfe", &self.nfe())
            .field("nacke", &self.nacke())
            .field("sale", &self.sale())
            .field("nale", &self.nale())
            .field("male", &self.male())
            .field("tmoe", &self.tmoe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    pub fn tmoe(&mut self) -> TmoeW<IcferSpec> {
        TmoeW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(&mut self) -> MaleW<IcferSpec> {
        MaleW::new(self, 1)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(&mut self) -> NaleW<IcferSpec> {
        NaleW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(&mut self) -> SaleW<IcferSpec> {
        SaleW::new(self, 3)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub fn nacke(&mut self) -> NackeW<IcferSpec> {
        NackeW::new(self, 4)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn nfe(&mut self) -> NfeW<IcferSpec> {
        NfeW::new(self, 5)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scle(&mut self) -> ScleW<IcferSpec> {
        ScleW::new(self, 6)
    }
}
#[doc = "I2C Bus Function Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icfer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcferSpec;
impl crate::RegisterSpec for IcferSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icfer::R`](R) reader structure"]
impl crate::Readable for IcferSpec {}
#[doc = "`write(|w| ..)` method takes [`icfer::W`](W) writer structure"]
impl crate::Writable for IcferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICFER to value 0x72"]
impl crate::Resettable for IcferSpec {
    const RESET_VALUE: u8 = 0x72;
}
