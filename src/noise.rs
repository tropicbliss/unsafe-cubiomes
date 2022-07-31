#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Xoroshiro {
    pub lo: uint64_t,
    pub hi: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PerlinNoise {
    pub d: [uint8_t; 512],
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub amplitude: libc::c_double,
    pub lacunarity: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OctaveNoise {
    pub octcnt: libc::c_int,
    pub octaves: *mut PerlinNoise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoublePerlinNoise {
    pub amplitude: libc::c_double,
    pub octA: OctaveNoise,
    pub octB: OctaveNoise,
}
#[inline(always)]
unsafe extern "C" fn rotl64(mut x: uint64_t, mut b: uint8_t) -> uint64_t {
    return x << b as libc::c_int | x >> 64 as libc::c_int - b as libc::c_int;
}
#[inline]
unsafe extern "C" fn next(mut seed: *mut uint64_t, bits: libc::c_int) -> libc::c_int {
    *seed = ((*seed)
        .wrapping_mul(0x5deece66d as libc::c_long as libc::c_ulong)
        .wrapping_add(0xb as libc::c_int as libc::c_ulong) as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 48 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)) as uint64_t;
    return (*seed as int64_t >> 48 as libc::c_int - bits) as libc::c_int;
}
#[inline]
unsafe extern "C" fn nextInt(mut seed: *mut uint64_t, n: libc::c_int) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let m: libc::c_int = n - 1 as libc::c_int;
    if m & n == 0 as libc::c_int {
        let mut x: uint64_t =
            (n as libc::c_ulong).wrapping_mul(next(seed, 31 as libc::c_int) as uint64_t);
        return (x as int64_t >> 31 as libc::c_int) as libc::c_int;
    }
    loop {
        bits = next(seed, 31 as libc::c_int);
        val = bits % n;
        if !(bits - val + m < 0 as libc::c_int) {
            break;
        }
    }
    return val;
}
#[inline]
unsafe extern "C" fn nextDouble(mut seed: *mut uint64_t) -> libc::c_double {
    let mut x: uint64_t = next(seed, 26 as libc::c_int) as uint64_t;
    x <<= 27 as libc::c_int;
    x = (x as libc::c_ulong).wrapping_add(next(seed, 27 as libc::c_int) as libc::c_ulong)
        as uint64_t as uint64_t;
    return x as int64_t as libc::c_double
        / ((1 as libc::c_ulonglong) << 53 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn skipNextN(mut seed: *mut uint64_t, mut n: uint64_t) {
    let mut m: uint64_t = 1 as libc::c_int as uint64_t;
    let mut a: uint64_t = 0 as libc::c_int as uint64_t;
    let mut im: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let mut ia: uint64_t = 0xb as libc::c_int as uint64_t;
    let mut k: uint64_t = 0;
    k = n;
    while k != 0 {
        if k & 1 as libc::c_int as libc::c_ulong != 0 {
            m = (m as libc::c_ulong).wrapping_mul(im) as uint64_t as uint64_t;
            a = im.wrapping_mul(a).wrapping_add(ia);
        }
        ia = im
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(ia);
        im = (im as libc::c_ulong).wrapping_mul(im) as uint64_t as uint64_t;
        k >>= 1 as libc::c_int;
    }
    *seed = (*seed).wrapping_mul(m).wrapping_add(a);
    *seed = (*seed as libc::c_ulonglong & 0xffffffffffff as libc::c_ulonglong) as uint64_t;
}
#[inline]
unsafe extern "C" fn xNextLong(mut xr: *mut Xoroshiro) -> uint64_t {
    let mut l: uint64_t = (*xr).lo;
    let mut h: uint64_t = (*xr).hi;
    let mut n: uint64_t = (rotl64(l.wrapping_add(h), 17 as libc::c_int as uint8_t)).wrapping_add(l);
    h ^= l;
    (*xr).lo = rotl64(l, 49 as libc::c_int as uint8_t) ^ h ^ h << 21 as libc::c_int;
    (*xr).hi = rotl64(h, 28 as libc::c_int as uint8_t);
    return n;
}
#[inline]
unsafe extern "C" fn xNextInt(mut xr: *mut Xoroshiro, mut n: uint32_t) -> libc::c_int {
    let mut r: uint64_t = (xNextLong(xr) & 0xffffffff as libc::c_uint as libc::c_ulong)
        .wrapping_mul(n as libc::c_ulong);
    if (r as uint32_t) < n {
        while (r as uint32_t)
            < (!n)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(n)
        {
            r = (xNextLong(xr) & 0xffffffff as libc::c_uint as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong);
        }
    }
    return (r >> 32 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn xNextDouble(mut xr: *mut Xoroshiro) -> libc::c_double {
    return (xNextLong(xr) >> 64 as libc::c_int - 53 as libc::c_int) as libc::c_double
        * 1.1102230246251565E-16f64;
}
#[inline]
unsafe extern "C" fn lerp(
    mut part: libc::c_double,
    mut from: libc::c_double,
    mut to: libc::c_double,
) -> libc::c_double {
    return from + part * (to - from);
}
#[no_mangle]
pub unsafe extern "C" fn maintainPrecision(mut x: libc::c_double) -> libc::c_double {
    return x - floor(x / 33554432.0f64 + 0.5f64) * 33554432.0f64;
}
#[inline(always)]
unsafe extern "C" fn indexedLerp(
    mut idx: libc::c_int,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: libc::c_double,
) -> libc::c_double {
    match idx & 0xf as libc::c_int {
        0 => return a + b,
        1 => return -a + b,
        2 => return a - b,
        3 => return -a - b,
        4 => return a + c,
        5 => return -a + c,
        6 => return a - c,
        7 => return -a - c,
        8 => return b + c,
        9 => return -b + c,
        10 => return b - c,
        11 => return -b - c,
        12 => return a + b,
        13 => return -b + c,
        14 => return -a + b,
        15 => return -b - c,
        _ => {}
    }
    unreachable!();
}
#[no_mangle]
pub unsafe extern "C" fn perlinInit(mut noise: *mut PerlinNoise, mut seed: *mut uint64_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    (*noise).a = nextDouble(seed) * 256.0f64;
    (*noise).b = nextDouble(seed) * 256.0f64;
    (*noise).c = nextDouble(seed) * 256.0f64;
    (*noise).amplitude = 1.0f64;
    (*noise).lacunarity = 1.0f64;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*noise).d[i as usize] = i as uint8_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut j: libc::c_int = nextInt(seed, 256 as libc::c_int - i) + i;
        let mut n: uint8_t = (*noise).d[i as usize];
        (*noise).d[i as usize] = (*noise).d[j as usize];
        (*noise).d[j as usize] = n;
        (*noise).d[(i + 256 as libc::c_int) as usize] = (*noise).d[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xPerlinInit(mut noise: *mut PerlinNoise, mut xr: *mut Xoroshiro) {
    let mut i: libc::c_int = 0 as libc::c_int;
    (*noise).a = xNextDouble(xr) * 256.0f64;
    (*noise).b = xNextDouble(xr) * 256.0f64;
    (*noise).c = xNextDouble(xr) * 256.0f64;
    (*noise).amplitude = 1.0f64;
    (*noise).lacunarity = 1.0f64;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*noise).d[i as usize] = i as uint8_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut j: libc::c_int = xNextInt(xr, (256 as libc::c_int - i) as uint32_t) + i;
        let mut n: uint8_t = (*noise).d[i as usize];
        (*noise).d[i as usize] = (*noise).d[j as usize];
        (*noise).d[j as usize] = n;
        (*noise).d[(i + 256 as libc::c_int) as usize] = (*noise).d[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn samplePerlin(
    mut noise: *const PerlinNoise,
    mut d1: libc::c_double,
    mut d2: libc::c_double,
    mut d3: libc::c_double,
    mut yamp: libc::c_double,
    mut ymin: libc::c_double,
) -> libc::c_double {
    d1 += (*noise).a;
    d2 += (*noise).b;
    d3 += (*noise).c;
    let mut i1: libc::c_int =
        d1 as libc::c_int - (d1 < 0 as libc::c_int as libc::c_double) as libc::c_int;
    let mut i2: libc::c_int =
        d2 as libc::c_int - (d2 < 0 as libc::c_int as libc::c_double) as libc::c_int;
    let mut i3: libc::c_int =
        d3 as libc::c_int - (d3 < 0 as libc::c_int as libc::c_double) as libc::c_int;
    d1 -= i1 as libc::c_double;
    d2 -= i2 as libc::c_double;
    d3 -= i3 as libc::c_double;
    let mut t1: libc::c_double = d1 * d1 * d1 * (d1 * (d1 * 6.0f64 - 15.0f64) + 10.0f64);
    let mut t2: libc::c_double = d2 * d2 * d2 * (d2 * (d2 * 6.0f64 - 15.0f64) + 10.0f64);
    let mut t3: libc::c_double = d3 * d3 * d3 * (d3 * (d3 * 6.0f64 - 15.0f64) + 10.0f64);
    if yamp != 0. {
        let mut yclamp: libc::c_double = if ymin < d2 { ymin } else { d2 };
        d2 -= floor(yclamp / yamp) * yamp;
    }
    i1 &= 0xff as libc::c_int;
    i2 &= 0xff as libc::c_int;
    i3 &= 0xff as libc::c_int;
    let mut a1: libc::c_int = (*noise).d[i1 as usize] as libc::c_int + i2;
    let mut a2: libc::c_int = (*noise).d[a1 as usize] as libc::c_int + i3;
    let mut a3: libc::c_int = (*noise).d[(a1 + 1 as libc::c_int) as usize] as libc::c_int + i3;
    let mut b1: libc::c_int = (*noise).d[(i1 + 1 as libc::c_int) as usize] as libc::c_int + i2;
    let mut b2: libc::c_int = (*noise).d[b1 as usize] as libc::c_int + i3;
    let mut b3: libc::c_int = (*noise).d[(b1 + 1 as libc::c_int) as usize] as libc::c_int + i3;
    let mut l1: libc::c_double = indexedLerp((*noise).d[a2 as usize] as libc::c_int, d1, d2, d3);
    let mut l2: libc::c_double = indexedLerp(
        (*noise).d[b2 as usize] as libc::c_int,
        d1 - 1 as libc::c_int as libc::c_double,
        d2,
        d3,
    );
    let mut l3: libc::c_double = indexedLerp(
        (*noise).d[a3 as usize] as libc::c_int,
        d1,
        d2 - 1 as libc::c_int as libc::c_double,
        d3,
    );
    let mut l4: libc::c_double = indexedLerp(
        (*noise).d[b3 as usize] as libc::c_int,
        d1 - 1 as libc::c_int as libc::c_double,
        d2 - 1 as libc::c_int as libc::c_double,
        d3,
    );
    let mut l5: libc::c_double = indexedLerp(
        (*noise).d[(a2 + 1 as libc::c_int) as usize] as libc::c_int,
        d1,
        d2,
        d3 - 1 as libc::c_int as libc::c_double,
    );
    let mut l6: libc::c_double = indexedLerp(
        (*noise).d[(b2 + 1 as libc::c_int) as usize] as libc::c_int,
        d1 - 1 as libc::c_int as libc::c_double,
        d2,
        d3 - 1 as libc::c_int as libc::c_double,
    );
    let mut l7: libc::c_double = indexedLerp(
        (*noise).d[(a3 + 1 as libc::c_int) as usize] as libc::c_int,
        d1,
        d2 - 1 as libc::c_int as libc::c_double,
        d3 - 1 as libc::c_int as libc::c_double,
    );
    let mut l8: libc::c_double = indexedLerp(
        (*noise).d[(b3 + 1 as libc::c_int) as usize] as libc::c_int,
        d1 - 1 as libc::c_int as libc::c_double,
        d2 - 1 as libc::c_int as libc::c_double,
        d3 - 1 as libc::c_int as libc::c_double,
    );
    l1 = lerp(t1, l1, l2);
    l3 = lerp(t1, l3, l4);
    l5 = lerp(t1, l5, l6);
    l7 = lerp(t1, l7, l8);
    l1 = lerp(t2, l1, l3);
    l5 = lerp(t2, l5, l7);
    return lerp(t3, l1, l5);
}
unsafe extern "C" fn simplexGrad(
    mut idx: libc::c_int,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut d: libc::c_double,
) -> libc::c_double {
    let mut con: libc::c_double = d - x * x - y * y - z * z;
    if con < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    con *= con;
    return con * con * indexedLerp(idx, x, y, z);
}
#[no_mangle]
pub unsafe extern "C" fn sampleSimplex2D(
    mut noise: *const PerlinNoise,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let SKEW: libc::c_double = 0.5f64 * (sqrt(3 as libc::c_int as libc::c_double) - 1.0f64);
    let UNSKEW: libc::c_double = (3.0f64 - sqrt(3 as libc::c_int as libc::c_double)) / 6.0f64;
    let mut hf: libc::c_double = (x + y) * SKEW;
    let mut hx: libc::c_int = floor(x + hf) as libc::c_int;
    let mut hz: libc::c_int = floor(y + hf) as libc::c_int;
    let mut mhxz: libc::c_double = (hx + hz) as libc::c_double * UNSKEW;
    let mut x0: libc::c_double = x - (hx as libc::c_double - mhxz);
    let mut y0: libc::c_double = y - (hz as libc::c_double - mhxz);
    let mut offx: libc::c_int = (x0 > y0) as libc::c_int;
    let mut offz: libc::c_int = (offx == 0) as libc::c_int;
    let mut x1: libc::c_double = x0 - offx as libc::c_double + UNSKEW;
    let mut y1: libc::c_double = y0 - offz as libc::c_double + UNSKEW;
    let mut x2: libc::c_double = x0 - 1.0f64 + 2.0f64 * UNSKEW;
    let mut y2: libc::c_double = y0 - 1.0f64 + 2.0f64 * UNSKEW;
    let mut gi0: libc::c_int = (*noise).d[(0xff as libc::c_int & hz) as usize] as libc::c_int;
    let mut gi1: libc::c_int =
        (*noise).d[(0xff as libc::c_int & hz + offz) as usize] as libc::c_int;
    let mut gi2: libc::c_int =
        (*noise).d[(0xff as libc::c_int & hz + 1 as libc::c_int) as usize] as libc::c_int;
    gi0 = (*noise).d[(0xff as libc::c_int & gi0 + hx) as usize] as libc::c_int;
    gi1 = (*noise).d[(0xff as libc::c_int & gi1 + hx + offx) as usize] as libc::c_int;
    gi2 = (*noise).d[(0xff as libc::c_int & gi2 + hx + 1 as libc::c_int) as usize] as libc::c_int;
    let mut t: libc::c_double = 0 as libc::c_int as libc::c_double;
    t += simplexGrad(gi0 % 12 as libc::c_int, x0, y0, 0.0f64, 0.5f64);
    t += simplexGrad(gi1 % 12 as libc::c_int, x1, y1, 0.0f64, 0.5f64);
    t += simplexGrad(gi2 % 12 as libc::c_int, x2, y2, 0.0f64, 0.5f64);
    return 70.0f64 * t;
}
#[no_mangle]
pub unsafe extern "C" fn octaveInit(
    mut noise: *mut OctaveNoise,
    mut seed: *mut uint64_t,
    mut octaves: *mut PerlinNoise,
    mut omin: libc::c_int,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = omin + len - 1 as libc::c_int;
    let mut persist: libc::c_double =
        1.0f64 / (((1 as libc::c_longlong) << len) as libc::c_double - 1.0f64);
    let mut lacuna: libc::c_double = pow(2.0f64, end as libc::c_double);
    if len < 1 as libc::c_int || end > 0 as libc::c_int {
        printf(
            b"octavePerlinInit(): unsupported octave range\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if end == 0 as libc::c_int {
        perlinInit(&mut *octaves.offset(0 as libc::c_int as isize), seed);
        (*octaves.offset(0 as libc::c_int as isize)).amplitude = persist;
        (*octaves.offset(0 as libc::c_int as isize)).lacunarity = lacuna;
        persist *= 2.0f64;
        lacuna *= 0.5f64;
        i = 1 as libc::c_int;
    } else {
        skipNextN(seed, (-end * 262 as libc::c_int) as uint64_t);
        i = 0 as libc::c_int;
    }
    while i < len {
        perlinInit(&mut *octaves.offset(i as isize), seed);
        (*octaves.offset(i as isize)).amplitude = persist;
        (*octaves.offset(i as isize)).lacunarity = lacuna;
        persist *= 2.0f64;
        lacuna *= 0.5f64;
        i += 1;
    }
    let ref mut fresh0 = (*noise).octaves;
    *fresh0 = octaves;
    (*noise).octcnt = len;
}
#[no_mangle]
pub unsafe extern "C" fn xOctaveInit(
    mut noise: *mut OctaveNoise,
    mut xr: *mut Xoroshiro,
    mut octaves: *mut PerlinNoise,
    mut amplitudes: *const libc::c_double,
    mut omin: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let md5_octave_n: [[uint64_t; 2]; 13] = [
        [
            0xb198de63a8012672 as libc::c_ulong,
            0x7b84cad43ef7b5a8 as libc::c_long as uint64_t,
        ],
        [
            0xfd787bfbc403ec3 as libc::c_long as uint64_t,
            0x74a4a31ca21b48b8 as libc::c_long as uint64_t,
        ],
        [
            0x36d326eed40efeb2 as libc::c_long as uint64_t,
            0x5be9ce18223c636a as libc::c_long as uint64_t,
        ],
        [
            0x82fe255f8be6631 as libc::c_long as uint64_t,
            0x4e96119e22dedc81 as libc::c_long as uint64_t,
        ],
        [
            0xef68ec68504005e as libc::c_long as uint64_t,
            0x48b6bf93a2789640 as libc::c_long as uint64_t,
        ],
        [
            0xf11268128982754f as libc::c_ulong,
            0x257a1d670430b0aa as libc::c_long as uint64_t,
        ],
        [
            0xe51c98ce7d1de664 as libc::c_ulong,
            0x5f9478a733040c45 as libc::c_long as uint64_t,
        ],
        [
            0x6d7b49e7e429850a as libc::c_long as uint64_t,
            0x2e3063c622a24777 as libc::c_long as uint64_t,
        ],
        [
            0xbd90d5377ba1b762 as libc::c_ulong,
            0xc07317d419a7548d as libc::c_ulong,
        ],
        [
            0x53d39c6752dac858 as libc::c_long as uint64_t,
            0xbcd1c5a80ab65b3e as libc::c_ulong,
        ],
        [
            0xb4a24d7a84e7677b as libc::c_ulong,
            0x23ff9668e89b5c4 as libc::c_long as uint64_t,
        ],
        [
            0xdffa22b534c5f608 as libc::c_ulong,
            0xb9b67517d3665ca9 as libc::c_ulong,
        ],
        [
            0xd50708086cef4d7c as libc::c_ulong,
            0x6e1651ecc7f43309 as libc::c_long as uint64_t,
        ],
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut lacuna: libc::c_double = pow(2.0f64, omin as libc::c_double);
    let mut persist: libc::c_double = pow(2.0f64, (len - 1 as libc::c_int) as libc::c_double)
        / (((1 as libc::c_longlong) << len) as libc::c_double - 1.0f64);
    let mut xlo: uint64_t = xNextLong(xr);
    let mut xhi: uint64_t = xNextLong(xr);
    while i < len {
        if !(*amplitudes.offset(i as isize) == 0 as libc::c_int as libc::c_double) {
            let mut pxr: Xoroshiro = Xoroshiro { lo: 0, hi: 0 };
            pxr.lo = xlo
                ^ md5_octave_n[(12 as libc::c_int + omin + i) as usize][0 as libc::c_int as usize];
            pxr.hi = xhi
                ^ md5_octave_n[(12 as libc::c_int + omin + i) as usize][1 as libc::c_int as usize];
            xPerlinInit(&mut *octaves.offset(n as isize), &mut pxr);
            (*octaves.offset(n as isize)).amplitude = *amplitudes.offset(i as isize) * persist;
            (*octaves.offset(n as isize)).lacunarity = lacuna;
            n += 1;
        }
        i += 1;
        lacuna *= 2.0f64;
        persist *= 0.5f64;
    }
    let ref mut fresh1 = (*noise).octaves;
    *fresh1 = octaves;
    (*noise).octcnt = n;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn sampleOctave(
    mut noise: *const OctaveNoise,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
) -> libc::c_double {
    let mut v: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*noise).octcnt {
        let mut p: *mut PerlinNoise = ((*noise).octaves).offset(i as isize);
        let mut lf: libc::c_double = (*p).lacunarity;
        let mut ax: libc::c_double = maintainPrecision(x * lf);
        let mut ay: libc::c_double = maintainPrecision(y * lf);
        let mut az: libc::c_double = maintainPrecision(z * lf);
        let mut pv: libc::c_double = samplePerlin(
            p,
            ax,
            ay,
            az,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        );
        v += (*p).amplitude * pv;
        i += 1;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn doublePerlinInit(
    mut noise: *mut DoublePerlinNoise,
    mut seed: *mut uint64_t,
    mut octavesA: *mut PerlinNoise,
    mut octavesB: *mut PerlinNoise,
    mut omin: libc::c_int,
    mut len: libc::c_int,
) {
    (*noise).amplitude =
        10.0f64 / 6.0f64 * len as libc::c_double / (len + 1 as libc::c_int) as libc::c_double;
    octaveInit(&mut (*noise).octA, seed, octavesA, omin, len);
    octaveInit(&mut (*noise).octB, seed, octavesB, omin, len);
}
#[no_mangle]
pub unsafe extern "C" fn xDoublePerlinInit(
    mut noise: *mut DoublePerlinNoise,
    mut xr: *mut Xoroshiro,
    mut octaves: *mut PerlinNoise,
    mut amplitudes: *const libc::c_double,
    mut omin: libc::c_int,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    n += xOctaveInit(
        &mut (*noise).octA,
        xr,
        octaves.offset(n as isize),
        amplitudes,
        omin,
        len,
    );
    n += xOctaveInit(
        &mut (*noise).octB,
        xr,
        octaves.offset(n as isize),
        amplitudes,
        omin,
        len,
    );
    i = len - 1 as libc::c_int;
    while i >= 0 as libc::c_int && *amplitudes.offset(i as isize) == 0.0f64 {
        len -= 1;
        i -= 1;
    }
    i = 0 as libc::c_int;
    while *amplitudes.offset(i as isize) == 0.0f64 {
        len -= 1;
        i += 1;
    }
    (*noise).amplitude =
        10.0f64 / 6.0f64 * len as libc::c_double / (len + 1 as libc::c_int) as libc::c_double;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn sampleDoublePerlin(
    mut noise: *const DoublePerlinNoise,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
) -> libc::c_double {
    let f: libc::c_double = 337.0f64 / 331.0f64;
    let mut v: libc::c_double = 0 as libc::c_int as libc::c_double;
    v += sampleOctave(&(*noise).octA, x, y, z);
    v += sampleOctave(&(*noise).octB, x * f, y * f, z * f);
    return v * (*noise).amplitude;
}
