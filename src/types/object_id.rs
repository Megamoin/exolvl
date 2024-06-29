use crate::{Read, Write, Error};


macro_rules! define_object_id {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum ObjectId {
            $($name = $number),*
        }

        impl TryFrom<i32> for ObjectId {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(ObjectId::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&ObjectId> for i32 {
            fn from(value: &ObjectId) -> Self {
                match value {
                    $(ObjectId::$name => $number,)*
                }
            }
        }
    };
}

define_object_id!(
    Terrain = 1078723247,
    TerrainRightTriangle = -1904467130,
    TerrainRightTriangle90 = 1470394341,
    TerrainRightTriangle180 = -1488316906,
    TerrainRightTriangle270 = -464756446,
    TerrainBridgeTop = -75331496,
    TerrainBridgeMiddle = -2147154161,
    TerrainBridgeBottom = 1455964140,
    TerrainWallLeft = -1695726836,
    TerrainWallRight = 1899249117,
    TerrainWallCenter = 1848566125,
    SpawnRight = -1356673834,
    SpawnLeft = -2104783614,
    EndFlag = -957301678,
    EndFlag90 = -368554856,
    EndFlag180 = 1616903581,
    EndFlag270 = -174650371,
    Grabber = 1873811173,
    GrabberFlip = 2016994698,
    Grabber90 = -536717004,
    GrabberFlip90 = 759250406,
    Bumper = 968991814,
    Bumper90 = 1256971976,
    Bumper180 = 286166962,
    Bumper270 = 678667577,
    Switcher = 1209212500,
    SwitcherFlip = 970946753,
    Switcher90 = -1930559234,
    SwitcherFlip90 = -1316016952,
    Switcher180 = -1885291810,
    SwitcherFlip180 = -1730457138,
    Switcher270 = 79288520,
    SwitcherFlip270 = -1829023042,
    Booster = -1162017657,
    BoosterFlip = 980351413,
    Booster90 = -2064406063,
    BoosterFlip90 = -1493226159,
    Booster180 = 1125065540,
    BoosterFlip180 = -2023480174,
    Booster270 = 39055262,
    BoosterFlip270 = -329653198,
    HookAnchor = 368289199,
    DoubleJumper = 1487075964,
    DoubleJumper90 = 1646408642,
    DoubleJumper180 = 2115137254,
    DoubleJumper270 = -385295808,
    Dasher = -956427239,
    Dasher90 = -1901440810,
    Dasher180 = -618576420,
    Dasher270 = 1457249366,
    Fan = 1652957997,
    Fan90 = 1954424648,
    Fan180 = 1603987169,
    Fan270 = -886048645,
    FloatingZone = 2127631306,
    Slingshot = 358722333,
    Slingshot90 = 1308719934,
    Slingshot180 = 1611983220,
    Slingshot270 = 1021410890,
    Button = 2032583278,
    Button90 = 1092404659,
    Button180 = 919160669,
    Button270 = 1399300457,
    Door = -433729213,
    Door90 = 581002357,
    RevivePad = 1135615979,
    RevivePad90 = 667829449,
    RevivePad180 = 371982216,
    RevivePad270 = 698312464,
    Checkpoint = 1127718751,
    Checkpoint90 = -1195788002,
    Checkpoint180 = -1241376005,
    Checkpoint270 = -1381750767,
    CheckpointOrb = -152083769,
    GravityPortal = 2064547365,
    GravityPortal90 = 2116407709,
    Ice = 95636698,
    IceRightTriangle = 2077933161,
    IceRightTriangle90 = 1480317903,
    IceRightTriangle180 = 1699758979,
    IceRightTriangle270 = 1746446512,
    IceSlopeIn = 1191037697,
    IceSlopeInFlip = -1979821018,
    IceSlopeIn90 = -2135841768,
    IceSlopeInFlip90 = 1656564393,
    IceSlopeIn180 = 714706500,
    IceSlopeInFlip180 = 533396868,
    IceSlopeIn270 = -1285793553,
    IceSlopeInFlip270 = -1492534997,
    IceSlopeOut = -1885590070,
    IceSlopeOutFlip = -273598634,
    IceSlopeOut90 = 741289287,
    IceSlopeOutFlip90 = 302597122,
    IceSlopeOut180 = 2004228293,
    IceSlopeOutFlip180 = -989728521,
    IceSlopeOut270 = -1868478563,
    IceSlopeOutFlip270 = -1406297645,
    IceSlope = -2071698530,
    IceSlope90 = 258071606,
    IceSlope180 = 2036775213,
    IceSlope270 = 1393198333,
    

    KillerSpike = -1832408413,
    KillerSpike90 = -1789668358,
    KillerSpike180 = -1800631278,
    KillerSpike270 = 1422770095,
    KillerBlock = -123683330,
    KillerSaw = 85353959,
    KillerSaw90 = -84282781,
    KillerSaw180 = -1790928268,
    KillerSaw270 = 1941275399,
    KillerSawBig = -1385136225,
    KillerSawBig90 = 1609812870,
    KillerSawBig180 = -1582901038,
    KillerSawBig270 = -345312146,
    KillerSawFull = -1358408877,
    KillerSawFullBig = -1727849296,
    SpriteSquare = 113491821,
    SpriteSquareOutline = 1296081014,
    SpriteRoundedSquare = -1718767673,
    SpriteRoundedSquareOutline = -168878848,
    SpriteCircle = -284493993,
    SpriteCircleOutline = 2044847310,
    SpriteSemicircle = 162939366,
    SpriteSemicircle90 = -619270393,
    SpriteSemicircle180 = 936710871,
    SpriteSemicircle270 = -2122278412,
    SpriteSemicircleOutline = -1628209802,
    SpriteSemicircleOutline90 = -1140350636,
    SpriteSemicircleOutline180 = 278022153,
    SpriteSemicircleOutline270 = 118137802,
    SpriteQuartercircle = 1417850755,
    SpriteQuartercircle90 = 1272863854,
    SpriteQuartercircle180 = 1318335448,
    SpriteQuartercircle270 = 1714711440,
    SpriteQuartercircleOutline = -2086966633,
    SpriteQuartercircleOutline90 = -2125743891,
    SpriteQuartercircleOutline180 = 1401774077,
    SpriteQuartercircleOutline270 = -359202566,
    SpriteTriangle = -2029048382,
    SpriteTriangle90 = 1217578697,
    SpriteTriangle180 = -1189381130,
    SpriteTriangle270 = 1849138844,
    SpriteRightTriangle = -319845761,
    SpriteRightTriangle90 = 64470063,
    SpriteRightTriangle180 = 120767220,
    SpriteRightTriangle270 = -485054060,
    SpritePentagon = -27695691,
    SpriteHexagon = -2142882175,
    SpriteHeptagon = -1221109328,
    SpriteOctagon = -1838047745,
    SpriteTrapezoid = -861526936,
    SpriteTrapezoid90 = 1718359304,
    SpriteTrapezoid180 = -1030637625,
    SpriteTrapezoid270 = 805033788,
    SpriteStar = -1686819095,
    SpriteStar4 = 512116583,
    SpriteStar6 = 1082987055,
    SpriteCross = -131202067,
    SpriteHeart = 471260314,
    SpriteMoon = -1624426220,
    SpriteMoonFlip = -1067600395,
    SpriteRhombus = 1539249723,
    SpriteDrop = 1568183189,
    SpriteSlope = 444076002,
    SpriteSlope90 = -310297049,
    SpriteSlope180 = -684763610,
    SpriteSlope270 = 2089430495,
    Image = 1718870758,
    Text = 589791300,
    Unit = 1787069168,
    Area = -1658710071,
    Point = 1514078320,
    ParticleSystem = -2072865068,
    Group = 1944401040
);

impl Read for ObjectId {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidOldActionType(value))
    }
}

impl Write for ObjectId {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
    }
}