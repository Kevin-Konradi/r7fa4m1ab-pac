#[doc = "Register `ICCR1` reader"]
pub type R = crate::R<Iccr1Spec>;
#[doc = "Register `ICCR1` writer"]
pub type W = crate::W<Iccr1Spec>;
#[doc = "SDA Line Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdai {
    #[doc = "0: SDAn line is low."]
    _0 = 0,
    #[doc = "1: SDAn line is high."]
    _1 = 1,
}
impl From<Sdai> for bool {
    #[inline(always)]
    fn from(variant: Sdai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAI` reader - SDA Line Monitor"]
pub type SdaiR = crate::BitReader<Sdai>;
impl SdaiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdai {
        match self.bits {
            false => Sdai::_0,
            true => Sdai::_1,
        }
    }
    #[doc = "SDAn line is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdai::_0
    }
    #[doc = "SDAn line is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdai::_1
    }
}
#[doc = "SCL Line Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scli {
    #[doc = "0: SCLn line is low."]
    _0 = 0,
    #[doc = "1: SCLn line is high."]
    _1 = 1,
}
impl From<Scli> for bool {
    #[inline(always)]
    fn from(variant: Scli) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLI` reader - SCL Line Monitor"]
pub type ScliR = crate::BitReader<Scli>;
impl ScliR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scli {
        match self.bits {
            false => Scli::_0,
            true => Scli::_1,
        }
    }
    #[doc = "SCLn line is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scli::_0
    }
    #[doc = "SCLn line is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scli::_1
    }
}
#[doc = "SDA Output Control/Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdao {
    #[doc = "0: (Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low."]
    _0 = 0,
    #[doc = "1: (Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin."]
    _1 = 1,
}
impl From<Sdao> for bool {
    #[inline(always)]
    fn from(variant: Sdao) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAO` reader - SDA Output Control/Monitor"]
pub type SdaoR = crate::BitReader<Sdao>;
impl SdaoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdao {
        match self.bits {
            false => Sdao::_0,
            true => Sdao::_1,
        }
    }
    #[doc = "(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdao::_0
    }
    #[doc = "(Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdao::_1
    }
}
#[doc = "Field `SDAO` writer - SDA Output Control/Monitor"]
pub type SdaoW<'a, REG> = crate::BitWriter<'a, REG, Sdao>;
impl<'a, REG> SdaoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Read)The RIIC has driven the SDAn pin low. / (Write)The RIIC drives the SDAn pin low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdao::_0)
    }
    #[doc = "(Read)The RIIC has released the SDAn pin./ (Write)The RIIC releases the SDAn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdao::_1)
    }
}
#[doc = "SCL Output Control/Monitor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sclo {
    #[doc = "0: (Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low."]
    _0 = 0,
    #[doc = "1: (Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin."]
    _1 = 1,
}
impl From<Sclo> for bool {
    #[inline(always)]
    fn from(variant: Sclo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLO` reader - SCL Output Control/Monitor"]
pub type ScloR = crate::BitReader<Sclo>;
impl ScloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclo {
        match self.bits {
            false => Sclo::_0,
            true => Sclo::_1,
        }
    }
    #[doc = "(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sclo::_0
    }
    #[doc = "(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sclo::_1
    }
}
#[doc = "Field `SCLO` writer - SCL Output Control/Monitor"]
pub type ScloW<'a, REG> = crate::BitWriter<'a, REG, Sclo>;
impl<'a, REG> ScloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(Read)The RIIC has driven the SCLn pin low. / (Write)The RIIC drives the SCLn pin low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sclo::_0)
    }
    #[doc = "(Read)The RIIC has released the SCLn pin. / (Write)The RIIC releases the SCLn pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclo::_1)
    }
}
#[doc = "SCLO/SDAO Write Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sowp {
    #[doc = "0: Bits SCLO and SDAO can be written"]
    _0 = 0,
    #[doc = "1: Bits SCLO and SDAO are protected."]
    _1 = 1,
}
impl From<Sowp> for bool {
    #[inline(always)]
    fn from(variant: Sowp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOWP` reader - SCLO/SDAO Write Protect"]
pub type SowpR = crate::BitReader<Sowp>;
impl SowpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sowp {
        match self.bits {
            false => Sowp::_0,
            true => Sowp::_1,
        }
    }
    #[doc = "Bits SCLO and SDAO can be written"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sowp::_0
    }
    #[doc = "Bits SCLO and SDAO are protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sowp::_1
    }
}
#[doc = "Field `SOWP` writer - SCLO/SDAO Write Protect"]
pub type SowpW<'a, REG> = crate::BitWriter<'a, REG, Sowp>;
impl<'a, REG> SowpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bits SCLO and SDAO can be written"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sowp::_0)
    }
    #[doc = "Bits SCLO and SDAO are protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sowp::_1)
    }
}
#[doc = "Extra SCL Clock Cycle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clo {
    #[doc = "0: Does not output an extra SCL clock cycle."]
    _0 = 0,
    #[doc = "1: Outputs an extra SCL clock cycle."]
    _1 = 1,
}
impl From<Clo> for bool {
    #[inline(always)]
    fn from(variant: Clo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLO` reader - Extra SCL Clock Cycle Output"]
pub type CloR = crate::BitReader<Clo>;
impl CloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clo {
        match self.bits {
            false => Clo::_0,
            true => Clo::_1,
        }
    }
    #[doc = "Does not output an extra SCL clock cycle."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Clo::_0
    }
    #[doc = "Outputs an extra SCL clock cycle."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Clo::_1
    }
}
#[doc = "Field `CLO` writer - Extra SCL Clock Cycle Output"]
pub type CloW<'a, REG> = crate::BitWriter<'a, REG, Clo>;
impl<'a, REG> CloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not output an extra SCL clock cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Clo::_0)
    }
    #[doc = "Outputs an extra SCL clock cycle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Clo::_1)
    }
}
#[doc = "I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicrst {
    #[doc = "0: Releases the RIIC reset or internal reset."]
    _0 = 0,
    #[doc = "1: Initiates the RIIC reset or internal reset."]
    _1 = 1,
}
impl From<Iicrst> for bool {
    #[inline(always)]
    fn from(variant: Iicrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICRST` reader - I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
pub type IicrstR = crate::BitReader<Iicrst>;
impl IicrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicrst {
        match self.bits {
            false => Iicrst::_0,
            true => Iicrst::_1,
        }
    }
    #[doc = "Releases the RIIC reset or internal reset."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicrst::_0
    }
    #[doc = "Initiates the RIIC reset or internal reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicrst::_1
    }
}
#[doc = "Field `IICRST` writer - I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
pub type IicrstW<'a, REG> = crate::BitWriter<'a, REG, Iicrst>;
impl<'a, REG> IicrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Releases the RIIC reset or internal reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrst::_0)
    }
    #[doc = "Initiates the RIIC reset or internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicrst::_1)
    }
}
#[doc = "I2C Bus Interface Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ice {
    #[doc = "0: Disable (SCLn and SDAn pins in inactive state)"]
    _0 = 0,
    #[doc = "1: Enable (SCLn and SDAn pins in active state)"]
    _1 = 1,
}
impl From<Ice> for bool {
    #[inline(always)]
    fn from(variant: Ice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICE` reader - I2C Bus Interface Enable"]
pub type IceR = crate::BitReader<Ice>;
impl IceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ice {
        match self.bits {
            false => Ice::_0,
            true => Ice::_1,
        }
    }
    #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ice::_0
    }
    #[doc = "Enable (SCLn and SDAn pins in active state)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ice::_1
    }
}
#[doc = "Field `ICE` writer - I2C Bus Interface Enable"]
pub type IceW<'a, REG> = crate::BitWriter<'a, REG, Ice>;
impl<'a, REG> IceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (SCLn and SDAn pins in inactive state)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ice::_0)
    }
    #[doc = "Enable (SCLn and SDAn pins in active state)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ice::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SDA Line Monitor"]
    #[inline(always)]
    pub fn sdai(&self) -> SdaiR {
        SdaiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Line Monitor"]
    #[inline(always)]
    pub fn scli(&self) -> ScliR {
        ScliR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDA Output Control/Monitor"]
    #[inline(always)]
    pub fn sdao(&self) -> SdaoR {
        SdaoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL Output Control/Monitor"]
    #[inline(always)]
    pub fn sclo(&self) -> ScloR {
        ScloR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCLO/SDAO Write Protect"]
    #[inline(always)]
    pub fn sowp(&self) -> SowpR {
        SowpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn clo(&self) -> CloR {
        CloR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
    #[inline(always)]
    pub fn iicrst(&self) -> IicrstR {
        IicrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Bus Interface Enable"]
    #[inline(always)]
    pub fn ice(&self) -> IceR {
        IceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICCR1")
            .field("ice", &self.ice())
            .field("iicrst", &self.iicrst())
            .field("clo", &self.clo())
            .field("sowp", &self.sowp())
            .field("sclo", &self.sclo())
            .field("sdao", &self.sdao())
            .field("scli", &self.scli())
            .field("sdai", &self.sdai())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - SDA Output Control/Monitor"]
    #[inline(always)]
    pub fn sdao(&mut self) -> SdaoW<Iccr1Spec> {
        SdaoW::new(self, 2)
    }
    #[doc = "Bit 3 - SCL Output Control/Monitor"]
    #[inline(always)]
    pub fn sclo(&mut self) -> ScloW<Iccr1Spec> {
        ScloW::new(self, 3)
    }
    #[doc = "Bit 4 - SCLO/SDAO Write Protect"]
    #[inline(always)]
    pub fn sowp(&mut self) -> SowpW<Iccr1Spec> {
        SowpW::new(self, 4)
    }
    #[doc = "Bit 5 - Extra SCL Clock Cycle Output"]
    #[inline(always)]
    pub fn clo(&mut self) -> CloW<Iccr1Spec> {
        CloW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Bus Interface Internal Reset Note:If an internal reset is initiated using the IICRST bit for a bus hang-up occurred during communication with the master device in slave mode, the states may become different between the slave device and the master device (due to the difference in the bit counter information)."]
    #[inline(always)]
    pub fn iicrst(&mut self) -> IicrstW<Iccr1Spec> {
        IicrstW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Bus Interface Enable"]
    #[inline(always)]
    pub fn ice(&mut self) -> IceW<Iccr1Spec> {
        IceW::new(self, 7)
    }
}
#[doc = "I2C Bus Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`iccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccr1Spec;
impl crate::RegisterSpec for Iccr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iccr1::R`](R) reader structure"]
impl crate::Readable for Iccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iccr1::W`](W) writer structure"]
impl crate::Writable for Iccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCR1 to value 0x1f"]
impl crate::Resettable for Iccr1Spec {
    const RESET_VALUE: u8 = 0x1f;
}
