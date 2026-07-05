use strum::{Display, EnumString, FromRepr};

/// ID3v1 genre list (codes 0–191).
///
/// Variants map 1-to-1 with their numeric discriminant so that
/// `Genre::try_from(n)` and `n as u32` are both O(1) operations.
///
/// `Display` / `FromStr` are derived via `strum`; the `to_string` attribute
/// sets the canonical display string and `serialize` lists accepted parse
/// aliases.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Display,
    EnumString,
    FromRepr,
)]
#[repr(u32)]
pub enum Genre {
    #[strum(to_string = "Blues")]
    Blues = 0,
    #[strum(to_string = "Classic Rock")]
    ClassicRock = 1,
    #[strum(to_string = "Country")]
    Country = 2,
    #[strum(to_string = "Dance")]
    Dance = 3,
    #[strum(to_string = "Disco")]
    Disco = 4,
    #[strum(to_string = "Funk")]
    Funk = 5,
    #[strum(to_string = "Grunge")]
    Grunge = 6,
    #[strum(to_string = "Hip-Hop")]
    HipHop = 7,
    #[strum(to_string = "Jazz")]
    Jazz = 8,
    #[strum(to_string = "Metal")]
    Metal = 9,
    #[strum(to_string = "New Age")]
    NewAge = 10,
    #[strum(to_string = "Oldies")]
    Oldies = 11,

    #[default]
    #[strum(to_string = "Other")]
    Other = 12,

    #[strum(to_string = "Pop")]
    Pop = 13,
    #[strum(to_string = "Rhythm and Blues", serialize = "Rhythm & Blues")]
    RhythmBlues = 14,
    #[strum(to_string = "Rap")]
    Rap = 15,
    #[strum(to_string = "Reggae")]
    Reggae = 16,
    #[strum(to_string = "Rock")]
    Rock = 17,
    #[strum(to_string = "Techno")]
    Techno = 18,
    #[strum(to_string = "Industrial")]
    Industrial = 19,
    #[strum(to_string = "Alternative")]
    Alternative = 20,
    #[strum(to_string = "Ska")]
    Ska = 21,
    #[strum(to_string = "Death Metal")]
    DeathMetal = 22,
    #[strum(to_string = "Pranks")]
    Pranks = 23,
    #[strum(to_string = "Soundtrack")]
    Soundtrack = 24,
    #[strum(to_string = "Euro-Techno")]
    EuroTechno = 25,
    #[strum(to_string = "Ambient")]
    Ambient = 26,
    #[strum(to_string = "Trip-Hop")]
    TripHop = 27,
    #[strum(to_string = "Vocal")]
    Vocal = 28,
    #[strum(to_string = "Jazz & Funk", serialize = "Jazz and Funk", serialize = "Jazz/Funk")]
    JazzFunk = 29,
    #[strum(to_string = "Fusion")]
    Fusion = 30,
    #[strum(to_string = "Trance")]
    Trance = 31,
    #[strum(to_string = "Classical")]
    Classical = 32,
    #[strum(to_string = "Instrumental")]
    Instrumental = 33,
    #[strum(to_string = "Acid")]
    Acid = 34,
    #[strum(to_string = "House")]
    House = 35,
    #[strum(to_string = "Game")]
    Game = 36,
    #[strum(to_string = "Sound Clip")]
    SoundClip = 37,
    #[strum(to_string = "Gospel")]
    Gospel = 38,
    #[strum(to_string = "Noise")]
    Noise = 39,
    #[strum(to_string = "Alternative Rock")]
    AlternativeRock = 40,
    #[strum(to_string = "Bass")]
    Bass = 41,
    #[strum(to_string = "Soul")]
    Soul = 42,
    #[strum(to_string = "Punk")]
    Punk = 43,
    #[strum(to_string = "Space")]
    Space = 44,
    #[strum(to_string = "Meditative")]
    Meditative = 45,
    #[strum(to_string = "Instrumental Pop")]
    InstrumentalPop = 46,
    #[strum(to_string = "Instrumental Rock")]
    InstrumentalRock = 47,
    #[strum(to_string = "Ethnic")]
    Ethnic = 48,
    #[strum(to_string = "Gothic")]
    Gothic = 49,
    #[strum(to_string = "Darkwave")]
    Darkwave = 50,
    #[strum(to_string = "Techno-Industrial")]
    TechnoIndustrial = 51,
    #[strum(to_string = "Electronic")]
    Electronic = 52,
    #[strum(to_string = "Pop-Folk")]
    PopFolk = 53,
    #[strum(to_string = "Eurodance")]
    Eurodance = 54,
    #[strum(to_string = "Dream")]
    Dream = 55,
    #[strum(to_string = "Southern Rock")]
    SouthernRock = 56,
    #[strum(to_string = "Comedy")]
    Comedy = 57,
    #[strum(to_string = "Cult")]
    Cult = 58,
    #[strum(to_string = "Gangsta")]
    Gangsta = 59,
    #[strum(to_string = "Top 40")]
    Top40 = 60,
    #[strum(to_string = "Christian Rap")]
    ChristianRap = 61,
    #[strum(to_string = "Pop/Funk")]
    PopFunk = 62,
    #[strum(to_string = "Jungle")]
    Jungle = 63,
    #[strum(to_string = "Native US")]
    NativeUS = 64,
    #[strum(to_string = "Cabaret")]
    Cabaret = 65,
    #[strum(to_string = "New Wave")]
    NewWave = 66,
    #[strum(to_string = "Psychedelic")]
    Psychedelic = 67,
    #[strum(to_string = "Rave")]
    Rave = 68,
    #[strum(to_string = "Show Tunes")]
    ShowTunes = 69,
    #[strum(to_string = "Trailer")]
    Trailer = 70,
    #[strum(to_string = "Lo-Fi")]
    LoFi = 71,
    #[strum(to_string = "Tribal")]
    Tribal = 72,
    #[strum(to_string = "Acid Punk")]
    AcidPunk = 73,
    #[strum(to_string = "Acid Jazz")]
    AcidJazz = 74,
    #[strum(to_string = "Polka")]
    Polka = 75,
    #[strum(to_string = "Retro")]
    Retro = 76,
    #[strum(to_string = "Musical")]
    Musical = 77,
    #[strum(to_string = "Rock 'n' Roll", serialize = "Rock and Roll", serialize = "Rock & Roll")]
    RockNRoll = 78,
    #[strum(to_string = "Hard Rock")]
    HardRock = 79,
    #[strum(to_string = "Folk")]
    Folk = 80,
    #[strum(to_string = "Folk Rock", serialize = "Folk-Rock")]
    FolkRock = 81,
    #[strum(to_string = "National Folk")]
    NationalFolk = 82,
    #[strum(to_string = "Swing")]
    Swing = 83,
    #[strum(to_string = "Fast Fusion")]
    FastFusion = 84,
    #[strum(to_string = "Bebop")]
    Bebop = 85,
    #[strum(to_string = "Latin")]
    Latin = 86,
    #[strum(to_string = "Revival")]
    Revival = 87,
    #[strum(to_string = "Celtic")]
    Celtic = 88,
    #[strum(to_string = "Bluegrass")]
    Bluegrass = 89,
    #[strum(to_string = "Avantgarde")]
    Avantgarde = 90,
    #[strum(to_string = "Gothic Rock")]
    GothicRock = 91,
    #[strum(to_string = "Progressive Rock")]
    ProgressiveRock = 92,
    #[strum(to_string = "Psychedelic Rock")]
    PsychedelicRock = 93,
    #[strum(to_string = "Symphonic Rock")]
    SymphonicRock = 94,
    #[strum(to_string = "Slow Rock")]
    SlowRock = 95,
    #[strum(to_string = "Big Band")]
    BigBand = 96,
    #[strum(to_string = "Chorus")]
    Chorus = 97,
    #[strum(to_string = "Easy Listening")]
    EasyListening = 98,
    #[strum(to_string = "Acoustic")]
    Acoustic = 99,
    #[strum(to_string = "Humour", serialize = "Humor")]
    Humour = 100,
    #[strum(to_string = "Speech")]
    Speech = 101,
    #[strum(to_string = "Chanson")]
    Chanson = 102,
    #[strum(to_string = "Opera")]
    Opera = 103,
    #[strum(to_string = "Chamber Music")]
    ChamberMusic = 104,
    #[strum(to_string = "Sonata")]
    Sonata = 105,
    #[strum(to_string = "Symphony")]
    Symphony = 106,
    #[strum(to_string = "Booty Bass")]
    BootyBass = 107,
    #[strum(to_string = "Primus")]
    Primus = 108,
    #[strum(to_string = "Porn Groove")]
    PornGroove = 109,
    #[strum(to_string = "Satire")]
    Satire = 110,
    #[strum(to_string = "Slow Jam")]
    SlowJam = 111,
    #[strum(to_string = "Club")]
    Club = 112,
    #[strum(to_string = "Tango")]
    Tango = 113,
    #[strum(to_string = "Samba")]
    Samba = 114,
    #[strum(to_string = "Folklore")]
    Folklore = 115,
    #[strum(to_string = "Ballad")]
    Ballad = 116,
    #[strum(to_string = "Power Ballad")]
    PowerBallad = 117,
    #[strum(to_string = "Rhythmic Soul")]
    RhythmicSoul = 118,
    #[strum(to_string = "Freestyle")]
    Freestyle = 119,
    #[strum(to_string = "Duet")]
    Duet = 120,
    #[strum(to_string = "Punk Rock")]
    PunkRock = 121,
    #[strum(to_string = "Drum Solo")]
    DrumSolo = 122,
    #[strum(to_string = "A Cappella")]
    ACappella = 123,
    #[strum(to_string = "Euro House")]
    EuroHouse = 124,
    #[strum(to_string = "Dancehall")]
    Dancehall = 125,
    #[strum(to_string = "Goa")]
    Goa = 126,
    #[strum(to_string = "Drum & Bass", serialize = "Drum 'n' Bass", serialize = "Drum and Bass")]
    DrumBass = 127,
    #[strum(to_string = "Club House", serialize = "Club-House")]
    ClubHouse = 128,
    #[strum(to_string = "Hardcore Techno")]
    HardcoreTechno = 129,
    #[strum(to_string = "Terror")]
    Terror = 130,
    #[strum(to_string = "Indie")]
    Indie = 131,
    #[strum(to_string = "BritPop", serialize = "Brit Pop", serialize = "Brit-Pop")]
    BritPop = 132,
    #[strum(to_string = "Negerpunk")]
    Negerpunk = 133,
    #[strum(to_string = "Polsk Punk")]
    PolskPunk = 134,
    #[strum(to_string = "Beat")]
    Beat = 135,
    #[strum(to_string = "Christian Gangsta Rap")]
    ChristianGangstaRap = 136,
    #[strum(to_string = "Heavy Metal")]
    HeavyMetal = 137,
    #[strum(to_string = "Black Metal")]
    BlackMetal = 138,
    #[strum(to_string = "Crossover")]
    Crossover = 139,
    #[strum(to_string = "Contemporary Christian")]
    ContemporaryChristian = 140,
    #[strum(to_string = "Christian Rock")]
    ChristianRock = 141,
    #[strum(to_string = "Merengue")]
    Merengue = 142,
    #[strum(to_string = "Salsa")]
    Salsa = 143,
    #[strum(to_string = "Thrash Metal")]
    ThrashMetal = 144,
    #[strum(to_string = "Anime")]
    Anime = 145,
    #[strum(to_string = "Jpop")]
    Jpop = 146,
    #[strum(to_string = "Synthpop")]
    Synthpop = 147,
    #[strum(to_string = "Abstract")]
    Abstract = 148,
    #[strum(to_string = "Art Rock")]
    ArtRock = 149,
    #[strum(to_string = "Baroque")]
    Baroque = 150,
    #[strum(to_string = "Bhangra")]
    Bhangra = 151,
    #[strum(to_string = "Big Beat")]
    BigBeat = 152,
    #[strum(to_string = "Breakbeat")]
    Breakbeat = 153,
    #[strum(to_string = "Chillout")]
    Chillout = 154,
    #[strum(to_string = "Downtempo")]
    Downtempo = 155,
    #[strum(to_string = "Dub")]
    Dub = 156,
    #[strum(to_string = "Electronic Body Music", serialize = "EBM")]
    ElectronicBodyMusic = 157,
    #[strum(to_string = "Eclectic")]
    Eclectic = 158,
    #[strum(to_string = "Electro")]
    Electro = 159,
    #[strum(to_string = "Electroclash")]
    Electroclash = 160,
    #[strum(to_string = "Emo")]
    Emo = 161,
    #[strum(to_string = "Experimental")]
    Experimental = 162,
    #[strum(to_string = "Garage")]
    Garage = 163,
    #[strum(to_string = "Global")]
    Global = 164,
    #[strum(to_string = "Intelligent Dance Music", serialize = "IDM")]
    IntelligentDanceMusic = 165,
    #[strum(to_string = "Illbient")]
    Illbient = 166,
    #[strum(to_string = "Industro-Goth")]
    IndustroGoth = 167,
    #[strum(to_string = "Jam Band")]
    JamBand = 168,
    #[strum(to_string = "Krautrock")]
    Krautrock = 169,
    #[strum(to_string = "Leftfield")]
    Leftfield = 170,
    #[strum(to_string = "Lounge")]
    Lounge = 171,
    #[strum(to_string = "Math Rock")]
    MathRock = 172,
    #[strum(to_string = "New Romantic")]
    NewRomantic = 173,
    #[strum(to_string = "Nu Breakz")]
    NuBreakz = 174,
    #[strum(to_string = "Post-Punk")]
    PostPunk = 175,
    #[strum(to_string = "Post-Rock")]
    PostRock = 176,
    #[strum(to_string = "Psytrance")]
    Psytrance = 177,
    #[strum(to_string = "Shoegaze")]
    Shoegaze = 178,
    #[strum(to_string = "Space Rock")]
    SpaceRock = 179,
    #[strum(to_string = "Trop Rock")]
    TropRock = 180,
    #[strum(to_string = "World Music")]
    WorldMusic = 181,
    #[strum(to_string = "Neoclassical")]
    Neoclassical = 182,
    #[strum(to_string = "Audiobook")]
    Audiobook = 183,
    #[strum(to_string = "Audio Theatre")]
    AudioTheatre = 184,
    #[strum(to_string = "Neue Deutche Welle")]
    NeueDeutcheWelle = 185,
    #[strum(to_string = "Podcast")]
    Podcast = 186,
    #[strum(to_string = "Indie-Rock")]
    IndieRock = 187,
    #[strum(to_string = "G-Funk")]
    GFunk = 188,
    #[strum(to_string = "Dubstep")]
    Dubstep = 189,
    #[strum(to_string = "Garage Rock")]
    GarageRock = 190,
    #[strum(to_string = "Psybient")]
    Psybient = 191,
}

impl TryFrom<u32> for Genre {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Genre::from_repr(v).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Display ---

    #[test]
    fn display_simple_single_word() {
        assert_eq!(Genre::Blues.to_string(), "Blues");
        assert_eq!(Genre::Jazz.to_string(), "Jazz");
        assert_eq!(Genre::Reggae.to_string(), "Reggae");
    }

    #[test]
    fn display_multi_word() {
        assert_eq!(Genre::ClassicRock.to_string(), "Classic Rock");
        assert_eq!(Genre::HardRock.to_string(), "Hard Rock");
        assert_eq!(Genre::EasyListening.to_string(), "Easy Listening");
    }

    #[test]
    fn display_hyphenated() {
        assert_eq!(Genre::HipHop.to_string(), "Hip-Hop");
        assert_eq!(Genre::TripHop.to_string(), "Trip-Hop");
        assert_eq!(Genre::EuroTechno.to_string(), "Euro-Techno");
        assert_eq!(Genre::LoFi.to_string(), "Lo-Fi");
        assert_eq!(Genre::PostPunk.to_string(), "Post-Punk");
        assert_eq!(Genre::PostRock.to_string(), "Post-Rock");
    }

    #[test]
    fn display_special_characters() {
        // RhythmBlues: canonical display is "Rhythm and Blues"; "Rhythm & Blues" is a parse alias only
        assert_eq!(Genre::RhythmBlues.to_string(), "Rhythm and Blues");
        assert_eq!(Genre::JazzFunk.to_string(), "Jazz & Funk");
        assert_eq!(Genre::RockNRoll.to_string(), "Rock 'n' Roll");
        assert_eq!(Genre::DrumBass.to_string(), "Drum & Bass");
        assert_eq!(Genre::ACappella.to_string(), "A Cappella");
    }

    #[test]
    fn display_default_is_other() {
        assert_eq!(Genre::Other.to_string(), "Other");
        assert_eq!(Genre::default().to_string(), "Other");
    }

    // --- FromStr ---

    #[test]
    fn from_str_canonical_strings() {
        assert_eq!("Blues".parse::<Genre>(), Ok(Genre::Blues));
        assert_eq!("Classic Rock".parse::<Genre>(), Ok(Genre::ClassicRock));
        assert_eq!("Hip-Hop".parse::<Genre>(), Ok(Genre::HipHop));
        assert_eq!("Jazz & Funk".parse::<Genre>(), Ok(Genre::JazzFunk));
        assert_eq!("Rock 'n' Roll".parse::<Genre>(), Ok(Genre::RockNRoll));
        assert_eq!("Drum & Bass".parse::<Genre>(), Ok(Genre::DrumBass));
        assert_eq!("Other".parse::<Genre>(), Ok(Genre::Other));
    }

    #[test]
    fn from_str_accepted_aliases() {
        // RhythmBlues has two accepted spellings
        assert_eq!("Rhythm & Blues".parse::<Genre>(), Ok(Genre::RhythmBlues));
        assert_eq!("Rhythm and Blues".parse::<Genre>(), Ok(Genre::RhythmBlues));

        // JazzFunk has three
        assert_eq!("Jazz and Funk".parse::<Genre>(), Ok(Genre::JazzFunk));
        assert_eq!("Jazz/Funk".parse::<Genre>(), Ok(Genre::JazzFunk));

        // RockNRoll has two extras
        assert_eq!("Rock and Roll".parse::<Genre>(), Ok(Genre::RockNRoll));
        assert_eq!("Rock & Roll".parse::<Genre>(), Ok(Genre::RockNRoll));

        // Humour/Humor
        assert_eq!("Humor".parse::<Genre>(), Ok(Genre::Humour));

        // DrumBass
        assert_eq!("Drum 'n' Bass".parse::<Genre>(), Ok(Genre::DrumBass));
        assert_eq!("Drum and Bass".parse::<Genre>(), Ok(Genre::DrumBass));

        // BritPop
        assert_eq!("Brit Pop".parse::<Genre>(), Ok(Genre::BritPop));
        assert_eq!("Brit-Pop".parse::<Genre>(), Ok(Genre::BritPop));

        // Electronic Body Music
        assert_eq!("EBM".parse::<Genre>(), Ok(Genre::ElectronicBodyMusic));

        // IDM
        assert_eq!("IDM".parse::<Genre>(), Ok(Genre::IntelligentDanceMusic));

        // FolkRock: strum also makes the to_string value ("Folk Rock") a valid parse input,
        // so "Folk Rock" now parses successfully. Previously only "Folk-Rock" was accepted.
        // This is an intentional backwards-compatible expansion.
        assert_eq!("Folk Rock".parse::<Genre>(), Ok(Genre::FolkRock));
        assert_eq!("Folk-Rock".parse::<Genre>(), Ok(Genre::FolkRock));
    }

    #[test]
    fn from_str_unknown_string_is_err() {
        assert!("NotAGenre".parse::<Genre>().is_err());
        assert!("blues".parse::<Genre>().is_err()); // case-sensitive
        assert!("".parse::<Genre>().is_err());
    }

    // --- TryFrom<u32> ---

    #[test]
    fn try_from_u32_boundaries() {
        assert_eq!(Genre::try_from(0u32), Ok(Genre::Blues));
        assert_eq!(Genre::try_from(12u32), Ok(Genre::Other));
        assert_eq!(Genre::try_from(191u32), Ok(Genre::Psybient));
    }

    #[test]
    fn try_from_u32_spot_checks() {
        assert_eq!(Genre::try_from(17u32), Ok(Genre::Rock));
        assert_eq!(Genre::try_from(32u32), Ok(Genre::Classical));
        assert_eq!(Genre::try_from(80u32), Ok(Genre::Folk));
        assert_eq!(Genre::try_from(147u32), Ok(Genre::Synthpop));
    }

    #[test]
    fn try_from_u32_out_of_range_is_err() {
        assert!(Genre::try_from(192u32).is_err());
        assert!(Genre::try_from(u32::MAX).is_err());
    }

    // --- Default ---

    #[test]
    fn default_is_other() {
        assert_eq!(Genre::default(), Genre::Other);
    }

    // --- Round-trip ---

    #[test]
    fn display_then_from_str_round_trips() {
        let genres = [
            Genre::Blues,
            Genre::ClassicRock,
            Genre::HipHop,
            Genre::JazzFunk,
            Genre::RockNRoll,
            Genre::Humour,
            Genre::DrumBass,
            Genre::ACappella,
            Genre::ElectronicBodyMusic,
            Genre::IntelligentDanceMusic,
            Genre::Psybient,
        ];
        for g in genres {
            let s = g.to_string();
            let parsed: Genre = s.parse().unwrap_or_else(|_| panic!("failed to parse '{s}'"));
            assert_eq!(parsed, g, "round-trip failed for {g}");
        }
    }
}
