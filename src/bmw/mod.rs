use crate::utils::read_u32_le;

const INITVAL: [u32; 16] = [
    0x4041_4243,
    0x4445_4647,
    0x4849_4A4B,
    0x4C4D_4E4F,
    0x5051_5253,
    0x5455_5657,
    0x5859_5A5B,
    0x5C5D_5E5F,
    0x6061_6263,
    0x6465_6667,
    0x6869_6A6B,
    0x6C6D_6E6F,
    0x7071_7273,
    0x7475_7677,
    0x7879_7A7B,
    0x7C7D_7E7F,
];

const FINAL: [u32; 16] = [
    0xaaaa_aaa0,
    0xaaaa_aaa1,
    0xaaaa_aaa2,
    0xaaaa_aaa3,
    0xaaaa_aaa4,
    0xaaaa_aaa5,
    0xaaaa_aaa6,
    0xaaaa_aaa7,
    0xaaaa_aaa8,
    0xaaaa_aaa9,
    0xaaaa_aaaa,
    0xaaaa_aaab,
    0xaaaa_aaac,
    0xaaaa_aaad,
    0xaaaa_aaae,
    0xaaaa_aaaf,
];

fn circular_left(x: u32, n: u32) -> u32 {
    (x << n) | (x >> (32 - n))
}

struct Bmw {
    m: [u32; 16],
    h: [u32; 16],
    h2: [u32; 16],
    q: [u32; 32],
}

fn new() -> Bmw {
    let mut b = Bmw {
        m: [0; 16],
        h: [0; 16],
        h2: [0; 16],
        q: [0; 32],
    };
    b.h[0] = INITVAL[0];
    b.h[1] = INITVAL[1];
    b.h[2] = INITVAL[2];
    b.h[3] = INITVAL[3];
    b.h[4] = INITVAL[4];
    b.h[5] = INITVAL[5];
    b.h[6] = INITVAL[6];
    b.h[7] = INITVAL[7];
    b.h[8] = INITVAL[8];
    b.h[9] = INITVAL[9];
    b.h[10] = INITVAL[10];
    b.h[11] = INITVAL[11];
    b.h[12] = INITVAL[12];
    b.h[13] = INITVAL[13];
    b.h[14] = INITVAL[14];
    b.h[15] = INITVAL[15];
    b
}

fn compress(mut b: Bmw, m: [u32; 16]) -> Bmw {
    let mut h = b.h;
    let mut q = b.q;
    q[0] = ((((m[5] ^ h[5])
        .wrapping_sub(m[7] ^ h[7])
        .wrapping_add(m[10] ^ h[10])
        .wrapping_add(m[13] ^ h[13])
        .wrapping_add(m[14] ^ h[14]))
        >> 1)
        ^ (((m[5] ^ h[5])
            .wrapping_sub(m[7] ^ h[7])
            .wrapping_add(m[10] ^ h[10])
            .wrapping_add(m[13] ^ h[13])
            .wrapping_add(m[14] ^ h[14]))
            << 3)
        ^ circular_left(
            (m[5] ^ h[5])
                .wrapping_sub(m[7] ^ h[7])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_add(m[13] ^ h[13])
                .wrapping_add(m[14] ^ h[14]),
            4,
        )
        ^ circular_left(
            (m[5] ^ h[5])
                .wrapping_sub(m[7] ^ h[7])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_add(m[13] ^ h[13])
                .wrapping_add(m[14] ^ h[14]),
            19,
        ))
    .wrapping_add(h[1]);
    q[1] = ((((m[6] ^ h[6])
        .wrapping_sub(m[8] ^ h[8])
        .wrapping_add(m[11] ^ h[11])
        .wrapping_add(m[14] ^ h[14])
        .wrapping_sub(m[15] ^ h[15]))
        >> 1)
        ^ (((m[6] ^ h[6])
            .wrapping_sub(m[8] ^ h[8])
            .wrapping_add(m[11] ^ h[11])
            .wrapping_add(m[14] ^ h[14])
            .wrapping_sub(m[15] ^ h[15]))
            << 2)
        ^ circular_left(
            (m[6] ^ h[6])
                .wrapping_sub(m[8] ^ h[8])
                .wrapping_add(m[11] ^ h[11])
                .wrapping_add(m[14] ^ h[14])
                .wrapping_sub(m[15] ^ h[15]),
            8,
        )
        ^ circular_left(
            (m[6] ^ h[6])
                .wrapping_sub(m[8] ^ h[8])
                .wrapping_add(m[11] ^ h[11])
                .wrapping_add(m[14] ^ h[14])
                .wrapping_sub(m[15] ^ h[15]),
            23,
        ))
    .wrapping_add(h[2]);
    q[2] = ((((m[0] ^ h[0])
        .wrapping_add(m[7] ^ h[7])
        .wrapping_add(m[9] ^ h[9])
        .wrapping_sub(m[12] ^ h[12])
        .wrapping_add(m[15] ^ h[15]))
        >> 2)
        ^ (((m[0] ^ h[0])
            .wrapping_add(m[7] ^ h[7])
            .wrapping_add(m[9] ^ h[9])
            .wrapping_sub(m[12] ^ h[12])
            .wrapping_add(m[15] ^ h[15]))
            << 1)
        ^ circular_left(
            (m[0] ^ h[0])
                .wrapping_add(m[7] ^ h[7])
                .wrapping_add(m[9] ^ h[9])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_add(m[15] ^ h[15]),
            12,
        )
        ^ circular_left(
            (m[0] ^ h[0])
                .wrapping_add(m[7] ^ h[7])
                .wrapping_add(m[9] ^ h[9])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_add(m[15] ^ h[15]),
            25,
        ))
    .wrapping_add(h[3]);
    q[3] = ((((m[0] ^ h[0])
        .wrapping_sub(m[1] ^ h[1])
        .wrapping_add(m[8] ^ h[8])
        .wrapping_sub(m[10] ^ h[10])
        .wrapping_add(m[13] ^ h[13]))
        >> 2)
        ^ (((m[0] ^ h[0])
            .wrapping_sub(m[1] ^ h[1])
            .wrapping_add(m[8] ^ h[8])
            .wrapping_sub(m[10] ^ h[10])
            .wrapping_add(m[13] ^ h[13]))
            << 2)
        ^ circular_left(
            (m[0] ^ h[0])
                .wrapping_sub(m[1] ^ h[1])
                .wrapping_add(m[8] ^ h[8])
                .wrapping_sub(m[10] ^ h[10])
                .wrapping_add(m[13] ^ h[13]),
            15,
        )
        ^ circular_left(
            (m[0] ^ h[0])
                .wrapping_sub(m[1] ^ h[1])
                .wrapping_add(m[8] ^ h[8])
                .wrapping_sub(m[10] ^ h[10])
                .wrapping_add(m[13] ^ h[13]),
            29,
        ))
    .wrapping_add(h[4]);
    q[4] = ((((m[1] ^ h[1])
        .wrapping_add(m[2] ^ h[2])
        .wrapping_add(m[9] ^ h[9])
        .wrapping_sub(m[11] ^ h[11])
        .wrapping_sub(m[14] ^ h[14]))
        >> 1)
        ^ ((m[1] ^ h[1])
            .wrapping_add(m[2] ^ h[2])
            .wrapping_add(m[9] ^ h[9])
            .wrapping_sub(m[11] ^ h[11])
            .wrapping_sub(m[14] ^ h[14])))
    .wrapping_add(h[5]);
    q[5] = ((((m[3] ^ h[3])
        .wrapping_sub(m[2] ^ h[2])
        .wrapping_add(m[10] ^ h[10])
        .wrapping_sub(m[12] ^ h[12])
        .wrapping_add(m[15] ^ h[15]))
        >> 1)
        ^ (((m[3] ^ h[3])
            .wrapping_sub(m[2] ^ h[2])
            .wrapping_add(m[10] ^ h[10])
            .wrapping_sub(m[12] ^ h[12])
            .wrapping_add(m[15] ^ h[15]))
            << 3)
        ^ circular_left(
            (m[3] ^ h[3])
                .wrapping_sub(m[2] ^ h[2])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_add(m[15] ^ h[15]),
            4,
        )
        ^ circular_left(
            (m[3] ^ h[3])
                .wrapping_sub(m[2] ^ h[2])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_add(m[15] ^ h[15]),
            19,
        ))
    .wrapping_add(h[6]);
    q[6] = ((((m[4] ^ h[4])
        .wrapping_sub(m[0] ^ h[0])
        .wrapping_sub(m[3] ^ h[3])
        .wrapping_sub(m[11] ^ h[11])
        .wrapping_add(m[13] ^ h[13]))
        >> 1)
        ^ (((m[4] ^ h[4])
            .wrapping_sub(m[0] ^ h[0])
            .wrapping_sub(m[3] ^ h[3])
            .wrapping_sub(m[11] ^ h[11])
            .wrapping_add(m[13] ^ h[13]))
            << 2)
        ^ circular_left(
            (m[4] ^ h[4])
                .wrapping_sub(m[0] ^ h[0])
                .wrapping_sub(m[3] ^ h[3])
                .wrapping_sub(m[11] ^ h[11])
                .wrapping_add(m[13] ^ h[13]),
            8,
        )
        ^ circular_left(
            (m[4] ^ h[4])
                .wrapping_sub(m[0] ^ h[0])
                .wrapping_sub(m[3] ^ h[3])
                .wrapping_sub(m[11] ^ h[11])
                .wrapping_add(m[13] ^ h[13]),
            23,
        ))
    .wrapping_add(h[7]);
    q[7] = ((((m[1] ^ h[1])
        .wrapping_sub(m[4] ^ h[4])
        .wrapping_sub(m[5] ^ h[5])
        .wrapping_sub(m[12] ^ h[12])
        .wrapping_sub(m[14] ^ h[14]))
        >> 2)
        ^ (((m[1] ^ h[1])
            .wrapping_sub(m[4] ^ h[4])
            .wrapping_sub(m[5] ^ h[5])
            .wrapping_sub(m[12] ^ h[12])
            .wrapping_sub(m[14] ^ h[14]))
            << 1)
        ^ circular_left(
            (m[1] ^ h[1])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_sub(m[14] ^ h[14]),
            12,
        )
        ^ circular_left(
            (m[1] ^ h[1])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_sub(m[12] ^ h[12])
                .wrapping_sub(m[14] ^ h[14]),
            25,
        ))
    .wrapping_add(h[8]);
    q[8] = ((((m[2] ^ h[2])
        .wrapping_sub(m[5] ^ h[5])
        .wrapping_sub(m[6] ^ h[6])
        .wrapping_add(m[13] ^ h[13])
        .wrapping_sub(m[15] ^ h[15]))
        >> 2)
        ^ (((m[2] ^ h[2])
            .wrapping_sub(m[5] ^ h[5])
            .wrapping_sub(m[6] ^ h[6])
            .wrapping_add(m[13] ^ h[13])
            .wrapping_sub(m[15] ^ h[15]))
            << 2)
        ^ circular_left(
            (m[2] ^ h[2])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_add(m[13] ^ h[13])
                .wrapping_sub(m[15] ^ h[15]),
            15,
        )
        ^ circular_left(
            (m[2] ^ h[2])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_add(m[13] ^ h[13])
                .wrapping_sub(m[15] ^ h[15]),
            29,
        ))
    .wrapping_add(h[9]);
    q[9] = ((((m[0] ^ h[0])
        .wrapping_sub(m[3] ^ h[3])
        .wrapping_add(m[6] ^ h[6])
        .wrapping_sub(m[7] ^ h[7])
        .wrapping_add(m[14] ^ h[14]))
        >> 1)
        ^ ((m[0] ^ h[0])
            .wrapping_sub(m[3] ^ h[3])
            .wrapping_add(m[6] ^ h[6])
            .wrapping_sub(m[7] ^ h[7])
            .wrapping_add(m[14] ^ h[14])))
    .wrapping_add(h[10]);
    q[10] = ((((m[8] ^ h[8])
        .wrapping_sub(m[1] ^ h[1])
        .wrapping_sub(m[4] ^ h[4])
        .wrapping_sub(m[7] ^ h[7])
        .wrapping_add(m[15] ^ h[15]))
        >> 1)
        ^ (((m[8] ^ h[8])
            .wrapping_sub(m[1] ^ h[1])
            .wrapping_sub(m[4] ^ h[4])
            .wrapping_sub(m[7] ^ h[7])
            .wrapping_add(m[15] ^ h[15]))
            << 3)
        ^ circular_left(
            (m[8] ^ h[8])
                .wrapping_sub(m[1] ^ h[1])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[7] ^ h[7])
                .wrapping_add(m[15] ^ h[15]),
            4,
        )
        ^ circular_left(
            (m[8] ^ h[8])
                .wrapping_sub(m[1] ^ h[1])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[7] ^ h[7])
                .wrapping_add(m[15] ^ h[15]),
            19,
        ))
    .wrapping_add(h[11]);
    q[11] = ((((m[8] ^ h[8])
        .wrapping_sub(m[0] ^ h[0])
        .wrapping_sub(m[2] ^ h[2])
        .wrapping_sub(m[5] ^ h[5])
        .wrapping_add(m[9] ^ h[9]))
        >> 1)
        ^ (((m[8] ^ h[8])
            .wrapping_sub(m[0] ^ h[0])
            .wrapping_sub(m[2] ^ h[2])
            .wrapping_sub(m[5] ^ h[5])
            .wrapping_add(m[9] ^ h[9]))
            << 2)
        ^ circular_left(
            (m[8] ^ h[8])
                .wrapping_sub(m[0] ^ h[0])
                .wrapping_sub(m[2] ^ h[2])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_add(m[9] ^ h[9]),
            8,
        )
        ^ circular_left(
            (m[8] ^ h[8])
                .wrapping_sub(m[0] ^ h[0])
                .wrapping_sub(m[2] ^ h[2])
                .wrapping_sub(m[5] ^ h[5])
                .wrapping_add(m[9] ^ h[9]),
            23,
        ))
    .wrapping_add(h[12]);
    q[12] = ((((m[1] ^ h[1])
        .wrapping_add(m[3] ^ h[3])
        .wrapping_sub(m[6] ^ h[6])
        .wrapping_sub(m[9] ^ h[9])
        .wrapping_add(m[10] ^ h[10]))
        >> 2)
        ^ (((m[1] ^ h[1])
            .wrapping_add(m[3] ^ h[3])
            .wrapping_sub(m[6] ^ h[6])
            .wrapping_sub(m[9] ^ h[9])
            .wrapping_add(m[10] ^ h[10]))
            << 1)
        ^ circular_left(
            (m[1] ^ h[1])
                .wrapping_add(m[3] ^ h[3])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_sub(m[9] ^ h[9])
                .wrapping_add(m[10] ^ h[10]),
            12,
        )
        ^ circular_left(
            (m[1] ^ h[1])
                .wrapping_add(m[3] ^ h[3])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_sub(m[9] ^ h[9])
                .wrapping_add(m[10] ^ h[10]),
            25,
        ))
    .wrapping_add(h[13]);
    q[13] = ((((m[2] ^ h[2])
        .wrapping_add(m[4] ^ h[4])
        .wrapping_add(m[7] ^ h[7])
        .wrapping_add(m[10] ^ h[10])
        .wrapping_add(m[11] ^ h[11]))
        >> 2)
        ^ (((m[2] ^ h[2])
            .wrapping_add(m[4] ^ h[4])
            .wrapping_add(m[7] ^ h[7])
            .wrapping_add(m[10] ^ h[10])
            .wrapping_add(m[11] ^ h[11]))
            << 2)
        ^ circular_left(
            (m[2] ^ h[2])
                .wrapping_add(m[4] ^ h[4])
                .wrapping_add(m[7] ^ h[7])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_add(m[11] ^ h[11]),
            15,
        )
        ^ circular_left(
            (m[2] ^ h[2])
                .wrapping_add(m[4] ^ h[4])
                .wrapping_add(m[7] ^ h[7])
                .wrapping_add(m[10] ^ h[10])
                .wrapping_add(m[11] ^ h[11]),
            29,
        ))
    .wrapping_add(h[14]);
    q[14] = ((((m[3] ^ h[3])
        .wrapping_sub(m[5] ^ h[5])
        .wrapping_add(m[8] ^ h[8])
        .wrapping_sub(m[11] ^ h[11])
        .wrapping_sub(m[12] ^ h[12]))
        >> 1)
        ^ ((m[3] ^ h[3])
            .wrapping_sub(m[5] ^ h[5])
            .wrapping_add(m[8] ^ h[8])
            .wrapping_sub(m[11] ^ h[11])
            .wrapping_sub(m[12] ^ h[12])))
    .wrapping_add(h[15]);
    q[15] = ((((m[12] ^ h[12])
        .wrapping_sub(m[4] ^ h[4])
        .wrapping_sub(m[6] ^ h[6])
        .wrapping_sub(m[9] ^ h[9])
        .wrapping_add(m[13] ^ h[13]))
        >> 1)
        ^ (((m[12] ^ h[12])
            .wrapping_sub(m[4] ^ h[4])
            .wrapping_sub(m[6] ^ h[6])
            .wrapping_sub(m[9] ^ h[9])
            .wrapping_add(m[13] ^ h[13]))
            << 3)
        ^ circular_left(
            (m[12] ^ h[12])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_sub(m[9] ^ h[9])
                .wrapping_add(m[13] ^ h[13]),
            4,
        )
        ^ circular_left(
            (m[12] ^ h[12])
                .wrapping_sub(m[4] ^ h[4])
                .wrapping_sub(m[6] ^ h[6])
                .wrapping_sub(m[9] ^ h[9])
                .wrapping_add(m[13] ^ h[13]),
            19,
        ))
    .wrapping_add(h[0]);
    q[16] = ((q[0] >> 1) ^ (q[0] << 2) ^ circular_left(q[0], 8) ^ circular_left(q[0], 23))
        .wrapping_add((q[1] >> 2) ^ (q[1] << 1) ^ circular_left(q[1], 12) ^ circular_left(q[1], 25))
        .wrapping_add((q[2] >> 2) ^ (q[2] << 2) ^ circular_left(q[2], 15) ^ circular_left(q[2], 29))
        .wrapping_add((q[3] >> 1) ^ (q[3] << 3) ^ circular_left(q[3], 4) ^ circular_left(q[3], 19))
        .wrapping_add((q[4] >> 1) ^ (q[4] << 2) ^ circular_left(q[4], 8) ^ circular_left(q[4], 23))
        .wrapping_add((q[5] >> 2) ^ (q[5] << 1) ^ circular_left(q[5], 12) ^ circular_left(q[5], 25))
        .wrapping_add((q[6] >> 2) ^ (q[6] << 2) ^ circular_left(q[6], 15) ^ circular_left(q[6], 29))
        .wrapping_add((q[7] >> 1) ^ (q[7] << 3) ^ circular_left(q[7], 4) ^ circular_left(q[7], 19))
        .wrapping_add((q[8] >> 1) ^ (q[8] << 2) ^ circular_left(q[8], 8) ^ circular_left(q[8], 23))
        .wrapping_add((q[9] >> 2) ^ (q[9] << 1) ^ circular_left(q[9], 12) ^ circular_left(q[9], 25))
        .wrapping_add(
            (q[10] >> 2) ^ (q[10] << 2) ^ circular_left(q[10], 15) ^ circular_left(q[10], 29),
        )
        .wrapping_add(
            (q[11] >> 1) ^ (q[11] << 3) ^ circular_left(q[11], 4) ^ circular_left(q[11], 19),
        )
        .wrapping_add(
            (q[12] >> 1) ^ (q[12] << 2) ^ circular_left(q[12], 8) ^ circular_left(q[12], 23),
        )
        .wrapping_add(
            (q[13] >> 2) ^ (q[13] << 1) ^ circular_left(q[13], 12) ^ circular_left(q[13], 25),
        )
        .wrapping_add(
            (q[14] >> 2) ^ (q[14] << 2) ^ circular_left(q[14], 15) ^ circular_left(q[14], 29),
        )
        .wrapping_add(
            (q[15] >> 1) ^ (q[15] << 3) ^ circular_left(q[15], 4) ^ circular_left(q[15], 19),
        )
        .wrapping_add(
            (circular_left(m[0], 1)
                .wrapping_add(circular_left(m[3], 4))
                .wrapping_sub(circular_left(m[10], 11))
                .wrapping_add(16 * 0x0555_5555))
                ^ h[7],
        );
    q[17] = ((q[1] >> 1) ^ (q[1] << 2) ^ circular_left(q[1], 8) ^ circular_left(q[1], 23))
        .wrapping_add((q[2] >> 2) ^ (q[2] << 1) ^ circular_left(q[2], 12) ^ circular_left(q[2], 25))
        .wrapping_add((q[3] >> 2) ^ (q[3] << 2) ^ circular_left(q[3], 15) ^ circular_left(q[3], 29))
        .wrapping_add((q[4] >> 1) ^ (q[4] << 3) ^ circular_left(q[4], 4) ^ circular_left(q[4], 19))
        .wrapping_add((q[5] >> 1) ^ (q[5] << 2) ^ circular_left(q[5], 8) ^ circular_left(q[5], 23))
        .wrapping_add((q[6] >> 2) ^ (q[6] << 1) ^ circular_left(q[6], 12) ^ circular_left(q[6], 25))
        .wrapping_add((q[7] >> 2) ^ (q[7] << 2) ^ circular_left(q[7], 15) ^ circular_left(q[7], 29))
        .wrapping_add((q[8] >> 1) ^ (q[8] << 3) ^ circular_left(q[8], 4) ^ circular_left(q[8], 19))
        .wrapping_add((q[9] >> 1) ^ (q[9] << 2) ^ circular_left(q[9], 8) ^ circular_left(q[9], 23))
        .wrapping_add(
            (q[10] >> 2) ^ (q[10] << 1) ^ circular_left(q[10], 12) ^ circular_left(q[10], 25),
        )
        .wrapping_add(
            (q[11] >> 2) ^ (q[11] << 2) ^ circular_left(q[11], 15) ^ circular_left(q[11], 29),
        )
        .wrapping_add(
            (q[12] >> 1) ^ (q[12] << 3) ^ circular_left(q[12], 4) ^ circular_left(q[12], 19),
        )
        .wrapping_add(
            (q[13] >> 1) ^ (q[13] << 2) ^ circular_left(q[13], 8) ^ circular_left(q[13], 23),
        )
        .wrapping_add(
            (q[14] >> 2) ^ (q[14] << 1) ^ circular_left(q[14], 12) ^ circular_left(q[14], 25),
        )
        .wrapping_add(
            (q[15] >> 2) ^ (q[15] << 2) ^ circular_left(q[15], 15) ^ circular_left(q[15], 29),
        )
        .wrapping_add(
            (q[16] >> 1) ^ (q[16] << 3) ^ circular_left(q[16], 4) ^ circular_left(q[16], 19),
        )
        .wrapping_add(
            (circular_left(m[1], 2)
                .wrapping_add(circular_left(m[4], 5))
                .wrapping_sub(circular_left(m[11], 12))
                .wrapping_add(17 * 0x0555_5555))
                ^ h[8],
        );
    q[18] = q[2]
        .wrapping_add(circular_left(q[3], 3))
        .wrapping_add(q[4])
        .wrapping_add(circular_left(q[5], 7))
        .wrapping_add(q[6])
        .wrapping_add(circular_left(q[7], 13))
        .wrapping_add(q[8])
        .wrapping_add(circular_left(q[9], 16))
        .wrapping_add(q[10])
        .wrapping_add(circular_left(q[11], 19))
        .wrapping_add(q[12])
        .wrapping_add(circular_left(q[13], 23))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 27))
        .wrapping_add((q[16] >> 1) ^ q[16])
        .wrapping_add((q[17] >> 2) ^ q[17])
        .wrapping_add(
            (circular_left(m[2], 3)
                .wrapping_add(circular_left(m[5], 6))
                .wrapping_sub(circular_left(m[12], 13))
                .wrapping_add(18 * 0x0555_5555))
                ^ h[9],
        );
    q[19] = q[3]
        .wrapping_add(circular_left(q[4], 3))
        .wrapping_add(q[5])
        .wrapping_add(circular_left(q[6], 7))
        .wrapping_add(q[7])
        .wrapping_add(circular_left(q[8], 13))
        .wrapping_add(q[9])
        .wrapping_add(circular_left(q[10], 16))
        .wrapping_add(q[11])
        .wrapping_add(circular_left(q[12], 19))
        .wrapping_add(q[13])
        .wrapping_add(circular_left(q[14], 23))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 27))
        .wrapping_add((q[17] >> 1) ^ q[17])
        .wrapping_add((q[18] >> 2) ^ q[18])
        .wrapping_add(
            (circular_left(m[3], 4)
                .wrapping_add(circular_left(m[6], 7))
                .wrapping_sub(circular_left(m[13], 14))
                .wrapping_add(19 * 0x0555_5555))
                ^ h[10],
        );
    q[20] = q[4]
        .wrapping_add(circular_left(q[5], 3))
        .wrapping_add(q[6])
        .wrapping_add(circular_left(q[7], 7))
        .wrapping_add(q[8])
        .wrapping_add(circular_left(q[9], 13))
        .wrapping_add(q[10])
        .wrapping_add(circular_left(q[11], 16))
        .wrapping_add(q[12])
        .wrapping_add(circular_left(q[13], 19))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 23))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 27))
        .wrapping_add((q[18] >> 1) ^ q[18])
        .wrapping_add((q[19] >> 2) ^ q[19])
        .wrapping_add(
            (circular_left(m[4], 5)
                .wrapping_add(circular_left(m[7], 8))
                .wrapping_sub(circular_left(m[14], 15))
                .wrapping_add(20 * 0x0555_5555))
                ^ h[11],
        );
    q[21] = q[5]
        .wrapping_add(circular_left(q[6], 3))
        .wrapping_add(q[7])
        .wrapping_add(circular_left(q[8], 7))
        .wrapping_add(q[9])
        .wrapping_add(circular_left(q[10], 13))
        .wrapping_add(q[11])
        .wrapping_add(circular_left(q[12], 16))
        .wrapping_add(q[13])
        .wrapping_add(circular_left(q[14], 19))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 23))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 27))
        .wrapping_add((q[19] >> 1) ^ q[19])
        .wrapping_add((q[20] >> 2) ^ q[20])
        .wrapping_add(
            (circular_left(m[5], 6)
                .wrapping_add(circular_left(m[8], 9))
                .wrapping_sub(circular_left(m[15], 16))
                .wrapping_add(21 * 0x0555_5555))
                ^ h[12],
        );
    q[22] = q[6]
        .wrapping_add(circular_left(q[7], 3))
        .wrapping_add(q[8])
        .wrapping_add(circular_left(q[9], 7))
        .wrapping_add(q[10])
        .wrapping_add(circular_left(q[11], 13))
        .wrapping_add(q[12])
        .wrapping_add(circular_left(q[13], 16))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 19))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 23))
        .wrapping_add(q[18])
        .wrapping_add(circular_left(q[19], 27))
        .wrapping_add((q[20] >> 1) ^ q[20])
        .wrapping_add((q[21] >> 2) ^ q[21])
        .wrapping_add(
            (circular_left(m[6], 7)
                .wrapping_add(circular_left(m[9], 10))
                .wrapping_sub(circular_left(m[0], 1))
                .wrapping_add(22 * 0x0555_5555))
                ^ h[13],
        );
    q[23] = q[7]
        .wrapping_add(circular_left(q[8], 3))
        .wrapping_add(q[9])
        .wrapping_add(circular_left(q[10], 7))
        .wrapping_add(q[11])
        .wrapping_add(circular_left(q[12], 13))
        .wrapping_add(q[13])
        .wrapping_add(circular_left(q[14], 16))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 19))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 23))
        .wrapping_add(q[19])
        .wrapping_add(circular_left(q[20], 27))
        .wrapping_add((q[21] >> 1) ^ q[21])
        .wrapping_add((q[22] >> 2) ^ q[22])
        .wrapping_add(
            (circular_left(m[7], 8)
                .wrapping_add(circular_left(m[10], 11))
                .wrapping_sub(circular_left(m[1], 2))
                .wrapping_add(23 * 0x0555_5555))
                ^ h[14],
        );
    q[24] = q[8]
        .wrapping_add(circular_left(q[9], 3))
        .wrapping_add(q[10])
        .wrapping_add(circular_left(q[11], 7))
        .wrapping_add(q[12])
        .wrapping_add(circular_left(q[13], 13))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 16))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 19))
        .wrapping_add(q[18])
        .wrapping_add(circular_left(q[19], 23))
        .wrapping_add(q[20])
        .wrapping_add(circular_left(q[21], 27))
        .wrapping_add((q[22] >> 1) ^ q[22])
        .wrapping_add((q[23] >> 2) ^ q[23])
        .wrapping_add(
            (circular_left(m[8], 9)
                .wrapping_add(circular_left(m[11], 12))
                .wrapping_sub(circular_left(m[2], 3))
                .wrapping_add(24 * 0x0555_5555))
                ^ h[15],
        );
    q[25] = q[9]
        .wrapping_add(circular_left(q[10], 3))
        .wrapping_add(q[11])
        .wrapping_add(circular_left(q[12], 7))
        .wrapping_add(q[13])
        .wrapping_add(circular_left(q[14], 13))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 16))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 19))
        .wrapping_add(q[19])
        .wrapping_add(circular_left(q[20], 23))
        .wrapping_add(q[21])
        .wrapping_add(circular_left(q[22], 27))
        .wrapping_add((q[23] >> 1) ^ q[23])
        .wrapping_add((q[24] >> 2) ^ q[24])
        .wrapping_add(
            (circular_left(m[9], 10)
                .wrapping_add(circular_left(m[12], 13))
                .wrapping_sub(circular_left(m[3], 4))
                .wrapping_add(25 * 0x0555_5555))
                ^ h[0],
        );
    q[26] = q[10]
        .wrapping_add(circular_left(q[11], 3))
        .wrapping_add(q[12])
        .wrapping_add(circular_left(q[13], 7))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 13))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 16))
        .wrapping_add(q[18])
        .wrapping_add(circular_left(q[19], 19))
        .wrapping_add(q[20])
        .wrapping_add(circular_left(q[21], 23))
        .wrapping_add(q[22])
        .wrapping_add(circular_left(q[23], 27))
        .wrapping_add((q[24] >> 1) ^ q[24])
        .wrapping_add((q[25] >> 2) ^ q[25])
        .wrapping_add(
            (circular_left(m[10], 11)
                .wrapping_add(circular_left(m[13], 14))
                .wrapping_sub(circular_left(m[4], 5))
                .wrapping_add(26 * 0x0555_5555))
                ^ h[1],
        );
    q[27] = q[11]
        .wrapping_add(circular_left(q[12], 3))
        .wrapping_add(q[13])
        .wrapping_add(circular_left(q[14], 7))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 13))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 16))
        .wrapping_add(q[19])
        .wrapping_add(circular_left(q[20], 19))
        .wrapping_add(q[21])
        .wrapping_add(circular_left(q[22], 23))
        .wrapping_add(q[23])
        .wrapping_add(circular_left(q[24], 27))
        .wrapping_add((q[25] >> 1) ^ q[25])
        .wrapping_add((q[26] >> 2) ^ q[26])
        .wrapping_add(
            (circular_left(m[11], 12)
                .wrapping_add(circular_left(m[14], 15))
                .wrapping_sub(circular_left(m[5], 6))
                .wrapping_add(27 * 0x0555_5555))
                ^ h[2],
        );
    q[28] = q[12]
        .wrapping_add(circular_left(q[13], 3))
        .wrapping_add(q[14])
        .wrapping_add(circular_left(q[15], 7))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 13))
        .wrapping_add(q[18])
        .wrapping_add(circular_left(q[19], 16))
        .wrapping_add(q[20])
        .wrapping_add(circular_left(q[21], 19))
        .wrapping_add(q[22])
        .wrapping_add(circular_left(q[23], 23))
        .wrapping_add(q[24])
        .wrapping_add(circular_left(q[25], 27))
        .wrapping_add((q[26] >> 1) ^ q[26])
        .wrapping_add((q[27] >> 2) ^ q[27])
        .wrapping_add(
            (circular_left(m[12], 13)
                .wrapping_add(circular_left(m[15], 16))
                .wrapping_sub(circular_left(m[6], 7))
                .wrapping_add(28 * 0x0555_5555))
                ^ h[3],
        );
    q[29] = q[13]
        .wrapping_add(circular_left(q[14], 3))
        .wrapping_add(q[15])
        .wrapping_add(circular_left(q[16], 7))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 13))
        .wrapping_add(q[19])
        .wrapping_add(circular_left(q[20], 16))
        .wrapping_add(q[21])
        .wrapping_add(circular_left(q[22], 19))
        .wrapping_add(q[23])
        .wrapping_add(circular_left(q[24], 23))
        .wrapping_add(q[25])
        .wrapping_add(circular_left(q[26], 27))
        .wrapping_add((q[27] >> 1) ^ q[27])
        .wrapping_add((q[28] >> 2) ^ q[28])
        .wrapping_add(
            (circular_left(m[13], 14)
                .wrapping_add(circular_left(m[0], 1))
                .wrapping_sub(circular_left(m[7], 8))
                .wrapping_add(29 * 0x0555_5555))
                ^ h[4],
        );
    q[30] = q[14]
        .wrapping_add(circular_left(q[15], 3))
        .wrapping_add(q[16])
        .wrapping_add(circular_left(q[17], 7))
        .wrapping_add(q[18])
        .wrapping_add(circular_left(q[19], 13))
        .wrapping_add(q[20])
        .wrapping_add(circular_left(q[21], 16))
        .wrapping_add(q[22])
        .wrapping_add(circular_left(q[23], 19))
        .wrapping_add(q[24])
        .wrapping_add(circular_left(q[25], 23))
        .wrapping_add(q[26])
        .wrapping_add(circular_left(q[27], 27))
        .wrapping_add((q[28] >> 1) ^ q[28])
        .wrapping_add((q[29] >> 2) ^ q[29])
        .wrapping_add(
            (circular_left(m[14], 15)
                .wrapping_add(circular_left(m[1], 2))
                .wrapping_sub(circular_left(m[8], 9))
                .wrapping_add(30 * 0x0555_5555))
                ^ h[5],
        );
    q[31] = q[15]
        .wrapping_add(circular_left(q[16], 3))
        .wrapping_add(q[17])
        .wrapping_add(circular_left(q[18], 7))
        .wrapping_add(q[19])
        .wrapping_add(circular_left(q[20], 13))
        .wrapping_add(q[21])
        .wrapping_add(circular_left(q[22], 16))
        .wrapping_add(q[23])
        .wrapping_add(circular_left(q[24], 19))
        .wrapping_add(q[25])
        .wrapping_add(circular_left(q[26], 23))
        .wrapping_add(q[27])
        .wrapping_add(circular_left(q[28], 27))
        .wrapping_add((q[29] >> 1) ^ q[29])
        .wrapping_add((q[30] >> 2) ^ q[30])
        .wrapping_add(
            (circular_left(m[15], 16)
                .wrapping_add(circular_left(m[2], 3))
                .wrapping_sub(circular_left(m[9], 10))
                .wrapping_add(31 * 0x0555_5555))
                ^ h[6],
        );
    let xl = q[16] ^ q[17] ^ q[18] ^ q[19] ^ q[20] ^ q[21] ^ q[22] ^ q[23];
    let xh = xl ^ q[24] ^ q[25] ^ q[26] ^ q[27] ^ q[28] ^ q[29] ^ q[30] ^ q[31];
    h[0] = ((xh << 5) ^ (q[16] >> 5) ^ m[0]).wrapping_add(xl ^ q[24] ^ q[0]);
    h[1] = ((xh >> 7) ^ (q[17] << 8) ^ m[1]).wrapping_add(xl ^ q[25] ^ q[1]);
    h[2] = ((xh >> 5) ^ (q[18] << 5) ^ m[2]).wrapping_add(xl ^ q[26] ^ q[2]);
    h[3] = ((xh >> 1) ^ (q[19] << 5) ^ m[3]).wrapping_add(xl ^ q[27] ^ q[3]);
    h[4] = ((xh >> 3) ^ q[20] ^ m[4]).wrapping_add(xl ^ q[28] ^ q[4]);
    h[5] = ((xh << 6) ^ (q[21] >> 6) ^ m[5]).wrapping_add(xl ^ q[29] ^ q[5]);
    h[6] = ((xh >> 4) ^ (q[22] << 6) ^ m[6]).wrapping_add(xl ^ q[30] ^ q[6]);
    h[7] = ((xh >> 11) ^ (q[23] << 2) ^ m[7]).wrapping_add(xl ^ q[31] ^ q[7]);
    h[8] = circular_left(h[4], 9)
        .wrapping_add(xh ^ q[24] ^ m[8])
        .wrapping_add((xl << 8) ^ q[23] ^ q[8]);
    h[9] = circular_left(h[5], 10)
        .wrapping_add(xh ^ q[25] ^ m[9])
        .wrapping_add((xl >> 6) ^ q[16] ^ q[9]);
    h[10] = circular_left(h[6], 11)
        .wrapping_add(xh ^ q[26] ^ m[10])
        .wrapping_add((xl << 6) ^ q[17] ^ q[10]);
    h[11] = circular_left(h[7], 12)
        .wrapping_add(xh ^ q[27] ^ m[11])
        .wrapping_add((xl << 4) ^ q[18] ^ q[11]);
    h[12] = circular_left(h[0], 13)
        .wrapping_add(xh ^ q[28] ^ m[12])
        .wrapping_add((xl >> 3) ^ q[19] ^ q[12]);
    h[13] = circular_left(h[1], 14)
        .wrapping_add(xh ^ q[29] ^ m[13])
        .wrapping_add((xl >> 4) ^ q[20] ^ q[13]);
    h[14] = circular_left(h[2], 15)
        .wrapping_add(xh ^ q[30] ^ m[14])
        .wrapping_add((xl >> 7) ^ q[21] ^ q[14]);
    h[15] = circular_left(h[3], 16)
        .wrapping_add(xh ^ q[31] ^ m[15])
        .wrapping_add((xl >> 2) ^ q[22] ^ q[15]);
    b.h[..16].clone_from_slice(&h[..16]);
    b.q[..32].clone_from_slice(&q[..32]);
    b
}

//sum calculates bmw256.
//length of data must be 32 bytes.
pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let mut b = new();
    let mut buf: Vec<u8> = Vec::with_capacity(64);
    for _i in 0..64 {
        if _i < input.len() {
            buf.push(input[_i]);
        } else {
            buf.push(0);
        }
    }
    buf[input.len()] = 0x80;
    let bit_len = ((input.len() as u64) << 3).to_le_bytes();
    buf[56] = bit_len[0];
    buf[57] = bit_len[1];
    buf[58] = bit_len[2];
    buf[59] = bit_len[3];
    buf[60] = bit_len[4];
    buf[61] = bit_len[5];
    buf[62] = bit_len[6];
    buf[63] = bit_len[7];
    let mut _i = 0;
    for _i in 0..16 {
        let start = 4 * _i;
        let end = start + 4;
        b.m[_i] = read_u32_le(&buf[start..end]);
    }
    let m = b.m;
    let mut b = compress(b, m);
    let h = b.h;
    b.h2[..16].clone_from_slice(&h[..16]);
    b.h[..16].clone_from_slice(&FINAL[..16]);
    let h2 = b.h2;
    let b = compress(b, h2);
    let mut out = vec![];
    out.extend_from_slice(&b.h[8].to_le_bytes());
    out.extend_from_slice(&b.h[9].to_le_bytes());
    out.extend_from_slice(&b.h[10].to_le_bytes());
    out.extend_from_slice(&b.h[11].to_le_bytes());
    out.extend_from_slice(&b.h[12].to_le_bytes());
    out.extend_from_slice(&b.h[13].to_le_bytes());
    out.extend_from_slice(&b.h[14].to_le_bytes());
    out.extend_from_slice(&b.h[15].to_le_bytes());
    out
}

#[test]
fn bmw_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let bmw_result1 = sum(base1);
    assert_eq!(
        "57d11fc94bdf98e6a0d0bf1d4ddda3f4205e873666a644b5bb585e171ad87d34",
        bmw_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base2 = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let bmw_result2 = sum(base2);
    assert_eq!(
        "034c3defeb35b38e23136ad700885007c0c051cd51cc45170cbbdcd68419214a",
        bmw_result2
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "😀😁😂".as_bytes().to_vec();
    let bmw_result3 = sum(base3);
    assert_eq!(
        "9978ccbde1db31dcc1bfd7a010d9ae2215e5fb91e9ecaf786be6252c621f60e3",
        bmw_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
