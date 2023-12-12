pub struct HandBid {
    pub hand: [char; 5],
    pub bid: u64,
}

// Transform with:
// '<,'>s/\(.\)\(.\)\(.\)\(.\)\(.\) \(\d\+\)/{ HandBid { hand: [ '\1', '\2', '\3', '\4', '\5'], bid: \6 } },

pub const SAMPLE_DATA: [HandBid; 5] = [
    {
        HandBid {
            hand: ['3', '2', 'T', '3', 'K'],
            bid: 765,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '5', 'J', '5'],
            bid: 684,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '6', '7', '7'],
            bid: 28,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', 'J', 'J', 'T'],
            bid: 220,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'J', 'A'],
            bid: 483,
        }
    },
];

pub const REAL_DATA: [HandBid; 1000] = [
    {
        HandBid {
            hand: ['J', '3', 'T', '3', 'T'],
            bid: 868,
        }
    },
    {
        HandBid {
            hand: ['6', 'Q', '4', '9', '9'],
            bid: 630,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '3', 'A', 'A'],
            bid: 335,
        }
    },
    {
        HandBid {
            hand: ['A', '7', '3', 'A', 'A'],
            bid: 593,
        }
    },
    {
        HandBid {
            hand: ['2', '8', '4', '4', '5'],
            bid: 591,
        }
    },
    {
        HandBid {
            hand: ['A', '9', 'Q', 'A', 'A'],
            bid: 563,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '3', '5', '3'],
            bid: 830,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', '7', 'A', '9'],
            bid: 230,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', '6', 'J', 'J'],
            bid: 588,
        }
    },
    {
        HandBid {
            hand: ['J', '6', 'K', '3', '8'],
            bid: 769,
        }
    },
    {
        HandBid {
            hand: ['5', 'J', 'A', '6', '5'],
            bid: 531,
        }
    },
    {
        HandBid {
            hand: ['7', '3', '7', 'Q', 'J'],
            bid: 827,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '5', '6', 'J'],
            bid: 962,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', '6', '6', '6'],
            bid: 481,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '8', '6', 'Q'],
            bid: 282,
        }
    },
    {
        HandBid {
            hand: ['J', '6', '5', '6', '6'],
            bid: 843,
        }
    },
    {
        HandBid {
            hand: ['3', 'T', 'T', '8', '8'],
            bid: 331,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', '8', 'K'],
            bid: 129,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', '8', '8', '4'],
            bid: 145,
        }
    },
    {
        HandBid {
            hand: ['T', '5', 'T', '7', '4'],
            bid: 118,
        }
    },
    {
        HandBid {
            hand: ['3', '2', 'J', '2', '2'],
            bid: 115,
        }
    },
    {
        HandBid {
            hand: ['9', 'Q', '8', '4', '3'],
            bid: 220,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'J', '9', 'J'],
            bid: 590,
        }
    },
    {
        HandBid {
            hand: ['3', '5', '2', '8', 'K'],
            bid: 291,
        }
    },
    {
        HandBid {
            hand: ['9', '5', '7', '8', '9'],
            bid: 959,
        }
    },
    {
        HandBid {
            hand: ['2', '9', '8', '6', 'K'],
            bid: 26,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', '6', '6', 'T'],
            bid: 989,
        }
    },
    {
        HandBid {
            hand: ['6', '3', '2', 'A', '7'],
            bid: 447,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '2', '8', '5'],
            bid: 217,
        }
    },
    {
        HandBid {
            hand: ['5', '2', '7', '5', 'J'],
            bid: 629,
        }
    },
    {
        HandBid {
            hand: ['7', '8', '3', '3', '7'],
            bid: 767,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '5', '5', '5'],
            bid: 894,
        }
    },
    {
        HandBid {
            hand: ['5', 'Q', '5', '5', '5'],
            bid: 262,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', '4', 'A', 'A'],
            bid: 320,
        }
    },
    {
        HandBid {
            hand: ['4', 'K', '4', '9', '9'],
            bid: 701,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', 'T', '2', '2'],
            bid: 759,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', '5', '2', 'J'],
            bid: 423,
        }
    },
    {
        HandBid {
            hand: ['J', 'J', '7', 'J', 'T'],
            bid: 22,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', '2', '2', '3'],
            bid: 698,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '5', '5', 'J'],
            bid: 795,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', '9', 'K', '5'],
            bid: 549,
        }
    },
    {
        HandBid {
            hand: ['Q', '2', '8', '2', '2'],
            bid: 515,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', 'Q', '9', 'Q'],
            bid: 754,
        }
    },
    {
        HandBid {
            hand: ['T', '8', 'T', '7', '9'],
            bid: 979,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', 'J', '7', '2'],
            bid: 866,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '9', '8', '9'],
            bid: 707,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', 'A', 'A', 'J'],
            bid: 224,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'A', '2', 'K'],
            bid: 869,
        }
    },
    {
        HandBid {
            hand: ['2', '6', 'J', '9', '5'],
            bid: 572,
        }
    },
    {
        HandBid {
            hand: ['K', '9', '3', 'K', '9'],
            bid: 149,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'Q', '5', 'Q'],
            bid: 867,
        }
    },
    {
        HandBid {
            hand: ['6', '9', '9', '9', '9'],
            bid: 778,
        }
    },
    {
        HandBid {
            hand: ['Q', '3', '3', 'J', 'Q'],
            bid: 506,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', 'A', 'A', '6'],
            bid: 765,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '3', 'J', '3'],
            bid: 540,
        }
    },
    {
        HandBid {
            hand: ['K', '8', 'J', 'K', 'K'],
            bid: 134,
        }
    },
    {
        HandBid {
            hand: ['9', 'Q', 'A', 'Q', 'A'],
            bid: 169,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', '6', '2', '9'],
            bid: 233,
        }
    },
    {
        HandBid {
            hand: ['6', '4', '6', '6', '7'],
            bid: 735,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', '6', 'Q', '2'],
            bid: 43,
        }
    },
    {
        HandBid {
            hand: ['9', 'K', 'J', 'T', 'J'],
            bid: 994,
        }
    },
    {
        HandBid {
            hand: ['K', 'Q', 'A', 'T', '7'],
            bid: 124,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', 'T', 'A', 'A'],
            bid: 8,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '7', '5', '5'],
            bid: 193,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '6', '2', '7'],
            bid: 742,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '9', 'A', '9'],
            bid: 139,
        }
    },
    {
        HandBid {
            hand: ['7', '8', 'A', 'T', '9'],
            bid: 864,
        }
    },
    {
        HandBid {
            hand: ['2', '8', 'A', '8', '2'],
            bid: 718,
        }
    },
    {
        HandBid {
            hand: ['3', '3', 'J', 'J', '3'],
            bid: 57,
        }
    },
    {
        HandBid {
            hand: ['5', '8', '7', '7', '6'],
            bid: 385,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'A', 'A', '2'],
            bid: 904,
        }
    },
    {
        HandBid {
            hand: ['7', 'T', 'T', '7', '7'],
            bid: 21,
        }
    },
    {
        HandBid {
            hand: ['A', '8', 'A', 'A', '8'],
            bid: 490,
        }
    },
    {
        HandBid {
            hand: ['T', '9', '7', '8', '4'],
            bid: 706,
        }
    },
    {
        HandBid {
            hand: ['K', '3', 'J', '3', 'K'],
            bid: 76,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '4', '4', '4'],
            bid: 451,
        }
    },
    {
        HandBid {
            hand: ['8', 'K', '4', '9', '7'],
            bid: 680,
        }
    },
    {
        HandBid {
            hand: ['5', 'Q', '4', '5', 'Q'],
            bid: 344,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '8', '4', '3'],
            bid: 670,
        }
    },
    {
        HandBid {
            hand: ['Q', 'K', '6', '4', '7'],
            bid: 161,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', 'Q', '4', '6'],
            bid: 634,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', 'Q', 'Q', '2'],
            bid: 471,
        }
    },
    {
        HandBid {
            hand: ['K', 'J', 'J', '7', '7'],
            bid: 123,
        }
    },
    {
        HandBid {
            hand: ['K', 'Q', '3', 'Q', '7'],
            bid: 412,
        }
    },
    {
        HandBid {
            hand: ['5', '2', '2', 'Q', 'T'],
            bid: 632,
        }
    },
    {
        HandBid {
            hand: ['5', '2', '6', '2', '5'],
            bid: 642,
        }
    },
    {
        HandBid {
            hand: ['3', '8', 'A', '2', '5'],
            bid: 508,
        }
    },
    {
        HandBid {
            hand: ['5', '4', 'A', '7', 'K'],
            bid: 892,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '4', '3', '9'],
            bid: 958,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', 'K', 'A', 'A'],
            bid: 682,
        }
    },
    {
        HandBid {
            hand: ['K', '7', '5', 'K', '7'],
            bid: 677,
        }
    },
    {
        HandBid {
            hand: ['2', '8', 'A', '2', '4'],
            bid: 743,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', '5', 'Q', '8'],
            bid: 722,
        }
    },
    {
        HandBid {
            hand: ['9', '2', '8', '9', '2'],
            bid: 783,
        }
    },
    {
        HandBid {
            hand: ['K', '8', '8', '2', '8'],
            bid: 173,
        }
    },
    {
        HandBid {
            hand: ['K', '9', 'A', '8', '9'],
            bid: 425,
        }
    },
    {
        HandBid {
            hand: ['K', '6', 'K', 'Q', '8'],
            bid: 657,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '7', '4', '7'],
            bid: 833,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '9', '9', '6'],
            bid: 625,
        }
    },
    {
        HandBid {
            hand: ['3', '5', 'Q', 'K', '4'],
            bid: 67,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', '7', '7'],
            bid: 268,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '8', '9', 'A'],
            bid: 610,
        }
    },
    {
        HandBid {
            hand: ['7', '5', 'A', '6', 'J'],
            bid: 49,
        }
    },
    {
        HandBid {
            hand: ['J', '8', '8', '7', '2'],
            bid: 967,
        }
    },
    {
        HandBid {
            hand: ['4', 'K', '6', '5', '8'],
            bid: 210,
        }
    },
    {
        HandBid {
            hand: ['2', '9', '4', '4', '9'],
            bid: 832,
        }
    },
    {
        HandBid {
            hand: ['A', '3', '6', '9', '4'],
            bid: 566,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'J', '5', 'A'],
            bid: 313,
        }
    },
    {
        HandBid {
            hand: ['J', '5', 'A', 'A', '9'],
            bid: 1000,
        }
    },
    {
        HandBid {
            hand: ['5', '7', 'A', '7', 'A'],
            bid: 917,
        }
    },
    {
        HandBid {
            hand: ['2', '8', '7', 'J', '4'],
            bid: 681,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '7', 'T', '7'],
            bid: 789,
        }
    },
    {
        HandBid {
            hand: ['3', 'J', '3', '2', '2'],
            bid: 375,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '2', '5'],
            bid: 522,
        }
    },
    {
        HandBid {
            hand: ['K', '5', '5', 'A', 'J'],
            bid: 565,
        }
    },
    {
        HandBid {
            hand: ['J', '9', 'T', '7', 'T'],
            bid: 740,
        }
    },
    {
        HandBid {
            hand: ['J', '4', '3', '5', '3'],
            bid: 363,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '2', '4', '4'],
            bid: 941,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', 'A', '5', 'A'],
            bid: 977,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', 'Q', '5', '5'],
            bid: 922,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', 'K', '8', 'T'],
            bid: 491,
        }
    },
    {
        HandBid {
            hand: ['J', '8', '5', '3', '3'],
            bid: 378,
        }
    },
    {
        HandBid {
            hand: ['2', '4', '3', '8', '8'],
            bid: 61,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '4', '8', 'T'],
            bid: 947,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', 'J', '2', 'K'],
            bid: 264,
        }
    },
    {
        HandBid {
            hand: ['7', '6', '7', '8', '6'],
            bid: 744,
        }
    },
    {
        HandBid {
            hand: ['K', '6', '6', '6', '6'],
            bid: 739,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', '5', 'K', '5'],
            bid: 631,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '2', '8', '4'],
            bid: 183,
        }
    },
    {
        HandBid {
            hand: ['T', '2', '3', 'T', '2'],
            bid: 48,
        }
    },
    {
        HandBid {
            hand: ['7', '9', '7', '2', '7'],
            bid: 158,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', '6', 'K', 'A'],
            bid: 92,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', '6', '8', '8'],
            bid: 140,
        }
    },
    {
        HandBid {
            hand: ['T', '7', 'T', 'K', '7'],
            bid: 143,
        }
    },
    {
        HandBid {
            hand: ['6', '9', '9', 'J', '6'],
            bid: 377,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', 'J', '3', '8'],
            bid: 809,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '9', 'T', '3'],
            bid: 181,
        }
    },
    {
        HandBid {
            hand: ['8', 'K', 'K', '9', 'K'],
            bid: 732,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '9', 'K', 'K'],
            bid: 901,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '2', 'J', '8'],
            bid: 329,
        }
    },
    {
        HandBid {
            hand: ['A', '3', 'J', '4', '3'],
            bid: 835,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'Q', '6', '6'],
            bid: 266,
        }
    },
    {
        HandBid {
            hand: ['9', '6', 'Q', '4', 'J'],
            bid: 368,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', '5', 'K', '6'],
            bid: 838,
        }
    },
    {
        HandBid {
            hand: ['6', '9', 'J', '8', '8'],
            bid: 263,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '3', 'K', 'A'],
            bid: 108,
        }
    },
    {
        HandBid {
            hand: ['T', 'A', 'J', 'A', 'T'],
            bid: 554,
        }
    },
    {
        HandBid {
            hand: ['7', 'Q', 'A', 'Q', 'A'],
            bid: 420,
        }
    },
    {
        HandBid {
            hand: ['3', 'K', 'Q', '6', '2'],
            bid: 819,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '4', '6', '5'],
            bid: 79,
        }
    },
    {
        HandBid {
            hand: ['J', 'J', 'J', 'J', 'J'],
            bid: 504,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '4', 'T', 'T'],
            bid: 663,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '4', 'J', '2'],
            bid: 801,
        }
    },
    {
        HandBid {
            hand: ['6', 'A', 'A', 'A', 'A'],
            bid: 299,
        }
    },
    {
        HandBid {
            hand: ['J', '7', '7', 'T', '7'],
            bid: 902,
        }
    },
    {
        HandBid {
            hand: ['4', '8', '4', '4', '4'],
            bid: 619,
        }
    },
    {
        HandBid {
            hand: ['A', '2', '7', '8', '5'],
            bid: 607,
        }
    },
    {
        HandBid {
            hand: ['7', '4', 'J', '8', 'T'],
            bid: 875,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '6', '8', '3'],
            bid: 250,
        }
    },
    {
        HandBid {
            hand: ['K', '2', 'Q', 'T', 'K'],
            bid: 824,
        }
    },
    {
        HandBid {
            hand: ['2', '6', 'A', 'A', 'A'],
            bid: 215,
        }
    },
    {
        HandBid {
            hand: ['4', '7', 'Q', '8', 'A'],
            bid: 553,
        }
    },
    {
        HandBid {
            hand: ['A', '6', 'T', 'T', '9'],
            bid: 639,
        }
    },
    {
        HandBid {
            hand: ['2', '6', '6', 'T', '6'],
            bid: 17,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', '9', 'J', 'A'],
            bid: 24,
        }
    },
    {
        HandBid {
            hand: ['J', '8', 'J', '9', '9'],
            bid: 405,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', '6', '6', '6'],
            bid: 544,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '4', 'J', '4'],
            bid: 327,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', '9', '4'],
            bid: 684,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', 'J', 'Q', 'A'],
            bid: 511,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', '2', '5', 'A'],
            bid: 757,
        }
    },
    {
        HandBid {
            hand: ['9', '6', '6', '6', 'J'],
            bid: 103,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '9', '9', '5'],
            bid: 346,
        }
    },
    {
        HandBid {
            hand: ['3', '6', '5', 'K', 'A'],
            bid: 734,
        }
    },
    {
        HandBid {
            hand: ['K', '3', '8', '8', 'T'],
            bid: 11,
        }
    },
    {
        HandBid {
            hand: ['5', '7', 'T', '5', 'T'],
            bid: 828,
        }
    },
    {
        HandBid {
            hand: ['2', '9', '9', '9', '2'],
            bid: 273,
        }
    },
    {
        HandBid {
            hand: ['4', 'A', '3', '6', 'Q'],
            bid: 34,
        }
    },
    {
        HandBid {
            hand: ['3', '9', '8', '8', '8'],
            bid: 204,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'A', '3', '3'],
            bid: 306,
        }
    },
    {
        HandBid {
            hand: ['9', 'Q', '9', '9', '3'],
            bid: 302,
        }
    },
    {
        HandBid {
            hand: ['7', '2', '6', '7', '7'],
            bid: 435,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'Q', '8', 'J'],
            bid: 94,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '7', 'J', '4'],
            bid: 289,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', 'K', '2', '2'],
            bid: 107,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '3', 'K', '9'],
            bid: 406,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '5', 'T', '7'],
            bid: 270,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '2', 'J', '4'],
            bid: 101,
        }
    },
    {
        HandBid {
            hand: ['4', 'K', '5', 'K', 'K'],
            bid: 903,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'T', '7', 'T'],
            bid: 473,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', 'A', 'K', 'K'],
            bid: 141,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', '8', '6', '8'],
            bid: 943,
        }
    },
    {
        HandBid {
            hand: ['T', '4', 'T', '4', '5'],
            bid: 660,
        }
    },
    {
        HandBid {
            hand: ['4', 'T', '9', '6', '8'],
            bid: 840,
        }
    },
    {
        HandBid {
            hand: ['6', 'A', 'A', '5', 'A'],
            bid: 163,
        }
    },
    {
        HandBid {
            hand: ['T', 'Q', 'J', 'T', 'T'],
            bid: 812,
        }
    },
    {
        HandBid {
            hand: ['J', '8', 'A', '8', '8'],
            bid: 345,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'Q', 'Q', 'J'],
            bid: 276,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', 'T', '9', '3'],
            bid: 290,
        }
    },
    {
        HandBid {
            hand: ['5', '8', 'Q', 'Q', '5'],
            bid: 534,
        }
    },
    {
        HandBid {
            hand: ['T', '7', '4', '5', '7'],
            bid: 849,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '4', '4', 'K'],
            bid: 479,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'Q', 'A', 'T'],
            bid: 851,
        }
    },
    {
        HandBid {
            hand: ['J', '4', '5', 'J', 'J'],
            bid: 750,
        }
    },
    {
        HandBid {
            hand: ['J', '8', 'T', '9', '8'],
            bid: 255,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '7', '5', '8'],
            bid: 839,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '8', 'Q', 'Q'],
            bid: 55,
        }
    },
    {
        HandBid {
            hand: ['7', '2', '5', 'A', '2'],
            bid: 693,
        }
    },
    {
        HandBid {
            hand: ['K', '4', 'T', 'T', '3'],
            bid: 431,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '5', '8', '8'],
            bid: 482,
        }
    },
    {
        HandBid {
            hand: ['2', 'K', '4', 'Q', 'J'],
            bid: 502,
        }
    },
    {
        HandBid {
            hand: ['8', '9', '9', '5', '5'],
            bid: 381,
        }
    },
    {
        HandBid {
            hand: ['T', '7', '8', '6', '9'],
            bid: 987,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', '7', '4', 'J'],
            bid: 408,
        }
    },
    {
        HandBid {
            hand: ['2', '7', '4', '3', '8'],
            bid: 211,
        }
    },
    {
        HandBid {
            hand: ['2', '6', '5', '8', 'A'],
            bid: 221,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', '7', 'A', 'A'],
            bid: 836,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'Q', 'J', 'T'],
            bid: 873,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', 'K', 'K'],
            bid: 633,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', 'A', 'A', 'A'],
            bid: 756,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', 'Q', 'Q', 'Q'],
            bid: 281,
        }
    },
    {
        HandBid {
            hand: ['9', '3', '7', 'Q', 'Q'],
            bid: 184,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', '7', '7', '7'],
            bid: 308,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '7', 'J', 'Q'],
            bid: 208,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '3', '5', '5'],
            bid: 478,
        }
    },
    {
        HandBid {
            hand: ['A', '5', 'J', 'K', '4'],
            bid: 571,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'J', '3', '5'],
            bid: 726,
        }
    },
    {
        HandBid {
            hand: ['9', '7', 'K', 'T', 'J'],
            bid: 357,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '4', 'A', '4'],
            bid: 294,
        }
    },
    {
        HandBid {
            hand: ['8', '4', '4', '3', '8'],
            bid: 83,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'K', 'Q', '2'],
            bid: 517,
        }
    },
    {
        HandBid {
            hand: ['7', '8', '6', '6', 'J'],
            bid: 575,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '4', 'T', '7'],
            bid: 434,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '6', '3', '3'],
            bid: 42,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '9', '7', '2'],
            bid: 671,
        }
    },
    {
        HandBid {
            hand: ['8', '6', '7', 'Q', '4'],
            bid: 106,
        }
    },
    {
        HandBid {
            hand: ['9', '5', 'J', '5', '5'],
            bid: 775,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '4', '4', '4'],
            bid: 168,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '3', '2', '8'],
            bid: 213,
        }
    },
    {
        HandBid {
            hand: ['8', '6', 'A', '8', '8'],
            bid: 198,
        }
    },
    {
        HandBid {
            hand: ['Q', '3', '5', '7', '8'],
            bid: 986,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', '2', 'Q'],
            bid: 831,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '8', 'Q', '8'],
            bid: 898,
        }
    },
    {
        HandBid {
            hand: ['K', '6', '9', 'A', '4'],
            bid: 749,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'K', 'A', 'A'],
            bid: 475,
        }
    },
    {
        HandBid {
            hand: ['T', '3', '4', '8', '7'],
            bid: 275,
        }
    },
    {
        HandBid {
            hand: ['5', '9', 'K', '6', 'Q'],
            bid: 850,
        }
    },
    {
        HandBid {
            hand: ['7', '6', 'J', '4', 'T'],
            bid: 248,
        }
    },
    {
        HandBid {
            hand: ['6', '9', 'Q', '7', 'Q'],
            bid: 97,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '3', 'Q', 'J'],
            bid: 182,
        }
    },
    {
        HandBid {
            hand: ['Q', '2', '5', '9', '8'],
            bid: 44,
        }
    },
    {
        HandBid {
            hand: ['5', '4', '4', '4', '4'],
            bid: 486,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', 'J', 'T', 'T'],
            bid: 708,
        }
    },
    {
        HandBid {
            hand: ['J', '2', 'J', 'J', 'K'],
            bid: 179,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', '8', '8'],
            bid: 287,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'Q', 'Q', '6'],
            bid: 694,
        }
    },
    {
        HandBid {
            hand: ['2', '5', '3', '7', 'A'],
            bid: 551,
        }
    },
    {
        HandBid {
            hand: ['2', 'K', 'Q', 'T', 'A'],
            bid: 501,
        }
    },
    {
        HandBid {
            hand: ['J', '9', '8', '7', '7'],
            bid: 171,
        }
    },
    {
        HandBid {
            hand: ['2', '4', '5', '6', 'T'],
            bid: 355,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'Q', '9', '9'],
            bid: 929,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '9', 'T', 'K'],
            bid: 637,
        }
    },
    {
        HandBid {
            hand: ['6', 'Q', 'T', 'Q', '7'],
            bid: 442,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', '8', '7', '7'],
            bid: 755,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '2', '9', '2'],
            bid: 574,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', 'K', '7', '9'],
            bid: 157,
        }
    },
    {
        HandBid {
            hand: ['J', 'K', '9', 'K', '9'],
            bid: 300,
        }
    },
    {
        HandBid {
            hand: ['5', '8', '5', 'K', '4'],
            bid: 85,
        }
    },
    {
        HandBid {
            hand: ['Q', 'K', '2', '2', 'A'],
            bid: 41,
        }
    },
    {
        HandBid {
            hand: ['A', '8', '9', '3', '4'],
            bid: 580,
        }
    },
    {
        HandBid {
            hand: ['T', '5', 'T', 'K', '9'],
            bid: 920,
        }
    },
    {
        HandBid {
            hand: ['K', '4', 'Q', '5', 'J'],
            bid: 122,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '2', '5', '7'],
            bid: 470,
        }
    },
    {
        HandBid {
            hand: ['J', '5', 'J', 'T', 'K'],
            bid: 203,
        }
    },
    {
        HandBid {
            hand: ['9', 'K', 'K', 'K', 'Q'],
            bid: 712,
        }
    },
    {
        HandBid {
            hand: ['6', '7', '7', 'Q', '3'],
            bid: 776,
        }
    },
    {
        HandBid {
            hand: ['3', '6', 'A', '3', '6'],
            bid: 792,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', 'A', 'J', '8'],
            bid: 219,
        }
    },
    {
        HandBid {
            hand: ['5', '2', 'T', '5', '5'],
            bid: 690,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '9', '5', 'J'],
            bid: 334,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'T', '6', '8'],
            bid: 484,
        }
    },
    {
        HandBid {
            hand: ['9', '7', '6', 'Q', '4'],
            bid: 667,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '4', 'T', 'Q'],
            bid: 105,
        }
    },
    {
        HandBid {
            hand: ['9', '8', 'J', '6', '7'],
            bid: 753,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', 'T', '8', '7'],
            bid: 58,
        }
    },
    {
        HandBid {
            hand: ['8', '6', 'Q', '9', '2'],
            bid: 802,
        }
    },
    {
        HandBid {
            hand: ['4', 'A', '4', 'A', 'A'],
            bid: 185,
        }
    },
    {
        HandBid {
            hand: ['6', '3', '3', '9', '9'],
            bid: 390,
        }
    },
    {
        HandBid {
            hand: ['K', '6', 'K', '2', '2'],
            bid: 954,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', '7', '7'],
            bid: 815,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', '9', '7'],
            bid: 546,
        }
    },
    {
        HandBid {
            hand: ['4', 'J', 'K', '4', '8'],
            bid: 997,
        }
    },
    {
        HandBid {
            hand: ['3', '3', 'A', '9', '6'],
            bid: 53,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', '4', 'A', '4'],
            bid: 601,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '8', 'T', '4'],
            bid: 992,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '6', 'J', '8'],
            bid: 784,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', '5', '7', '5'],
            bid: 418,
        }
    },
    {
        HandBid {
            hand: ['4', '5', 'T', 'Q', 'K'],
            bid: 367,
        }
    },
    {
        HandBid {
            hand: ['7', '6', 'A', '9', 'K'],
            bid: 370,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '2', '2', '6'],
            bid: 449,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', 'K', '3', 'K'],
            bid: 458,
        }
    },
    {
        HandBid {
            hand: ['6', '9', '9', '5', '5'],
            bid: 636,
        }
    },
    {
        HandBid {
            hand: ['K', '9', 'Q', '9', '3'],
            bid: 249,
        }
    },
    {
        HandBid {
            hand: ['T', 'A', 'T', '5', '3'],
            bid: 746,
        }
    },
    {
        HandBid {
            hand: ['K', 'J', 'J', '3', 'K'],
            bid: 137,
        }
    },
    {
        HandBid {
            hand: ['8', '4', '4', '7', '7'],
            bid: 332,
        }
    },
    {
        HandBid {
            hand: ['J', '5', 'A', 'Q', '6'],
            bid: 325,
        }
    },
    {
        HandBid {
            hand: ['3', '8', '7', 'A', 'A'],
            bid: 382,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '3', '3', 'J'],
            bid: 142,
        }
    },
    {
        HandBid {
            hand: ['5', '6', 'Q', '6', 'A'],
            bid: 909,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '3', '9', '3'],
            bid: 459,
        }
    },
    {
        HandBid {
            hand: ['K', '3', 'K', 'K', '9'],
            bid: 398,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '6', 'A', 'J'],
            bid: 160,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '7', '7', '8'],
            bid: 499,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'T', '4', '4'],
            bid: 933,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'T', 'Q', 'Q'],
            bid: 737,
        }
    },
    {
        HandBid {
            hand: ['3', 'T', '3', 'T', '4'],
            bid: 596,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', 'J', 'Q', '3'],
            bid: 552,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'J', 'J', '5'],
            bid: 897,
        }
    },
    {
        HandBid {
            hand: ['9', '5', '9', '9', '5'],
            bid: 518,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', 'Q', 'Q', '4'],
            bid: 500,
        }
    },
    {
        HandBid {
            hand: ['4', 'Q', 'Q', 'Q', 'T'],
            bid: 773,
        }
    },
    {
        HandBid {
            hand: ['A', '2', '5', '2', '2'],
            bid: 336,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '9', '7', '2'],
            bid: 785,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', 'K', '3', 'J'],
            bid: 587,
        }
    },
    {
        HandBid {
            hand: ['7', '3', 'K', '7', '7'],
            bid: 448,
        }
    },
    {
        HandBid {
            hand: ['7', 'Q', '7', '5', '5'],
            bid: 309,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '6', '6', 'K'],
            bid: 397,
        }
    },
    {
        HandBid {
            hand: ['T', 'K', 'Q', 'J', 'A'],
            bid: 228,
        }
    },
    {
        HandBid {
            hand: ['4', '6', '6', '6', '6'],
            bid: 609,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'J', 'J', '4'],
            bid: 853,
        }
    },
    {
        HandBid {
            hand: ['T', 'A', '5', 'A', 'T'],
            bid: 19,
        }
    },
    {
        HandBid {
            hand: ['3', '5', '4', 'Q', '6'],
            bid: 131,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'K', '9', '7'],
            bid: 258,
        }
    },
    {
        HandBid {
            hand: ['J', '3', '3', '5', '5'],
            bid: 664,
        }
    },
    {
        HandBid {
            hand: ['K', '2', '9', 'T', '6'],
            bid: 720,
        }
    },
    {
        HandBid {
            hand: ['T', '8', '6', '3', '5'],
            bid: 132,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '4', '7', '4'],
            bid: 521,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '9', '4', '3'],
            bid: 119,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'K', '7', '7'],
            bid: 238,
        }
    },
    {
        HandBid {
            hand: ['8', '4', 'Q', '4', '7'],
            bid: 15,
        }
    },
    {
        HandBid {
            hand: ['A', '5', '5', '6', 'T'],
            bid: 347,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '7', 'J', '7'],
            bid: 582,
        }
    },
    {
        HandBid {
            hand: ['2', '4', '5', '5', 'T'],
            bid: 661,
        }
    },
    {
        HandBid {
            hand: ['J', 'T', '4', '3', 'T'],
            bid: 351,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'A', '2', '2'],
            bid: 243,
        }
    },
    {
        HandBid {
            hand: ['3', '8', '3', '3', '5'],
            bid: 655,
        }
    },
    {
        HandBid {
            hand: ['9', 'A', '4', '7', 'Q'],
            bid: 133,
        }
    },
    {
        HandBid {
            hand: ['9', '2', 'T', '2', '9'],
            bid: 887,
        }
    },
    {
        HandBid {
            hand: ['Q', '9', '8', 'J', 'J'],
            bid: 394,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', 'K', 'Q'],
            bid: 781,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '5', 'J', 'K'],
            bid: 231,
        }
    },
    {
        HandBid {
            hand: ['4', 'T', 'J', 'J', '4'],
            bid: 285,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'J', 'K', 'K'],
            bid: 863,
        }
    },
    {
        HandBid {
            hand: ['3', 'K', '5', 'T', 'Q'],
            bid: 165,
        }
    },
    {
        HandBid {
            hand: ['J', '7', 'T', '3', '2'],
            bid: 882,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '3', '5', 'T'],
            bid: 557,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '3', '7', 'A'],
            bid: 952,
        }
    },
    {
        HandBid {
            hand: ['6', '9', '2', 'T', '6'],
            bid: 88,
        }
    },
    {
        HandBid {
            hand: ['5', '2', 'J', '5', '5'],
            bid: 547,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', 'T', 'K', 'T'],
            bid: 659,
        }
    },
    {
        HandBid {
            hand: ['6', '8', 'K', '5', '9'],
            bid: 648,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'J', '4', '7'],
            bid: 799,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', '3', '4', '8'],
            bid: 116,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', '3', '8', 'A'],
            bid: 304,
        }
    },
    {
        HandBid {
            hand: ['T', '2', '5', '5', 'T'],
            bid: 46,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '6', '8', '2'],
            bid: 600,
        }
    },
    {
        HandBid {
            hand: ['7', '6', '4', 'T', '6'],
            bid: 700,
        }
    },
    {
        HandBid {
            hand: ['J', 'K', '9', '3', '9'],
            bid: 301,
        }
    },
    {
        HandBid {
            hand: ['3', '7', 'J', '7', '7'],
            bid: 536,
        }
    },
    {
        HandBid {
            hand: ['T', '6', '9', 'T', '8'],
            bid: 35,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'Q', '7', '6'],
            bid: 387,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '8', 'T', '5'],
            bid: 279,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '4', 'K', '3'],
            bid: 259,
        }
    },
    {
        HandBid {
            hand: ['T', '8', '9', '9', '3'],
            bid: 568,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'J', '8', '8'],
            bid: 328,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '9', '8', '8'],
            bid: 665,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', '8', 'Q', '5'],
            bid: 50,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '9', '5', 'Q'],
            bid: 365,
        }
    },
    {
        HandBid {
            hand: ['8', '6', 'K', '6', 'K'],
            bid: 207,
        }
    },
    {
        HandBid {
            hand: ['9', 'K', '5', '3', '6'],
            bid: 485,
        }
    },
    {
        HandBid {
            hand: ['7', 'Q', '7', '4', '9'],
            bid: 417,
        }
    },
    {
        HandBid {
            hand: ['8', '9', '8', '3', 'T'],
            bid: 150,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '2', 'Q', '6'],
            bid: 319,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', '2', 'K', '2'],
            bid: 796,
        }
    },
    {
        HandBid {
            hand: ['J', '3', '5', 'T', '5'],
            bid: 714,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '9', '9', 'K'],
            bid: 925,
        }
    },
    {
        HandBid {
            hand: ['T', 'K', '2', '4', '4'],
            bid: 199,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', 'A', 'J', '3'],
            bid: 823,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', '9', '7', '9'],
            bid: 315,
        }
    },
    {
        HandBid {
            hand: ['9', '6', '2', 'K', 'J'],
            bid: 33,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '5', 'Q', 'Q'],
            bid: 842,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', '6', '3', 'Q'],
            bid: 350,
        }
    },
    {
        HandBid {
            hand: ['4', '2', '3', '3', '4'],
            bid: 28,
        }
    },
    {
        HandBid {
            hand: ['9', '7', '7', '7', '9'],
            bid: 603,
        }
    },
    {
        HandBid {
            hand: ['7', '8', 'T', '6', '8'],
            bid: 890,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '3', 'A', '3'],
            bid: 855,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '5', 'T'],
            bid: 908,
        }
    },
    {
        HandBid {
            hand: ['9', '5', 'J', '8', 'K'],
            bid: 489,
        }
    },
    {
        HandBid {
            hand: ['A', '8', '7', 'T', 'T'],
            bid: 537,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'K', '2', '3'],
            bid: 626,
        }
    },
    {
        HandBid {
            hand: ['4', 'J', '8', '4', '8'],
            bid: 641,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'A', '2', '6'],
            bid: 758,
        }
    },
    {
        HandBid {
            hand: ['9', '3', '4', 'Q', 'T'],
            bid: 614,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '4', '7', '4'],
            bid: 679,
        }
    },
    {
        HandBid {
            hand: ['7', '2', '2', '7', 'J'],
            bid: 467,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', 'A', 'A', 'A'],
            bid: 606,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'J', '7', '9'],
            bid: 341,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', 'A', '3', '3'],
            bid: 821,
        }
    },
    {
        HandBid {
            hand: ['6', '8', '2', '6', 'J'],
            bid: 583,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'J', '5', '5'],
            bid: 415,
        }
    },
    {
        HandBid {
            hand: ['K', '7', 'A', 'T', '6'],
            bid: 724,
        }
    },
    {
        HandBid {
            hand: ['3', 'T', 'T', 'T', '8'],
            bid: 433,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', '3', '4', '3'],
            bid: 807,
        }
    },
    {
        HandBid {
            hand: ['J', '8', '5', '5', '2'],
            bid: 439,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'K', 'K'],
            bid: 232,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '6', '7', '8'],
            bid: 135,
        }
    },
    {
        HandBid {
            hand: ['3', '3', 'J', '3', 'T'],
            bid: 969,
        }
    },
    {
        HandBid {
            hand: ['A', '2', '5', '4', '5'],
            bid: 516,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '9', '6', 'T'],
            bid: 686,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', 'T', 'Q', 'Q'],
            bid: 703,
        }
    },
    {
        HandBid {
            hand: ['T', 'K', 'T', 'K', 'K'],
            bid: 426,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'J', '4', '4'],
            bid: 559,
        }
    },
    {
        HandBid {
            hand: ['8', '3', 'K', '3', '3'],
            bid: 73,
        }
    },
    {
        HandBid {
            hand: ['2', '9', '9', '9', '9'],
            bid: 343,
        }
    },
    {
        HandBid {
            hand: ['K', '7', '3', '9', 'J'],
            bid: 996,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'A', '5', 'A'],
            bid: 687,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '3', '3', '4'],
            bid: 885,
        }
    },
    {
        HandBid {
            hand: ['T', '6', 'T', '6', 'T'],
            bid: 396,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '3', '3', 'Q'],
            bid: 923,
        }
    },
    {
        HandBid {
            hand: ['6', 'A', '6', '6', 'A'],
            bid: 798,
        }
    },
    {
        HandBid {
            hand: ['2', 'A', 'A', 'A', 'A'],
            bid: 844,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '2', '2', '3'],
            bid: 374,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', 'K', '4', '5'],
            bid: 419,
        }
    },
    {
        HandBid {
            hand: ['2', '4', '4', 'Q', '2'],
            bid: 683,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '4', '5'],
            bid: 624,
        }
    },
    {
        HandBid {
            hand: ['J', '9', '9', '9', '9'],
            bid: 770,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '2', '4', '2'],
            bid: 298,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '9', 'K', 'A'],
            bid: 581,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'A', '9', '7'],
            bid: 668,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', 'K', '8', '7'],
            bid: 861,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '4', 'A', 'Q'],
            bid: 229,
        }
    },
    {
        HandBid {
            hand: ['4', 'A', '6', '6', 'Q'],
            bid: 803,
        }
    },
    {
        HandBid {
            hand: ['K', 'J', 'K', 'J', 'K'],
            bid: 794,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '3', '4', 'Q'],
            bid: 272,
        }
    },
    {
        HandBid {
            hand: ['J', '4', '4', '9', '4'],
            bid: 545,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'K', '6', 'K'],
            bid: 295,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '7', '3', '3'],
            bid: 594,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', 'Q', 'Q', '5'],
            bid: 620,
        }
    },
    {
        HandBid {
            hand: ['9', '6', '2', '6', '9'],
            bid: 78,
        }
    },
    {
        HandBid {
            hand: ['9', '4', '7', '7', '3'],
            bid: 710,
        }
    },
    {
        HandBid {
            hand: ['4', '9', '3', '9', 'Q'],
            bid: 246,
        }
    },
    {
        HandBid {
            hand: ['2', '9', '9', '5', '9'],
            bid: 303,
        }
    },
    {
        HandBid {
            hand: ['8', '6', '8', '8', '8'],
            bid: 364,
        }
    },
    {
        HandBid {
            hand: ['T', '4', '8', '2', 'K'],
            bid: 474,
        }
    },
    {
        HandBid {
            hand: ['9', '4', '8', '4', '4'],
            bid: 407,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', 'Q', 'Q', 'Q'],
            bid: 685,
        }
    },
    {
        HandBid {
            hand: ['2', 'A', 'T', '9', '2'],
            bid: 709,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', '8', '8', '8'],
            bid: 921,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', '5', '3', 'J'],
            bid: 63,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '8', '5', '7'],
            bid: 893,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'T', 'J', 'K'],
            bid: 2,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', '5', '5', '5'],
            bid: 359,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '3', '7', '7'],
            bid: 966,
        }
    },
    {
        HandBid {
            hand: ['T', 'K', 'K', 'K', 'K'],
            bid: 859,
        }
    },
    {
        HandBid {
            hand: ['8', 'T', '4', '8', '7'],
            bid: 982,
        }
    },
    {
        HandBid {
            hand: ['K', 'A', '4', 'K', '4'],
            bid: 284,
        }
    },
    {
        HandBid {
            hand: ['9', '4', '3', '3', 'J'],
            bid: 62,
        }
    },
    {
        HandBid {
            hand: ['5', '2', 'A', '4', 'K'],
            bid: 5,
        }
    },
    {
        HandBid {
            hand: ['K', '6', 'K', 'A', 'J'],
            bid: 197,
        }
    },
    {
        HandBid {
            hand: ['K', '4', 'T', 'A', 'J'],
            bid: 252,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', 'Q', '8', 'T'],
            bid: 530,
        }
    },
    {
        HandBid {
            hand: ['7', '3', '6', 'Q', '2'],
            bid: 968,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'Q', 'Q', 'Q'],
            bid: 3,
        }
    },
    {
        HandBid {
            hand: ['6', '5', '8', '8', '8'],
            bid: 762,
        }
    },
    {
        HandBid {
            hand: ['8', '2', '8', '2', '8'],
            bid: 975,
        }
    },
    {
        HandBid {
            hand: ['K', 'A', 'K', '6', 'T'],
            bid: 512,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'T', 'J', 'T'],
            bid: 487,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '9', 'Q', 'Q'],
            bid: 60,
        }
    },
    {
        HandBid {
            hand: ['3', '4', 'J', '4', '6'],
            bid: 533,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'J', 'A', '2'],
            bid: 871,
        }
    },
    {
        HandBid {
            hand: ['7', '6', 'K', '5', 'A'],
            bid: 468,
        }
    },
    {
        HandBid {
            hand: ['9', '5', '2', 'A', '9'],
            bid: 400,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '8', '8', '7'],
            bid: 623,
        }
    },
    {
        HandBid {
            hand: ['J', '4', 'K', '4', '5'],
            bid: 427,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', 'J', '6'],
            bid: 312,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', 'K', '2', 'K'],
            bid: 791,
        }
    },
    {
        HandBid {
            hand: ['J', '3', '3', '3', '6'],
            bid: 597,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '3', '7', '7'],
            bid: 56,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'A', 'A', 'K'],
            bid: 562,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'Q', 'Q', 'T'],
            bid: 95,
        }
    },
    {
        HandBid {
            hand: ['T', 'A', '6', '6', 'Q'],
            bid: 914,
        }
    },
    {
        HandBid {
            hand: ['5', '2', '2', '2', '5'],
            bid: 187,
        }
    },
    {
        HandBid {
            hand: ['7', '6', 'K', 'J', 'J'],
            bid: 189,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'T', '6', '9'],
            bid: 156,
        }
    },
    {
        HandBid {
            hand: ['A', '9', '9', 'A', '9'],
            bid: 509,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', 'A', 'A', 'A'],
            bid: 316,
        }
    },
    {
        HandBid {
            hand: ['3', '3', 'T', '3', '3'],
            bid: 212,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'J', '2', 'J'],
            bid: 983,
        }
    },
    {
        HandBid {
            hand: ['4', '6', '4', '6', '6'],
            bid: 881,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'Q', 'T'],
            bid: 314,
        }
    },
    {
        HandBid {
            hand: ['4', '9', '4', '9', 'J'],
            bid: 373,
        }
    },
    {
        HandBid {
            hand: ['K', 'A', 'J', '2', 'K'],
            bid: 87,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '2', '5', '2'],
            bid: 352,
        }
    },
    {
        HandBid {
            hand: ['6', '7', '7', '7', '7'],
            bid: 645,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', 'T', '3', '5'],
            bid: 376,
        }
    },
    {
        HandBid {
            hand: ['4', '2', '4', '4', '4'],
            bid: 891,
        }
    },
    {
        HandBid {
            hand: ['Q', '2', '9', 'Q', '9'],
            bid: 226,
        }
    },
    {
        HandBid {
            hand: ['T', '4', '3', 'K', 'Q'],
            bid: 305,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '9', '9', 'A'],
            bid: 441,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'T', '5', 'A'],
            bid: 453,
        }
    },
    {
        HandBid {
            hand: ['9', '7', 'K', '6', 'Q'],
            bid: 110,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '9', '7', '9'],
            bid: 422,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'A', 'K'],
            bid: 293,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', 'T', 'J', 'K'],
            bid: 476,
        }
    },
    {
        HandBid {
            hand: ['Q', '7', 'J', 'T', 'T'],
            bid: 870,
        }
    },
    {
        HandBid {
            hand: ['2', '3', '2', '2', '2'],
            bid: 436,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', '5', 'Q', '9'],
            bid: 321,
        }
    },
    {
        HandBid {
            hand: ['4', 'Q', 'Q', 'K', 'Q'],
            bid: 90,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', '9', 'A', '9'],
            bid: 942,
        }
    },
    {
        HandBid {
            hand: ['K', '2', 'Q', 'T', '6'],
            bid: 429,
        }
    },
    {
        HandBid {
            hand: ['Q', '9', 'Q', '6', '2'],
            bid: 788,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '4', '9', '9'],
            bid: 186,
        }
    },
    {
        HandBid {
            hand: ['A', '5', '2', 'T', '3'],
            bid: 205,
        }
    },
    {
        HandBid {
            hand: ['6', '5', '2', 'A', 'A'],
            bid: 702,
        }
    },
    {
        HandBid {
            hand: ['J', '4', '8', '8', 'A'],
            bid: 900,
        }
    },
    {
        HandBid {
            hand: ['6', '4', '6', '2', '4'],
            bid: 584,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', 'K', '3', 'K'],
            bid: 223,
        }
    },
    {
        HandBid {
            hand: ['4', 'J', 'Q', '2', '5'],
            bid: 804,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'Q', 'K', 'T'],
            bid: 810,
        }
    },
    {
        HandBid {
            hand: ['6', '2', '9', '4', '6'],
            bid: 760,
        }
    },
    {
        HandBid {
            hand: ['4', '2', '4', '2', '2'],
            bid: 322,
        }
    },
    {
        HandBid {
            hand: ['A', '3', '5', '3', '3'],
            bid: 895,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'J', '9', '7'],
            bid: 790,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'K', '5', '5'],
            bid: 154,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', 'T', '2', '2'],
            bid: 519,
        }
    },
    {
        HandBid {
            hand: ['2', '6', '7', '7', '2'],
            bid: 592,
        }
    },
    {
        HandBid {
            hand: ['7', '6', '8', '6', '6'],
            bid: 813,
        }
    },
    {
        HandBid {
            hand: ['5', '6', 'A', 'J', 'A'],
            bid: 573,
        }
    },
    {
        HandBid {
            hand: ['K', 'Q', 'K', 'Q', 'Q'],
            bid: 880,
        }
    },
    {
        HandBid {
            hand: ['7', 'T', 'A', 'Q', '8'],
            bid: 649,
        }
    },
    {
        HandBid {
            hand: ['Q', '2', '2', '2', '9'],
            bid: 166,
        }
    },
    {
        HandBid {
            hand: ['6', 'A', 'T', 'J', '6'],
            bid: 837,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '7', '8', '4'],
            bid: 644,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', '4', 'J'],
            bid: 120,
        }
    },
    {
        HandBid {
            hand: ['T', '9', 'K', '6', 'T'],
            bid: 274,
        }
    },
    {
        HandBid {
            hand: ['J', 'K', 'J', 'J', 'K'],
            bid: 342,
        }
    },
    {
        HandBid {
            hand: ['2', 'T', 'K', '3', '5'],
            bid: 577,
        }
    },
    {
        HandBid {
            hand: ['7', '6', '6', '6', '6'],
            bid: 403,
        }
    },
    {
        HandBid {
            hand: ['5', 'J', '8', 'Q', '3'],
            bid: 239,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'J', '7'],
            bid: 52,
        }
    },
    {
        HandBid {
            hand: ['9', 'Q', 'Q', 'Q', 'J'],
            bid: 995,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '2', '8', '6'],
            bid: 457,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '7', '7', '7'],
            bid: 999,
        }
    },
    {
        HandBid {
            hand: ['2', '6', '6', '6', '2'],
            bid: 349,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'Q', 'A', 'J'],
            bid: 452,
        }
    },
    {
        HandBid {
            hand: ['8', '3', '8', 'Q', '8'],
            bid: 808,
        }
    },
    {
        HandBid {
            hand: ['3', '4', '3', '3', '9'],
            bid: 886,
        }
    },
    {
        HandBid {
            hand: ['7', '9', 'T', 'Q', '4'],
            bid: 366,
        }
    },
    {
        HandBid {
            hand: ['2', '7', '7', '8', '2'],
            bid: 148,
        }
    },
    {
        HandBid {
            hand: ['6', '3', '8', '6', '8'],
            bid: 764,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '7', '4'],
            bid: 622,
        }
    },
    {
        HandBid {
            hand: ['K', '2', 'T', '9', 'J'],
            bid: 939,
        }
    },
    {
        HandBid {
            hand: ['3', '4', '2', '2', '5'],
            bid: 652,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', 'T', 'T', 'T'],
            bid: 971,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '6', '4', 'T'],
            bid: 13,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '5', '9', 'T'],
            bid: 260,
        }
    },
    {
        HandBid {
            hand: ['9', '7', '9', 'A', 'A'],
            bid: 981,
        }
    },
    {
        HandBid {
            hand: ['7', '5', 'K', 'K', 'T'],
            bid: 727,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'K', '4', '9'],
            bid: 692,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', 'T', 'J', '9'],
            bid: 940,
        }
    },
    {
        HandBid {
            hand: ['A', 'T', '7', '5', 'T'],
            bid: 498,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '3', 'Q', 'K'],
            bid: 883,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', 'Q', '4', 'A'],
            bid: 970,
        }
    },
    {
        HandBid {
            hand: ['8', '9', '6', 'K', '8'],
            bid: 155,
        }
    },
    {
        HandBid {
            hand: ['J', '9', '5', '9', '5'],
            bid: 704,
        }
    },
    {
        HandBid {
            hand: ['2', '5', 'J', '5', '2'],
            bid: 817,
        }
    },
    {
        HandBid {
            hand: ['Q', '3', '4', '7', 'A'],
            bid: 288,
        }
    },
    {
        HandBid {
            hand: ['K', '9', '9', '9', '9'],
            bid: 51,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', '9', 'K', '9'],
            bid: 98,
        }
    },
    {
        HandBid {
            hand: ['K', '4', '9', 'J', '7'],
            bid: 697,
        }
    },
    {
        HandBid {
            hand: ['A', '3', '2', 'A', 'J'],
            bid: 410,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '2', 'Q', 'J'],
            bid: 466,
        }
    },
    {
        HandBid {
            hand: ['3', '6', '3', 'Q', 'Q'],
            bid: 23,
        }
    },
    {
        HandBid {
            hand: ['5', '3', 'J', '3', 'A'],
            bid: 725,
        }
    },
    {
        HandBid {
            hand: ['3', '2', '3', '4', '3'],
            bid: 829,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'Q', '2'],
            bid: 69,
        }
    },
    {
        HandBid {
            hand: ['3', 'K', '3', 'J', '3'],
            bid: 605,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'A', 'J', 'A'],
            bid: 699,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '5', '7', 'T'],
            bid: 117,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '4', 'K', '4'],
            bid: 280,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '7', '7', 'K'],
            bid: 147,
        }
    },
    {
        HandBid {
            hand: ['4', '6', '6', '4', '9'],
            bid: 526,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '6', '6', 'K'],
            bid: 541,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'Q', '7', 'K'],
            bid: 409,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', '2', '6'],
            bid: 472,
        }
    },
    {
        HandBid {
            hand: ['5', '4', 'Q', 'K', '4'],
            bid: 527,
        }
    },
    {
        HandBid {
            hand: ['A', '2', '5', 'T', 'T'],
            bid: 12,
        }
    },
    {
        HandBid {
            hand: ['T', '6', 'Q', 'T', 'T'],
            bid: 404,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '7', 'K', 'K'],
            bid: 550,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '2', '8', '2'],
            bid: 391,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '4', '5', '5'],
            bid: 37,
        }
    },
    {
        HandBid {
            hand: ['3', 'T', '3', 'T', '3'],
            bid: 816,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '5', 'A', '5'],
            bid: 111,
        }
    },
    {
        HandBid {
            hand: ['3', '6', 'T', '9', '8'],
            bid: 241,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'J', '5', '8'],
            bid: 430,
        }
    },
    {
        HandBid {
            hand: ['5', 'Q', 'T', 'Q', '7'],
            bid: 353,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '8', '9', '9'],
            bid: 386,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', 'A', '8', '8'],
            bid: 227,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '8', '8', '7'],
            bid: 713,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '5', 'J', '7'],
            bid: 848,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '8', '8'],
            bid: 38,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', 'K', '8', 'A'],
            bid: 389,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', 'A', '5', '4'],
            bid: 555,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', 'Q', '4'],
            bid: 911,
        }
    },
    {
        HandBid {
            hand: ['6', '9', '9', 'T', '7'],
            bid: 766,
        }
    },
    {
        HandBid {
            hand: ['5', 'J', 'Q', 'T', 'J'],
            bid: 330,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', 'K', '9', '8'],
            bid: 780,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'J', 'J', 'A'],
            bid: 752,
        }
    },
    {
        HandBid {
            hand: ['6', '2', '7', '9', 'T'],
            bid: 857,
        }
    },
    {
        HandBid {
            hand: ['9', '5', '7', 'T', '4'],
            bid: 621,
        }
    },
    {
        HandBid {
            hand: ['5', '5', 'A', 'A', 'A'],
            bid: 379,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '9', 'Q', '6'],
            bid: 29,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '8', 'J', '7'],
            bid: 818,
        }
    },
    {
        HandBid {
            hand: ['4', '7', 'A', 'Q', 'T'],
            bid: 348,
        }
    },
    {
        HandBid {
            hand: ['K', 'Q', '4', '8', 'T'],
            bid: 955,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', '4', '9', 'T'],
            bid: 494,
        }
    },
    {
        HandBid {
            hand: ['A', '7', 'A', '4', '2'],
            bid: 535,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'Q', '4'],
            bid: 180,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '9', '6', '3'],
            bid: 777,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '3', '6', '6'],
            bid: 730,
        }
    },
    {
        HandBid {
            hand: ['3', '7', 'J', '5', '9'],
            bid: 520,
        }
    },
    {
        HandBid {
            hand: ['8', 'T', 'T', '8', '8'],
            bid: 945,
        }
    },
    {
        HandBid {
            hand: ['Q', 'A', '4', 'T', 'T'],
            bid: 561,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '3', 'J', '7'],
            bid: 542,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '5', '4', 'K'],
            bid: 825,
        }
    },
    {
        HandBid {
            hand: ['4', '6', 'Q', 'J', '6'],
            bid: 437,
        }
    },
    {
        HandBid {
            hand: ['T', '8', '8', '8', '5'],
            bid: 669,
        }
    },
    {
        HandBid {
            hand: ['8', '3', 'K', '7', '6'],
            bid: 751,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', '8', '5', '3'],
            bid: 948,
        }
    },
    {
        HandBid {
            hand: ['6', '4', '8', '4', '4'],
            bid: 36,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '4', 'J', '4'],
            bid: 77,
        }
    },
    {
        HandBid {
            hand: ['A', '7', '7', '7', '7'],
            bid: 372,
        }
    },
    {
        HandBid {
            hand: ['3', '2', '3', 'J', '3'],
            bid: 946,
        }
    },
    {
        HandBid {
            hand: ['4', '5', '3', '8', '7'],
            bid: 944,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '7', '6', '9'],
            bid: 202,
        }
    },
    {
        HandBid {
            hand: ['9', '3', '3', '3', '9'],
            bid: 497,
        }
    },
    {
        HandBid {
            hand: ['8', '5', 'Q', '7', '9'],
            bid: 14,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'A', 'J', '2'],
            bid: 413,
        }
    },
    {
        HandBid {
            hand: ['9', '8', 'J', 'A', '8'],
            bid: 311,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'J', '3'],
            bid: 326,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'Q', 'A', 'Q'],
            bid: 463,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'Q', '4', 'Q'],
            bid: 151,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', 'A', '9', '8'],
            bid: 96,
        }
    },
    {
        HandBid {
            hand: ['T', 'A', 'T', 'T', 'A'],
            bid: 965,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', '3', '7', 'A'],
            bid: 421,
        }
    },
    {
        HandBid {
            hand: ['K', '3', '8', 'Q', '9'],
            bid: 635,
        }
    },
    {
        HandBid {
            hand: ['J', '7', '2', 'T', 'K'],
            bid: 59,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', '8', '5', 'A'],
            bid: 164,
        }
    },
    {
        HandBid {
            hand: ['3', '9', '9', '9', '9'],
            bid: 854,
        }
    },
    {
        HandBid {
            hand: ['5', '9', '5', '7', '2'],
            bid: 109,
        }
    },
    {
        HandBid {
            hand: ['5', '7', 'T', 'J', '5'],
            bid: 74,
        }
    },
    {
        HandBid {
            hand: ['8', '3', '8', '8', '8'],
            bid: 862,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'A', '2', 'J'],
            bid: 763,
        }
    },
    {
        HandBid {
            hand: ['J', 'J', '4', '5', '5'],
            bid: 771,
        }
    },
    {
        HandBid {
            hand: ['T', 'K', 'J', 'A', '2'],
            bid: 834,
        }
    },
    {
        HandBid {
            hand: ['5', 'Q', 'J', '8', '5'],
            bid: 495,
        }
    },
    {
        HandBid {
            hand: ['J', '4', '4', '4', '4'],
            bid: 932,
        }
    },
    {
        HandBid {
            hand: ['2', 'T', 'T', '3', 'Q'],
            bid: 856,
        }
    },
    {
        HandBid {
            hand: ['9', '8', 'K', 'T', '3'],
            bid: 438,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', 'K', 'K', '7'],
            bid: 627,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '6', 'J', 'A'],
            bid: 638,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '9', 'Q', '9'],
            bid: 628,
        }
    },
    {
        HandBid {
            hand: ['J', '6', '5', '6', '5'],
            bid: 245,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', 'K', 'T', 'A'],
            bid: 711,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'Q', 'Q', '9'],
            bid: 247,
        }
    },
    {
        HandBid {
            hand: ['3', '4', 'Q', 'T', '4'],
            bid: 162,
        }
    },
    {
        HandBid {
            hand: ['6', '8', '4', 'K', 'K'],
            bid: 731,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', '5', '7', 'A'],
            bid: 254,
        }
    },
    {
        HandBid {
            hand: ['T', '9', '7', '5', '5'],
            bid: 297,
        }
    },
    {
        HandBid {
            hand: ['9', 'A', '9', '6', 'K'],
            bid: 658,
        }
    },
    {
        HandBid {
            hand: ['3', '4', '4', '8', '4'],
            bid: 240,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '6', '6', 'J'],
            bid: 450,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', 'K', '4'],
            bid: 570,
        }
    },
    {
        HandBid {
            hand: ['A', '5', 'A', 'Q', 'K'],
            bid: 878,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', '6', '6', '2'],
            bid: 905,
        }
    },
    {
        HandBid {
            hand: ['A', '6', '6', '2', '2'],
            bid: 822,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'Q', '3', '9'],
            bid: 177,
        }
    },
    {
        HandBid {
            hand: ['8', '5', '5', '5', '5'],
            bid: 858,
        }
    },
    {
        HandBid {
            hand: ['7', 'T', '5', '3', 'Q'],
            bid: 178,
        }
    },
    {
        HandBid {
            hand: ['8', 'K', 'A', '6', 'T'],
            bid: 931,
        }
    },
    {
        HandBid {
            hand: ['2', '9', 'K', '9', '9'],
            bid: 104,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'T', 'Q', '4'],
            bid: 960,
        }
    },
    {
        HandBid {
            hand: ['A', 'J', '5', '5', '5'],
            bid: 514,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', 'T', '3', 'T'],
            bid: 214,
        }
    },
    {
        HandBid {
            hand: ['K', 'A', '8', '8', 'J'],
            bid: 912,
        }
    },
    {
        HandBid {
            hand: ['J', 'J', 'J', 'J', '8'],
            bid: 102,
        }
    },
    {
        HandBid {
            hand: ['3', '6', '3', '3', 'A'],
            bid: 748,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', '4', '4', '7'],
            bid: 705,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '4', '4', '4'],
            bid: 45,
        }
    },
    {
        HandBid {
            hand: ['2', 'Q', 'A', 'Q', '2'],
            bid: 125,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', 'A', '7', 'T'],
            bid: 721,
        }
    },
    {
        HandBid {
            hand: ['T', '6', 'Q', 'J', 'K'],
            bid: 324,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', 'A', 'A', 'A'],
            bid: 617,
        }
    },
    {
        HandBid {
            hand: ['4', 'A', 'Q', 'A', 'J'],
            bid: 585,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', 'J', '7', '7'],
            bid: 576,
        }
    },
    {
        HandBid {
            hand: ['A', '3', '2', 'T', 'Q'],
            bid: 747,
        }
    },
    {
        HandBid {
            hand: ['7', '6', '6', '7', '7'],
            bid: 618,
        }
    },
    {
        HandBid {
            hand: ['A', 'J', '5', 'Q', '9'],
            bid: 70,
        }
    },
    {
        HandBid {
            hand: ['8', '9', '6', '2', '7'],
            bid: 360,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', '5', 'T', '7'],
            bid: 602,
        }
    },
    {
        HandBid {
            hand: ['A', '8', 'T', '3', '6'],
            bid: 896,
        }
    },
    {
        HandBid {
            hand: ['5', 'A', 'A', '3', '6'],
            bid: 411,
        }
    },
    {
        HandBid {
            hand: ['6', '2', '5', '5', 'J'],
            bid: 841,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '6', '4', '4'],
            bid: 612,
        }
    },
    {
        HandBid {
            hand: ['3', 'T', 'J', 'T', 'K'],
            bid: 66,
        }
    },
    {
        HandBid {
            hand: ['4', 'J', 'T', '4', 'T'],
            bid: 640,
        }
    },
    {
        HandBid {
            hand: ['J', 'T', '4', 'T', '8'],
            bid: 146,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '3', '3', 'J'],
            bid: 973,
        }
    },
    {
        HandBid {
            hand: ['K', '6', '9', '4', '4'],
            bid: 256,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', '5', 'K', 'K'],
            bid: 936,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', 'T', '6', 'K'],
            bid: 388,
        }
    },
    {
        HandBid {
            hand: ['3', 'J', 'Q', '3', '5'],
            bid: 793,
        }
    },
    {
        HandBid {
            hand: ['9', '4', '9', '9', '4'],
            bid: 296,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '5', '3'],
            bid: 930,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'Q', '7'],
            bid: 646,
        }
    },
    {
        HandBid {
            hand: ['A', '2', 'A', 'A', 'Q'],
            bid: 399,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', 'A', '6'],
            bid: 32,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '3', '2', '3'],
            bid: 75,
        }
    },
    {
        HandBid {
            hand: ['6', '2', '2', '6', '2'],
            bid: 957,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '4', '5', 'T'],
            bid: 54,
        }
    },
    {
        HandBid {
            hand: ['8', '2', '8', 'J', '8'],
            bid: 974,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', 'T', 'Q'],
            bid: 383,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', '6', 'Q', '9'],
            bid: 261,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '5', '2', '2'],
            bid: 27,
        }
    },
    {
        HandBid {
            hand: ['Q', 'K', '6', '6', '6'],
            bid: 716,
        }
    },
    {
        HandBid {
            hand: ['7', '8', 'K', 'K', '5'],
            bid: 242,
        }
    },
    {
        HandBid {
            hand: ['T', '2', 'Q', '2', '2'],
            bid: 728,
        }
    },
    {
        HandBid {
            hand: ['J', '9', 'K', 'K', 'Q'],
            bid: 424,
        }
    },
    {
        HandBid {
            hand: ['3', '8', '9', 'A', '6'],
            bid: 738,
        }
    },
    {
        HandBid {
            hand: ['9', '8', '9', '8', '9'],
            bid: 292,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '8', '3', 'K'],
            bid: 68,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '4', '4', 'J'],
            bid: 578,
        }
    },
    {
        HandBid {
            hand: ['2', 'Q', '6', '6', '5'],
            bid: 899,
        }
    },
    {
        HandBid {
            hand: ['5', '8', '8', '9', '5'],
            bid: 938,
        }
    },
    {
        HandBid {
            hand: ['6', '3', '3', '6', '3'],
            bid: 1,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '5', '8', '4'],
            bid: 172,
        }
    },
    {
        HandBid {
            hand: ['J', 'J', 'T', '8', '3'],
            bid: 206,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '8', '8', '4'],
            bid: 695,
        }
    },
    {
        HandBid {
            hand: ['J', '9', 'K', '8', 'J'],
            bid: 579,
        }
    },
    {
        HandBid {
            hand: ['7', '2', '5', '3', '2'],
            bid: 414,
        }
    },
    {
        HandBid {
            hand: ['Q', '9', 'T', '8', 'A'],
            bid: 877,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '3', 'K', '3'],
            bid: 318,
        }
    },
    {
        HandBid {
            hand: ['K', '3', '7', 'Q', '4'],
            bid: 253,
        }
    },
    {
        HandBid {
            hand: ['9', '9', '9', '9', 'Q'],
            bid: 675,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'Q', 'Q', '5'],
            bid: 884,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', 'T', '5', '4'],
            bid: 4,
        }
    },
    {
        HandBid {
            hand: ['3', 'K', '3', 'T', 'T'],
            bid: 209,
        }
    },
    {
        HandBid {
            hand: ['4', 'K', 'J', '4', '4'],
            bid: 401,
        }
    },
    {
        HandBid {
            hand: ['5', 'J', '2', '7', '9'],
            bid: 225,
        }
    },
    {
        HandBid {
            hand: ['2', '2', '2', 'J', '2'],
            bid: 333,
        }
    },
    {
        HandBid {
            hand: ['6', '7', 'J', '8', '8'],
            bid: 543,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '2', 'T', 'Q'],
            bid: 175,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', '9', '6', 'Q'],
            bid: 745,
        }
    },
    {
        HandBid {
            hand: ['T', '4', '6', 'T', '6'],
            bid: 113,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '5', '5', '7'],
            bid: 361,
        }
    },
    {
        HandBid {
            hand: ['A', '4', '8', 'A', '4'],
            bid: 130,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'Q', '2', 'Q'],
            bid: 10,
        }
    },
    {
        HandBid {
            hand: ['4', '2', '2', '9', '2'],
            bid: 558,
        }
    },
    {
        HandBid {
            hand: ['Q', 'K', 'Q', 'Q', 'Q'],
            bid: 523,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'J', 'Q', 'A'],
            bid: 188,
        }
    },
    {
        HandBid {
            hand: ['T', '6', '7', '6', 'T'],
            bid: 65,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', '5', 'T', '6'],
            bid: 510,
        }
    },
    {
        HandBid {
            hand: ['4', '2', '6', 'Q', '9'],
            bid: 72,
        }
    },
    {
        HandBid {
            hand: ['2', 'Q', '2', '2', '2'],
            bid: 934,
        }
    },
    {
        HandBid {
            hand: ['7', '5', '7', 'T', 'Q'],
            bid: 888,
        }
    },
    {
        HandBid {
            hand: ['8', '9', '9', 'T', 'T'],
            bid: 673,
        }
    },
    {
        HandBid {
            hand: ['5', '9', 'K', 'Q', '4'],
            bid: 723,
        }
    },
    {
        HandBid {
            hand: ['J', '6', '6', '6', 'A'],
            bid: 927,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', '7', 'Q', '7'],
            bid: 286,
        }
    },
    {
        HandBid {
            hand: ['J', '8', '8', '6', 'Q'],
            bid: 774,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', '2', 'Q', 'Q'],
            bid: 599,
        }
    },
    {
        HandBid {
            hand: ['K', '4', '4', '8', 'K'],
            bid: 93,
        }
    },
    {
        HandBid {
            hand: ['Q', 'K', 'K', 'Q', 'K'],
            bid: 689,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '5', '5', '6'],
            bid: 480,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '5', 'A', 'A'],
            bid: 951,
        }
    },
    {
        HandBid {
            hand: ['T', '7', 'T', '8', '7'],
            bid: 462,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '8', '4', 'J'],
            bid: 216,
        }
    },
    {
        HandBid {
            hand: ['J', 'T', '4', 'T', '6'],
            bid: 988,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '3', 'Q', 'K'],
            bid: 654,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '2', '7', '7'],
            bid: 337,
        }
    },
    {
        HandBid {
            hand: ['A', '9', 'Q', '6', '6'],
            bid: 317,
        }
    },
    {
        HandBid {
            hand: ['8', '2', 'J', '8', '2'],
            bid: 64,
        }
    },
    {
        HandBid {
            hand: ['J', '7', '5', '9', '5'],
            bid: 935,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', '2', 'Q', '6'],
            bid: 81,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '4', '2', '7'],
            bid: 152,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '2', 'K', 'Q'],
            bid: 589,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '2', '2', '2'],
            bid: 787,
        }
    },
    {
        HandBid {
            hand: ['7', '8', '6', '8', 'K'],
            bid: 826,
        }
    },
    {
        HandBid {
            hand: ['7', '5', 'A', '3', 'J'],
            bid: 916,
        }
    },
    {
        HandBid {
            hand: ['9', 'A', '2', '9', '2'],
            bid: 950,
        }
    },
    {
        HandBid {
            hand: ['6', 'J', 'T', '4', '4'],
            bid: 237,
        }
    },
    {
        HandBid {
            hand: ['T', '7', 'T', 'J', '6'],
            bid: 846,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', 'K', 'J', 'A'],
            bid: 564,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', '8', '4', '4'],
            bid: 676,
        }
    },
    {
        HandBid {
            hand: ['Q', '4', '3', '8', '4'],
            bid: 538,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', 'A', '6', '8'],
            bid: 114,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', '8', '6', '8'],
            bid: 505,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '5', 'T', 'K'],
            bid: 820,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '4', '4'],
            bid: 874,
        }
    },
    {
        HandBid {
            hand: ['A', 'J', 'K', 'A', 'K'],
            bid: 30,
        }
    },
    {
        HandBid {
            hand: ['8', '2', 'A', '3', '4'],
            bid: 915,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', '8', '3', 'Q'],
            bid: 251,
        }
    },
    {
        HandBid {
            hand: ['9', '7', 'T', 'J', 'A'],
            bid: 496,
        }
    },
    {
        HandBid {
            hand: ['4', '5', '7', '4', '4'],
            bid: 608,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '6', 'Q', 'Q'],
            bid: 234,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', '9', 'K'],
            bid: 271,
        }
    },
    {
        HandBid {
            hand: ['3', 'A', '9', '9', 'A'],
            bid: 465,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '7', '3', '4'],
            bid: 444,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '6', '6', 'T'],
            bid: 507,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'J', 'K', 'K'],
            bid: 20,
        }
    },
    {
        HandBid {
            hand: ['2', 'J', '6', '7', '3'],
            bid: 492,
        }
    },
    {
        HandBid {
            hand: ['5', 'J', '7', '4', '4'],
            bid: 416,
        }
    },
    {
        HandBid {
            hand: ['9', '3', 'K', '6', '4'],
            bid: 369,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', 'K', 'K', '7'],
            bid: 998,
        }
    },
    {
        HandBid {
            hand: ['3', '3', '2', '3', '2'],
            bid: 719,
        }
    },
    {
        HandBid {
            hand: ['A', '3', 'A', '3', '3'],
            bid: 91,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', '3', 'J', '7'],
            bid: 267,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '5', '5', '6'],
            bid: 972,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'Q', '8', 'Q'],
            bid: 616,
        }
    },
    {
        HandBid {
            hand: ['T', '4', '4', '4', 'J'],
            bid: 548,
        }
    },
    {
        HandBid {
            hand: ['T', 'J', '3', 'T', 'J'],
            bid: 586,
        }
    },
    {
        HandBid {
            hand: ['9', '2', '8', '5', '6'],
            bid: 963,
        }
    },
    {
        HandBid {
            hand: ['K', '9', '8', 'T', '5'],
            bid: 432,
        }
    },
    {
        HandBid {
            hand: ['J', '6', 'J', '6', 'J'],
            bid: 772,
        }
    },
    {
        HandBid {
            hand: ['4', 'J', '3', '9', 'Q'],
            bid: 643,
        }
    },
    {
        HandBid {
            hand: ['3', '4', '5', '3', '5'],
            bid: 39,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '7', 'Q', '7'],
            bid: 953,
        }
    },
    {
        HandBid {
            hand: ['T', '4', '2', 'J', '4'],
            bid: 805,
        }
    },
    {
        HandBid {
            hand: ['2', '4', '4', '2', 'A'],
            bid: 47,
        }
    },
    {
        HandBid {
            hand: ['9', '5', '4', 'A', '8'],
            bid: 395,
        }
    },
    {
        HandBid {
            hand: ['K', '4', '4', '6', '6'],
            bid: 991,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'A', 'A', 'A'],
            bid: 174,
        }
    },
    {
        HandBid {
            hand: ['J', '3', '9', 'K', 'A'],
            bid: 532,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '2', 'Q', '3'],
            bid: 672,
        }
    },
    {
        HandBid {
            hand: ['2', '5', '5', '7', '7'],
            bid: 428,
        }
    },
    {
        HandBid {
            hand: ['Q', 'T', '6', '5', '9'],
            bid: 715,
        }
    },
    {
        HandBid {
            hand: ['J', '6', '6', '9', '8'],
            bid: 910,
        }
    },
    {
        HandBid {
            hand: ['3', '5', '3', '5', '5'],
            bid: 84,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '8', '9', '5'],
            bid: 218,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '6', '7', '4'],
            bid: 6,
        }
    },
    {
        HandBid {
            hand: ['K', '7', '5', '3', '7'],
            bid: 567,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', '6', '3', '4'],
            bid: 380,
        }
    },
    {
        HandBid {
            hand: ['6', '8', '2', '8', 'J'],
            bid: 811,
        }
    },
    {
        HandBid {
            hand: ['7', 'T', 'J', '8', 'A'],
            bid: 82,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '7', 'Q', '7'],
            bid: 674,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '4', '8', 'A'],
            bid: 244,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '2', '8', '8'],
            bid: 257,
        }
    },
    {
        HandBid {
            hand: ['K', '9', '9', '3', '3'],
            bid: 889,
        }
    },
    {
        HandBid {
            hand: ['T', '6', '2', '6', 'T'],
            bid: 269,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', '4', '2', 'Q'],
            bid: 443,
        }
    },
    {
        HandBid {
            hand: ['2', 'K', 'A', '9', 'Q'],
            bid: 167,
        }
    },
    {
        HandBid {
            hand: ['K', '4', 'K', 'K', 'K'],
            bid: 16,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '7', '5', '7'],
            bid: 159,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', 'T', 'Q', '6'],
            bid: 845,
        }
    },
    {
        HandBid {
            hand: ['4', '4', '8', '8', '8'],
            bid: 560,
        }
    },
    {
        HandBid {
            hand: ['K', '6', '5', 'A', 'T'],
            bid: 191,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', '6', '8', 'Q'],
            bid: 176,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'J', 'J', '8'],
            bid: 978,
        }
    },
    {
        HandBid {
            hand: ['6', 'K', '5', '3', 'J'],
            bid: 615,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'J', '2', '2'],
            bid: 525,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'J', '3', 'J'],
            bid: 99,
        }
    },
    {
        HandBid {
            hand: ['9', '8', '8', '9', '8'],
            bid: 906,
        }
    },
    {
        HandBid {
            hand: ['4', '8', 'J', 'A', '2'],
            bid: 454,
        }
    },
    {
        HandBid {
            hand: ['8', '6', 'J', '3', '6'],
            bid: 879,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '8', '6', '6'],
            bid: 524,
        }
    },
    {
        HandBid {
            hand: ['3', 'A', 'A', '3', 'A'],
            bid: 872,
        }
    },
    {
        HandBid {
            hand: ['6', '8', 'Q', '4', '6'],
            bid: 733,
        }
    },
    {
        HandBid {
            hand: ['3', '8', 'J', 'Q', '3'],
            bid: 338,
        }
    },
    {
        HandBid {
            hand: ['6', '5', 'J', '7', '5'],
            bid: 128,
        }
    },
    {
        HandBid {
            hand: ['5', '7', 'J', '7', '7'],
            bid: 455,
        }
    },
    {
        HandBid {
            hand: ['A', 'K', 'A', '8', 'K'],
            bid: 138,
        }
    },
    {
        HandBid {
            hand: ['2', '2', 'J', '7', '2'],
            bid: 200,
        }
    },
    {
        HandBid {
            hand: ['9', 'T', '7', 'A', '6'],
            bid: 310,
        }
    },
    {
        HandBid {
            hand: ['4', '7', '7', '7', '7'],
            bid: 25,
        }
    },
    {
        HandBid {
            hand: ['A', '3', '6', '8', '5'],
            bid: 503,
        }
    },
    {
        HandBid {
            hand: ['T', '5', 'A', '6', '8'],
            bid: 100,
        }
    },
    {
        HandBid {
            hand: ['9', '7', '9', 'J', '8'],
            bid: 528,
        }
    },
    {
        HandBid {
            hand: ['9', 'J', 'A', '2', '2'],
            bid: 782,
        }
    },
    {
        HandBid {
            hand: ['K', 'J', 'K', 'K', 'Q'],
            bid: 913,
        }
    },
    {
        HandBid {
            hand: ['6', '4', '9', '4', '2'],
            bid: 907,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', '9', '5', '4'],
            bid: 598,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', 'K', 'K', 'K'],
            bid: 876,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', '5', 'T', 'T'],
            bid: 779,
        }
    },
    {
        HandBid {
            hand: ['3', '9', '9', '2', 'T'],
            bid: 678,
        }
    },
    {
        HandBid {
            hand: ['5', 'Q', 'Q', 'Q', '9'],
            bid: 265,
        }
    },
    {
        HandBid {
            hand: ['3', '5', '9', '2', 'K'],
            bid: 924,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '5', '4', 'A'],
            bid: 493,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '6', '9', '8'],
            bid: 170,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '9', '6', 'J'],
            bid: 488,
        }
    },
    {
        HandBid {
            hand: ['T', 'T', '4', '7', '4'],
            bid: 192,
        }
    },
    {
        HandBid {
            hand: ['J', 'K', '9', '5', 'A'],
            bid: 80,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'J', 'J', 'J'],
            bid: 40,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '6', '4', '4'],
            bid: 307,
        }
    },
    {
        HandBid {
            hand: ['J', '5', '7', 'K', '5'],
            bid: 650,
        }
    },
    {
        HandBid {
            hand: ['J', '8', 'K', '8', '2'],
            bid: 961,
        }
    },
    {
        HandBid {
            hand: ['8', 'J', '9', '9', '8'],
            bid: 688,
        }
    },
    {
        HandBid {
            hand: ['T', '9', '9', '8', '4'],
            bid: 9,
        }
    },
    {
        HandBid {
            hand: ['3', '3', 'J', '3', '5'],
            bid: 89,
        }
    },
    {
        HandBid {
            hand: ['J', '2', 'A', 'T', '9'],
            bid: 985,
        }
    },
    {
        HandBid {
            hand: ['6', '2', '3', '9', 'J'],
            bid: 662,
        }
    },
    {
        HandBid {
            hand: ['8', '8', 'J', 'J', '8'],
            bid: 595,
        }
    },
    {
        HandBid {
            hand: ['J', 'K', '8', '5', '2'],
            bid: 604,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '3', '4', '7'],
            bid: 741,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', 'T', '6', '9'],
            bid: 278,
        }
    },
    {
        HandBid {
            hand: ['5', '5', '6', '6', '5'],
            bid: 696,
        }
    },
    {
        HandBid {
            hand: ['7', '9', '4', '2', 'J'],
            bid: 736,
        }
    },
    {
        HandBid {
            hand: ['3', '5', 'A', 'K', '4'],
            bid: 358,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '5', '5', 'J'],
            bid: 384,
        }
    },
    {
        HandBid {
            hand: ['5', '8', 'A', '8', '4'],
            bid: 613,
        }
    },
    {
        HandBid {
            hand: ['5', 'T', 'T', 'T', 'T'],
            bid: 919,
        }
    },
    {
        HandBid {
            hand: ['7', 'A', '9', 'T', '7'],
            bid: 7,
        }
    },
    {
        HandBid {
            hand: ['K', '3', '3', 'Q', 'K'],
            bid: 354,
        }
    },
    {
        HandBid {
            hand: ['A', 'Q', 'Q', 'A', 'A'],
            bid: 235,
        }
    },
    {
        HandBid {
            hand: ['3', '9', 'T', 'J', 'T'],
            bid: 196,
        }
    },
    {
        HandBid {
            hand: ['2', '6', '2', '2', '9'],
            bid: 976,
        }
    },
    {
        HandBid {
            hand: ['6', '6', '6', '4', 'Q'],
            bid: 865,
        }
    },
    {
        HandBid {
            hand: ['9', '9', 'Q', 'Q', 'Q'],
            bid: 236,
        }
    },
    {
        HandBid {
            hand: ['3', '7', '4', 'A', '6'],
            bid: 956,
        }
    },
    {
        HandBid {
            hand: ['8', 'Q', 'T', 'T', '8'],
            bid: 964,
        }
    },
    {
        HandBid {
            hand: ['A', '7', '6', 'A', 'Q'],
            bid: 112,
        }
    },
    {
        HandBid {
            hand: ['T', '8', '6', '6', '8'],
            bid: 611,
        }
    },
    {
        HandBid {
            hand: ['6', '6', 'J', 'J', '6'],
            bid: 729,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '6', 'Q', '6'],
            bid: 814,
        }
    },
    {
        HandBid {
            hand: ['K', 'K', '4', '4', 'J'],
            bid: 993,
        }
    },
    {
        HandBid {
            hand: ['6', '2', 'A', 'J', '8'],
            bid: 127,
        }
    },
    {
        HandBid {
            hand: ['A', 'T', 'J', '6', '4'],
            bid: 402,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '5', '9', '4'],
            bid: 483,
        }
    },
    {
        HandBid {
            hand: ['9', '6', '4', '9', '6'],
            bid: 990,
        }
    },
    {
        HandBid {
            hand: ['7', '4', '2', '4', 'J'],
            bid: 761,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '8', 'K', '3'],
            bid: 653,
        }
    },
    {
        HandBid {
            hand: ['4', '8', '6', 'K', '3'],
            bid: 980,
        }
    },
    {
        HandBid {
            hand: ['J', 'Q', 'A', '4', '4'],
            bid: 852,
        }
    },
    {
        HandBid {
            hand: ['2', 'T', 'J', '6', 'J'],
            bid: 339,
        }
    },
    {
        HandBid {
            hand: ['4', '4', 'T', '4', '4'],
            bid: 446,
        }
    },
    {
        HandBid {
            hand: ['7', '3', 'K', 'T', 'K'],
            bid: 121,
        }
    },
    {
        HandBid {
            hand: ['T', '7', '7', '8', '7'],
            bid: 513,
        }
    },
    {
        HandBid {
            hand: ['K', '6', '7', 'Q', 'T'],
            bid: 717,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '2', '8', '7'],
            bid: 136,
        }
    },
    {
        HandBid {
            hand: ['7', 'Q', 'Q', 'J', '8'],
            bid: 18,
        }
    },
    {
        HandBid {
            hand: ['4', '3', '4', '2', 'T'],
            bid: 768,
        }
    },
    {
        HandBid {
            hand: ['5', 'K', 'K', '5', 'J'],
            bid: 847,
        }
    },
    {
        HandBid {
            hand: ['Q', '2', 'Q', '7', 'T'],
            bid: 666,
        }
    },
    {
        HandBid {
            hand: ['6', 'Q', 'J', 'T', '2'],
            bid: 529,
        }
    },
    {
        HandBid {
            hand: ['7', 'T', '7', 'A', '7'],
            bid: 392,
        }
    },
    {
        HandBid {
            hand: ['J', '6', '6', '7', '6'],
            bid: 445,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '3', '3', 'K'],
            bid: 647,
        }
    },
    {
        HandBid {
            hand: ['Q', '6', '7', '6', 'A'],
            bid: 926,
        }
    },
    {
        HandBid {
            hand: ['Q', 'J', 'Q', 'J', 'Q'],
            bid: 806,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', 'J', '8'],
            bid: 928,
        }
    },
    {
        HandBid {
            hand: ['4', '8', '6', '9', '2'],
            bid: 691,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', 'A', '5', '5'],
            bid: 984,
        }
    },
    {
        HandBid {
            hand: ['J', '2', '4', '2', '3'],
            bid: 800,
        }
    },
    {
        HandBid {
            hand: ['3', '2', 'J', 'Q', '8'],
            bid: 362,
        }
    },
    {
        HandBid {
            hand: ['T', '2', '7', '4', '4'],
            bid: 937,
        }
    },
    {
        HandBid {
            hand: ['7', 'J', '9', 'T', 'Q'],
            bid: 277,
        }
    },
    {
        HandBid {
            hand: ['7', 'K', '3', '2', '6'],
            bid: 86,
        }
    },
    {
        HandBid {
            hand: ['3', 'K', '9', 'T', 'J'],
            bid: 860,
        }
    },
    {
        HandBid {
            hand: ['7', '7', 'J', 'K', 'T'],
            bid: 71,
        }
    },
    {
        HandBid {
            hand: ['4', '6', '6', '4', '4'],
            bid: 31,
        }
    },
    {
        HandBid {
            hand: ['2', '3', 'J', '2', 'Q'],
            bid: 323,
        }
    },
    {
        HandBid {
            hand: ['J', '3', 'K', '9', '6'],
            bid: 393,
        }
    },
    {
        HandBid {
            hand: ['8', '7', 'Q', 'K', 'K'],
            bid: 456,
        }
    },
    {
        HandBid {
            hand: ['T', 'Q', '5', 'K', '2'],
            bid: 569,
        }
    },
    {
        HandBid {
            hand: ['A', 'A', 'K', '9', 'T'],
            bid: 949,
        }
    },
    {
        HandBid {
            hand: ['5', '6', '9', '3', '2'],
            bid: 539,
        }
    },
    {
        HandBid {
            hand: ['Q', '5', 'Q', 'Q', 'Q'],
            bid: 356,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', '4', 'A', '2'],
            bid: 126,
        }
    },
    {
        HandBid {
            hand: ['T', '5', '3', 'J', '2'],
            bid: 283,
        }
    },
    {
        HandBid {
            hand: ['8', '7', '3', '2', 'Q'],
            bid: 461,
        }
    },
    {
        HandBid {
            hand: ['K', 'T', 'T', 'K', 'T'],
            bid: 556,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', 'J', '3', 'K'],
            bid: 918,
        }
    },
    {
        HandBid {
            hand: ['J', 'A', 'A', 'A', '7'],
            bid: 340,
        }
    },
    {
        HandBid {
            hand: ['2', '5', '5', 'K', 'Q'],
            bid: 469,
        }
    },
    {
        HandBid {
            hand: ['8', 'A', '9', 'A', 'J'],
            bid: 201,
        }
    },
    {
        HandBid {
            hand: ['Q', '8', '9', '6', '5'],
            bid: 144,
        }
    },
    {
        HandBid {
            hand: ['7', '7', '7', 'J', '2'],
            bid: 477,
        }
    },
    {
        HandBid {
            hand: ['A', '8', 'A', 'J', 'Q'],
            bid: 797,
        }
    },
    {
        HandBid {
            hand: ['3', 'Q', 'Q', 'Q', '9'],
            bid: 786,
        }
    },
    {
        HandBid {
            hand: ['6', 'Q', '7', 'T', '4'],
            bid: 460,
        }
    },
    {
        HandBid {
            hand: ['Q', 'Q', 'Q', '8', '2'],
            bid: 656,
        }
    },
    {
        HandBid {
            hand: ['4', 'K', '3', '2', '9'],
            bid: 222,
        }
    },
    {
        HandBid {
            hand: ['A', '9', '9', '7', '9'],
            bid: 190,
        }
    },
    {
        HandBid {
            hand: ['6', 'T', '6', '6', '4'],
            bid: 440,
        }
    },
    {
        HandBid {
            hand: ['4', 'A', '4', '3', '4'],
            bid: 153,
        }
    },
    {
        HandBid {
            hand: ['Q', '7', 'J', 'Q', '2'],
            bid: 194,
        }
    },
    {
        HandBid {
            hand: ['7', '9', '9', '3', 'J'],
            bid: 651,
        }
    },
    {
        HandBid {
            hand: ['8', '8', '2', '2', 'K'],
            bid: 195,
        }
    },
    {
        HandBid {
            hand: ['7', 'Q', 'Q', '7', '7'],
            bid: 464,
        }
    },
    {
        HandBid {
            hand: ['5', '2', '2', '9', '5'],
            bid: 371,
        }
    },
];
