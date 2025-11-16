#[doc = "Register `NMIER` reader"]
pub type R = crate::R<NmierSpec>;
#[doc = "Register `NMIER` writer"]
pub type W = crate::W<NmierSpec>;
#[doc = "IWDT Underflow/Refresh Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iwdten {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Iwdten> for bool {
    #[inline(always)]
    fn from(variant: Iwdten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDTEN` reader - IWDT Underflow/Refresh Error Interrupt Enable"]
pub type IwdtenR = crate::BitReader<Iwdten>;
impl IwdtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iwdten {
        match self.bits {
            false => Iwdten::_0,
            true => Iwdten::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iwdten::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iwdten::_1
    }
}
#[doc = "Field `IWDTEN` writer - IWDT Underflow/Refresh Error Interrupt Enable"]
pub type IwdtenW<'a, REG> = crate::BitWriter<'a, REG, Iwdten>;
impl<'a, REG> IwdtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdten::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iwdten::_1)
    }
}
#[doc = "WDT Underflow/Refresh Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdten {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Wdten> for bool {
    #[inline(always)]
    fn from(variant: Wdten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTEN` reader - WDT Underflow/Refresh Error Interrupt Enable"]
pub type WdtenR = crate::BitReader<Wdten>;
impl WdtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdten {
        match self.bits {
            false => Wdten::_0,
            true => Wdten::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Wdten::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Wdten::_1
    }
}
#[doc = "Field `WDTEN` writer - WDT Underflow/Refresh Error Interrupt Enable"]
pub type WdtenW<'a, REG> = crate::BitWriter<'a, REG, Wdten>;
impl<'a, REG> WdtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdten::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdten::_1)
    }
}
#[doc = "Voltage-Monitoring 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd1en {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Lvd1en> for bool {
    #[inline(always)]
    fn from(variant: Lvd1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD1EN` reader - Voltage-Monitoring 1 Interrupt Enable"]
pub type Lvd1enR = crate::BitReader<Lvd1en>;
impl Lvd1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd1en {
        match self.bits {
            false => Lvd1en::_0,
            true => Lvd1en::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd1en::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd1en::_1
    }
}
#[doc = "Field `LVD1EN` writer - Voltage-Monitoring 1 Interrupt Enable"]
pub type Lvd1enW<'a, REG> = crate::BitWriter<'a, REG, Lvd1en>;
impl<'a, REG> Lvd1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1en::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd1en::_1)
    }
}
#[doc = "Voltage-Monitoring 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd2en {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Lvd2en> for bool {
    #[inline(always)]
    fn from(variant: Lvd2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD2EN` reader - Voltage-Monitoring 2 Interrupt Enable"]
pub type Lvd2enR = crate::BitReader<Lvd2en>;
impl Lvd2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd2en {
        match self.bits {
            false => Lvd2en::_0,
            true => Lvd2en::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lvd2en::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lvd2en::_1
    }
}
#[doc = "Field `LVD2EN` writer - Voltage-Monitoring 2 Interrupt Enable"]
pub type Lvd2enW<'a, REG> = crate::BitWriter<'a, REG, Lvd2en>;
impl<'a, REG> Lvd2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2en::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvd2en::_1)
    }
}
#[doc = "VBATT monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatten {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Vbatten> for bool {
    #[inline(always)]
    fn from(variant: Vbatten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATTEN` reader - VBATT monitor Interrupt Enable"]
pub type VbattenR = crate::BitReader<Vbatten>;
impl VbattenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatten {
        match self.bits {
            false => Vbatten::_0,
            true => Vbatten::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vbatten::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vbatten::_1
    }
}
#[doc = "Field `VBATTEN` writer - VBATT monitor Interrupt Enable"]
pub type VbattenW<'a, REG> = crate::BitWriter<'a, REG, Vbatten>;
impl<'a, REG> VbattenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatten::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatten::_1)
    }
}
#[doc = "Oscillation Stop Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osten {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Osten> for bool {
    #[inline(always)]
    fn from(variant: Osten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSTEN` reader - Oscillation Stop Detection Interrupt Enable"]
pub type OstenR = crate::BitReader<Osten>;
impl OstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osten {
        match self.bits {
            false => Osten::_0,
            true => Osten::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Osten::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Osten::_1
    }
}
#[doc = "Field `OSTEN` writer - Oscillation Stop Detection Interrupt Enable"]
pub type OstenW<'a, REG> = crate::BitWriter<'a, REG, Osten>;
impl<'a, REG> OstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Osten::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Osten::_1)
    }
}
#[doc = "NMI Pin Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmien {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Nmien> for bool {
    #[inline(always)]
    fn from(variant: Nmien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIEN` reader - NMI Pin Interrupt Enable"]
pub type NmienR = crate::BitReader<Nmien>;
impl NmienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmien {
        match self.bits {
            false => Nmien::_0,
            true => Nmien::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nmien::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nmien::_1
    }
}
#[doc = "Field `NMIEN` writer - NMI Pin Interrupt Enable"]
pub type NmienW<'a, REG> = crate::BitWriter<'a, REG, Nmien>;
impl<'a, REG> NmienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmien::_1)
    }
}
#[doc = "RAM Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpeen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Rpeen> for bool {
    #[inline(always)]
    fn from(variant: Rpeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPEEN` reader - RAM Parity Error Interrupt Enable"]
pub type RpeenR = crate::BitReader<Rpeen>;
impl RpeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpeen {
        match self.bits {
            false => Rpeen::_0,
            true => Rpeen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpeen::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpeen::_1
    }
}
#[doc = "Field `RPEEN` writer - RAM Parity Error Interrupt Enable"]
pub type RpeenW<'a, REG> = crate::BitWriter<'a, REG, Rpeen>;
impl<'a, REG> RpeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeen::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpeen::_1)
    }
}
#[doc = "RAM ECC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reccen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Reccen> for bool {
    #[inline(always)]
    fn from(variant: Reccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECCEN` reader - RAM ECC Error Interrupt Enable"]
pub type ReccenR = crate::BitReader<Reccen>;
impl ReccenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reccen {
        match self.bits {
            false => Reccen::_0,
            true => Reccen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reccen::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reccen::_1
    }
}
#[doc = "Field `RECCEN` writer - RAM ECC Error Interrupt Enable"]
pub type ReccenW<'a, REG> = crate::BitWriter<'a, REG, Reccen>;
impl<'a, REG> ReccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reccen::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reccen::_1)
    }
}
#[doc = "MPU Bus Slave Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bussen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Bussen> for bool {
    #[inline(always)]
    fn from(variant: Bussen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSSEN` reader - MPU Bus Slave Error Interrupt Enable"]
pub type BussenR = crate::BitReader<Bussen>;
impl BussenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bussen {
        match self.bits {
            false => Bussen::_0,
            true => Bussen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bussen::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bussen::_1
    }
}
#[doc = "Field `BUSSEN` writer - MPU Bus Slave Error Interrupt Enable"]
pub type BussenW<'a, REG> = crate::BitWriter<'a, REG, Bussen>;
impl<'a, REG> BussenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bussen::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bussen::_1)
    }
}
#[doc = "MPU Bus Master Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busmen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Busmen> for bool {
    #[inline(always)]
    fn from(variant: Busmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSMEN` reader - MPU Bus Master Error Interrupt Enable"]
pub type BusmenR = crate::BitReader<Busmen>;
impl BusmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busmen {
        match self.bits {
            false => Busmen::_0,
            true => Busmen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Busmen::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Busmen::_1
    }
}
#[doc = "Field `BUSMEN` writer - MPU Bus Master Error Interrupt Enable"]
pub type BusmenW<'a, REG> = crate::BitWriter<'a, REG, Busmen>;
impl<'a, REG> BusmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Busmen::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Busmen::_1)
    }
}
#[doc = "CPU Stack pointer monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<Speen> for bool {
    #[inline(always)]
    fn from(variant: Speen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPEEN` reader - CPU Stack pointer monitor Interrupt Enable"]
pub type SpeenR = crate::BitReader<Speen>;
impl SpeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Speen {
        match self.bits {
            false => Speen::_0,
            true => Speen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Speen::_0
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Speen::_1
    }
}
#[doc = "Field `SPEEN` writer - CPU Stack pointer monitor Interrupt Enable"]
pub type SpeenW<'a, REG> = crate::BitWriter<'a, REG, Speen>;
impl<'a, REG> SpeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Speen::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Speen::_1)
    }
}
impl R {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(&self) -> IwdtenR {
        IwdtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn wdten(&self) -> WdtenR {
        WdtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage-Monitoring 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd1en(&self) -> Lvd1enR {
        Lvd1enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage-Monitoring 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd2en(&self) -> Lvd2enR {
        Lvd2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATT monitor Interrupt Enable"]
    #[inline(always)]
    pub fn vbatten(&self) -> VbattenR {
        VbattenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn osten(&self) -> OstenR {
        OstenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(&self) -> NmienR {
        NmienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(&self) -> RpeenR {
        RpeenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn reccen(&self) -> ReccenR {
        ReccenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPU Bus Slave Error Interrupt Enable"]
    #[inline(always)]
    pub fn bussen(&self) -> BussenR {
        BussenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPU Bus Master Error Interrupt Enable"]
    #[inline(always)]
    pub fn busmen(&self) -> BusmenR {
        BusmenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack pointer monitor Interrupt Enable"]
    #[inline(always)]
    pub fn speen(&self) -> SpeenR {
        SpeenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMIER")
            .field("speen", &self.speen())
            .field("busmen", &self.busmen())
            .field("bussen", &self.bussen())
            .field("reccen", &self.reccen())
            .field("rpeen", &self.rpeen())
            .field("nmien", &self.nmien())
            .field("osten", &self.osten())
            .field("vbatten", &self.vbatten())
            .field("lvd2en", &self.lvd2en())
            .field("lvd1en", &self.lvd1en())
            .field("wdten", &self.wdten())
            .field("iwdten", &self.iwdten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn iwdten(&mut self) -> IwdtenW<NmierSpec> {
        IwdtenW::new(self, 0)
    }
    #[doc = "Bit 1 - WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WdtenW<NmierSpec> {
        WdtenW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage-Monitoring 1 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd1en(&mut self) -> Lvd1enW<NmierSpec> {
        Lvd1enW::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage-Monitoring 2 Interrupt Enable"]
    #[inline(always)]
    pub fn lvd2en(&mut self) -> Lvd2enW<NmierSpec> {
        Lvd2enW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATT monitor Interrupt Enable"]
    #[inline(always)]
    pub fn vbatten(&mut self) -> VbattenW<NmierSpec> {
        VbattenW::new(self, 4)
    }
    #[doc = "Bit 6 - Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn osten(&mut self) -> OstenW<NmierSpec> {
        OstenW::new(self, 6)
    }
    #[doc = "Bit 7 - NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub fn nmien(&mut self) -> NmienW<NmierSpec> {
        NmienW::new(self, 7)
    }
    #[doc = "Bit 8 - RAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn rpeen(&mut self) -> RpeenW<NmierSpec> {
        RpeenW::new(self, 8)
    }
    #[doc = "Bit 9 - RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    pub fn reccen(&mut self) -> ReccenW<NmierSpec> {
        ReccenW::new(self, 9)
    }
    #[doc = "Bit 10 - MPU Bus Slave Error Interrupt Enable"]
    #[inline(always)]
    pub fn bussen(&mut self) -> BussenW<NmierSpec> {
        BussenW::new(self, 10)
    }
    #[doc = "Bit 11 - MPU Bus Master Error Interrupt Enable"]
    #[inline(always)]
    pub fn busmen(&mut self) -> BusmenW<NmierSpec> {
        BusmenW::new(self, 11)
    }
    #[doc = "Bit 12 - CPU Stack pointer monitor Interrupt Enable"]
    #[inline(always)]
    pub fn speen(&mut self) -> SpeenW<NmierSpec> {
        SpeenW::new(self, 12)
    }
}
#[doc = "Non-Maskable Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmierSpec;
impl crate::RegisterSpec for NmierSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`nmier::R`](R) reader structure"]
impl crate::Readable for NmierSpec {}
#[doc = "`write(|w| ..)` method takes [`nmier::W`](W) writer structure"]
impl crate::Writable for NmierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMIER to value 0"]
impl crate::Resettable for NmierSpec {}
