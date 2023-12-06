pub struct Mapping {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

pub struct Almanac<
    const N: usize,
    const S: usize,
    const F: usize,
    const W: usize,
    const L: usize,
    const T: usize,
    const H: usize,
    const M: usize,
> {
    pub seeds: [u64; N],
    pub seed_to_soil: [Mapping; S],
    pub soil_to_fertilizer: [Mapping; F],
    pub fertilizer_to_water: [Mapping; W],
    pub water_to_light: [Mapping; L],
    pub light_to_temperature: [Mapping; T],
    pub temperature_to_humidity: [Mapping; H],
    pub humidity_to_location: [Mapping; M],
}

pub const SAMPLE_DATA: Almanac<4, 2, 3, 4, 2, 3, 2, 2> = Almanac {
    seeds: [79, 14, 55, 13],
    seed_to_soil: [
        {
            Mapping {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            }
        },
        {
            Mapping {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            }
        },
    ],
    soil_to_fertilizer: [
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            }
        },
        {
            Mapping {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            }
        },
        {
            Mapping {
                destination_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            }
        },
    ],
    fertilizer_to_water: [
        {
            Mapping {
                destination_range_start: 49,
                source_range_start: 53,
                range_length: 8,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 11,
                range_length: 42,
            }
        },
        {
            Mapping {
                destination_range_start: 42,
                source_range_start: 0,
                range_length: 7,
            }
        },
        {
            Mapping {
                destination_range_start: 57,
                source_range_start: 7,
                range_length: 4,
            }
        },
    ],
    water_to_light: [
        {
            Mapping {
                destination_range_start: 88,
                source_range_start: 18,
                range_length: 7,
            }
        },
        {
            Mapping {
                destination_range_start: 18,
                source_range_start: 25,
                range_length: 70,
            }
        },
    ],
    light_to_temperature: [
        {
            Mapping {
                destination_range_start: 45,
                source_range_start: 77,
                range_length: 23,
            }
        },
        {
            Mapping {
                destination_range_start: 81,
                source_range_start: 45,
                range_length: 19,
            }
        },
        {
            Mapping {
                destination_range_start: 68,
                source_range_start: 64,
                range_length: 13,
            }
        },
    ],
    temperature_to_humidity: [
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 69,
                range_length: 1,
            }
        },
        {
            Mapping {
                destination_range_start: 1,
                source_range_start: 0,
                range_length: 69,
            }
        },
    ],
    humidity_to_location: [
        {
            Mapping {
                destination_range_start: 60,
                source_range_start: 56,
                range_length: 37,
            }
        },
        {
            Mapping {
                destination_range_start: 56,
                source_range_start: 93,
                range_length: 4,
            }
        },
    ],
};

pub const REAL_DATA: Almanac<20, 17, 31, 39, 17, 19, 30, 38> = Almanac {
    seeds: [
        919339981, 562444630, 3366006921, 67827214, 1496677366, 101156779, 4140591657, 5858311,
        2566406753, 71724353, 2721360939, 35899538, 383860877, 424668759, 3649554897, 442182562,
        2846055542, 49953829, 2988140126, 256306471,
    ],

    seed_to_soil: [
        {
            Mapping {
                destination_range_start: 627617777,
                source_range_start: 1691901751,
                range_length: 235673208,
            }
        },
        {
            Mapping {
                destination_range_start: 2425244517,
                source_range_start: 2483951770,
                range_length: 157286279,
            }
        },
        {
            Mapping {
                destination_range_start: 1339042890,
                source_range_start: 1549225044,
                range_length: 142676707,
            }
        },
        {
            Mapping {
                destination_range_start: 481294110,
                source_range_start: 381503165,
                range_length: 89539853,
            }
        },
        {
            Mapping {
                destination_range_start: 863290985,
                source_range_start: 1007717708,
                range_length: 39103521,
            }
        },
        {
            Mapping {
                destination_range_start: 570833963,
                source_range_start: 324719351,
                range_length: 56783814,
            }
        },
        {
            Mapping {
                destination_range_start: 3953140805,
                source_range_start: 3714151881,
                range_length: 155523737,
            }
        },
        {
            Mapping {
                destination_range_start: 902394506,
                source_range_start: 1941176275,
                range_length: 61481385,
            }
        },
        {
            Mapping {
                destination_range_start: 963875891,
                source_range_start: 675869083,
                range_length: 331848625,
            }
        },
        {
            Mapping {
                destination_range_start: 1922840702,
                source_range_start: 1046821229,
                range_length: 502403815,
            }
        },
        {
            Mapping {
                destination_range_start: 1481719597,
                source_range_start: 1927574959,
                range_length: 13601316,
            }
        },
        {
            Mapping {
                destination_range_start: 1820040264,
                source_range_start: 573068645,
                range_length: 102800438,
            }
        },
        {
            Mapping {
                destination_range_start: 1295724516,
                source_range_start: 2641238049,
                range_length: 43318374,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 2002657660,
                range_length: 481294110,
            }
        },
        {
            Mapping {
                destination_range_start: 1495320913,
                source_range_start: 0,
                range_length: 324719351,
            }
        },
        {
            Mapping {
                destination_range_start: 2582530796,
                source_range_start: 471043018,
                range_length: 102025627,
            }
        },
        {
            Mapping {
                destination_range_start: 3714151881,
                source_range_start: 3869675618,
                range_length: 238988924,
            }
        },
    ],

    soil_to_fertilizer: [
        {
            Mapping {
                destination_range_start: 1288462652,
                source_range_start: 3191328122,
                range_length: 309853381,
            }
        },
        {
            Mapping {
                destination_range_start: 3216116191,
                source_range_start: 1774097401,
                range_length: 151922673,
            }
        },
        {
            Mapping {
                destination_range_start: 1739360920,
                source_range_start: 2276789875,
                range_length: 44162492,
            }
        },
        {
            Mapping {
                destination_range_start: 220941763,
                source_range_start: 1080382821,
                range_length: 325206789,
            }
        },
        {
            Mapping {
                destination_range_start: 2416141229,
                source_range_start: 3949354107,
                range_length: 345613189,
            }
        },
        {
            Mapping {
                destination_range_start: 1783523412,
                source_range_start: 765967805,
                range_length: 117439234,
            }
        },
        {
            Mapping {
                destination_range_start: 2876214366,
                source_range_start: 1010215130,
                range_length: 70167691,
            }
        },
        {
            Mapping {
                destination_range_start: 2761754418,
                source_range_start: 3142514478,
                range_length: 48813644,
            }
        },
        {
            Mapping {
                destination_range_start: 3840090062,
                source_range_start: 3541735844,
                range_length: 44042641,
            }
        },
        {
            Mapping {
                destination_range_start: 2946382057,
                source_range_start: 1417076798,
                range_length: 106871618,
            }
        },
        {
            Mapping {
                destination_range_start: 3750331405,
                source_range_start: 3501181503,
                range_length: 29948437,
            }
        },
        {
            Mapping {
                destination_range_start: 1105413398,
                source_range_start: 1405589610,
                range_length: 11487188,
            }
        },
        {
            Mapping {
                destination_range_start: 2867709608,
                source_range_start: 3585778485,
                range_length: 8504758,
            }
        },
        {
            Mapping {
                destination_range_start: 3053253675,
                source_range_start: 3594283243,
                range_length: 162862516,
            }
        },
        {
            Mapping {
                destination_range_start: 15940804,
                source_range_start: 0,
                range_length: 90040993,
            }
        },
        {
            Mapping {
                destination_range_start: 2192246779,
                source_range_start: 537548320,
                range_length: 223894450,
            }
        },
        {
            Mapping {
                destination_range_start: 210335859,
                source_range_start: 3531129940,
                range_length: 10605904,
            }
        },
        {
            Mapping {
                destination_range_start: 546148552,
                source_range_start: 2351877346,
                range_length: 559264846,
            }
        },
        {
            Mapping {
                destination_range_start: 1598316033,
                source_range_start: 883407039,
                range_length: 110119908,
            }
        },
        {
            Mapping {
                destination_range_start: 2851021425,
                source_range_start: 993526947,
                range_length: 16688183,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 90040993,
                range_length: 15940804,
            }
        },
        {
            Mapping {
                destination_range_start: 4134281688,
                source_range_start: 3757145759,
                range_length: 160685608,
            }
        },
        {
            Mapping {
                destination_range_start: 3718808665,
                source_range_start: 3917831367,
                range_length: 31522740,
            }
        },
        {
            Mapping {
                destination_range_start: 1116900586,
                source_range_start: 2970952412,
                range_length: 171562066,
            }
        },
        {
            Mapping {
                destination_range_start: 2810568062,
                source_range_start: 210335859,
                range_length: 40453363,
            }
        },
        {
            Mapping {
                destination_range_start: 3780279842,
                source_range_start: 2911142192,
                range_length: 59810220,
            }
        },
        {
            Mapping {
                destination_range_start: 1900962646,
                source_range_start: 761442770,
                range_length: 4525035,
            }
        },
        {
            Mapping {
                destination_range_start: 1905487681,
                source_range_start: 250789222,
                range_length: 286759098,
            }
        },
        {
            Mapping {
                destination_range_start: 3368038864,
                source_range_start: 1926020074,
                range_length: 350769801,
            }
        },
        {
            Mapping {
                destination_range_start: 1708435941,
                source_range_start: 2320952367,
                range_length: 30924979,
            }
        },
        {
            Mapping {
                destination_range_start: 3884132703,
                source_range_start: 1523948416,
                range_length: 250148985,
            }
        },
    ],

    fertilizer_to_water: [
        {
            Mapping {
                destination_range_start: 2450598719,
                source_range_start: 3993777626,
                range_length: 178688420,
            }
        },
        {
            Mapping {
                destination_range_start: 990387307,
                source_range_start: 2284751995,
                range_length: 36035279,
            }
        },
        {
            Mapping {
                destination_range_start: 1479519873,
                source_range_start: 1430606124,
                range_length: 25388869,
            }
        },
        {
            Mapping {
                destination_range_start: 2928263979,
                source_range_start: 2233553333,
                range_length: 51198662,
            }
        },
        {
            Mapping {
                destination_range_start: 1825810145,
                source_range_start: 1093816339,
                range_length: 30855096,
            }
        },
        {
            Mapping {
                destination_range_start: 3131362216,
                source_range_start: 1499369679,
                range_length: 38622365,
            }
        },
        {
            Mapping {
                destination_range_start: 2629287139,
                source_range_start: 1980229176,
                range_length: 243592551,
            }
        },
        {
            Mapping {
                destination_range_start: 2979462641,
                source_range_start: 1827354243,
                range_length: 146328983,
            }
        },
        {
            Mapping {
                destination_range_start: 937194740,
                source_range_start: 3188384434,
                range_length: 9817881,
            }
        },
        {
            Mapping {
                destination_range_start: 2069360741,
                source_range_start: 1973683226,
                range_length: 975358,
            }
        },
        {
            Mapping {
                destination_range_start: 1856665241,
                source_range_start: 3781082126,
                range_length: 212695500,
            }
        },
        {
            Mapping {
                destination_range_start: 642830371,
                source_range_start: 945238436,
                range_length: 148577903,
            }
        },
        {
            Mapping {
                destination_range_start: 4114154511,
                source_range_start: 1124671435,
                range_length: 95554104,
            }
        },
        {
            Mapping {
                destination_range_start: 3777484412,
                source_range_start: 483834992,
                range_length: 305990165,
            }
        },
        {
            Mapping {
                destination_range_start: 3264059981,
                source_range_start: 2427531551,
                range_length: 296117669,
            }
        },
        {
            Mapping {
                destination_range_start: 272170557,
                source_range_start: 2723649220,
                range_length: 370659814,
            }
        },
        {
            Mapping {
                destination_range_start: 262438951,
                source_range_start: 2223821727,
                range_length: 9731606,
            }
        },
        {
            Mapping {
                destination_range_start: 1615429560,
                source_range_start: 1220225539,
                range_length: 210380585,
            }
        },
        {
            Mapping {
                destination_range_start: 1028349507,
                source_range_start: 226445100,
                range_length: 83554339,
            }
        },
        {
            Mapping {
                destination_range_start: 4209708615,
                source_range_start: 3198202315,
                range_length: 85258681,
            }
        },
        {
            Mapping {
                destination_range_start: 3169984581,
                source_range_start: 3094309034,
                range_length: 94075400,
            }
        },
        {
            Mapping {
                destination_range_start: 4083474577,
                source_range_start: 309999439,
                range_length: 30679934,
            }
        },
        {
            Mapping {
                destination_range_start: 2872879690,
                source_range_start: 115495593,
                range_length: 55384289,
            }
        },
        {
            Mapping {
                destination_range_start: 2138362191,
                source_range_start: 340679373,
                range_length: 4164957,
            }
        },
        {
            Mapping {
                destination_range_start: 947012621,
                source_range_start: 1455994993,
                range_length: 43374686,
            }
        },
        {
            Mapping {
                destination_range_start: 791408274,
                source_range_start: 1606018136,
                range_length: 145786466,
            }
        },
        {
            Mapping {
                destination_range_start: 1026422586,
                source_range_start: 1825427322,
                range_length: 1926921,
            }
        },
        {
            Mapping {
                destination_range_start: 1578531462,
                source_range_start: 789825157,
                range_length: 8469921,
            }
        },
        {
            Mapping {
                destination_range_start: 3670740135,
                source_range_start: 2320787274,
                range_length: 106744277,
            }
        },
        {
            Mapping {
                destination_range_start: 2272532251,
                source_range_start: 170879882,
                range_length: 55565218,
            }
        },
        {
            Mapping {
                destination_range_start: 3125791624,
                source_range_start: 1974658584,
                range_length: 5570592,
            }
        },
        {
            Mapping {
                destination_range_start: 1111903846,
                source_range_start: 3283460996,
                range_length: 367616027,
            }
        },
        {
            Mapping {
                destination_range_start: 2070336099,
                source_range_start: 1537992044,
                range_length: 68026092,
            }
        },
        {
            Mapping {
                destination_range_start: 1504908742,
                source_range_start: 1751804602,
                range_length: 73622720,
            }
        },
        {
            Mapping {
                destination_range_start: 2142527148,
                source_range_start: 3651077023,
                range_length: 130005103,
            }
        },
        {
            Mapping {
                destination_range_start: 2328097469,
                source_range_start: 4172466046,
                range_length: 122501250,
            }
        },
        {
            Mapping {
                destination_range_start: 1587001383,
                source_range_start: 344844330,
                range_length: 28428177,
            }
        },
        {
            Mapping {
                destination_range_start: 3560177650,
                source_range_start: 373272507,
                range_length: 110562485,
            }
        },
        {
            Mapping {
                destination_range_start: 115495593,
                source_range_start: 798295078,
                range_length: 146943358,
            }
        },
    ],

    water_to_light: [
        {
            Mapping {
                destination_range_start: 220561404,
                source_range_start: 643114856,
                range_length: 123713527,
            }
        },
        {
            Mapping {
                destination_range_start: 1087373312,
                source_range_start: 4123526389,
                range_length: 171440907,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 766828383,
                range_length: 220561404,
            }
        },
        {
            Mapping {
                destination_range_start: 2907464644,
                source_range_start: 3955968868,
                range_length: 167557521,
            }
        },
        {
            Mapping {
                destination_range_start: 376259478,
                source_range_start: 31984547,
                range_length: 611130309,
            }
        },
        {
            Mapping {
                destination_range_start: 344274931,
                source_range_start: 0,
                range_length: 31984547,
            }
        },
        {
            Mapping {
                destination_range_start: 4209967986,
                source_range_start: 1266450159,
                range_length: 84999310,
            }
        },
        {
            Mapping {
                destination_range_start: 3234177104,
                source_range_start: 1246528251,
                range_length: 19921908,
            }
        },
        {
            Mapping {
                destination_range_start: 4075482989,
                source_range_start: 2729348487,
                range_length: 134484997,
            }
        },
        {
            Mapping {
                destination_range_start: 1258814219,
                source_range_start: 1870560966,
                range_length: 858787521,
            }
        },
        {
            Mapping {
                destination_range_start: 3075022165,
                source_range_start: 1087373312,
                range_length: 159154939,
            }
        },
        {
            Mapping {
                destination_range_start: 2439602258,
                source_range_start: 3906331676,
                range_length: 49637192,
            }
        },
        {
            Mapping {
                destination_range_start: 2297013316,
                source_range_start: 3685217461,
                range_length: 142588942,
            }
        },
        {
            Mapping {
                destination_range_start: 2567764723,
                source_range_start: 1530861045,
                range_length: 339699921,
            }
        },
        {
            Mapping {
                destination_range_start: 3254099012,
                source_range_start: 2863833484,
                range_length: 821383977,
            }
        },
        {
            Mapping {
                destination_range_start: 2489239450,
                source_range_start: 3827806403,
                range_length: 78525273,
            }
        },
        {
            Mapping {
                destination_range_start: 2117601740,
                source_range_start: 1351449469,
                range_length: 179411576,
            }
        },
    ],

    light_to_temperature: [
        {
            Mapping {
                destination_range_start: 83647742,
                source_range_start: 560398800,
                range_length: 311449275,
            }
        },
        {
            Mapping {
                destination_range_start: 4201716419,
                source_range_start: 3337900402,
                range_length: 93250877,
            }
        },
        {
            Mapping {
                destination_range_start: 1071565427,
                source_range_start: 3024001249,
                range_length: 4392221,
            }
        },
        {
            Mapping {
                destination_range_start: 3424420254,
                source_range_start: 3520028211,
                range_length: 161472091,
            }
        },
        {
            Mapping {
                destination_range_start: 1227070419,
                source_range_start: 2828368402,
                range_length: 116428819,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 2744720660,
                range_length: 83647742,
            }
        },
        {
            Mapping {
                destination_range_start: 4199359339,
                source_range_start: 3431151279,
                range_length: 2357080,
            }
        },
        {
            Mapping {
                destination_range_start: 1807547949,
                source_range_start: 2359072262,
                range_length: 384093366,
            }
        },
        {
            Mapping {
                destination_range_start: 3336003529,
                source_range_start: 3486377667,
                range_length: 33650544,
            }
        },
        {
            Mapping {
                destination_range_start: 1075957648,
                source_range_start: 871848075,
                range_length: 149557739,
            }
        },
        {
            Mapping {
                destination_range_start: 3585892345,
                source_range_start: 3681500302,
                range_length: 613466994,
            }
        },
        {
            Mapping {
                destination_range_start: 955495817,
                source_range_start: 2243002652,
                range_length: 116069610,
            }
        },
        {
            Mapping {
                destination_range_start: 2191641315,
                source_range_start: 1406250497,
                range_length: 836752155,
            }
        },
        {
            Mapping {
                destination_range_start: 3369654073,
                source_range_start: 3433508359,
                range_length: 52869308,
            }
        },
        {
            Mapping {
                destination_range_start: 1343499238,
                source_range_start: 2944797221,
                range_length: 79204028,
            }
        },
        {
            Mapping {
                destination_range_start: 3422523381,
                source_range_start: 3336003529,
                range_length: 1896873,
            }
        },
        {
            Mapping {
                destination_range_start: 395097017,
                source_range_start: 0,
                range_length: 560398800,
            }
        },
        {
            Mapping {
                destination_range_start: 1225515387,
                source_range_start: 2743165628,
                range_length: 1555032,
            }
        },
        {
            Mapping {
                destination_range_start: 1422703266,
                source_range_start: 1021405814,
                range_length: 384844683,
            }
        },
    ],

    temperature_to_humidity: [
        {
            Mapping {
                destination_range_start: 3340701300,
                source_range_start: 3627322367,
                range_length: 174281926,
            }
        },
        {
            Mapping {
                destination_range_start: 3514983226,
                source_range_start: 4012096602,
                range_length: 282870694,
            }
        },
        {
            Mapping {
                destination_range_start: 403969772,
                source_range_start: 2344934648,
                range_length: 125307793,
            }
        },
        {
            Mapping {
                destination_range_start: 356249525,
                source_range_start: 1299750763,
                range_length: 47720247,
            }
        },
        {
            Mapping {
                destination_range_start: 529277565,
                source_range_start: 0,
                range_length: 19150241,
            }
        },
        {
            Mapping {
                destination_range_start: 2425473363,
                source_range_start: 606907612,
                range_length: 15105367,
            }
        },
        {
            Mapping {
                destination_range_start: 1622667408,
                source_range_start: 1693081570,
                range_length: 188915118,
            }
        },
        {
            Mapping {
                destination_range_start: 1858084350,
                source_range_start: 1347471010,
                range_length: 47053630,
            }
        },
        {
            Mapping {
                destination_range_start: 4223873328,
                source_range_start: 2752952198,
                range_length: 71093968,
            }
        },
        {
            Mapping {
                destination_range_start: 2749048741,
                source_range_start: 3035669808,
                range_length: 188955792,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 2134178305,
                range_length: 190659232,
            }
        },
        {
            Mapping {
                destination_range_start: 1112200615,
                source_range_start: 96440819,
                range_length: 510466793,
            }
        },
        {
            Mapping {
                destination_range_start: 4148180263,
                source_range_start: 2749048741,
                range_length: 3903457,
            }
        },
        {
            Mapping {
                destination_range_start: 1811582526,
                source_range_start: 1113827741,
                range_length: 11131074,
            }
        },
        {
            Mapping {
                destination_range_start: 548427806,
                source_range_start: 2324837537,
                range_length: 20097111,
            }
        },
        {
            Mapping {
                destination_range_start: 1905137980,
                source_range_start: 2051821050,
                range_length: 82357255,
            }
        },
        {
            Mapping {
                destination_range_start: 3116507691,
                source_range_start: 3224625600,
                range_length: 224193609,
            }
        },
        {
            Mapping {
                destination_range_start: 1822713600,
                source_range_start: 1657710820,
                range_length: 35370750,
            }
        },
        {
            Mapping {
                destination_range_start: 568524917,
                source_range_start: 622012979,
                range_length: 491814762,
            }
        },
        {
            Mapping {
                destination_range_start: 2440578730,
                source_range_start: 66777108,
                range_length: 29663711,
            }
        },
        {
            Mapping {
                destination_range_start: 2250681415,
                source_range_start: 1124958815,
                range_length: 174791948,
            }
        },
        {
            Mapping {
                destination_range_start: 4009477562,
                source_range_start: 3873393901,
                range_length: 138702701,
            }
        },
        {
            Mapping {
                destination_range_start: 190659232,
                source_range_start: 1881996688,
                range_length: 165590293,
            }
        },
        {
            Mapping {
                destination_range_start: 1987495235,
                source_range_start: 1394524640,
                range_length: 263186180,
            }
        },
        {
            Mapping {
                destination_range_start: 1107966546,
                source_range_start: 2047586981,
                range_length: 4234069,
            }
        },
        {
            Mapping {
                destination_range_start: 1060339679,
                source_range_start: 19150241,
                range_length: 47626867,
            }
        },
        {
            Mapping {
                destination_range_start: 4210822392,
                source_range_start: 3801604293,
                range_length: 13050936,
            }
        },
        {
            Mapping {
                destination_range_start: 2938004533,
                source_range_start: 3448819209,
                range_length: 178503158,
            }
        },
        {
            Mapping {
                destination_range_start: 3797853920,
                source_range_start: 2824046166,
                range_length: 211623642,
            }
        },
        {
            Mapping {
                destination_range_start: 4152083720,
                source_range_start: 3814655229,
                range_length: 58738672,
            }
        },
    ],

    humidity_to_location: [
        {
            Mapping {
                destination_range_start: 2102802203,
                source_range_start: 2756269781,
                range_length: 87468599,
            }
        },
        {
            Mapping {
                destination_range_start: 2877112183,
                source_range_start: 2931341663,
                range_length: 22549647,
            }
        },
        {
            Mapping {
                destination_range_start: 3494879649,
                source_range_start: 1477788233,
                range_length: 182041959,
            }
        },
        {
            Mapping {
                destination_range_start: 2436026725,
                source_range_start: 2953891310,
                range_length: 81993792,
            }
        },
        {
            Mapping {
                destination_range_start: 2899661830,
                source_range_start: 373999642,
                range_length: 244663414,
            }
        },
        {
            Mapping {
                destination_range_start: 2190270802,
                source_range_start: 850802545,
                range_length: 169763790,
            }
        },
        {
            Mapping {
                destination_range_start: 3866824018,
                source_range_start: 90439080,
                range_length: 13573105,
            }
        },
        {
            Mapping {
                destination_range_start: 320139638,
                source_range_start: 1659830192,
                range_length: 113214403,
            }
        },
        {
            Mapping {
                destination_range_start: 1131501111,
                source_range_start: 1773044595,
                range_length: 246249128,
            }
        },
        {
            Mapping {
                destination_range_start: 2417984929,
                source_range_start: 828638896,
                range_length: 16505804,
            }
        },
        {
            Mapping {
                destination_range_start: 3839766923,
                source_range_start: 845144700,
                range_length: 5657845,
            }
        },
        {
            Mapping {
                destination_range_start: 4161134012,
                source_range_start: 2843738380,
                range_length: 87603283,
            }
        },
        {
            Mapping {
                destination_range_start: 3731370785,
                source_range_start: 3801964390,
                range_length: 108396138,
            }
        },
        {
            Mapping {
                destination_range_start: 1003659815,
                source_range_start: 3442085055,
                range_length: 127841296,
            }
        },
        {
            Mapping {
                destination_range_start: 918426975,
                source_range_start: 3986208955,
                range_length: 85232840,
            }
        },
        {
            Mapping {
                destination_range_start: 433354041,
                source_range_start: 618663056,
                range_length: 209975840,
            }
        },
        {
            Mapping {
                destination_range_start: 2811371438,
                source_range_start: 0,
                range_length: 65740745,
            }
        },
        {
            Mapping {
                destination_range_start: 2518020517,
                source_range_start: 1329719948,
                range_length: 148068285,
            }
        },
        {
            Mapping {
                destination_range_start: 651651860,
                source_range_start: 3035885102,
                range_length: 266775115,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 2256642811,
                range_length: 320139638,
            }
        },
        {
            Mapping {
                destination_range_start: 2666088802,
                source_range_start: 228717006,
                range_length: 145282636,
            }
        },
        {
            Mapping {
                destination_range_start: 4136435677,
                source_range_start: 65740745,
                range_length: 24698335,
            }
        },
        {
            Mapping {
                destination_range_start: 3144325244,
                source_range_start: 1046598285,
                range_length: 188874270,
            }
        },
        {
            Mapping {
                destination_range_start: 1657153777,
                source_range_start: 2576782449,
                range_length: 179487332,
            }
        },
        {
            Mapping {
                destination_range_start: 1844732214,
                source_range_start: 3569926351,
                range_length: 232038039,
            }
        },
        {
            Mapping {
                destination_range_start: 643329881,
                source_range_start: 2077244060,
                range_length: 8321979,
            }
        },
        {
            Mapping {
                destination_range_start: 1471997632,
                source_range_start: 4071441795,
                range_length: 175759508,
            }
        },
        {
            Mapping {
                destination_range_start: 2076770253,
                source_range_start: 1020566335,
                range_length: 26031950,
            }
        },
        {
            Mapping {
                destination_range_start: 3676921608,
                source_range_start: 3931759778,
                range_length: 54449177,
            }
        },
        {
            Mapping {
                destination_range_start: 3997010839,
                source_range_start: 3302660217,
                range_length: 139424838,
            }
        },
        {
            Mapping {
                destination_range_start: 3845424768,
                source_range_start: 3910360528,
                range_length: 21399250,
            }
        },
        {
            Mapping {
                destination_range_start: 1836641109,
                source_range_start: 104012185,
                range_length: 8091105,
            }
        },
        {
            Mapping {
                destination_range_start: 3333199514,
                source_range_start: 2085566039,
                range_length: 161680135,
            }
        },
        {
            Mapping {
                destination_range_start: 3880397123,
                source_range_start: 112103290,
                range_length: 116613716,
            }
        },
        {
            Mapping {
                destination_range_start: 2360034592,
                source_range_start: 2019293723,
                range_length: 57950337,
            }
        },
        {
            Mapping {
                destination_range_start: 1377750239,
                source_range_start: 1235472555,
                range_length: 94247393,
            }
        },
        {
            Mapping {
                destination_range_start: 1647757140,
                source_range_start: 2247246174,
                range_length: 9396637,
            }
        },
        {
            Mapping {
                destination_range_start: 2434490733,
                source_range_start: 4247201303,
                range_length: 1535992,
            }
        },
    ],
};
