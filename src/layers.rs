#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
    fn maintainPrecision(x: libc::c_double) -> libc::c_double;
    fn perlinInit(noise: *mut PerlinNoise, seed: *mut uint64_t);
    fn samplePerlin(
        noise: *const PerlinNoise,
        x: libc::c_double,
        y: libc::c_double,
        z: libc::c_double,
        yamp: libc::c_double,
        ymin: libc::c_double,
    ) -> libc::c_double;
    fn sampleSimplex2D(
        noise: *const PerlinNoise,
        x: libc::c_double,
        y: libc::c_double,
    ) -> libc::c_double;
    fn octaveInit(
        noise: *mut OctaveNoise,
        seed: *mut uint64_t,
        octaves: *mut PerlinNoise,
        omin: libc::c_int,
        len: libc::c_int,
    );
    fn doublePerlinInit(
        noise: *mut DoublePerlinNoise,
        seed: *mut uint64_t,
        octavesA: *mut PerlinNoise,
        octavesB: *mut PerlinNoise,
        omin: libc::c_int,
        len: libc::c_int,
    );
    fn xDoublePerlinInit(
        noise: *mut DoublePerlinNoise,
        xr: *mut Xoroshiro,
        octaves: *mut PerlinNoise,
        amplitudes: *const libc::c_double,
        omin: libc::c_int,
        len: libc::c_int,
    ) -> libc::c_int;
    fn sampleDoublePerlin(
        noise: *const DoublePerlinNoise,
        x: libc::c_double,
        y: libc::c_double,
        z: libc::c_double,
    ) -> libc::c_double;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn p2overworld(
        mc: libc::c_int,
        np: *const uint64_t,
        dat: *mut uint64_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type MCVersion = libc::c_uint;
pub const MC_NEWEST: MCVersion = 19;
pub const MC_1_19: MCVersion = 19;
pub const MC_1_18: MCVersion = 18;
pub const MC_1_17: MCVersion = 17;
pub const MC_1_16: MCVersion = 16;
pub const MC_1_15: MCVersion = 15;
pub const MC_1_14: MCVersion = 14;
pub const MC_1_13: MCVersion = 13;
pub const MC_1_12: MCVersion = 12;
pub const MC_1_11: MCVersion = 11;
pub const MC_1_10: MCVersion = 10;
pub const MC_1_9: MCVersion = 9;
pub const MC_1_8: MCVersion = 8;
pub const MC_1_7: MCVersion = 7;
pub const MC_1_6: MCVersion = 6;
pub const MC_1_5: MCVersion = 5;
pub const MC_1_4: MCVersion = 4;
pub const MC_1_3: MCVersion = 3;
pub const MC_1_2: MCVersion = 2;
pub const MC_1_1: MCVersion = 1;
pub const MC_1_0: MCVersion = 0;
pub type BiomeID = libc::c_int;
pub const mangrove_swamp: BiomeID = 184;
pub const deep_dark: BiomeID = 183;
pub const wooded_badlands: BiomeID = 38;
pub const windswept_savanna: BiomeID = 163;
pub const windswept_gravelly_hills: BiomeID = 131;
pub const windswept_forest: BiomeID = 34;
pub const windswept_hills: BiomeID = 3;
pub const stony_shore: BiomeID = 25;
pub const sparse_jungle: BiomeID = 23;
pub const snowy_plains: BiomeID = 12;
pub const old_growth_spruce_taiga: BiomeID = 160;
pub const old_growth_pine_taiga: BiomeID = 32;
pub const old_growth_birch_forest: BiomeID = 155;
pub const stony_peaks: BiomeID = 182;
pub const frozen_peaks: BiomeID = 181;
pub const jagged_peaks: BiomeID = 180;
pub const snowy_slopes: BiomeID = 179;
pub const grove: BiomeID = 178;
pub const meadow: BiomeID = 177;
pub const lush_caves: BiomeID = 175;
pub const dripstone_caves: BiomeID = 174;
pub const basalt_deltas: BiomeID = 173;
pub const warped_forest: BiomeID = 172;
pub const crimson_forest: BiomeID = 171;
pub const soul_sand_valley: BiomeID = 170;
pub const bamboo_jungle_hills: BiomeID = 169;
pub const bamboo_jungle: BiomeID = 168;
pub const modified_badlands_plateau: BiomeID = 167;
pub const modified_wooded_badlands_plateau: BiomeID = 166;
pub const eroded_badlands: BiomeID = 165;
pub const shattered_savanna_plateau: BiomeID = 164;
pub const shattered_savanna: BiomeID = 163;
pub const modified_gravelly_mountains: BiomeID = 162;
pub const giant_spruce_taiga_hills: BiomeID = 161;
pub const giant_spruce_taiga: BiomeID = 160;
pub const snowy_taiga_mountains: BiomeID = 158;
pub const dark_forest_hills: BiomeID = 157;
pub const tall_birch_hills: BiomeID = 156;
pub const tall_birch_forest: BiomeID = 155;
pub const modified_jungle_edge: BiomeID = 151;
pub const modified_jungle: BiomeID = 149;
pub const ice_spikes: BiomeID = 140;
pub const swamp_hills: BiomeID = 134;
pub const taiga_mountains: BiomeID = 133;
pub const flower_forest: BiomeID = 132;
pub const gravelly_mountains: BiomeID = 131;
pub const desert_lakes: BiomeID = 130;
pub const sunflower_plains: BiomeID = 129;
pub const the_void: BiomeID = 127;
pub const BIOME_NUM: BiomeID = 51;
pub const frozenDeepOcean: BiomeID = 50;
pub const deep_frozen_ocean: BiomeID = 50;
pub const coldDeepOcean: BiomeID = 49;
pub const deep_cold_ocean: BiomeID = 49;
pub const lukewarmDeepOcean: BiomeID = 48;
pub const deep_lukewarm_ocean: BiomeID = 48;
pub const warmDeepOcean: BiomeID = 47;
pub const deep_warm_ocean: BiomeID = 47;
pub const coldOcean: BiomeID = 46;
pub const cold_ocean: BiomeID = 46;
pub const lukewarmOcean: BiomeID = 45;
pub const lukewarm_ocean: BiomeID = 45;
pub const warmOcean: BiomeID = 44;
pub const warm_ocean: BiomeID = 44;
pub const end_barrens: BiomeID = 43;
pub const end_highlands: BiomeID = 42;
pub const end_midlands: BiomeID = 41;
pub const small_end_islands: BiomeID = 40;
pub const mesaPlateau: BiomeID = 39;
pub const badlands_plateau: BiomeID = 39;
pub const mesaPlateau_F: BiomeID = 38;
pub const wooded_badlands_plateau: BiomeID = 38;
pub const mesa: BiomeID = 37;
pub const badlands: BiomeID = 37;
pub const savannaPlateau: BiomeID = 36;
pub const savanna_plateau: BiomeID = 36;
pub const savanna: BiomeID = 35;
pub const extremeHillsPlus: BiomeID = 34;
pub const wooded_mountains: BiomeID = 34;
pub const megaTaigaHills: BiomeID = 33;
pub const giant_tree_taiga_hills: BiomeID = 33;
pub const megaTaiga: BiomeID = 32;
pub const giant_tree_taiga: BiomeID = 32;
pub const coldTaigaHills: BiomeID = 31;
pub const snowy_taiga_hills: BiomeID = 31;
pub const coldTaiga: BiomeID = 30;
pub const snowy_taiga: BiomeID = 30;
pub const roofedForest: BiomeID = 29;
pub const dark_forest: BiomeID = 29;
pub const birchForestHills: BiomeID = 28;
pub const birch_forest_hills: BiomeID = 28;
pub const birchForest: BiomeID = 27;
pub const birch_forest: BiomeID = 27;
pub const coldBeach: BiomeID = 26;
pub const snowy_beach: BiomeID = 26;
pub const stoneBeach: BiomeID = 25;
pub const stone_shore: BiomeID = 25;
pub const deepOcean: BiomeID = 24;
pub const deep_ocean: BiomeID = 24;
pub const jungleEdge: BiomeID = 23;
pub const jungle_edge: BiomeID = 23;
pub const jungleHills: BiomeID = 22;
pub const jungle_hills: BiomeID = 22;
pub const jungle: BiomeID = 21;
pub const extremeHillsEdge: BiomeID = 20;
pub const mountain_edge: BiomeID = 20;
pub const taigaHills: BiomeID = 19;
pub const taiga_hills: BiomeID = 19;
pub const forestHills: BiomeID = 18;
pub const wooded_hills: BiomeID = 18;
pub const desertHills: BiomeID = 17;
pub const desert_hills: BiomeID = 17;
pub const beach: BiomeID = 16;
pub const mushroomIslandShore: BiomeID = 15;
pub const mushroom_field_shore: BiomeID = 15;
pub const mushroomIsland: BiomeID = 14;
pub const mushroom_fields: BiomeID = 14;
pub const iceMountains: BiomeID = 13;
pub const snowy_mountains: BiomeID = 13;
pub const icePlains: BiomeID = 12;
pub const snowy_tundra: BiomeID = 12;
pub const frozenRiver: BiomeID = 11;
pub const frozen_river: BiomeID = 11;
pub const frozenOcean: BiomeID = 10;
pub const frozen_ocean: BiomeID = 10;
pub const sky: BiomeID = 9;
pub const the_end: BiomeID = 9;
pub const hell: BiomeID = 8;
pub const nether_wastes: BiomeID = 8;
pub const river: BiomeID = 7;
pub const swampland: BiomeID = 6;
pub const swamp: BiomeID = 6;
pub const taiga: BiomeID = 5;
pub const forest: BiomeID = 4;
pub const extremeHills: BiomeID = 3;
pub const mountains: BiomeID = 3;
pub const desert: BiomeID = 2;
pub const plains: BiomeID = 1;
pub const ocean: BiomeID = 0;
pub const none: BiomeID = -1;
pub type BiomeTempCategory = libc::c_uint;
pub const Special: BiomeTempCategory = 5;
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub scale: libc::c_int,
    pub x: libc::c_int,
    pub z: libc::c_int,
    pub sx: libc::c_int,
    pub sz: libc::c_int,
    pub y: libc::c_int,
    pub sy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Layer {
    pub getMap: Option::<mapfunc_t>,
    pub mc: int8_t,
    pub zoom: int8_t,
    pub edge: int8_t,
    pub scale: libc::c_int,
    pub layerSalt: uint64_t,
    pub startSalt: uint64_t,
    pub startSeed: uint64_t,
    pub noise: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub p: *mut Layer,
    pub p2: *mut Layer,
}
pub type mapfunc_t = unsafe extern "C" fn(
    *const Layer,
    *mut libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
    libc::c_int,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetherNoise {
    pub temperature: DoublePerlinNoise,
    pub humidity: DoublePerlinNoise,
    pub oct: [PerlinNoise; 8],
}
pub type EndNoise = PerlinNoise;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SurfaceNoise {
    pub xzScale: libc::c_double,
    pub yScale: libc::c_double,
    pub xzFactor: libc::c_double,
    pub yFactor: libc::c_double,
    pub octmin: OctaveNoise,
    pub octmax: OctaveNoise,
    pub octmain: OctaveNoise,
    pub oct: [PerlinNoise; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Spline {
    pub len: libc::c_int,
    pub typ: libc::c_int,
    pub loc: [libc::c_float; 12],
    pub der: [libc::c_float; 12],
    pub val: [*mut Spline; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FixSpline {
    pub len: libc::c_int,
    pub val: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SplineStack {
    pub stack: [Spline; 42],
    pub fstack: [FixSpline; 151],
    pub len: libc::c_int,
    pub flen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BiomeNoise {
    pub shift: DoublePerlinNoise,
    pub temperature: DoublePerlinNoise,
    pub humidity: DoublePerlinNoise,
    pub continentalness: DoublePerlinNoise,
    pub erosion: DoublePerlinNoise,
    pub weirdness: DoublePerlinNoise,
    pub oct: [PerlinNoise; 46],
    pub sp: *mut Spline,
    pub ss: SplineStack,
    pub previdx: libc::c_int,
    pub mc: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const SAMPLE_NO_BIOME: C2RustUnnamed = 4;
pub const SAMPLE_NO_DEPTH: C2RustUnnamed = 2;
pub const SAMPLE_NO_SHIFT: C2RustUnnamed = 1;
pub const RIDGES: C2RustUnnamed_0 = 2;
pub const EROSION: C2RustUnnamed_0 = 1;
pub const CONTINENTALNESS: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const WEIRDNESS: C2RustUnnamed_0 = 3;
#[inline(always)]
unsafe extern "C" fn rotl64(mut x: uint64_t, mut b: uint8_t) -> uint64_t {
    return x << b as libc::c_int | x >> 64 as libc::c_int - b as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn rotr32(mut a: uint32_t, mut b: uint8_t) -> uint32_t {
    return a >> b as libc::c_int | a << 32 as libc::c_int - b as libc::c_int;
}
#[inline]
unsafe extern "C" fn setSeed(mut seed: *mut uint64_t, mut value: uint64_t) {
    *seed = ((value ^ 0x5deece66d as libc::c_long as libc::c_ulong) as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 48 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)) as uint64_t;
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
        ia = im.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_mul(ia);
        im = (im as libc::c_ulong).wrapping_mul(im) as uint64_t as uint64_t;
        k >>= 1 as libc::c_int;
    }
    *seed = (*seed).wrapping_mul(m).wrapping_add(a);
    *seed = (*seed as libc::c_ulonglong & 0xffffffffffff as libc::c_ulonglong)
        as uint64_t;
}
#[inline]
unsafe extern "C" fn xSetSeed(mut xr: *mut Xoroshiro, mut value: uint64_t) {
    let XL: uint64_t = 0x9e3779b97f4a7c15 as libc::c_ulonglong as uint64_t;
    let XH: uint64_t = 0x6a09e667f3bcc909 as libc::c_ulonglong as uint64_t;
    let A: uint64_t = 0xbf58476d1ce4e5b9 as libc::c_ulonglong as uint64_t;
    let B: uint64_t = 0x94d049bb133111eb as libc::c_ulonglong as uint64_t;
    let mut l: uint64_t = value ^ XH;
    let mut h: uint64_t = l.wrapping_add(XL);
    l = (l ^ l >> 30 as libc::c_int).wrapping_mul(A);
    h = (h ^ h >> 30 as libc::c_int).wrapping_mul(A);
    l = (l ^ l >> 27 as libc::c_int).wrapping_mul(B);
    h = (h ^ h >> 27 as libc::c_int).wrapping_mul(B);
    l = l ^ l >> 31 as libc::c_int;
    h = h ^ h >> 31 as libc::c_int;
    (*xr).lo = l;
    (*xr).hi = h;
}
#[inline]
unsafe extern "C" fn xNextLong(mut xr: *mut Xoroshiro) -> uint64_t {
    let mut l: uint64_t = (*xr).lo;
    let mut h: uint64_t = (*xr).hi;
    let mut n: uint64_t = (rotl64(l.wrapping_add(h), 17 as libc::c_int as uint8_t))
        .wrapping_add(l);
    h ^= l;
    (*xr).lo = rotl64(l, 49 as libc::c_int as uint8_t) ^ h ^ h << 21 as libc::c_int;
    (*xr).hi = rotl64(h, 28 as libc::c_int as uint8_t);
    return n;
}
#[inline]
unsafe extern "C" fn mcStepSeed(mut s: uint64_t, mut salt: uint64_t) -> uint64_t {
    return (s as libc::c_ulonglong)
        .wrapping_mul(
            (s as libc::c_ulonglong)
                .wrapping_mul(6364136223846793005 as libc::c_ulonglong)
                .wrapping_add(1442695040888963407 as libc::c_ulonglong),
        )
        .wrapping_add(salt as libc::c_ulonglong) as uint64_t;
}
#[inline]
unsafe extern "C" fn mcFirstInt(mut s: uint64_t, mut mod_0: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = ((s as int64_t >> 24 as libc::c_int)
        % mod_0 as libc::c_long) as libc::c_int;
    if ret < 0 as libc::c_int {
        ret += mod_0;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn mcFirstIsZero(
    mut s: uint64_t,
    mut mod_0: libc::c_int,
) -> libc::c_int {
    return (((s as int64_t >> 24 as libc::c_int) % mod_0 as libc::c_long) as libc::c_int
        == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn getChunkSeed(
    mut ss: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> uint64_t {
    let mut cs: uint64_t = ss.wrapping_add(x as libc::c_ulong);
    cs = mcStepSeed(cs, z as uint64_t);
    cs = mcStepSeed(cs, x as uint64_t);
    cs = mcStepSeed(cs, z as uint64_t);
    return cs;
}
#[inline]
unsafe extern "C" fn getLayerSalt(mut salt: uint64_t) -> uint64_t {
    let mut ls: uint64_t = mcStepSeed(salt, salt);
    ls = mcStepSeed(ls, salt);
    ls = mcStepSeed(ls, salt);
    return ls;
}
#[inline]
unsafe extern "C" fn lerp(
    mut part: libc::c_double,
    mut from: libc::c_double,
    mut to: libc::c_double,
) -> libc::c_double {
    return from + part * (to - from);
}
#[inline]
unsafe extern "C" fn lerp2(
    mut dx: libc::c_double,
    mut dy: libc::c_double,
    mut v00: libc::c_double,
    mut v10: libc::c_double,
    mut v01: libc::c_double,
    mut v11: libc::c_double,
) -> libc::c_double {
    return lerp(dy, lerp(dx, v00, v10), lerp(dx, v01, v11));
}
#[inline]
unsafe extern "C" fn lerp3(
    mut dx: libc::c_double,
    mut dy: libc::c_double,
    mut dz: libc::c_double,
    mut v000: libc::c_double,
    mut v100: libc::c_double,
    mut v010: libc::c_double,
    mut v110: libc::c_double,
    mut v001: libc::c_double,
    mut v101: libc::c_double,
    mut v011: libc::c_double,
    mut v111: libc::c_double,
) -> libc::c_double {
    v000 = lerp2(dx, dy, v000, v100, v010, v110);
    v001 = lerp2(dx, dy, v001, v101, v011, v111);
    return lerp(dz, v000, v001);
}
#[inline]
unsafe extern "C" fn clampedLerp(
    mut part: libc::c_double,
    mut from: libc::c_double,
    mut to: libc::c_double,
) -> libc::c_double {
    if part <= 0 as libc::c_int as libc::c_double {
        return from;
    }
    if part >= 1 as libc::c_int as libc::c_double {
        return to;
    }
    return lerp(part, from, to);
}
#[no_mangle]
pub unsafe extern "C" fn biomeExists(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> libc::c_int {
    if mc >= MC_1_18 as libc::c_int {
        if id >= soul_sand_valley as libc::c_int && id <= basalt_deltas as libc::c_int {
            return 1 as libc::c_int;
        }
        if id >= small_end_islands as libc::c_int && id <= end_barrens as libc::c_int {
            return 1 as libc::c_int;
        }
        if id == deep_dark as libc::c_int || id == mangrove_swamp as libc::c_int {
            return (mc >= MC_1_19 as libc::c_int) as libc::c_int;
        }
        let mut current_block_7: u64;
        match id {
            12 => {
                current_block_7 = 13302909616349351127;
            }
            23 => {
                current_block_7 = 13302909616349351127;
            }
            25 => {
                current_block_7 = 13302909616349351127;
            }
            32 => {
                current_block_7 = 3404472414225319279;
            }
            34 => {
                current_block_7 = 3404472414225319279;
            }
            38 => {
                current_block_7 = 13302909616349351127;
            }
            131 => {
                current_block_7 = 13302909616349351127;
            }
            155 => {
                current_block_7 = 16629200814096015414;
            }
            160 => {
                current_block_7 = 16629200814096015414;
            }
            163 => {
                current_block_7 = 7913822139267075812;
            }
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 11 | 14 | 16 | 21 | 24 | 26 | 27
            | 29 | 30 | 35 | 36 | 37 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 129 | 132 | 140
            | 165 | 168 | 174 | 175 | 177 | 178 | 179 | 182 | 180 | 181 => {
                current_block_7 = 13302909616349351127;
            }
            _ => return 0 as libc::c_int,
        }
        match current_block_7 {
            3404472414225319279 => {
                current_block_7 = 13302909616349351127;
            }
            16629200814096015414 => {
                current_block_7 = 7913822139267075812;
            }
            _ => {}
        }
        match current_block_7 {
            7913822139267075812 => {}
            _ => {}
        }
        return 1 as libc::c_int;
    }
    if id >= ocean as libc::c_int && id <= mountain_edge as libc::c_int {
        return 1 as libc::c_int;
    }
    if id >= jungle as libc::c_int && id <= jungle_hills as libc::c_int {
        return (mc >= MC_1_2 as libc::c_int) as libc::c_int;
    }
    if id >= jungle_edge as libc::c_int && id <= badlands_plateau as libc::c_int {
        return (mc >= MC_1_7 as libc::c_int) as libc::c_int;
    }
    if id >= small_end_islands as libc::c_int && id <= end_barrens as libc::c_int {
        return (mc >= MC_1_9 as libc::c_int) as libc::c_int;
    }
    if id >= warm_ocean as libc::c_int && id <= deep_frozen_ocean as libc::c_int {
        return (mc >= MC_1_13 as libc::c_int) as libc::c_int;
    }
    match id {
        127 => return (mc >= MC_1_9 as libc::c_int) as libc::c_int,
        129 | 130 | 131 | 132 | 133 | 134 | 140 | 149 | 151 | 155 | 156 | 157 | 158 | 160
        | 161 | 162 | 163 | 164 | 165 | 166 | 167 => {
            return (mc >= MC_1_7 as libc::c_int) as libc::c_int;
        }
        168 | 169 => return (mc >= MC_1_14 as libc::c_int) as libc::c_int,
        170 | 171 | 172 | 173 => return (mc >= MC_1_16 as libc::c_int) as libc::c_int,
        174 | 175 => return (mc >= MC_1_17 as libc::c_int) as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn isOverworld(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> libc::c_int {
    if biomeExists(mc, id) == 0 {
        return 0 as libc::c_int;
    }
    if id >= small_end_islands as libc::c_int && id <= end_barrens as libc::c_int {
        return 0 as libc::c_int;
    }
    if id >= soul_sand_valley as libc::c_int && id <= basalt_deltas as libc::c_int {
        return 0 as libc::c_int;
    }
    match id {
        8 | 9 => return 0 as libc::c_int,
        10 => {
            return (mc <= MC_1_6 as libc::c_int || mc >= MC_1_13 as libc::c_int)
                as libc::c_int;
        }
        20 => return (mc <= MC_1_6 as libc::c_int) as libc::c_int,
        47 | 127 => return 0 as libc::c_int,
        156 => {
            return (mc <= MC_1_8 as libc::c_int || mc >= MC_1_11 as libc::c_int)
                as libc::c_int;
        }
        174 | 175 => return (mc >= MC_1_18 as libc::c_int) as libc::c_int,
        _ => {}
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getMutated(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> libc::c_int {
    match id {
        1 => return sunflower_plains as libc::c_int,
        2 => return desert_lakes as libc::c_int,
        3 => return gravelly_mountains as libc::c_int,
        4 => return flower_forest as libc::c_int,
        5 => return taiga_mountains as libc::c_int,
        6 => return swamp_hills as libc::c_int,
        12 => return ice_spikes as libc::c_int,
        21 => return modified_jungle as libc::c_int,
        23 => return modified_jungle_edge as libc::c_int,
        27 => {
            return if mc >= MC_1_9 as libc::c_int && mc <= MC_1_10 as libc::c_int {
                tall_birch_hills as libc::c_int
            } else {
                tall_birch_forest as libc::c_int
            };
        }
        28 => {
            return if mc >= MC_1_9 as libc::c_int && mc <= MC_1_10 as libc::c_int {
                none as libc::c_int
            } else {
                tall_birch_hills as libc::c_int
            };
        }
        29 => return dark_forest_hills as libc::c_int,
        30 => return snowy_taiga_mountains as libc::c_int,
        32 => return giant_spruce_taiga as libc::c_int,
        33 => return giant_spruce_taiga_hills as libc::c_int,
        34 => return modified_gravelly_mountains as libc::c_int,
        35 => return shattered_savanna as libc::c_int,
        36 => return shattered_savanna_plateau as libc::c_int,
        37 => return eroded_badlands as libc::c_int,
        38 => return modified_wooded_badlands_plateau as libc::c_int,
        39 => return modified_badlands_plateau as libc::c_int,
        _ => return none as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn getCategory(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> libc::c_int {
    match id {
        16 | 26 => return beach as libc::c_int,
        2 | 17 | 130 => return desert as libc::c_int,
        3 | 20 | 34 | 131 | 162 => return mountains as libc::c_int,
        4 | 18 | 27 | 28 | 29 | 132 | 155 | 156 | 157 => return forest as libc::c_int,
        12 | 13 | 140 => return snowy_tundra as libc::c_int,
        21 | 22 | 23 | 149 | 151 | 168 | 169 => return jungle as libc::c_int,
        37 | 165 | 166 | 167 => return mesa as libc::c_int,
        38 | 39 => {
            return if mc <= MC_1_15 as libc::c_int {
                mesa as libc::c_int
            } else {
                badlands_plateau as libc::c_int
            };
        }
        14 | 15 => return mushroom_fields as libc::c_int,
        25 => return stone_shore as libc::c_int,
        0 | 10 | 24 | 44 | 45 | 46 | 47 | 48 | 49 | 50 => return ocean as libc::c_int,
        1 | 129 => return plains as libc::c_int,
        7 | 11 => return river as libc::c_int,
        35 | 36 | 163 | 164 => return savanna as libc::c_int,
        6 | 134 => return swamp as libc::c_int,
        5 | 19 | 30 | 31 | 32 | 33 | 133 | 158 | 160 | 161 => return taiga as libc::c_int,
        _ => return none as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn areSimilar(
    mut mc: libc::c_int,
    mut id1: libc::c_int,
    mut id2: libc::c_int,
) -> libc::c_int {
    if id1 == id2 {
        return 1 as libc::c_int;
    }
    if mc <= MC_1_15 as libc::c_int {
        if id1 == wooded_badlands_plateau as libc::c_int
            || id1 == badlands_plateau as libc::c_int
        {
            return (id2 == wooded_badlands_plateau as libc::c_int
                || id2 == badlands_plateau as libc::c_int) as libc::c_int;
        }
    }
    return (getCategory(mc, id1) == getCategory(mc, id2)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isMesa(mut id: libc::c_int) -> libc::c_int {
    match id {
        37 | 165 | 166 | 167 | 38 | 39 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn isShallowOcean(mut id: libc::c_int) -> libc::c_int {
    let shallow_bits: uint64_t = ((1 as libc::c_ulonglong) << ocean as libc::c_int
        | (1 as libc::c_ulonglong) << frozen_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << warm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << lukewarm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << cold_ocean as libc::c_int) as uint64_t;
    return ((id as uint32_t) < 64 as libc::c_int as libc::c_uint
        && (1 as libc::c_ulonglong) << id & shallow_bits as libc::c_ulonglong != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isDeepOcean(mut id: libc::c_int) -> libc::c_int {
    let deep_bits: uint64_t = ((1 as libc::c_ulonglong) << deep_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_warm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_lukewarm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_cold_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_frozen_ocean as libc::c_int) as uint64_t;
    return ((id as uint32_t) < 64 as libc::c_int as libc::c_uint
        && (1 as libc::c_ulonglong) << id & deep_bits as libc::c_ulonglong != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isOceanic(mut id: libc::c_int) -> libc::c_int {
    let ocean_bits: uint64_t = ((1 as libc::c_ulonglong) << ocean as libc::c_int
        | (1 as libc::c_ulonglong) << frozen_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << warm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << lukewarm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << cold_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_warm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_lukewarm_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_cold_ocean as libc::c_int
        | (1 as libc::c_ulonglong) << deep_frozen_ocean as libc::c_int) as uint64_t;
    return ((id as uint32_t) < 64 as libc::c_int as libc::c_uint
        && (1 as libc::c_ulonglong) << id & ocean_bits as libc::c_ulonglong != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isSnowy(mut id: libc::c_int) -> libc::c_int {
    match id {
        10 | 11 | 12 | 13 | 26 | 30 | 31 | 140 | 158 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn initBiomes() {}
#[no_mangle]
pub unsafe extern "C" fn setLayerSeed(mut layer: *mut Layer, mut worldSeed: uint64_t) {
    if !((*layer).p2).is_null() {
        setLayerSeed((*layer).p2, worldSeed);
    }
    if !((*layer).p).is_null() {
        setLayerSeed((*layer).p, worldSeed);
    }
    if !((*layer).noise).is_null() {
        let mut s: uint64_t = 0;
        setSeed(&mut s, worldSeed);
        perlinInit((*layer).noise as *mut PerlinNoise, &mut s);
    }
    let mut ls: uint64_t = (*layer).layerSalt;
    if ls == 0 as libc::c_int as libc::c_ulong {
        (*layer).startSalt = 0 as libc::c_int as uint64_t;
        (*layer).startSeed = 0 as libc::c_int as uint64_t;
    } else if ls as libc::c_ulonglong == !(0 as libc::c_ulonglong) {
        (*layer).startSalt = getVoronoiSHA(worldSeed);
        (*layer).startSeed = 0 as libc::c_int as uint64_t;
    } else {
        let mut st: uint64_t = worldSeed;
        st = mcStepSeed(st, ls);
        st = mcStepSeed(st, ls);
        st = mcStepSeed(st, ls);
        (*layer).startSalt = st;
        (*layer).startSeed = mcStepSeed(st, 0 as libc::c_int as uint64_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn initSurfaceNoise(
    mut rnd: *mut SurfaceNoise,
    mut seed: *mut uint64_t,
    mut xzScale: libc::c_double,
    mut yScale: libc::c_double,
    mut xzFactor: libc::c_double,
    mut yFactor: libc::c_double,
) {
    (*rnd).xzScale = xzScale;
    (*rnd).yScale = yScale;
    (*rnd).xzFactor = xzFactor;
    (*rnd).yFactor = yFactor;
    octaveInit(
        &mut (*rnd).octmin,
        seed,
        ((*rnd).oct).as_mut_ptr().offset(0 as libc::c_int as isize),
        -(15 as libc::c_int),
        16 as libc::c_int,
    );
    octaveInit(
        &mut (*rnd).octmax,
        seed,
        ((*rnd).oct).as_mut_ptr().offset(16 as libc::c_int as isize),
        -(15 as libc::c_int),
        16 as libc::c_int,
    );
    octaveInit(
        &mut (*rnd).octmain,
        seed,
        ((*rnd).oct).as_mut_ptr().offset(32 as libc::c_int as isize),
        -(7 as libc::c_int),
        8 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn initSurfaceNoiseEnd(
    mut rnd: *mut SurfaceNoise,
    mut seed: uint64_t,
) {
    let mut s: uint64_t = 0;
    setSeed(&mut s, seed);
    initSurfaceNoise(rnd, &mut s, 2.0f64, 1.0f64, 80.0f64, 160.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn sampleSurfaceNoise(
    mut rnd: *const SurfaceNoise,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_double {
    let mut xzScale: libc::c_double = 684.412f64 * (*rnd).xzScale;
    let mut yScale: libc::c_double = 684.412f64 * (*rnd).yScale;
    let mut xzStep: libc::c_double = xzScale / (*rnd).xzFactor;
    let mut yStep: libc::c_double = yScale / (*rnd).yFactor;
    let mut minNoise: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut maxNoise: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut mainNoise: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut persist: libc::c_double = 1.0f64;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    let mut sy: libc::c_double = 0.;
    let mut ty: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        dx = maintainPrecision(x as libc::c_double * xzScale * persist);
        dy = maintainPrecision(y as libc::c_double * yScale * persist);
        dz = maintainPrecision(z as libc::c_double * xzScale * persist);
        sy = yScale * persist;
        ty = y as libc::c_double * sy;
        minNoise
            += samplePerlin(
                &mut *((*rnd).octmin.octaves).offset(i as isize),
                dx,
                dy,
                dz,
                sy,
                ty,
            ) / persist;
        maxNoise
            += samplePerlin(
                &mut *((*rnd).octmax.octaves).offset(i as isize),
                dx,
                dy,
                dz,
                sy,
                ty,
            ) / persist;
        if i < 8 as libc::c_int {
            dx = maintainPrecision(x as libc::c_double * xzStep * persist);
            dy = maintainPrecision(y as libc::c_double * yStep * persist);
            dz = maintainPrecision(z as libc::c_double * xzStep * persist);
            sy = yStep * persist;
            ty = y as libc::c_double * sy;
            mainNoise
                += samplePerlin(
                    &mut *((*rnd).octmain.octaves).offset(i as isize),
                    dx,
                    dy,
                    dz,
                    sy,
                    ty,
                ) / persist;
        }
        persist /= 2.0f64;
        i += 1;
    }
    return clampedLerp(
        0.5f64 + 0.05f64 * mainNoise,
        minNoise / 512.0f64,
        maxNoise / 512.0f64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn setNetherSeed(mut nn: *mut NetherNoise, mut seed: uint64_t) {
    let mut s: uint64_t = 0;
    setSeed(&mut s, seed);
    doublePerlinInit(
        &mut (*nn).temperature,
        &mut s,
        &mut *((*nn).oct).as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *((*nn).oct).as_mut_ptr().offset(2 as libc::c_int as isize),
        -(7 as libc::c_int),
        2 as libc::c_int,
    );
    setSeed(&mut s, seed.wrapping_add(1 as libc::c_int as libc::c_ulong));
    doublePerlinInit(
        &mut (*nn).humidity,
        &mut s,
        &mut *((*nn).oct).as_mut_ptr().offset(4 as libc::c_int as isize),
        &mut *((*nn).oct).as_mut_ptr().offset(6 as libc::c_int as isize),
        -(7 as libc::c_int),
        2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getNetherBiome(
    mut nn: *const NetherNoise,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut ndel: *mut libc::c_float,
) -> libc::c_int {
    let npoints: [[libc::c_float; 4]; 5] = [
        [
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            nether_wastes as libc::c_int as libc::c_float,
        ],
        [
            0 as libc::c_int as libc::c_float,
            -0.5f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
            soul_sand_valley as libc::c_int as libc::c_float,
        ],
        [
            0.4f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            crimson_forest as libc::c_int as libc::c_float,
        ],
        [
            0 as libc::c_int as libc::c_float,
            0.5f64 as libc::c_float,
            (0.375f64 * 0.375f64) as libc::c_float,
            warped_forest as libc::c_int as libc::c_float,
        ],
        [
            -0.5f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
            (0.175f64 * 0.175f64) as libc::c_float,
            basalt_deltas as libc::c_int as libc::c_float,
        ],
    ];
    y = 0 as libc::c_int;
    let mut temp: libc::c_float = sampleDoublePerlin(
        &(*nn).temperature,
        x as libc::c_double,
        y as libc::c_double,
        z as libc::c_double,
    ) as libc::c_float;
    let mut humidity: libc::c_float = sampleDoublePerlin(
        &(*nn).humidity,
        x as libc::c_double,
        y as libc::c_double,
        z as libc::c_double,
    ) as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0 as libc::c_int;
    let mut dmin: libc::c_float = 3.40282347e+38f32;
    let mut dmin2: libc::c_float = 3.40282347e+38f32;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut dx: libc::c_float = npoints[i as usize][0 as libc::c_int as usize]
            - temp;
        let mut dy: libc::c_float = npoints[i as usize][1 as libc::c_int as usize]
            - humidity;
        let mut dsq: libc::c_float = dx * dx + dy * dy
            + npoints[i as usize][2 as libc::c_int as usize];
        if dsq < dmin {
            dmin2 = dmin;
            dmin = dsq;
            id = i;
        } else if dsq < dmin2 {
            dmin2 = dsq;
        }
        i += 1;
    }
    if !ndel.is_null() {
        *ndel = sqrtf(dmin2) - sqrtf(dmin);
    }
    id = npoints[id as usize][3 as libc::c_int as usize] as libc::c_int;
    return id;
}
unsafe extern "C" fn fillRad3D(
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut sz: libc::c_int,
    mut id: libc::c_int,
    mut rad: libc::c_float,
) {
    let mut r: libc::c_int = 0;
    let mut rsq: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    r = rad as libc::c_int;
    if r <= 0 as libc::c_int {
        return;
    }
    rsq = floor((rad * rad) as libc::c_double) as libc::c_int;
    k = -r;
    while k <= r {
        let mut ak: libc::c_int = y + k;
        if !(ak < 0 as libc::c_int || ak >= sy) {
            let mut ksq: libc::c_int = k * k;
            let mut yout: *mut libc::c_int = &mut *out.offset((sx * sz * ak) as isize)
                as *mut libc::c_int;
            j = -r;
            while j <= r {
                let mut aj: libc::c_int = z + j;
                if !(aj < 0 as libc::c_int || aj >= sz) {
                    let mut jksq: libc::c_int = j * j + ksq;
                    i = -r;
                    while i <= r {
                        let mut ai: libc::c_int = x + i;
                        if !(ai < 0 as libc::c_int || ai >= sx) {
                            let mut ijksq: libc::c_int = i * i + jksq;
                            if !(ijksq > rsq) {
                                *yout.offset((aj * sx + ai) as isize) = id;
                            }
                        }
                        i += 1;
                    }
                }
                j += 1;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapNether3D(
    mut nn: *const NetherNoise,
    mut out: *mut libc::c_int,
    mut r: Range,
    mut confidence: libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if r.sy <= 0 as libc::c_int {
        r.sy = 1 as libc::c_int;
    }
    if r.scale <= 3 as libc::c_int {
        printf(
            b"mapNether3D() invalid scale for this function\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut scale: libc::c_int = r.scale / 4 as libc::c_int;
    memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(r.sx as libc::c_ulong)
            .wrapping_mul(r.sy as libc::c_ulong)
            .wrapping_mul(r.sz as libc::c_ulong),
    );
    let mut invgrad: libc::c_float = (1.0f64
        / (confidence as libc::c_double * 0.05f64 * 2 as libc::c_int as libc::c_double)
        / scale as libc::c_double) as libc::c_float;
    k = 0 as libc::c_int;
    while k < r.sy {
        let mut yout: *mut libc::c_int = &mut *out.offset((r.sx * r.sz * k) as isize)
            as *mut libc::c_int;
        j = 0 as libc::c_int;
        while j < r.sz {
            i = 0 as libc::c_int;
            while i < r.sx {
                if !(*yout.offset((j * r.sx + i) as isize) != 0) {
                    let mut noisedelta: libc::c_float = 0.;
                    let mut xi: libc::c_int = (r.x + i) * scale;
                    let mut yk: libc::c_int = r.y + k;
                    let mut zj: libc::c_int = (r.z + j) * scale;
                    let mut v: libc::c_int = getNetherBiome(
                        nn,
                        xi,
                        yk,
                        zj,
                        &mut noisedelta,
                    );
                    *yout.offset((j * r.sx + i) as isize) = v;
                    let mut cellrad: libc::c_float = noisedelta * invgrad;
                    fillRad3D(out, i, j, k, r.sx, r.sy, r.sz, v, cellrad);
                }
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapNether2D(
    mut nn: *const NetherNoise,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut r: Range = {
        let mut init = Range {
            scale: 4 as libc::c_int,
            x: x,
            z: z,
            sx: w,
            sz: h,
            y: 0 as libc::c_int,
            sy: 1 as libc::c_int,
        };
        init
    };
    return mapNether3D(nn, out, r, 1.0f64 as libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn genNetherScaled(
    mut nn: *const NetherNoise,
    mut out: *mut libc::c_int,
    mut r: Range,
    mut mc: libc::c_int,
    mut sha: uint64_t,
) -> libc::c_int {
    if r.scale <= 0 as libc::c_int {
        r.scale = 4 as libc::c_int;
    }
    if r.sy == 0 as libc::c_int {
        r.sy = 1 as libc::c_int;
    }
    let mut siz: uint64_t = (r.sx as uint64_t)
        .wrapping_mul(r.sy as libc::c_ulong)
        .wrapping_mul(r.sz as libc::c_ulong);
    if mc < MC_1_16 as libc::c_int {
        let mut i: uint64_t = 0;
        i = 0 as libc::c_int as uint64_t;
        while i < siz {
            *out.offset(i as isize) = nether_wastes as libc::c_int;
            i = i.wrapping_add(1);
        }
        return 0 as libc::c_int;
    }
    if r.scale == 1 as libc::c_int {
        let mut s: Range = getVoronoiSrcRange(r);
        let mut src: *mut libc::c_int = 0 as *mut libc::c_int;
        if siz > 1 as libc::c_int as libc::c_ulong {
            src = out.offset(siz as isize);
            let mut err: libc::c_int = mapNether3D(nn, src, s, 1.0f64 as libc::c_float);
            if err != 0 {
                return err;
            }
        } else {
            src = 0 as *mut libc::c_int;
        }
        let mut i_0: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut p: *mut libc::c_int = out;
        k = 0 as libc::c_int;
        while k < r.sy {
            j = 0 as libc::c_int;
            while j < r.sz {
                i_0 = 0 as libc::c_int;
                while i_0 < r.sx {
                    let mut x4: libc::c_int = 0;
                    let mut z4: libc::c_int = 0;
                    let mut y4: libc::c_int = 0;
                    voronoiAccess3D(
                        sha,
                        r.x + i_0,
                        r.y + k,
                        r.z + j,
                        &mut x4,
                        &mut y4,
                        &mut z4,
                    );
                    if !src.is_null() {
                        x4 -= s.x;
                        y4 -= s.y;
                        z4 -= s.z;
                        *p = *src.offset((y4 * s.sx * s.sz + z4 * s.sx + x4) as isize);
                    } else {
                        *p = getNetherBiome(nn, x4, y4, z4, 0 as *mut libc::c_float);
                    }
                    p = p.offset(1);
                    i_0 += 1;
                }
                j += 1;
            }
            k += 1;
        }
        return 0 as libc::c_int;
    } else {
        return mapNether3D(nn, out, r, 1.0f64 as libc::c_float)
    };
}
#[no_mangle]
pub unsafe extern "C" fn setEndSeed(mut en: *mut EndNoise, mut seed: uint64_t) {
    let mut s: uint64_t = 0;
    setSeed(&mut s, seed);
    skipNextN(&mut s, 17292 as libc::c_int as uint64_t);
    perlinInit(en, &mut s);
}
unsafe extern "C" fn getEndBiome(
    mut hx: libc::c_int,
    mut hz: libc::c_int,
    mut hmap: *const uint16_t,
    mut hw: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let ds: [uint16_t; 26] = [
        625 as libc::c_int as uint16_t,
        529 as libc::c_int as uint16_t,
        441 as libc::c_int as uint16_t,
        361 as libc::c_int as uint16_t,
        289 as libc::c_int as uint16_t,
        225 as libc::c_int as uint16_t,
        169 as libc::c_int as uint16_t,
        121 as libc::c_int as uint16_t,
        81 as libc::c_int as uint16_t,
        49 as libc::c_int as uint16_t,
        25 as libc::c_int as uint16_t,
        9 as libc::c_int as uint16_t,
        1 as libc::c_int as uint16_t,
        1 as libc::c_int as uint16_t,
        9 as libc::c_int as uint16_t,
        25 as libc::c_int as uint16_t,
        49 as libc::c_int as uint16_t,
        81 as libc::c_int as uint16_t,
        121 as libc::c_int as uint16_t,
        169 as libc::c_int as uint16_t,
        225 as libc::c_int as uint16_t,
        289 as libc::c_int as uint16_t,
        361 as libc::c_int as uint16_t,
        441 as libc::c_int as uint16_t,
        529 as libc::c_int as uint16_t,
        625 as libc::c_int as uint16_t,
    ];
    let mut p_dsi: *const uint16_t = ds
        .as_ptr()
        .offset((hx < 0 as libc::c_int) as libc::c_int as isize);
    let mut p_dsj: *const uint16_t = ds
        .as_ptr()
        .offset((hz < 0 as libc::c_int) as libc::c_int as isize);
    let mut p_elev: *const uint16_t = hmap;
    let mut h: uint32_t = 0;
    if abs(hx) > 15 as libc::c_int || abs(hz) > 15 as libc::c_int {
        h = 14401 as libc::c_int as uint32_t;
    } else {
        h = (64 as libc::c_int * (hx * hx + hz * hz)) as uint32_t;
    }
    j = 0 as libc::c_int;
    while j < 25 as libc::c_int {
        let mut dsj: uint16_t = *p_dsj.offset(j as isize);
        let mut e: uint16_t = 0;
        let mut u: uint32_t = 0;
        i = 0 as libc::c_int;
        while i < 25 as libc::c_int {
            e = *p_elev.offset((i + 0 as libc::c_int) as isize);
            if e as libc::c_long != 0
                && {
                    u = (*p_dsi.offset((i + 0 as libc::c_int) as isize) as libc::c_uint)
                        .wrapping_add(dsj as uint32_t)
                        .wrapping_mul(e as libc::c_uint);
                    u < h
                }
            {
                h = u;
            }
            e = *p_elev.offset((i + 1 as libc::c_int) as isize);
            if e as libc::c_long != 0
                && {
                    u = (*p_dsi.offset((i + 1 as libc::c_int) as isize) as libc::c_uint)
                        .wrapping_add(dsj as uint32_t)
                        .wrapping_mul(e as libc::c_uint);
                    u < h
                }
            {
                h = u;
            }
            e = *p_elev.offset((i + 2 as libc::c_int) as isize);
            if e as libc::c_long != 0
                && {
                    u = (*p_dsi.offset((i + 2 as libc::c_int) as isize) as libc::c_uint)
                        .wrapping_add(dsj as uint32_t)
                        .wrapping_mul(e as libc::c_uint);
                    u < h
                }
            {
                h = u;
            }
            e = *p_elev.offset((i + 3 as libc::c_int) as isize);
            if e as libc::c_long != 0
                && {
                    u = (*p_dsi.offset((i + 3 as libc::c_int) as isize) as libc::c_uint)
                        .wrapping_add(dsj as uint32_t)
                        .wrapping_mul(e as libc::c_uint);
                    u < h
                }
            {
                h = u;
            }
            e = *p_elev.offset((i + 4 as libc::c_int) as isize);
            if e as libc::c_long != 0
                && {
                    u = (*p_dsi.offset((i + 4 as libc::c_int) as isize) as libc::c_uint)
                        .wrapping_add(dsj as uint32_t)
                        .wrapping_mul(e as libc::c_uint);
                    u < h
                }
            {
                h = u;
            }
            i += 5 as libc::c_int;
        }
        p_elev = p_elev.offset(hw as isize);
        j += 1;
    }
    if h < 3600 as libc::c_int as libc::c_uint {
        return end_highlands as libc::c_int
    } else {
        if h <= 10000 as libc::c_int as libc::c_uint {
            return end_midlands as libc::c_int
        } else {
            if h <= 14400 as libc::c_int as libc::c_uint {
                return end_barrens as libc::c_int;
            }
        }
    }
    return small_end_islands as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapEndBiome(
    mut en: *const EndNoise,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hw: libc::c_int = w + 26 as libc::c_int;
    let mut hh: libc::c_int = h + 26 as libc::c_int;
    let mut hmap: *mut uint16_t = malloc(
        ((hw * hh) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint16_t;
    j = 0 as libc::c_int;
    while j < hh {
        i = 0 as libc::c_int;
        while i < hw {
            let mut rx: int64_t = (x + i - 12 as libc::c_int) as int64_t;
            let mut rz: int64_t = (z + j - 12 as libc::c_int) as int64_t;
            let mut v: uint16_t = 0 as libc::c_int as uint16_t;
            if rx * rx + rz * rz > 4096 as libc::c_int as libc::c_long
                && sampleSimplex2D(en, rx as libc::c_double, rz as libc::c_double)
                    < -0.9f32 as libc::c_double
            {
                v = ((llabs(rx as libc::c_longlong)
                    * 3439 as libc::c_int as libc::c_longlong
                    + llabs(rz as libc::c_longlong)
                        * 147 as libc::c_int as libc::c_longlong)
                    % 13 as libc::c_int as libc::c_longlong
                    + 9 as libc::c_int as libc::c_longlong) as uint16_t;
                v = (v as libc::c_int * v as libc::c_int) as uint16_t;
            }
            *hmap.offset((j * hw + i) as isize) = v;
            i += 1;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut hx: int64_t = (i + x) as int64_t;
            let mut hz: int64_t = (j + z) as int64_t;
            if hx * hx + hz * hz <= 4096 as libc::c_long {
                *out.offset((j * w + i) as isize) = the_end as libc::c_int;
            } else {
                hx = 2 as libc::c_int as libc::c_long * hx
                    + 1 as libc::c_int as libc::c_long;
                hz = 2 as libc::c_int as libc::c_long * hz
                    + 1 as libc::c_int as libc::c_long;
                let mut p_elev: *mut uint16_t = &mut *hmap
                    .offset(
                        ((hz / 2 as libc::c_int as libc::c_long - z as libc::c_long)
                            * hw as libc::c_long
                            + (hx / 2 as libc::c_int as libc::c_long
                                - x as libc::c_long)) as isize,
                    ) as *mut uint16_t;
                *out
                    .offset(
                        (j * w + i) as isize,
                    ) = getEndBiome(hx as libc::c_int, hz as libc::c_int, p_elev, hw);
            }
            i += 1;
        }
        j += 1;
    }
    free(hmap as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapEnd(
    mut en: *const EndNoise,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut cx: libc::c_int = x >> 2 as libc::c_int;
    let mut cz: libc::c_int = z >> 2 as libc::c_int;
    let mut cw: libc::c_int = (x + w >> 2 as libc::c_int) + 1 as libc::c_int - cx;
    let mut ch: libc::c_int = (z + h >> 2 as libc::c_int) + 1 as libc::c_int - cz;
    let mut buf: *mut libc::c_int = malloc(
        ((cw * ch) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    mapEndBiome(en, buf, cx, cz, cw, ch);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        let mut cj: libc::c_int = (z + j >> 2 as libc::c_int) - cz;
        i = 0 as libc::c_int;
        while i < w {
            let mut ci: libc::c_int = (x + i >> 2 as libc::c_int) - cx;
            let mut v: libc::c_int = *buf.offset((cj * cw + ci) as isize);
            *out.offset((j * w + i) as isize) = v;
            i += 1;
        }
        j += 1;
    }
    free(buf as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getEndHeightNoise(
    mut en: *const EndNoise,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_float {
    let mut hx: libc::c_int = x / 2 as libc::c_int;
    let mut hz: libc::c_int = z / 2 as libc::c_int;
    let mut oddx: libc::c_int = x % 2 as libc::c_int;
    let mut oddz: libc::c_int = z % 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut h: int64_t = 64 as libc::c_int as libc::c_long
        * (x as libc::c_long * x as int64_t + z as libc::c_long * z as int64_t);
    j = -(12 as libc::c_int);
    while j <= 12 as libc::c_int {
        i = -(12 as libc::c_int);
        while i <= 12 as libc::c_int {
            let mut rx: int64_t = (hx + i) as int64_t;
            let mut rz: int64_t = (hz + j) as int64_t;
            let mut v: uint16_t = 0 as libc::c_int as uint16_t;
            if rx * rx + rz * rz > 4096 as libc::c_int as libc::c_long
                && sampleSimplex2D(en, rx as libc::c_double, rz as libc::c_double)
                    < -0.9f32 as libc::c_double
            {
                v = ((llabs(rx as libc::c_longlong)
                    * 3439 as libc::c_int as libc::c_longlong
                    + llabs(rz as libc::c_longlong)
                        * 147 as libc::c_int as libc::c_longlong)
                    % 13 as libc::c_int as libc::c_longlong
                    + 9 as libc::c_int as libc::c_longlong) as uint16_t;
                rx = (oddx - i * 2 as libc::c_int) as int64_t;
                rz = (oddz - j * 2 as libc::c_int) as int64_t;
                let mut noise: int64_t = (rx * rx + rz * rz) * v as libc::c_long
                    * v as libc::c_long;
                if noise < h {
                    h = noise;
                }
            }
            i += 1;
        }
        j += 1;
    }
    let mut ret: libc::c_float = 100 as libc::c_int as libc::c_float
        - sqrtf(h as libc::c_float);
    if ret < -(100 as libc::c_int) as libc::c_float {
        ret = -(100 as libc::c_int) as libc::c_float;
    }
    if ret > 80 as libc::c_int as libc::c_float {
        ret = 80 as libc::c_int as libc::c_float;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sampleNoiseColumnEnd(
    mut column: *mut libc::c_double,
    mut sn: *const SurfaceNoise,
    mut en: *const EndNoise,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut colymin: libc::c_int,
    mut colymax: libc::c_int,
) {
    let mut depth: libc::c_double = (getEndHeightNoise(en, x, z) - 8.0f32)
        as libc::c_double;
    let mut y: libc::c_int = 0;
    y = colymin;
    while y <= colymax {
        let mut noise: libc::c_double = sampleSurfaceNoise(sn, x, y, z);
        noise += depth;
        noise = clampedLerp(
            (32 as libc::c_int + 46 as libc::c_int - y) as libc::c_double / 64.0f64,
            -(3000 as libc::c_int) as libc::c_double,
            noise,
        );
        noise = clampedLerp(
            (y - 1 as libc::c_int) as libc::c_double / 7.0f64,
            -(30 as libc::c_int) as libc::c_double,
            noise,
        );
        *column.offset((y - colymin) as isize) = noise;
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getSurfaceHeight(
    mut ncol00: *const libc::c_double,
    mut ncol01: *const libc::c_double,
    mut ncol10: *const libc::c_double,
    mut ncol11: *const libc::c_double,
    mut colymin: libc::c_int,
    mut colymax: libc::c_int,
    mut blockspercell: libc::c_int,
    mut dx: libc::c_double,
    mut dz: libc::c_double,
) -> libc::c_int {
    let mut y: libc::c_int = 0;
    let mut celly: libc::c_int = 0;
    celly = colymax - 1 as libc::c_int;
    while celly >= colymin {
        let mut idx: libc::c_int = celly - colymin;
        let mut v000: libc::c_double = *ncol00.offset(idx as isize);
        let mut v001: libc::c_double = *ncol01.offset(idx as isize);
        let mut v100: libc::c_double = *ncol10.offset(idx as isize);
        let mut v101: libc::c_double = *ncol11.offset(idx as isize);
        let mut v010: libc::c_double = *ncol00.offset((idx + 1 as libc::c_int) as isize);
        let mut v011: libc::c_double = *ncol01.offset((idx + 1 as libc::c_int) as isize);
        let mut v110: libc::c_double = *ncol10.offset((idx + 1 as libc::c_int) as isize);
        let mut v111: libc::c_double = *ncol11.offset((idx + 1 as libc::c_int) as isize);
        y = blockspercell - 1 as libc::c_int;
        while y >= 0 as libc::c_int {
            let mut dy: libc::c_double = y as libc::c_double
                / blockspercell as libc::c_double;
            let mut noise: libc::c_double = lerp3(
                dy,
                dx,
                dz,
                v000,
                v010,
                v100,
                v110,
                v001,
                v011,
                v101,
                v111,
            );
            if noise > 0 as libc::c_int as libc::c_double {
                return celly * blockspercell + y;
            }
            y -= 1;
        }
        celly -= 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getSurfaceHeightEnd(
    mut mc: libc::c_int,
    mut seed: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_int {
    let mut en: EndNoise = EndNoise {
        d: [0; 512],
        a: 0.,
        b: 0.,
        c: 0.,
        amplitude: 0.,
        lacunarity: 0.,
    };
    setEndSeed(&mut en, seed);
    let mut sn: SurfaceNoise = SurfaceNoise {
        xzScale: 0.,
        yScale: 0.,
        xzFactor: 0.,
        yFactor: 0.,
        octmin: OctaveNoise {
            octcnt: 0,
            octaves: 0 as *mut PerlinNoise,
        },
        octmax: OctaveNoise {
            octcnt: 0,
            octaves: 0 as *mut PerlinNoise,
        },
        octmain: OctaveNoise {
            octcnt: 0,
            octaves: 0 as *mut PerlinNoise,
        },
        oct: [EndNoise {
            d: [0; 512],
            a: 0.,
            b: 0.,
            c: 0.,
            amplitude: 0.,
            lacunarity: 0.,
        }; 40],
    };
    initSurfaceNoiseEnd(&mut sn, seed);
    let mut cellx: libc::c_int = x >> 3 as libc::c_int;
    let mut cellz: libc::c_int = z >> 3 as libc::c_int;
    let mut dx: libc::c_double = (x & 7 as libc::c_int) as libc::c_double / 8.0f64;
    let mut dz: libc::c_double = (z & 7 as libc::c_int) as libc::c_double / 8.0f64;
    let y0: libc::c_int = 0 as libc::c_int;
    let y1: libc::c_int = 32 as libc::c_int;
    let yn: libc::c_int = y1 - y0 + 1 as libc::c_int;
    let mut ncol00: [libc::c_double; 33] = [0.; 33];
    let mut ncol01: [libc::c_double; 33] = [0.; 33];
    let mut ncol10: [libc::c_double; 33] = [0.; 33];
    let mut ncol11: [libc::c_double; 33] = [0.; 33];
    sampleNoiseColumnEnd(ncol00.as_mut_ptr(), &mut sn, &mut en, cellx, cellz, y0, y1);
    sampleNoiseColumnEnd(
        ncol01.as_mut_ptr(),
        &mut sn,
        &mut en,
        cellx,
        cellz + 1 as libc::c_int,
        y0,
        y1,
    );
    sampleNoiseColumnEnd(
        ncol10.as_mut_ptr(),
        &mut sn,
        &mut en,
        cellx + 1 as libc::c_int,
        cellz,
        y0,
        y1,
    );
    sampleNoiseColumnEnd(
        ncol11.as_mut_ptr(),
        &mut sn,
        &mut en,
        cellx + 1 as libc::c_int,
        cellz + 1 as libc::c_int,
        y0,
        y1,
    );
    return getSurfaceHeight(
        ncol00.as_mut_ptr() as *const libc::c_double,
        ncol01.as_mut_ptr() as *const libc::c_double,
        ncol10.as_mut_ptr() as *const libc::c_double,
        ncol11.as_mut_ptr() as *const libc::c_double,
        y0,
        y1,
        4 as libc::c_int,
        dx,
        dz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn genEndScaled(
    mut en: *const EndNoise,
    mut out: *mut libc::c_int,
    mut r: Range,
    mut mc: libc::c_int,
    mut sha: uint64_t,
) -> libc::c_int {
    if r.scale != 1 as libc::c_int && r.scale != 4 as libc::c_int
        && r.scale != 16 as libc::c_int && r.scale != 64 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if r.sy == 0 as libc::c_int {
        r.sy = 1 as libc::c_int;
    }
    if mc < MC_1_9 as libc::c_int {
        let mut i: uint64_t = 0;
        let mut siz: uint64_t = (r.sx as uint64_t)
            .wrapping_mul(r.sy as libc::c_ulong)
            .wrapping_mul(r.sz as libc::c_ulong);
        i = 0 as libc::c_int as uint64_t;
        while i < siz {
            *out.offset(i as isize) = the_end as libc::c_int;
            i = i.wrapping_add(1);
        }
        return 0 as libc::c_int;
    }
    let mut err: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    if r.scale == 1 as libc::c_int {
        let mut s: Range = getVoronoiSrcRange(r);
        err = mapEnd(en, out, s.x, s.z, s.sx, s.sz);
        if err != 0 {
            return err;
        }
        if mc <= MC_1_14 as libc::c_int {
            let mut lvoronoi: Layer = Layer {
                getMap: None,
                mc: 0,
                zoom: 0,
                edge: 0,
                scale: 0,
                layerSalt: 0,
                startSalt: 0,
                startSeed: 0,
                noise: 0 as *mut libc::c_void,
                data: 0 as *mut libc::c_void,
                p: 0 as *mut Layer,
                p2: 0 as *mut Layer,
            };
            memset(
                &mut lvoronoi as *mut Layer as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<Layer>() as libc::c_ulong,
            );
            lvoronoi.startSalt = getLayerSalt(10 as libc::c_int as uint64_t);
            err = mapVoronoi114(&mut lvoronoi, out, r.x, r.z, r.sx, r.sz);
            if err != 0 {
                return err;
            }
        } else {
            let mut src: *mut libc::c_int = out.offset((r.sx * r.sy * r.sz) as isize);
            memmove(
                src as *mut libc::c_void,
                out as *const libc::c_void,
                ((s.sx * s.sz) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            iy = 0 as libc::c_int;
            while iy < r.sy {
                mapVoronoiPlane(
                    sha,
                    out.offset((r.sx * r.sz * iy) as isize),
                    src,
                    r.x,
                    r.z,
                    r.sx,
                    r.sz,
                    r.y + iy,
                    s.x,
                    s.z,
                    s.sx,
                    s.sz,
                );
                iy += 1;
            }
            return 0 as libc::c_int;
        }
    } else if r.scale == 4 as libc::c_int {
        err = mapEnd(en, out, r.x, r.z, r.sx, r.sz);
        if err != 0 {
            return err;
        }
    } else if r.scale == 16 as libc::c_int {
        err = mapEndBiome(en, out, r.x, r.z, r.sx, r.sz);
        if err != 0 {
            return err;
        }
    } else if r.scale == 64 as libc::c_int {
        let mut i_0: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut di: libc::c_int = 0;
        let mut dj: libc::c_int = 0;
        let mut d: libc::c_int = 4 as libc::c_int;
        let mut hw: libc::c_int = (2 as libc::c_int + r.sx) * d + 1 as libc::c_int;
        let mut hh: libc::c_int = (2 as libc::c_int + r.sz) * d + 1 as libc::c_int;
        let mut hmap: *mut int16_t = calloc(
            (hw * hh) as libc::c_ulong,
            ::std::mem::size_of::<int16_t>() as libc::c_ulong,
        ) as *mut int16_t;
        j = 0 as libc::c_int;
        while j < r.sz {
            i_0 = 0 as libc::c_int;
            while i_0 < r.sx {
                let mut hx: int64_t = ((i_0 + r.x) * d) as int64_t;
                let mut hz: int64_t = ((j + r.z) * d) as int64_t;
                if hx * hx + hz * hz <= 4096 as libc::c_long {
                    *out.offset((j * r.sx + i_0) as isize) = the_end as libc::c_int;
                } else {
                    let mut h: int64_t = (64 as libc::c_int * 16 as libc::c_int
                        * 16 as libc::c_int) as int64_t;
                    dj = -d;
                    while dj < d {
                        di = -d;
                        while di < d {
                            let mut rx: int64_t = hx + di as libc::c_long;
                            let mut rz: int64_t = hz + dj as libc::c_long;
                            let mut hi: libc::c_int = i_0 * d + di + d;
                            let mut hj: libc::c_int = j * d + dj + d;
                            let mut p: *mut int16_t = &mut *hmap
                                .offset((hj * hw + hi) as isize) as *mut int16_t;
                            if *p as libc::c_int == 0 as libc::c_int {
                                if sampleSimplex2D(
                                    en,
                                    rx as libc::c_double,
                                    rz as libc::c_double,
                                ) < -0.9f32 as libc::c_double
                                {
                                    *p = ((llabs(rx as libc::c_longlong)
                                        * 3439 as libc::c_int as libc::c_longlong
                                        + llabs(rz as libc::c_longlong)
                                            * 147 as libc::c_int as libc::c_longlong)
                                        % 13 as libc::c_int as libc::c_longlong
                                        + 9 as libc::c_int as libc::c_longlong) as int16_t;
                                    *p = (*p as libc::c_int * *p as libc::c_int) as int16_t;
                                } else {
                                    *p = -(1 as libc::c_int) as int16_t;
                                }
                            }
                            if *p as libc::c_int > 0 as libc::c_int {
                                let mut noise: int64_t = (4 as libc::c_int
                                    * (di * di + dj * dj) * *p as libc::c_int) as int64_t;
                                if noise < h {
                                    h = noise;
                                }
                            }
                            di += 1;
                        }
                        dj += 1;
                    }
                    if h < 3600 as libc::c_int as libc::c_long {
                        *out
                            .offset(
                                (j * r.sx + i_0) as isize,
                            ) = end_highlands as libc::c_int;
                    } else if h <= 10000 as libc::c_int as libc::c_long {
                        *out
                            .offset(
                                (j * r.sx + i_0) as isize,
                            ) = end_midlands as libc::c_int;
                    } else if h <= 14400 as libc::c_int as libc::c_long {
                        *out
                            .offset(
                                (j * r.sx + i_0) as isize,
                            ) = end_barrens as libc::c_int;
                    } else {
                        *out
                            .offset(
                                (j * r.sx + i_0) as isize,
                            ) = small_end_islands as libc::c_int;
                    }
                }
                i_0 += 1;
            }
            j += 1;
        }
        free(hmap as *mut libc::c_void);
    } else {
        return 1 as libc::c_int
    }
    iy = 1 as libc::c_int;
    while iy < r.sy {
        let mut i_1: libc::c_int = 0;
        let mut siz_0: libc::c_int = r.sx * r.sz;
        i_1 = 0 as libc::c_int;
        while i_1 < siz_0 {
            *out.offset((iy * siz_0 + i_1) as isize) = *out.offset(i_1 as isize);
            i_1 += 1;
        }
        iy += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setBiomeSeed(
    mut bn: *mut BiomeNoise,
    mut seed: uint64_t,
    mut large: libc::c_int,
) {
    let mut pxr: Xoroshiro = Xoroshiro { lo: 0, hi: 0 };
    let mut n: libc::c_int = 0 as libc::c_int;
    xSetSeed(&mut pxr, seed);
    let mut xlo: uint64_t = xNextLong(&mut pxr);
    let mut xhi: uint64_t = xNextLong(&mut pxr);
    let mut amp_s: [libc::c_double; 4] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    pxr.lo = xlo ^ 0x80518cf6af25384 as libc::c_long as libc::c_ulong;
    pxr.hi = xhi ^ 0x3f3dfb40a54febd5 as libc::c_long as libc::c_ulong;
    n
        += xDoublePerlinInit(
            &mut (*bn).shift,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_s.as_mut_ptr(),
            -(3 as libc::c_int),
            (::std::mem::size_of::<[libc::c_double; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    let mut amp_t: [libc::c_double; 6] = [
        1.5f64,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    pxr
        .lo = xlo
        ^ (if large != 0 {
            0x944b0073edf549db as libc::c_ulong
        } else {
            0x5c7e6b29735f0d7f as libc::c_long as libc::c_ulong
        });
    pxr
        .hi = xhi
        ^ (if large != 0 {
            0x4ff44347e9d22b96 as libc::c_long as libc::c_ulong
        } else {
            0xf7d86f1bbc734988 as libc::c_ulong
        });
    n
        += xDoublePerlinInit(
            &mut (*bn).temperature,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_t.as_mut_ptr(),
            if large != 0 { -(12 as libc::c_int) } else { -(10 as libc::c_int) },
            (::std::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    let mut amp_h: [libc::c_double; 6] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    pxr
        .lo = xlo
        ^ (if large != 0 {
            0x71b8ab943dbd5301 as libc::c_long as libc::c_ulong
        } else {
            0x81bb4d22e8dc168e as libc::c_ulong
        });
    pxr
        .hi = xhi
        ^ (if large != 0 {
            0xbb63ddcf39ff7a2b as libc::c_ulong
        } else {
            0xf1c8b4bea16303cd as libc::c_ulong
        });
    n
        += xDoublePerlinInit(
            &mut (*bn).humidity,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_h.as_mut_ptr(),
            if large != 0 { -(10 as libc::c_int) } else { -(8 as libc::c_int) },
            (::std::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    let mut amp_c: [libc::c_double; 9] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    pxr
        .lo = xlo
        ^ (if large != 0 {
            0x9a3f51a113fce8dc as libc::c_ulong
        } else {
            0x83886c9d0ae3a662 as libc::c_ulong
        });
    pxr
        .hi = xhi
        ^ (if large != 0 {
            0xee2dbd157e5dcdad as libc::c_ulong
        } else {
            0xafa638a61b42e8ad as libc::c_ulong
        });
    n
        += xDoublePerlinInit(
            &mut (*bn).continentalness,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_c.as_mut_ptr(),
            if large != 0 { -(11 as libc::c_int) } else { -(9 as libc::c_int) },
            (::std::mem::size_of::<[libc::c_double; 9]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    let mut amp_e: [libc::c_double; 5] = [
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    pxr
        .lo = xlo
        ^ (if large != 0 {
            0x8c984b1f8702a951 as libc::c_ulong
        } else {
            0xd02491e6058f6fd8 as libc::c_ulong
        });
    pxr
        .hi = xhi
        ^ (if large != 0 {
            0xead7b1f92bae535f as libc::c_ulong
        } else {
            0x4792512c94c17a80 as libc::c_long as libc::c_ulong
        });
    n
        += xDoublePerlinInit(
            &mut (*bn).erosion,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_e.as_mut_ptr(),
            if large != 0 { -(11 as libc::c_int) } else { -(9 as libc::c_int) },
            (::std::mem::size_of::<[libc::c_double; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    let mut amp_w: [libc::c_double; 6] = [
        1 as libc::c_int as libc::c_double,
        2 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    pxr.lo = xlo ^ 0xefc8ef4d36102b34 as libc::c_ulong;
    pxr.hi = xhi ^ 0x1beeeb324a0f24ea as libc::c_long as libc::c_ulong;
    n
        += xDoublePerlinInit(
            &mut (*bn).weirdness,
            &mut pxr,
            ((*bn).oct).as_mut_ptr().offset(n as isize),
            amp_w.as_mut_ptr(),
            -(7 as libc::c_int),
            (::std::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
        );
    if n as size_t
        > (::std::mem::size_of::<[PerlinNoise; 46]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<PerlinNoise>() as libc::c_ulong)
    {
        printf(
            b"setBiomeSeed(): BiomeNoise is malformed, buffer too small\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*bn).previdx = 0 as libc::c_int;
}
unsafe extern "C" fn addSplineVal(
    mut rsp: *mut Spline,
    mut loc: libc::c_float,
    mut val: *mut Spline,
    mut der: libc::c_float,
) {
    (*rsp).loc[(*rsp).len as usize] = loc;
    let ref mut fresh0 = (*rsp).val[(*rsp).len as usize];
    *fresh0 = val;
    (*rsp).der[(*rsp).len as usize] = der;
    let ref mut fresh1 = (*rsp).len;
    *fresh1 += 1;
}
unsafe extern "C" fn createFixSpline(
    mut ss: *mut SplineStack,
    mut val: libc::c_float,
) -> *mut Spline {
    let ref mut fresh2 = (*ss).flen;
    let fresh3 = *fresh2;
    *fresh2 = *fresh2 + 1;
    let mut sp: *mut FixSpline = &mut *((*ss).fstack)
        .as_mut_ptr()
        .offset(fresh3 as isize) as *mut FixSpline;
    (*sp).len = 1 as libc::c_int;
    (*sp).val = val;
    return sp as *mut Spline;
}
unsafe extern "C" fn getOffsetValue(
    mut weirdness: libc::c_float,
    mut continentalness: libc::c_float,
) -> libc::c_float {
    let mut f0: libc::c_float = 1.0f32 - (1.0f32 - continentalness) * 0.5f32;
    let mut f1: libc::c_float = 0.5f32 * (1.0f32 - continentalness);
    let mut f2: libc::c_float = (weirdness + 1.17f32) * 0.46082947f32;
    let mut off: libc::c_float = f2 * f0 - f1;
    if weirdness < -0.7f32 {
        return if off > -0.2222f32 { off } else { -0.2222f32 }
    } else {
        return if off > 0 as libc::c_int as libc::c_float {
            off
        } else {
            0 as libc::c_int as libc::c_float
        }
    };
}
unsafe extern "C" fn createSpline_38219(
    mut ss: *mut SplineStack,
    mut f: libc::c_float,
    mut bl: libc::c_int,
) -> *mut Spline {
    let ref mut fresh4 = (*ss).len;
    let fresh5 = *fresh4;
    *fresh4 = *fresh4 + 1;
    let mut sp: *mut Spline = &mut *((*ss).stack).as_mut_ptr().offset(fresh5 as isize)
        as *mut Spline;
    (*sp).typ = RIDGES as libc::c_int;
    let mut i: libc::c_float = getOffsetValue(-1.0f32, f);
    let mut k: libc::c_float = getOffsetValue(1.0f32, f);
    let mut l: libc::c_float = 1.0f32 - (1.0f32 - f) * 0.5f32;
    let mut u: libc::c_float = 0.5f32 * (1.0f32 - f);
    l = u / (0.46082947f32 * l) - 1.17f32;
    if -0.65f32 < l && l < 1.0f32 {
        let mut p: libc::c_float = 0.;
        let mut q: libc::c_float = 0.;
        let mut r: libc::c_float = 0.;
        let mut s: libc::c_float = 0.;
        u = getOffsetValue(-0.65f32, f);
        p = getOffsetValue(-0.75f32, f);
        q = (p - i) * 4.0f32;
        r = getOffsetValue(l, f);
        s = (k - r) / (1.0f32 - l);
        addSplineVal(sp, -1.0f32, createFixSpline(ss, i), q);
        addSplineVal(
            sp,
            -0.75f32,
            createFixSpline(ss, p),
            0 as libc::c_int as libc::c_float,
        );
        addSplineVal(
            sp,
            -0.65f32,
            createFixSpline(ss, u),
            0 as libc::c_int as libc::c_float,
        );
        addSplineVal(
            sp,
            l - 0.01f32,
            createFixSpline(ss, r),
            0 as libc::c_int as libc::c_float,
        );
        addSplineVal(sp, l, createFixSpline(ss, r), s);
        addSplineVal(sp, 1.0f32, createFixSpline(ss, k), s);
    } else {
        u = (k - i) * 0.5f32;
        if bl != 0 {
            addSplineVal(
                sp,
                -1.0f32,
                createFixSpline(
                    ss,
                    (if i as libc::c_double > 0.2f64 {
                        i as libc::c_double
                    } else {
                        0.2f64
                    }) as libc::c_float,
                ),
                0 as libc::c_int as libc::c_float,
            );
            addSplineVal(
                sp,
                0.0f32,
                createFixSpline(
                    ss,
                    lerp(
                        0.5f32 as libc::c_double,
                        i as libc::c_double,
                        k as libc::c_double,
                    ) as libc::c_float,
                ),
                u,
            );
        } else {
            addSplineVal(sp, -1.0f32, createFixSpline(ss, i), u);
        }
        addSplineVal(sp, 1.0f32, createFixSpline(ss, k), u);
    }
    return sp;
}
unsafe extern "C" fn createFlatOffsetSpline(
    mut ss: *mut SplineStack,
    mut f: libc::c_float,
    mut g: libc::c_float,
    mut h: libc::c_float,
    mut i: libc::c_float,
    mut j: libc::c_float,
    mut k: libc::c_float,
) -> *mut Spline {
    let ref mut fresh6 = (*ss).len;
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    let mut sp: *mut Spline = &mut *((*ss).stack).as_mut_ptr().offset(fresh7 as isize)
        as *mut Spline;
    (*sp).typ = RIDGES as libc::c_int;
    let mut l: libc::c_float = 0.5f32 * (g - f);
    if l < k {
        l = k;
    }
    let mut m: libc::c_float = 5.0f32 * (h - g);
    addSplineVal(sp, -1.0f32, createFixSpline(ss, f), l);
    addSplineVal(sp, -0.4f32, createFixSpline(ss, g), if l < m { l } else { m });
    addSplineVal(sp, 0.0f32, createFixSpline(ss, h), m);
    addSplineVal(sp, 0.4f32, createFixSpline(ss, i), 2.0f32 * (i - h));
    addSplineVal(sp, 1.0f32, createFixSpline(ss, j), 0.7f32 * (j - i));
    return sp;
}
unsafe extern "C" fn createLandSpline(
    mut ss: *mut SplineStack,
    mut f: libc::c_float,
    mut g: libc::c_float,
    mut h: libc::c_float,
    mut i: libc::c_float,
    mut j: libc::c_float,
    mut k: libc::c_float,
    mut bl: libc::c_int,
) -> *mut Spline {
    let mut sp1: *mut Spline = createSpline_38219(
        ss,
        lerp(i as libc::c_double, 0.6f32 as libc::c_double, 1.5f32 as libc::c_double)
            as libc::c_float,
        bl,
    );
    let mut sp2: *mut Spline = createSpline_38219(
        ss,
        lerp(i as libc::c_double, 0.6f32 as libc::c_double, 1.0f32 as libc::c_double)
            as libc::c_float,
        bl,
    );
    let mut sp3: *mut Spline = createSpline_38219(ss, i, bl);
    let ih: libc::c_float = 0.5f32 * i;
    let mut sp4: *mut Spline = createFlatOffsetSpline(
        ss,
        f - 0.15f32,
        ih,
        ih,
        ih,
        i * 0.6f32,
        0.5f32,
    );
    let mut sp5: *mut Spline = createFlatOffsetSpline(
        ss,
        f,
        j * i,
        g * i,
        ih,
        i * 0.6f32,
        0.5f32,
    );
    let mut sp6: *mut Spline = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5f32);
    let mut sp7: *mut Spline = createFlatOffsetSpline(ss, f, j, j, g, h, 0.5f32);
    let ref mut fresh8 = (*ss).len;
    let fresh9 = *fresh8;
    *fresh8 = *fresh8 + 1;
    let mut sp8: *mut Spline = &mut *((*ss).stack).as_mut_ptr().offset(fresh9 as isize)
        as *mut Spline;
    (*sp8).typ = RIDGES as libc::c_int;
    addSplineVal(sp8, -1.0f32, createFixSpline(ss, f), 0.0f32);
    addSplineVal(sp8, -0.4f32, sp6, 0.0f32);
    addSplineVal(sp8, 0.0f32, createFixSpline(ss, h + 0.07f32), 0.0f32);
    let mut sp9: *mut Spline = createFlatOffsetSpline(ss, -0.02f32, k, k, g, h, 0.0f32);
    let ref mut fresh10 = (*ss).len;
    let fresh11 = *fresh10;
    *fresh10 = *fresh10 + 1;
    let mut sp: *mut Spline = &mut *((*ss).stack).as_mut_ptr().offset(fresh11 as isize)
        as *mut Spline;
    (*sp).typ = EROSION as libc::c_int;
    addSplineVal(sp, -0.85f32, sp1, 0.0f32);
    addSplineVal(sp, -0.7f32, sp2, 0.0f32);
    addSplineVal(sp, -0.4f32, sp3, 0.0f32);
    addSplineVal(sp, -0.35f32, sp4, 0.0f32);
    addSplineVal(sp, -0.1f32, sp5, 0.0f32);
    addSplineVal(sp, 0.2f32, sp6, 0.0f32);
    if bl != 0 {
        addSplineVal(sp, 0.4f32, sp7, 0.0f32);
        addSplineVal(sp, 0.45f32, sp8, 0.0f32);
        addSplineVal(sp, 0.55f32, sp8, 0.0f32);
        addSplineVal(sp, 0.58f32, sp7, 0.0f32);
    }
    addSplineVal(sp, 0.7f32, sp9, 0.0f32);
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn getSpline(
    mut sp: *const Spline,
    mut vals: *const libc::c_float,
) -> libc::c_float {
    if sp.is_null() || (*sp).len <= 0 as libc::c_int || (*sp).len >= 12 as libc::c_int {
        printf(b"getSpline(): bad parameters\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if (*sp).len == 1 as libc::c_int {
        return (*(sp as *mut FixSpline)).val;
    }
    let mut f: libc::c_float = *vals.offset((*sp).typ as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*sp).len {
        if (*sp).loc[i as usize] >= f {
            break;
        }
        i += 1;
    }
    if i == 0 as libc::c_int || i == (*sp).len {
        if i != 0 {
            i -= 1;
        }
        let mut v: libc::c_float = getSpline((*sp).val[i as usize], vals);
        return v + (*sp).der[i as usize] * (f - (*sp).loc[i as usize]);
    }
    let mut sp1: *const Spline = (*sp).val[(i - 1 as libc::c_int) as usize];
    let mut sp2: *const Spline = (*sp).val[i as usize];
    let mut g: libc::c_float = (*sp).loc[(i - 1 as libc::c_int) as usize];
    let mut h: libc::c_float = (*sp).loc[i as usize];
    let mut k: libc::c_float = (f - g) / (h - g);
    let mut l: libc::c_float = (*sp).der[(i - 1 as libc::c_int) as usize];
    let mut m: libc::c_float = (*sp).der[i as usize];
    let mut n: libc::c_float = getSpline(sp1, vals);
    let mut o: libc::c_float = getSpline(sp2, vals);
    let mut p: libc::c_float = l * (h - g) - (o - n);
    let mut q: libc::c_float = -m * (h - g) + (o - n);
    let mut r: libc::c_float = (lerp(
        k as libc::c_double,
        n as libc::c_double,
        o as libc::c_double,
    )
        + (k * (1.0f32 - k)) as libc::c_double
            * lerp(k as libc::c_double, p as libc::c_double, q as libc::c_double))
        as libc::c_float;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn initBiomeNoise(mut bn: *mut BiomeNoise, mut mc: libc::c_int) {
    let mut ss: *mut SplineStack = &mut (*bn).ss;
    memset(
        ss as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SplineStack>() as libc::c_ulong,
    );
    let ref mut fresh12 = (*ss).len;
    let fresh13 = *fresh12;
    *fresh12 = *fresh12 + 1;
    let mut sp: *mut Spline = &mut *((*ss).stack).as_mut_ptr().offset(fresh13 as isize)
        as *mut Spline;
    (*sp).typ = CONTINENTALNESS as libc::c_int;
    let mut sp1: *mut Spline = createLandSpline(
        ss,
        -0.15f32,
        0.00f32,
        0.0f32,
        0.1f32,
        0.00f32,
        -0.03f32,
        0 as libc::c_int,
    );
    let mut sp2: *mut Spline = createLandSpline(
        ss,
        -0.10f32,
        0.03f32,
        0.1f32,
        0.1f32,
        0.01f32,
        -0.03f32,
        0 as libc::c_int,
    );
    let mut sp3: *mut Spline = createLandSpline(
        ss,
        -0.10f32,
        0.03f32,
        0.1f32,
        0.7f32,
        0.01f32,
        -0.03f32,
        1 as libc::c_int,
    );
    let mut sp4: *mut Spline = createLandSpline(
        ss,
        -0.05f32,
        0.03f32,
        0.1f32,
        1.0f32,
        0.01f32,
        0.01f32,
        1 as libc::c_int,
    );
    addSplineVal(sp, -1.10f32, createFixSpline(ss, 0.044f32), 0.0f32);
    addSplineVal(sp, -1.02f32, createFixSpline(ss, -0.2222f32), 0.0f32);
    addSplineVal(sp, -0.51f32, createFixSpline(ss, -0.2222f32), 0.0f32);
    addSplineVal(sp, -0.44f32, createFixSpline(ss, -0.12f32), 0.0f32);
    addSplineVal(sp, -0.18f32, createFixSpline(ss, -0.12f32), 0.0f32);
    addSplineVal(sp, -0.16f32, sp1, 0.0f32);
    addSplineVal(sp, -0.15f32, sp1, 0.0f32);
    addSplineVal(sp, -0.10f32, sp2, 0.0f32);
    addSplineVal(sp, 0.25f32, sp3, 0.0f32);
    addSplineVal(sp, 1.00f32, sp4, 0.0f32);
    let ref mut fresh14 = (*bn).sp;
    *fresh14 = sp;
    (*bn).mc = mc;
}
#[no_mangle]
pub unsafe extern "C" fn sampleBiomeNoise(
    mut bn: *const BiomeNoise,
    mut np: *mut int64_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut dat: *mut uint64_t,
    mut flags: uint32_t,
) -> libc::c_int {
    let mut t: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut h: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut c: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut e: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut d: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut w: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut px: libc::c_double = x as libc::c_double;
    let mut pz: libc::c_double = z as libc::c_double;
    if flags & SAMPLE_NO_SHIFT as libc::c_int as libc::c_uint == 0 {
        px
            += sampleDoublePerlin(
                &(*bn).shift,
                x as libc::c_double,
                0 as libc::c_int as libc::c_double,
                z as libc::c_double,
            ) * 4.0f64;
        pz
            += sampleDoublePerlin(
                &(*bn).shift,
                z as libc::c_double,
                x as libc::c_double,
                0 as libc::c_int as libc::c_double,
            ) * 4.0f64;
    }
    c = sampleDoublePerlin(
        &(*bn).continentalness,
        px,
        0 as libc::c_int as libc::c_double,
        pz,
    ) as libc::c_float;
    e = sampleDoublePerlin(&(*bn).erosion, px, 0 as libc::c_int as libc::c_double, pz)
        as libc::c_float;
    w = sampleDoublePerlin(&(*bn).weirdness, px, 0 as libc::c_int as libc::c_double, pz)
        as libc::c_float;
    if flags & SAMPLE_NO_DEPTH as libc::c_int as libc::c_uint == 0 {
        let mut np_param: [libc::c_float; 4] = [
            c,
            e,
            -3.0f32 * (fabsf(fabsf(w) - 0.6666667f32) - 0.33333334f32),
            w,
        ];
        let mut off: libc::c_double = (getSpline((*bn).sp, np_param.as_mut_ptr())
            + 0.015f32) as libc::c_double;
        d = (1.0f64 - (y << 2 as libc::c_int) as libc::c_double / 128.0f64
            - 83.0f64 / 160.0f64 + off) as libc::c_float;
    }
    t = sampleDoublePerlin(
        &(*bn).temperature,
        px,
        0 as libc::c_int as libc::c_double,
        pz,
    ) as libc::c_float;
    h = sampleDoublePerlin(&(*bn).humidity, px, 0 as libc::c_int as libc::c_double, pz)
        as libc::c_float;
    let mut l_np: [int64_t; 6] = [0; 6];
    let mut p_np: *mut int64_t = if !np.is_null() { np } else { l_np.as_mut_ptr() };
    *p_np.offset(0 as libc::c_int as isize) = (10000.0f32 * t) as int64_t;
    *p_np.offset(1 as libc::c_int as isize) = (10000.0f32 * h) as int64_t;
    *p_np.offset(2 as libc::c_int as isize) = (10000.0f32 * c) as int64_t;
    *p_np.offset(3 as libc::c_int as isize) = (10000.0f32 * e) as int64_t;
    *p_np.offset(4 as libc::c_int as isize) = (10000.0f32 * d) as int64_t;
    *p_np.offset(5 as libc::c_int as isize) = (10000.0f32 * w) as int64_t;
    let mut id: libc::c_int = none as libc::c_int;
    if flags & SAMPLE_NO_BIOME as libc::c_int as libc::c_uint == 0 {
        id = p2overworld((*bn).mc, p_np as *const uint64_t, dat);
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn genBiomeNoiseChunkSection(
    mut bn: *const BiomeNoise,
    mut out: *mut [[libc::c_int; 4]; 4],
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut cz: libc::c_int,
    mut dat: *mut uint64_t,
) {
    let mut buf: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x4: libc::c_int = cx << 2 as libc::c_int;
    let mut y4: libc::c_int = cy << 2 as libc::c_int;
    let mut z4: libc::c_int = cz << 2 as libc::c_int;
    if dat.is_null() {
        dat = &mut buf;
    }
    if *dat == 0 as libc::c_int as libc::c_ulong {
        sampleBiomeNoise(
            bn,
            0 as *mut int64_t,
            x4 + 3 as libc::c_int,
            y4 - 1 as libc::c_int,
            z4 + 3 as libc::c_int,
            dat,
            0 as libc::c_int as uint32_t,
        );
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                (*out
                    .offset(
                        i as isize,
                    ))[j
                    as usize][k
                    as usize] = sampleBiomeNoise(
                    bn,
                    0 as *mut int64_t,
                    x4 + i,
                    y4 + j,
                    z4 + k,
                    dat,
                    0 as libc::c_int as uint32_t,
                );
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn genBiomeNoise3D(
    mut bn: *const BiomeNoise,
    mut out: *mut libc::c_int,
    mut r: Range,
    mut opt: libc::c_int,
) {
    let mut dat: uint64_t = 0 as libc::c_int as uint64_t;
    let mut p_dat: *mut uint64_t = if opt != 0 { &mut dat } else { 0 as *mut uint64_t };
    let mut flags: uint32_t = (if opt != 0 {
        SAMPLE_NO_SHIFT as libc::c_int
    } else {
        0 as libc::c_int
    }) as uint32_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_int = out;
    let mut scale: libc::c_int = if r.scale > 4 as libc::c_int {
        r.scale / 4 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut mid: libc::c_int = scale / 2 as libc::c_int;
    k = 0 as libc::c_int;
    while k < r.sy {
        let mut yk: libc::c_int = r.y + k;
        j = 0 as libc::c_int;
        while j < r.sz {
            let mut zj: libc::c_int = (r.z + j) * scale + mid;
            i = 0 as libc::c_int;
            while i < r.sx {
                let mut xi: libc::c_int = (r.x + i) * scale + mid;
                *p = sampleBiomeNoise(bn, 0 as *mut int64_t, xi, yk, zj, p_dat, flags);
                p = p.offset(1);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn genBiomeNoiseScaled(
    mut bn: *const BiomeNoise,
    mut out: *mut libc::c_int,
    mut r: Range,
    mut mc: libc::c_int,
    mut sha: uint64_t,
) -> libc::c_int {
    if mc <= MC_1_17 as libc::c_int {
        return 1 as libc::c_int;
    }
    if r.sy == 0 as libc::c_int {
        r.sy = 1 as libc::c_int;
    }
    let mut siz: uint64_t = (r.sx as uint64_t)
        .wrapping_mul(r.sy as libc::c_ulong)
        .wrapping_mul(r.sz as libc::c_ulong);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if r.scale == 1 as libc::c_int {
        let mut s: Range = getVoronoiSrcRange(r);
        let mut src: *mut libc::c_int = 0 as *mut libc::c_int;
        if siz > 1 as libc::c_int as libc::c_ulong {
            src = out.offset(siz as isize);
            genBiomeNoise3D(bn, src, s, 0 as libc::c_int);
        } else {
            src = 0 as *mut libc::c_int;
        }
        let mut p: *mut libc::c_int = out;
        k = 0 as libc::c_int;
        while k < r.sy {
            j = 0 as libc::c_int;
            while j < r.sz {
                i = 0 as libc::c_int;
                while i < r.sx {
                    let mut x4: libc::c_int = 0;
                    let mut z4: libc::c_int = 0;
                    let mut y4: libc::c_int = 0;
                    voronoiAccess3D(
                        sha,
                        r.x + i,
                        r.y + k,
                        r.z + j,
                        &mut x4,
                        &mut y4,
                        &mut z4,
                    );
                    if !src.is_null() {
                        x4 -= s.x;
                        y4 -= s.y;
                        z4 -= s.z;
                        *p = *src.offset((y4 * s.sx * s.sz + z4 * s.sx + x4) as isize);
                    } else {
                        *p = sampleBiomeNoise(
                            bn,
                            0 as *mut int64_t,
                            x4,
                            y4,
                            z4,
                            0 as *mut uint64_t,
                            0 as libc::c_int as uint32_t,
                        );
                    }
                    p = p.offset(1);
                    i += 1;
                }
                j += 1;
            }
            k += 1;
        }
    } else {
        genBiomeNoise3D(bn, out, r, (r.scale > 4 as libc::c_int) as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn isAny4(
    mut id: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    return (id == a || id == b || id == c || id == d) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapContinent(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            cs = getChunkSeed(ss, i + x, j + z);
            *out.offset((i + j * w) as isize) = mcFirstIsZero(cs, 10 as libc::c_int);
            i += 1;
        }
        j += 1;
    }
    if x > -w && x <= 0 as libc::c_int && z > -h && z <= 0 as libc::c_int {
        *out.offset((-x + -z * w) as isize) = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapZoomFuzzy(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x >> 1 as libc::c_int;
    let mut pZ: libc::c_int = z >> 1 as libc::c_int;
    let mut pW: libc::c_int = (x + w >> 1 as libc::c_int) - pX + 1 as libc::c_int;
    let mut pH: libc::c_int = (z + h >> 1 as libc::c_int) - pZ + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut newW: libc::c_int = pW << 1 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut v00: libc::c_int = 0;
    let mut v01: libc::c_int = 0;
    let mut v10: libc::c_int = 0;
    let mut v11: libc::c_int = 0;
    let mut buf: *mut libc::c_int = out.offset((pW * pH) as isize);
    let st: uint32_t = (*l).startSalt as uint32_t;
    let ss: uint32_t = (*l).startSeed as uint32_t;
    j = 0 as libc::c_int;
    while j < pH {
        idx = (j << 1 as libc::c_int) * newW;
        v00 = *out.offset(((j + 0 as libc::c_int) * pW) as isize);
        v01 = *out.offset(((j + 1 as libc::c_int) * pW) as isize);
        i = 0 as libc::c_int;
        while i < pW {
            v10 = *out
                .offset((i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize);
            v11 = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v00 == v01 && v00 == v10 && v00 == v11 {
                *buf.offset(idx as isize) = v00;
                *buf.offset((idx + 1 as libc::c_int) as isize) = v00;
                *buf.offset((idx + newW) as isize) = v00;
                *buf.offset((idx + newW + 1 as libc::c_int) as isize) = v00;
                idx += 2 as libc::c_int;
            } else {
                let mut chunkX: libc::c_int = (((i + pX) as uint32_t)
                    << 1 as libc::c_int) as libc::c_int;
                let mut chunkZ: libc::c_int = (((j + pZ) as uint32_t)
                    << 1 as libc::c_int) as libc::c_int;
                let mut cs: uint32_t = ss;
                cs = (cs as libc::c_uint).wrapping_add(chunkX as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkZ as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkX as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkZ as libc::c_uint)
                    as uint32_t as uint32_t;
                *buf.offset(idx as isize) = v00;
                *buf
                    .offset(
                        (idx + newW) as isize,
                    ) = if cs >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint
                    != 0
                {
                    v01
                } else {
                    v00
                };
                idx += 1;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(st) as uint32_t as uint32_t;
                *buf
                    .offset(
                        idx as isize,
                    ) = if cs >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint
                    != 0
                {
                    v10
                } else {
                    v00
                };
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(st) as uint32_t as uint32_t;
                let mut r: libc::c_int = (cs >> 24 as libc::c_int
                    & 3 as libc::c_int as libc::c_uint) as libc::c_int;
                *buf
                    .offset(
                        (idx + newW) as isize,
                    ) = if r == 0 as libc::c_int {
                    v00
                } else if r == 1 as libc::c_int {
                    v10
                } else if r == 2 as libc::c_int {
                    v01
                } else {
                    v11
                };
                idx += 1;
            }
            i += 1;
            v00 = v10;
            v01 = v11;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < h {
        memmove(
            &mut *out.offset((j * w) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf
                .offset(
                    ((j + (z & 1 as libc::c_int)) * newW + (x & 1 as libc::c_int))
                        as isize,
                ) as *mut libc::c_int as *const libc::c_void,
            (w as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        j += 1;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn select4(
    mut cs: uint32_t,
    mut st: uint32_t,
    mut v00: libc::c_int,
    mut v01: libc::c_int,
    mut v10: libc::c_int,
    mut v11: libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut cv00: libc::c_int = (v00 == v10) as libc::c_int + (v00 == v01) as libc::c_int
        + (v00 == v11) as libc::c_int;
    let mut cv10: libc::c_int = (v10 == v01) as libc::c_int
        + (v10 == v11) as libc::c_int;
    let mut cv01: libc::c_int = (v01 == v11) as libc::c_int;
    if cv00 > cv10 && cv00 > cv01 {
        v = v00;
    } else if cv10 > cv00 {
        v = v10;
    } else if cv01 > cv00 {
        v = v01;
    } else {
        cs = (cs as libc::c_long
            * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint) as libc::c_long
                + 4150755663 as libc::c_long)) as uint32_t;
        cs = (cs as libc::c_uint).wrapping_add(st) as uint32_t as uint32_t;
        let mut r: libc::c_int = (cs >> 24 as libc::c_int
            & 3 as libc::c_int as libc::c_uint) as libc::c_int;
        v = if r == 0 as libc::c_int {
            v00
        } else if r == 1 as libc::c_int {
            v10
        } else if r == 2 as libc::c_int {
            v01
        } else {
            v11
        };
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn mapZoom(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x >> 1 as libc::c_int;
    let mut pZ: libc::c_int = z >> 1 as libc::c_int;
    let mut pW: libc::c_int = (x + w >> 1 as libc::c_int) - pX + 1 as libc::c_int;
    let mut pH: libc::c_int = (z + h >> 1 as libc::c_int) - pZ + 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut newW: libc::c_int = pW << 1 as libc::c_int;
    let mut idx: libc::c_int = 0;
    let mut v00: libc::c_int = 0;
    let mut v01: libc::c_int = 0;
    let mut v10: libc::c_int = 0;
    let mut v11: libc::c_int = 0;
    let mut buf: *mut libc::c_int = out.offset((pW * pH) as isize);
    let st: uint32_t = (*l).startSalt as uint32_t;
    let ss: uint32_t = (*l).startSeed as uint32_t;
    j = 0 as libc::c_int;
    while j < pH {
        idx = (j << 1 as libc::c_int) * newW;
        v00 = *out.offset(((j + 0 as libc::c_int) * pW) as isize);
        v01 = *out.offset(((j + 1 as libc::c_int) * pW) as isize);
        i = 0 as libc::c_int;
        while i < pW {
            v10 = *out
                .offset((i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize);
            v11 = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v00 == v01 && v00 == v10 && v00 == v11 {
                *buf.offset(idx as isize) = v00;
                *buf.offset((idx + 1 as libc::c_int) as isize) = v00;
                *buf.offset((idx + newW) as isize) = v00;
                *buf.offset((idx + newW + 1 as libc::c_int) as isize) = v00;
                idx += 2 as libc::c_int;
            } else {
                let mut chunkX: libc::c_int = (((i + pX) as uint32_t)
                    << 1 as libc::c_int) as libc::c_int;
                let mut chunkZ: libc::c_int = (((j + pZ) as uint32_t)
                    << 1 as libc::c_int) as libc::c_int;
                let mut cs: uint32_t = ss;
                cs = (cs as libc::c_uint).wrapping_add(chunkX as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkZ as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkX as libc::c_uint)
                    as uint32_t as uint32_t;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(chunkZ as libc::c_uint)
                    as uint32_t as uint32_t;
                *buf.offset(idx as isize) = v00;
                *buf
                    .offset(
                        (idx + newW) as isize,
                    ) = if cs >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint
                    != 0
                {
                    v01
                } else {
                    v00
                };
                idx += 1;
                cs = (cs as libc::c_long
                    * (cs.wrapping_mul(1284865837 as libc::c_int as libc::c_uint)
                        as libc::c_long + 4150755663 as libc::c_long)) as uint32_t;
                cs = (cs as libc::c_uint).wrapping_add(st) as uint32_t as uint32_t;
                *buf
                    .offset(
                        idx as isize,
                    ) = if cs >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint
                    != 0
                {
                    v10
                } else {
                    v00
                };
                *buf.offset((idx + newW) as isize) = select4(cs, st, v00, v01, v10, v11);
                idx += 1;
            }
            i += 1;
            v00 = v10;
            v01 = v11;
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < h {
        memmove(
            &mut *out.offset((j * w) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf
                .offset(
                    ((j + (z & 1 as libc::c_int)) * newW + (x & 1 as libc::c_int))
                        as isize,
                ) as *mut libc::c_int as *const libc::c_void,
            (w as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapLand(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut st: uint64_t = (*l).startSalt;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        let mut v00: libc::c_int = *vz0.offset(0 as libc::c_int as isize);
        let mut vt0: libc::c_int = *vz0.offset(1 as libc::c_int as isize);
        let mut v02: libc::c_int = *vz2.offset(0 as libc::c_int as isize);
        let mut vt2: libc::c_int = *vz2.offset(1 as libc::c_int as isize);
        let mut v20: libc::c_int = 0;
        let mut v22: libc::c_int = 0;
        let mut v11: libc::c_int = 0;
        let mut v: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < w {
            v11 = *vz1.offset((i + 1 as libc::c_int) as isize);
            v20 = *vz0.offset((i + 2 as libc::c_int) as isize);
            v22 = *vz2.offset((i + 2 as libc::c_int) as isize);
            v = v11;
            match v11 {
                0 => {
                    if v00 != 0 || v20 != 0 || v02 != 0 || v22 != 0 {
                        cs = getChunkSeed(ss, i + x, j + z);
                        let mut inc: libc::c_int = 0 as libc::c_int;
                        v = 1 as libc::c_int;
                        if v00 != ocean as libc::c_int {
                            inc += 1;
                            v = v00;
                            cs = mcStepSeed(cs, st);
                        }
                        if v20 != ocean as libc::c_int {
                            inc += 1;
                            if inc == 1 as libc::c_int
                                || mcFirstIsZero(cs, 2 as libc::c_int) != 0
                            {
                                v = v20;
                            }
                            cs = mcStepSeed(cs, st);
                        }
                        if v02 != ocean as libc::c_int {
                            inc += 1;
                            match inc {
                                1 => {
                                    v = v02;
                                }
                                2 => {
                                    if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                        v = v02;
                                    }
                                }
                                _ => {
                                    if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                        v = v02;
                                    }
                                }
                            }
                            cs = mcStepSeed(cs, st);
                        }
                        if v22 != ocean as libc::c_int {
                            inc += 1;
                            match inc {
                                1 => {
                                    v = v22;
                                }
                                2 => {
                                    if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                        v = v22;
                                    }
                                }
                                3 => {
                                    if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                        v = v22;
                                    }
                                }
                                _ => {
                                    if mcFirstIsZero(cs, 4 as libc::c_int) != 0 {
                                        v = v22;
                                    }
                                }
                            }
                            cs = mcStepSeed(cs, st);
                        }
                        if v != forest as libc::c_int {
                            if mcFirstIsZero(cs, 3 as libc::c_int) == 0 {
                                v = ocean as libc::c_int;
                            }
                        }
                    }
                }
                4 => {}
                _ => {
                    if v00 == 0 as libc::c_int || v20 == 0 as libc::c_int
                        || v02 == 0 as libc::c_int || v22 == 0 as libc::c_int
                    {
                        cs = getChunkSeed(ss, i + x, j + z);
                        if mcFirstIsZero(cs, 5 as libc::c_int) != 0 {
                            v = 0 as libc::c_int;
                        }
                    }
                }
            }
            *out.offset((i + j * w) as isize) = v;
            v00 = vt0;
            vt0 = v20;
            v02 = vt2;
            vt2 = v22;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapLand16(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut st: uint64_t = (*l).startSalt;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        let mut v00: libc::c_int = *vz0.offset(0 as libc::c_int as isize);
        let mut vt0: libc::c_int = *vz0.offset(1 as libc::c_int as isize);
        let mut v02: libc::c_int = *vz2.offset(0 as libc::c_int as isize);
        let mut vt2: libc::c_int = *vz2.offset(1 as libc::c_int as isize);
        let mut v20: libc::c_int = 0;
        let mut v22: libc::c_int = 0;
        let mut v11: libc::c_int = 0;
        let mut v: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < w {
            v11 = *vz1.offset((i + 1 as libc::c_int) as isize);
            v20 = *vz0.offset((i + 2 as libc::c_int) as isize);
            v22 = *vz2.offset((i + 2 as libc::c_int) as isize);
            v = v11;
            if v11 != 0 as libc::c_int
                || v00 == 0 as libc::c_int && v20 == 0 as libc::c_int
                    && v02 == 0 as libc::c_int && v22 == 0 as libc::c_int
            {
                if v11 != 0 as libc::c_int
                    && (v00 == 0 as libc::c_int || v20 == 0 as libc::c_int
                        || v02 == 0 as libc::c_int || v22 == 0 as libc::c_int)
                {
                    cs = getChunkSeed(ss, i + x, j + z);
                    if mcFirstIsZero(cs, 5 as libc::c_int) != 0 {
                        v = if v == snowy_tundra as libc::c_int {
                            frozen_ocean as libc::c_int
                        } else {
                            ocean as libc::c_int
                        };
                    }
                }
            } else {
                cs = getChunkSeed(ss, i + x, j + z);
                let mut inc: libc::c_int = 0 as libc::c_int;
                v = 1 as libc::c_int;
                if v00 != ocean as libc::c_int {
                    inc += 1;
                    v = v00;
                    cs = mcStepSeed(cs, st);
                }
                if v20 != ocean as libc::c_int {
                    inc += 1;
                    if inc == 1 as libc::c_int
                        || mcFirstIsZero(cs, 2 as libc::c_int) != 0
                    {
                        v = v20;
                    }
                    cs = mcStepSeed(cs, st);
                }
                if v02 != ocean as libc::c_int {
                    inc += 1;
                    match inc {
                        1 => {
                            v = v02;
                        }
                        2 => {
                            if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                v = v02;
                            }
                        }
                        _ => {
                            if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                v = v02;
                            }
                        }
                    }
                    cs = mcStepSeed(cs, st);
                }
                if v22 != ocean as libc::c_int {
                    inc += 1;
                    match inc {
                        1 => {
                            v = v22;
                        }
                        2 => {
                            if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                v = v22;
                            }
                        }
                        3 => {
                            if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                v = v22;
                            }
                        }
                        _ => {
                            if mcFirstIsZero(cs, 4 as libc::c_int) != 0 {
                                v = v22;
                            }
                        }
                    }
                    cs = mcStepSeed(cs, st);
                }
                if mcFirstIsZero(cs, 3 as libc::c_int) == 0 {
                    v = if v == snowy_tundra as libc::c_int {
                        frozen_ocean as libc::c_int
                    } else {
                        ocean as libc::c_int
                    };
                }
            }
            *out.offset((i + j * w) as isize) = v;
            v00 = vt0;
            vt0 = v20;
            v02 = vt2;
            vt2 = v22;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapIsland(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            *out.offset((i + j * w) as isize) = v11;
            if v11 == Oceanic as libc::c_int {
                if !(*out
                    .offset(
                        (i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                    ) != Oceanic as libc::c_int)
                {
                    if !(*out
                        .offset(
                            (i + 2 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                        ) != Oceanic as libc::c_int)
                    {
                        if !(*out
                            .offset(
                                (i + 0 as libc::c_int + (j + 1 as libc::c_int) * pW)
                                    as isize,
                            ) != Oceanic as libc::c_int)
                        {
                            if !(*out
                                .offset(
                                    (i + 1 as libc::c_int + (j + 2 as libc::c_int) * pW)
                                        as isize,
                                ) != Oceanic as libc::c_int)
                            {
                                cs = getChunkSeed(ss, i + x, j + z);
                                if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                    *out.offset((i + j * w) as isize) = 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSnow16(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v11 == 0 as libc::c_int {
                *out.offset((i + j * w) as isize) = v11;
            } else {
                cs = getChunkSeed(ss, i + x, j + z);
                *out
                    .offset(
                        (i + j * w) as isize,
                    ) = if mcFirstIsZero(cs, 5 as libc::c_int) != 0 {
                    snowy_tundra as libc::c_int
                } else {
                    plains as libc::c_int
                };
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSnow(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if isShallowOcean(v11) != 0 {
                *out.offset((i + j * w) as isize) = v11;
            } else {
                cs = getChunkSeed(ss, i + x, j + z);
                let mut r: libc::c_int = mcFirstInt(cs, 6 as libc::c_int);
                let mut v: libc::c_int = 0;
                if r == 0 as libc::c_int {
                    v = Freezing as libc::c_int;
                } else if r <= 1 as libc::c_int {
                    v = Cold as libc::c_int;
                } else {
                    v = Warm as libc::c_int;
                }
                *out.offset((i + j * w) as isize) = v;
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapCool(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v11 == Warm as libc::c_int {
                let mut v10: libc::c_int = *out
                    .offset(
                        (i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                    );
                let mut v21: libc::c_int = *out
                    .offset(
                        (i + 2 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                    );
                let mut v01: libc::c_int = *out
                    .offset(
                        (i + 0 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                    );
                let mut v12: libc::c_int = *out
                    .offset(
                        (i + 1 as libc::c_int + (j + 2 as libc::c_int) * pW) as isize,
                    );
                if isAny4(Cold as libc::c_int, v10, v21, v01, v12) != 0
                    || isAny4(Freezing as libc::c_int, v10, v21, v01, v12) != 0
                {
                    v11 = Lush as libc::c_int;
                }
            }
            *out.offset((i + j * w) as isize) = v11;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapHeat(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v11 == Freezing as libc::c_int {
                let mut v10: libc::c_int = *out
                    .offset(
                        (i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                    );
                let mut v21: libc::c_int = *out
                    .offset(
                        (i + 2 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                    );
                let mut v01: libc::c_int = *out
                    .offset(
                        (i + 0 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                    );
                let mut v12: libc::c_int = *out
                    .offset(
                        (i + 1 as libc::c_int + (j + 2 as libc::c_int) * pW) as isize,
                    );
                if isAny4(Warm as libc::c_int, v10, v21, v01, v12) != 0
                    || isAny4(Lush as libc::c_int, v10, v21, v01, v12) != 0
                {
                    v11 = Cold as libc::c_int;
                }
            }
            *out.offset((i + j * w) as isize) = v11;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSpecial(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut st: uint64_t = (*l).startSalt;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v: libc::c_int = *out.offset((i + j * w) as isize);
            if !(v == 0 as libc::c_int) {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(cs, 13 as libc::c_int) != 0 {
                    cs = mcStepSeed(cs, st);
                    v
                        |= 1 as libc::c_int + mcFirstInt(cs, 15 as libc::c_int)
                            << 8 as libc::c_int & 0xf00 as libc::c_int;
                    *out.offset((i + j * w) as isize) = v;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapMushroom(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if v11 == 0 as libc::c_int
                && *out
                    .offset(
                        (i + 0 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                    ) == 0
                && *out
                    .offset(
                        (i + 2 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                    ) == 0
                && *out
                    .offset(
                        (i + 0 as libc::c_int + (j + 2 as libc::c_int) * pW) as isize,
                    ) == 0
                && *out
                    .offset(
                        (i + 2 as libc::c_int + (j + 2 as libc::c_int) * pW) as isize,
                    ) == 0
            {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(cs, 100 as libc::c_int) != 0 {
                    v11 = mushroom_fields as libc::c_int;
                }
            }
            *out.offset((i + j * w) as isize) = v11;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapDeepOcean(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            if isShallowOcean(v11) != 0 {
                let mut oceans: libc::c_int = 0 as libc::c_int;
                if isShallowOcean(
                    *out
                        .offset(
                            (i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW) as isize,
                        ),
                ) != 0
                {
                    oceans += 1;
                }
                if isShallowOcean(
                    *out
                        .offset(
                            (i + 2 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                        ),
                ) != 0
                {
                    oceans += 1;
                }
                if isShallowOcean(
                    *out
                        .offset(
                            (i + 0 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize,
                        ),
                ) != 0
                {
                    oceans += 1;
                }
                if isShallowOcean(
                    *out
                        .offset(
                            (i + 1 as libc::c_int + (j + 2 as libc::c_int) * pW) as isize,
                        ),
                ) != 0
                {
                    oceans += 1;
                }
                if oceans >= 4 as libc::c_int {
                    match v11 {
                        44 => {
                            v11 = deep_warm_ocean as libc::c_int;
                        }
                        45 => {
                            v11 = deep_lukewarm_ocean as libc::c_int;
                        }
                        0 => {
                            v11 = deep_ocean as libc::c_int;
                        }
                        46 => {
                            v11 = deep_cold_ocean as libc::c_int;
                        }
                        10 => {
                            v11 = deep_frozen_ocean as libc::c_int;
                        }
                        _ => {
                            v11 = deep_ocean as libc::c_int;
                        }
                    }
                }
            }
            *out.offset((i + j * w) as isize) = v11;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut warmBiomes: [libc::c_int; 6] = [
    desert as libc::c_int,
    desert as libc::c_int,
    desert as libc::c_int,
    savanna as libc::c_int,
    savanna as libc::c_int,
    plains as libc::c_int,
];
#[no_mangle]
pub static mut lushBiomes: [libc::c_int; 6] = [
    forest as libc::c_int,
    dark_forest as libc::c_int,
    mountains as libc::c_int,
    plains as libc::c_int,
    birch_forest as libc::c_int,
    swamp as libc::c_int,
];
#[no_mangle]
pub static mut coldBiomes: [libc::c_int; 4] = [
    forest as libc::c_int,
    mountains as libc::c_int,
    taiga as libc::c_int,
    plains as libc::c_int,
];
#[no_mangle]
pub static mut snowBiomes: [libc::c_int; 4] = [
    snowy_tundra as libc::c_int,
    snowy_tundra as libc::c_int,
    snowy_tundra as libc::c_int,
    snowy_taiga as libc::c_int,
];
#[no_mangle]
pub static mut oldBiomes: [libc::c_int; 7] = [
    desert as libc::c_int,
    forest as libc::c_int,
    mountains as libc::c_int,
    swamp as libc::c_int,
    plains as libc::c_int,
    taiga as libc::c_int,
    jungle as libc::c_int,
];
#[no_mangle]
pub static mut oldBiomes11: [libc::c_int; 6] = [
    desert as libc::c_int,
    forest as libc::c_int,
    mountains as libc::c_int,
    swamp as libc::c_int,
    plains as libc::c_int,
    taiga as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn mapBiome(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        let mut current_block_26: u64;
        i = 0 as libc::c_int;
        while i < w {
            let mut v: libc::c_int = 0;
            let mut idx: libc::c_int = i + j * w;
            let mut id: libc::c_int = *out.offset(idx as isize);
            let mut hasHighBit: libc::c_int = id & 0xf00 as libc::c_int;
            id &= !(0xf00 as libc::c_int);
            if mc <= MC_1_6 as libc::c_int {
                if id == ocean as libc::c_int || id == mushroom_fields as libc::c_int {
                    *out.offset(idx as isize) = id;
                    current_block_26 = 7351195479953500246;
                } else {
                    cs = getChunkSeed(ss, i + x, j + z);
                    if mc <= MC_1_1 as libc::c_int {
                        v = oldBiomes11[mcFirstInt(cs, 6 as libc::c_int) as usize];
                    } else {
                        v = oldBiomes[mcFirstInt(cs, 7 as libc::c_int) as usize];
                    }
                    if id != plains as libc::c_int
                        && (v != taiga as libc::c_int || mc <= MC_1_2 as libc::c_int)
                    {
                        v = snowy_tundra as libc::c_int;
                    }
                    current_block_26 = 15090052786889560393;
                }
            } else if isOceanic(id) != 0 || id == mushroom_fields as libc::c_int {
                *out.offset(idx as isize) = id;
                current_block_26 = 7351195479953500246;
            } else {
                cs = getChunkSeed(ss, i + x, j + z);
                match id {
                    1 => {
                        if hasHighBit != 0 {
                            v = if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                badlands_plateau as libc::c_int
                            } else {
                                wooded_badlands_plateau as libc::c_int
                            };
                        } else {
                            v = warmBiomes[mcFirstInt(cs, 6 as libc::c_int) as usize];
                        }
                    }
                    2 => {
                        if hasHighBit != 0 {
                            v = jungle as libc::c_int;
                        } else {
                            v = lushBiomes[mcFirstInt(cs, 6 as libc::c_int) as usize];
                        }
                    }
                    3 => {
                        if hasHighBit != 0 {
                            v = giant_tree_taiga as libc::c_int;
                        } else {
                            v = coldBiomes[mcFirstInt(cs, 4 as libc::c_int) as usize];
                        }
                    }
                    4 => {
                        v = snowBiomes[mcFirstInt(cs, 4 as libc::c_int) as usize];
                    }
                    _ => {
                        v = mushroom_fields as libc::c_int;
                    }
                }
                current_block_26 = 15090052786889560393;
            }
            match current_block_26 {
                15090052786889560393 => {
                    *out.offset(idx as isize) = v;
                }
                _ => {}
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapNoise(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    let mut mod_0: libc::c_int = if (*l).mc as libc::c_int <= MC_1_6 as libc::c_int {
        2 as libc::c_int
    } else {
        299999 as libc::c_int
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            if *out.offset((i + j * w) as isize) > 0 as libc::c_int {
                cs = getChunkSeed(ss, i + x, j + z);
                *out
                    .offset(
                        (i + j * w) as isize,
                    ) = mcFirstInt(cs, mod_0) + 2 as libc::c_int;
            } else {
                *out.offset((i + j * w) as isize) = 0 as libc::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapBamboo(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut idx: libc::c_int = i + j * w;
            if !(*out.offset(idx as isize) != jungle as libc::c_int) {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(cs, 10 as libc::c_int) != 0 {
                    *out.offset(idx as isize) = bamboo_jungle as libc::c_int;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn replaceEdge(
    mut out: *mut libc::c_int,
    mut idx: libc::c_int,
    mut mc: libc::c_int,
    mut v10: libc::c_int,
    mut v21: libc::c_int,
    mut v01: libc::c_int,
    mut v12: libc::c_int,
    mut id: libc::c_int,
    mut baseID: libc::c_int,
    mut edgeID: libc::c_int,
) -> libc::c_int {
    if id != baseID {
        return 0 as libc::c_int;
    }
    if areSimilar(mc, v10, baseID) != 0 && areSimilar(mc, v21, baseID) != 0
        && areSimilar(mc, v01, baseID) != 0 && areSimilar(mc, v12, baseID) != 0
    {
        *out.offset(idx as isize) = id;
    } else {
        *out.offset(idx as isize) = edgeID;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapBiomeEdge(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *vz1.offset((i + 1 as libc::c_int) as isize);
            let mut v10: libc::c_int = *vz0.offset((i + 1 as libc::c_int) as isize);
            let mut v21: libc::c_int = *vz1.offset((i + 2 as libc::c_int) as isize);
            let mut v01: libc::c_int = *vz1.offset((i + 0 as libc::c_int) as isize);
            let mut v12: libc::c_int = *vz2.offset((i + 1 as libc::c_int) as isize);
            if replaceEdge(
                out,
                i + j * w,
                mc,
                v10,
                v21,
                v01,
                v12,
                v11,
                wooded_badlands_plateau as libc::c_int,
                badlands as libc::c_int,
            ) == 0
                && replaceEdge(
                    out,
                    i + j * w,
                    mc,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    badlands_plateau as libc::c_int,
                    badlands as libc::c_int,
                ) == 0
                && replaceEdge(
                    out,
                    i + j * w,
                    mc,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    giant_tree_taiga as libc::c_int,
                    taiga as libc::c_int,
                ) == 0
            {
                if v11 == desert as libc::c_int {
                    if isAny4(snowy_tundra as libc::c_int, v10, v21, v01, v12) == 0 {
                        *out.offset((i + j * w) as isize) = v11;
                    } else {
                        *out
                            .offset(
                                (i + j * w) as isize,
                            ) = wooded_mountains as libc::c_int;
                    }
                } else if v11 == swamp as libc::c_int {
                    if isAny4(desert as libc::c_int, v10, v21, v01, v12) == 0
                        && isAny4(snowy_taiga as libc::c_int, v10, v21, v01, v12) == 0
                        && isAny4(snowy_tundra as libc::c_int, v10, v21, v01, v12) == 0
                    {
                        if isAny4(jungle as libc::c_int, v10, v21, v01, v12) == 0
                            && isAny4(bamboo_jungle as libc::c_int, v10, v21, v01, v12)
                                == 0
                        {
                            *out.offset((i + j * w) as isize) = v11;
                        } else {
                            *out
                                .offset((i + j * w) as isize) = jungle_edge as libc::c_int;
                        }
                    } else {
                        *out.offset((i + j * w) as isize) = plains as libc::c_int;
                    }
                } else {
                    *out.offset((i + j * w) as isize) = v11;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapHills(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if ((*l).p2 == 0 as *mut libc::c_void as *mut Layer) as libc::c_int as libc::c_long
        != 0
    {
        printf(
            b"mapHills() requires two parents! Use setupMultiLayer()\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut err: libc::c_int = 0;
    err = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut riv: *mut libc::c_int = out.offset((pW * pH) as isize);
    err = ((*(*l).p2).getMap)
        .expect("non-null function pointer")((*l).p2, riv, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    let mut st: uint64_t = (*l).startSalt;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut a11: libc::c_int = *out
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            let mut b11: libc::c_int = *riv
                .offset((i + 1 as libc::c_int + (j + 1 as libc::c_int) * pW) as isize);
            let mut idx: libc::c_int = i + j * w;
            let mut bn: libc::c_int = -(1 as libc::c_int);
            if mc >= MC_1_7 as libc::c_int {
                bn = (b11 - 2 as libc::c_int) % 29 as libc::c_int;
            }
            if bn == 1 as libc::c_int && b11 >= 2 as libc::c_int
                && isShallowOcean(a11) == 0
            {
                let mut m: libc::c_int = getMutated(mc, a11);
                if m > 0 as libc::c_int {
                    *out.offset(idx as isize) = m;
                } else {
                    *out.offset(idx as isize) = a11;
                }
            } else {
                cs = getChunkSeed(ss, i + x, j + z);
                if bn == 0 as libc::c_int || mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                    let mut hillID: libc::c_int = a11;
                    match a11 {
                        2 => {
                            hillID = desert_hills as libc::c_int;
                        }
                        4 => {
                            hillID = wooded_hills as libc::c_int;
                        }
                        27 => {
                            hillID = birch_forest_hills as libc::c_int;
                        }
                        29 => {
                            hillID = plains as libc::c_int;
                        }
                        5 => {
                            hillID = taiga_hills as libc::c_int;
                        }
                        32 => {
                            hillID = giant_tree_taiga_hills as libc::c_int;
                        }
                        30 => {
                            hillID = snowy_taiga_hills as libc::c_int;
                        }
                        1 => {
                            if mc <= MC_1_6 as libc::c_int {
                                hillID = forest as libc::c_int;
                            } else {
                                cs = mcStepSeed(cs, st);
                                hillID = if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                    wooded_hills as libc::c_int
                                } else {
                                    forest as libc::c_int
                                };
                            }
                        }
                        12 => {
                            hillID = snowy_mountains as libc::c_int;
                        }
                        21 => {
                            hillID = jungle_hills as libc::c_int;
                        }
                        168 => {
                            hillID = bamboo_jungle_hills as libc::c_int;
                        }
                        0 => {
                            if mc >= MC_1_7 as libc::c_int {
                                hillID = deep_ocean as libc::c_int;
                            }
                        }
                        3 => {
                            if mc >= MC_1_7 as libc::c_int {
                                hillID = wooded_mountains as libc::c_int;
                            }
                        }
                        35 => {
                            hillID = savanna_plateau as libc::c_int;
                        }
                        _ => {
                            if areSimilar(
                                mc,
                                a11,
                                wooded_badlands_plateau as libc::c_int,
                            ) != 0
                            {
                                hillID = badlands as libc::c_int;
                            } else if isDeepOcean(a11) != 0 {
                                cs = mcStepSeed(cs, st);
                                if mcFirstIsZero(cs, 3 as libc::c_int) != 0 {
                                    cs = mcStepSeed(cs, st);
                                    hillID = if mcFirstIsZero(cs, 2 as libc::c_int) != 0 {
                                        plains as libc::c_int
                                    } else {
                                        forest as libc::c_int
                                    };
                                }
                            }
                        }
                    }
                    if bn == 0 as libc::c_int && hillID != a11 {
                        hillID = getMutated(mc, hillID);
                        if hillID < 0 as libc::c_int {
                            hillID = a11;
                        }
                    }
                    if hillID != a11 {
                        let mut a10: libc::c_int = *out
                            .offset(
                                (i + 1 as libc::c_int + (j + 0 as libc::c_int) * pW)
                                    as isize,
                            );
                        let mut a21: libc::c_int = *out
                            .offset(
                                (i + 2 as libc::c_int + (j + 1 as libc::c_int) * pW)
                                    as isize,
                            );
                        let mut a01: libc::c_int = *out
                            .offset(
                                (i + 0 as libc::c_int + (j + 1 as libc::c_int) * pW)
                                    as isize,
                            );
                        let mut a12: libc::c_int = *out
                            .offset(
                                (i + 1 as libc::c_int + (j + 2 as libc::c_int) * pW)
                                    as isize,
                            );
                        let mut equals: libc::c_int = 0 as libc::c_int;
                        if areSimilar(mc, a10, a11) != 0 {
                            equals += 1;
                        }
                        if areSimilar(mc, a21, a11) != 0 {
                            equals += 1;
                        }
                        if areSimilar(mc, a01, a11) != 0 {
                            equals += 1;
                        }
                        if areSimilar(mc, a12, a11) != 0 {
                            equals += 1;
                        }
                        if equals
                            >= 3 as libc::c_int
                                + (mc <= MC_1_6 as libc::c_int) as libc::c_int
                        {
                            *out.offset(idx as isize) = hillID;
                        } else {
                            *out.offset(idx as isize) = a11;
                        }
                    } else {
                        *out.offset(idx as isize) = a11;
                    }
                } else {
                    *out.offset(idx as isize) = a11;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn reduceID(mut id: libc::c_int) -> libc::c_int {
    return if id >= 2 as libc::c_int {
        2 as libc::c_int + (id & 1 as libc::c_int)
    } else {
        id
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapRiver(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        let mut current_block_15: u64;
        i = 0 as libc::c_int;
        while i < w {
            let mut v01: libc::c_int = *vz1.offset((i + 0 as libc::c_int) as isize);
            let mut v11: libc::c_int = *vz1.offset((i + 1 as libc::c_int) as isize);
            let mut v21: libc::c_int = *vz1.offset((i + 2 as libc::c_int) as isize);
            let mut v10: libc::c_int = *vz0.offset((i + 1 as libc::c_int) as isize);
            let mut v12: libc::c_int = *vz2.offset((i + 1 as libc::c_int) as isize);
            if mc >= MC_1_7 as libc::c_int {
                v01 = reduceID(v01);
                v11 = reduceID(v11);
                v21 = reduceID(v21);
                v10 = reduceID(v10);
                v12 = reduceID(v12);
                current_block_15 = 1109700713171191020;
            } else if v11 == 0 as libc::c_int {
                *out.offset((i + j * w) as isize) = river as libc::c_int;
                current_block_15 = 13109137661213826276;
            } else {
                current_block_15 = 1109700713171191020;
            }
            match current_block_15 {
                1109700713171191020 => {
                    if v11 == v01 && v11 == v10 && v11 == v12 && v11 == v21 {
                        *out.offset((i + j * w) as isize) = -(1 as libc::c_int);
                    } else {
                        *out.offset((i + j * w) as isize) = river as libc::c_int;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSmooth(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *vz1.offset((i + 1 as libc::c_int) as isize);
            let mut v01: libc::c_int = *vz1.offset((i + 0 as libc::c_int) as isize);
            let mut v10: libc::c_int = *vz0.offset((i + 1 as libc::c_int) as isize);
            if v11 != v01 || v11 != v10 {
                let mut v21: libc::c_int = *vz1.offset((i + 2 as libc::c_int) as isize);
                let mut v12: libc::c_int = *vz2.offset((i + 1 as libc::c_int) as isize);
                if v01 == v21 && v10 == v12 {
                    cs = getChunkSeed(ss, i + x, j + z);
                    if cs & (1 as libc::c_int as uint64_t) << 24 as libc::c_int != 0 {
                        v11 = v10;
                    } else {
                        v11 = v01;
                    }
                } else {
                    if v01 == v21 {
                        v11 = v01;
                    }
                    if v10 == v12 {
                        v11 = v10;
                    }
                }
            }
            *out.offset((i + j * w) as isize) = v11;
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSunflower(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v: libc::c_int = *out.offset((i + j * w) as isize);
            if v == plains as libc::c_int {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(cs, 57 as libc::c_int) != 0 {
                    *out.offset((i + j * w) as isize) = sunflower_plains as libc::c_int;
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn replaceOcean(
    mut out: *mut libc::c_int,
    mut idx: libc::c_int,
    mut v10: libc::c_int,
    mut v21: libc::c_int,
    mut v01: libc::c_int,
    mut v12: libc::c_int,
    mut id: libc::c_int,
    mut replaceID: libc::c_int,
) -> libc::c_int {
    if isOceanic(id) != 0 {
        return 0 as libc::c_int;
    }
    if isOceanic(v10) != 0 || isOceanic(v21) != 0 || isOceanic(v01) != 0
        || isOceanic(v12) != 0
    {
        *out.offset(idx as isize) = replaceID;
    } else {
        *out.offset(idx as isize) = id;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn isAll4JFTO(
    mut mc: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    return ((getCategory(mc, a) == jungle as libc::c_int || a == forest as libc::c_int
        || a == taiga as libc::c_int || isOceanic(a) != 0)
        && (getCategory(mc, b) == jungle as libc::c_int || b == forest as libc::c_int
            || b == taiga as libc::c_int || isOceanic(b) != 0)
        && (getCategory(mc, c) == jungle as libc::c_int || c == forest as libc::c_int
            || c == taiga as libc::c_int || isOceanic(c) != 0)
        && (getCategory(mc, d) == jungle as libc::c_int || d == forest as libc::c_int
            || d == taiga as libc::c_int || isOceanic(d) != 0)) as libc::c_int;
}
#[inline]
unsafe extern "C" fn isAny4Oceanic(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    return (isOceanic(a) != 0 || isOceanic(b) != 0 || isOceanic(c) != 0
        || isOceanic(d) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapShore(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut pX: libc::c_int = x - 1 as libc::c_int;
    let mut pZ: libc::c_int = z - 1 as libc::c_int;
    let mut pW: libc::c_int = w + 2 as libc::c_int;
    let mut pH: libc::c_int = h + 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    j = 0 as libc::c_int;
    while j < h {
        let mut vz0: *mut libc::c_int = out
            .offset(((j + 0 as libc::c_int) * pW) as isize);
        let mut vz1: *mut libc::c_int = out
            .offset(((j + 1 as libc::c_int) * pW) as isize);
        let mut vz2: *mut libc::c_int = out
            .offset(((j + 2 as libc::c_int) * pW) as isize);
        i = 0 as libc::c_int;
        while i < w {
            let mut v11: libc::c_int = *vz1.offset((i + 1 as libc::c_int) as isize);
            let mut v10: libc::c_int = *vz0.offset((i + 1 as libc::c_int) as isize);
            let mut v21: libc::c_int = *vz1.offset((i + 2 as libc::c_int) as isize);
            let mut v01: libc::c_int = *vz1.offset((i + 0 as libc::c_int) as isize);
            let mut v12: libc::c_int = *vz2.offset((i + 1 as libc::c_int) as isize);
            if v11 == mushroom_fields as libc::c_int {
                if isAny4(ocean as libc::c_int, v10, v21, v01, v12) != 0 {
                    *out
                        .offset(
                            (i + j * w) as isize,
                        ) = mushroom_field_shore as libc::c_int;
                } else {
                    *out.offset((i + j * w) as isize) = v11;
                }
            } else if mc <= MC_1_0 as libc::c_int {
                *out.offset((i + j * w) as isize) = v11;
            } else if mc <= MC_1_6 as libc::c_int {
                if v11 == mountains as libc::c_int {
                    if v10 != mountains as libc::c_int || v21 != mountains as libc::c_int
                        || v01 != mountains as libc::c_int
                        || v12 != mountains as libc::c_int
                    {
                        v11 = mountain_edge as libc::c_int;
                    }
                } else if v11 != ocean as libc::c_int && v11 != river as libc::c_int
                        && v11 != swamp as libc::c_int
                    {
                    if isAny4(ocean as libc::c_int, v10, v21, v01, v12) != 0 {
                        v11 = beach as libc::c_int;
                    }
                }
                *out.offset((i + j * w) as isize) = v11;
            } else if getCategory(mc, v11) == jungle as libc::c_int {
                if isAll4JFTO(mc, v10, v21, v01, v12) != 0 {
                    if isAny4Oceanic(v10, v21, v01, v12) != 0 {
                        *out.offset((i + j * w) as isize) = beach as libc::c_int;
                    } else {
                        *out.offset((i + j * w) as isize) = v11;
                    }
                } else {
                    *out.offset((i + j * w) as isize) = jungle_edge as libc::c_int;
                }
            } else if v11 == mountains as libc::c_int
                    || v11 == wooded_mountains as libc::c_int
                {
                replaceOcean(
                    out,
                    i + j * w,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    stone_shore as libc::c_int,
                );
            } else if isSnowy(v11) != 0 {
                replaceOcean(
                    out,
                    i + j * w,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    snowy_beach as libc::c_int,
                );
            } else if v11 == badlands as libc::c_int
                    || v11 == wooded_badlands_plateau as libc::c_int
                {
                if isAny4Oceanic(v10, v21, v01, v12) == 0 {
                    if isMesa(v10) != 0 && isMesa(v21) != 0 && isMesa(v01) != 0
                        && isMesa(v12) != 0
                    {
                        *out.offset((i + j * w) as isize) = v11;
                    } else {
                        *out.offset((i + j * w) as isize) = desert as libc::c_int;
                    }
                } else {
                    *out.offset((i + j * w) as isize) = v11;
                }
            } else if v11 != ocean as libc::c_int && v11 != deep_ocean as libc::c_int
                    && v11 != river as libc::c_int && v11 != swamp as libc::c_int
                {
                if isAny4Oceanic(v10, v21, v01, v12) != 0 {
                    *out.offset((i + j * w) as isize) = beach as libc::c_int;
                } else {
                    *out.offset((i + j * w) as isize) = v11;
                }
            } else {
                *out.offset((i + j * w) as isize) = v11;
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapSwampRiver(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut v: libc::c_int = *out.offset((i + j * w) as isize);
            if !(v != swamp as libc::c_int && v != jungle as libc::c_int
                && v != jungle_hills as libc::c_int)
            {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(
                    cs,
                    if v == swamp as libc::c_int {
                        6 as libc::c_int
                    } else {
                        8 as libc::c_int
                    },
                ) != 0
                {
                    v = river as libc::c_int;
                }
                *out.offset((i + j * w) as isize) = v;
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapRiverMix(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    if ((*l).p2 == 0 as *mut libc::c_void as *mut Layer) as libc::c_int as libc::c_long
        != 0
    {
        printf(
            b"mapRiverMix() requires two parents! Use setupMultiLayer()\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut err: libc::c_int = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut idx: libc::c_int = 0;
    let mut mc: libc::c_int = (*l).mc as libc::c_int;
    let mut len: libc::c_int = w * h;
    let mut buf: *mut libc::c_int = out.offset(len as isize);
    err = ((*(*l).p2).getMap)
        .expect("non-null function pointer")((*l).p2, buf, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    idx = 0 as libc::c_int;
    while idx < len {
        let mut v: libc::c_int = *out.offset(idx as isize);
        if *buf.offset(idx as isize) == river as libc::c_int && v != ocean as libc::c_int
            && (mc < MC_1_7 as libc::c_int || isOceanic(v) == 0)
        {
            if v == snowy_tundra as libc::c_int {
                v = frozen_river as libc::c_int;
            } else if v == mushroom_fields as libc::c_int
                    || v == mushroom_field_shore as libc::c_int
                {
                v = mushroom_field_shore as libc::c_int;
            } else {
                v = river as libc::c_int;
            }
        }
        *out.offset(idx as isize) = v;
        idx += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapOceanTemp(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rnd: *const PerlinNoise = (*l).noise as *const PerlinNoise;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut tmp: libc::c_double = samplePerlin(
                rnd,
                (i + x) as libc::c_double / 8.0f64,
                (j + z) as libc::c_double / 8.0f64,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
                0 as libc::c_int as libc::c_double,
            );
            if tmp > 0.4f64 {
                *out.offset((i + j * w) as isize) = warm_ocean as libc::c_int;
            } else if tmp > 0.2f64 {
                *out.offset((i + j * w) as isize) = lukewarm_ocean as libc::c_int;
            } else if tmp < -0.4f64 {
                *out.offset((i + j * w) as isize) = frozen_ocean as libc::c_int;
            } else if tmp < -0.2f64 {
                *out.offset((i + j * w) as isize) = cold_ocean as libc::c_int;
            } else {
                *out.offset((i + j * w) as isize) = ocean as libc::c_int;
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapOceanMix(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lx0: libc::c_int = 0;
    let mut lx1: libc::c_int = 0;
    let mut lz0: libc::c_int = 0;
    let mut lz1: libc::c_int = 0;
    let mut lw: libc::c_int = 0;
    let mut lh: libc::c_int = 0;
    if ((*l).p2 == 0 as *mut libc::c_void as *mut Layer) as libc::c_int as libc::c_long
        != 0
    {
        printf(
            b"mapOceanMix() requires two parents! Use setupMultiLayer()\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut err: libc::c_int = ((*(*l).p2).getMap)
        .expect("non-null function pointer")((*l).p2, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    lx0 = 0 as libc::c_int;
    lx1 = w;
    lz0 = 0 as libc::c_int;
    lz1 = h;
    j = 0 as libc::c_int;
    while j < h {
        let mut jcentre: libc::c_int = (j - 8 as libc::c_int > 0 as libc::c_int
            && (j + 9 as libc::c_int) < h) as libc::c_int;
        i = 0 as libc::c_int;
        while i < w {
            if !(jcentre != 0 && i - 8 as libc::c_int > 0 as libc::c_int
                && (i + 9 as libc::c_int) < w)
            {
                let mut oceanID: libc::c_int = *out.offset((i + j * w) as isize);
                if oceanID == warm_ocean as libc::c_int
                    || oceanID == frozen_ocean as libc::c_int
                {
                    if (i - 8 as libc::c_int) < lx0 {
                        lx0 = i - 8 as libc::c_int;
                    }
                    if i + 9 as libc::c_int > lx1 {
                        lx1 = i + 9 as libc::c_int;
                    }
                    if (j - 8 as libc::c_int) < lz0 {
                        lz0 = j - 8 as libc::c_int;
                    }
                    if j + 9 as libc::c_int > lz1 {
                        lz1 = j + 9 as libc::c_int;
                    }
                }
            }
            i += 1;
        }
        j += 1;
    }
    let mut land: *mut libc::c_int = out.offset((w * h) as isize);
    lw = lx1 - lx0;
    lh = lz1 - lz0;
    err = ((*(*l).p).getMap)
        .expect("non-null function pointer")((*l).p, land, x + lx0, z + lz0, lw, lh);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    j = 0 as libc::c_int;
    while j < h {
        let mut current_block_48: u64;
        i = 0 as libc::c_int;
        while i < w {
            let mut landID: libc::c_int = *land
                .offset((i - lx0 + (j - lz0) * lw) as isize);
            let mut oceanID_0: libc::c_int = *out.offset((i + j * w) as isize);
            let mut replaceID: libc::c_int = 0 as libc::c_int;
            let mut ii: libc::c_int = 0;
            let mut jj: libc::c_int = 0;
            if isOceanic(landID) == 0 {
                *out.offset((i + j * w) as isize) = landID;
            } else {
                if oceanID_0 == warm_ocean as libc::c_int {
                    replaceID = lukewarm_ocean as libc::c_int;
                }
                if oceanID_0 == frozen_ocean as libc::c_int {
                    replaceID = cold_ocean as libc::c_int;
                }
                if replaceID != 0 {
                    ii = -(8 as libc::c_int);
                    's_231: loop {
                        if !(ii <= 8 as libc::c_int) {
                            current_block_48 = 9353995356876505083;
                            break;
                        }
                        jj = -(8 as libc::c_int);
                        while jj <= 8 as libc::c_int {
                            let mut id: libc::c_int = *land
                                .offset((i + ii - lx0 + (j + jj - lz0) * lw) as isize);
                            if isOceanic(id) == 0 {
                                *out.offset((i + j * w) as isize) = replaceID;
                                current_block_48 = 7333393191927787629;
                                break 's_231;
                            } else {
                                jj += 4 as libc::c_int;
                            }
                        }
                        ii += 4 as libc::c_int;
                    }
                } else {
                    current_block_48 = 9353995356876505083;
                }
                match current_block_48 {
                    7333393191927787629 => {}
                    _ => {
                        if landID == deep_ocean as libc::c_int {
                            match oceanID_0 {
                                45 => {
                                    oceanID_0 = deep_lukewarm_ocean as libc::c_int;
                                }
                                0 => {
                                    oceanID_0 = deep_ocean as libc::c_int;
                                }
                                46 => {
                                    oceanID_0 = deep_cold_ocean as libc::c_int;
                                }
                                10 => {
                                    oceanID_0 = deep_frozen_ocean as libc::c_int;
                                }
                                _ => {}
                            }
                        }
                        *out.offset((i + j * w) as isize) = oceanID_0;
                    }
                }
            }
            i += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getVoronoiSrcRange(mut r: Range) -> Range {
    if r.scale != 1 as libc::c_int {
        printf(
            b"getVoronoiSrcRange() expects input range with scale 1:1\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut s: Range = Range {
        scale: 0,
        x: 0,
        z: 0,
        sx: 0,
        sz: 0,
        y: 0,
        sy: 0,
    };
    let mut tx: libc::c_int = r.x - 2 as libc::c_int;
    let mut tz: libc::c_int = r.z - 2 as libc::c_int;
    s.scale = 4 as libc::c_int;
    s.x = tx >> 2 as libc::c_int;
    s.z = tz >> 2 as libc::c_int;
    s.sx = (tx + r.sx >> 2 as libc::c_int) - s.x + 2 as libc::c_int;
    s.sz = (tz + r.sz >> 2 as libc::c_int) - s.z + 2 as libc::c_int;
    if r.sy < 1 as libc::c_int {
        s.sy = 0 as libc::c_int;
        s.y = s.sy;
    } else {
        let mut ty: libc::c_int = r.y - 2 as libc::c_int;
        s.y = ty >> 2 as libc::c_int;
        s.sy = (ty + r.sy >> 2 as libc::c_int) - s.y + 2 as libc::c_int;
    }
    return s;
}
#[inline]
unsafe extern "C" fn getVoronoiCell(
    mut sha: uint64_t,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut z: *mut libc::c_int,
) {
    let mut s: uint64_t = sha;
    s = mcStepSeed(s, a as uint64_t);
    s = mcStepSeed(s, b as uint64_t);
    s = mcStepSeed(s, c as uint64_t);
    s = mcStepSeed(s, a as uint64_t);
    s = mcStepSeed(s, b as uint64_t);
    s = mcStepSeed(s, c as uint64_t);
    *x = (s >> 24 as libc::c_int & 1023 as libc::c_int as libc::c_ulong)
        .wrapping_sub(512 as libc::c_int as libc::c_ulong)
        .wrapping_mul(36 as libc::c_int as libc::c_ulong) as libc::c_int;
    s = mcStepSeed(s, sha);
    *y = (s >> 24 as libc::c_int & 1023 as libc::c_int as libc::c_ulong)
        .wrapping_sub(512 as libc::c_int as libc::c_ulong)
        .wrapping_mul(36 as libc::c_int as libc::c_ulong) as libc::c_int;
    s = mcStepSeed(s, sha);
    *z = (s >> 24 as libc::c_int & 1023 as libc::c_int as libc::c_ulong)
        .wrapping_sub(512 as libc::c_int as libc::c_ulong)
        .wrapping_mul(36 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapVoronoiPlane(
    mut sha: uint64_t,
    mut out: *mut libc::c_int,
    mut src: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut y: libc::c_int,
    mut px: libc::c_int,
    mut pz: libc::c_int,
    mut pw: libc::c_int,
    mut ph: libc::c_int,
) {
    let mut x000: libc::c_int = 0;
    let mut x001: libc::c_int = 0;
    let mut x010: libc::c_int = 0;
    let mut x011: libc::c_int = 0;
    let mut x100: libc::c_int = 0;
    let mut x101: libc::c_int = 0;
    let mut x110: libc::c_int = 0;
    let mut x111: libc::c_int = 0;
    let mut y000: libc::c_int = 0;
    let mut y001: libc::c_int = 0;
    let mut y010: libc::c_int = 0;
    let mut y011: libc::c_int = 0;
    let mut y100: libc::c_int = 0;
    let mut y101: libc::c_int = 0;
    let mut y110: libc::c_int = 0;
    let mut y111: libc::c_int = 0;
    let mut z000: libc::c_int = 0;
    let mut z001: libc::c_int = 0;
    let mut z010: libc::c_int = 0;
    let mut z011: libc::c_int = 0;
    let mut z100: libc::c_int = 0;
    let mut z101: libc::c_int = 0;
    let mut z110: libc::c_int = 0;
    let mut z111: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut pj: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut dx: libc::c_int = 0;
    let mut dz: libc::c_int = 0;
    let mut pjz: libc::c_int = 0;
    let mut pix: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut j4: libc::c_int = 0;
    let mut v00: libc::c_int = 0;
    let mut v01: libc::c_int = 0;
    let mut v10: libc::c_int = 0;
    let mut v11: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut prev_skip: libc::c_int = 0;
    let mut r: int64_t = 0;
    let mut d: uint64_t = 0;
    let mut dmin: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    pj = 0 as libc::c_int;
    while pj < ph - 1 as libc::c_int {
        v00 = *src.offset(((pj + 0 as libc::c_int) * pw) as isize);
        v10 = *src.offset(((pj + 1 as libc::c_int) * pw) as isize);
        pjz = pz + pj;
        j4 = (pjz << 2 as libc::c_int) - z;
        prev_skip = 1 as libc::c_int;
        pi = 0 as libc::c_int;
        while pi < pw - 1 as libc::c_int {
            v01 = *src
                .offset(
                    ((pj + 0 as libc::c_int) * pw + (pi + 1 as libc::c_int)) as isize,
                );
            v11 = *src
                .offset(
                    ((pj + 1 as libc::c_int) * pw + (pi + 1 as libc::c_int)) as isize,
                );
            pix = px + pi;
            i4 = (pix << 2 as libc::c_int) - x;
            if v00 == v01 && v00 == v10 && v00 == v11 {
                jj = 0 as libc::c_int;
                while jj < 4 as libc::c_int {
                    j = j4 + jj;
                    if !(j < 0 as libc::c_int || j >= h) {
                        ii = 0 as libc::c_int;
                        while ii < 4 as libc::c_int {
                            i = i4 + ii;
                            if !(i < 0 as libc::c_int || i >= w) {
                                *out.offset((j * w + i) as isize) = v00;
                            }
                            ii += 1;
                        }
                    }
                    jj += 1;
                }
                prev_skip = 1 as libc::c_int;
            } else {
                if prev_skip != 0 {
                    getVoronoiCell(
                        sha,
                        pix,
                        y - 1 as libc::c_int,
                        pjz + 0 as libc::c_int,
                        &mut x000,
                        &mut y000,
                        &mut z000,
                    );
                    getVoronoiCell(
                        sha,
                        pix,
                        y + 0 as libc::c_int,
                        pjz + 0 as libc::c_int,
                        &mut x001,
                        &mut y001,
                        &mut z001,
                    );
                    getVoronoiCell(
                        sha,
                        pix,
                        y - 1 as libc::c_int,
                        pjz + 1 as libc::c_int,
                        &mut x100,
                        &mut y100,
                        &mut z100,
                    );
                    getVoronoiCell(
                        sha,
                        pix,
                        y + 0 as libc::c_int,
                        pjz + 1 as libc::c_int,
                        &mut x101,
                        &mut y101,
                        &mut z101,
                    );
                    prev_skip = 0 as libc::c_int;
                }
                getVoronoiCell(
                    sha,
                    pix + 1 as libc::c_int,
                    y - 1 as libc::c_int,
                    pjz + 0 as libc::c_int,
                    &mut x010,
                    &mut y010,
                    &mut z010,
                );
                getVoronoiCell(
                    sha,
                    pix + 1 as libc::c_int,
                    y + 0 as libc::c_int,
                    pjz + 0 as libc::c_int,
                    &mut x011,
                    &mut y011,
                    &mut z011,
                );
                getVoronoiCell(
                    sha,
                    pix + 1 as libc::c_int,
                    y - 1 as libc::c_int,
                    pjz + 1 as libc::c_int,
                    &mut x110,
                    &mut y110,
                    &mut z110,
                );
                getVoronoiCell(
                    sha,
                    pix + 1 as libc::c_int,
                    y + 0 as libc::c_int,
                    pjz + 1 as libc::c_int,
                    &mut x111,
                    &mut y111,
                    &mut z111,
                );
                jj = 0 as libc::c_int;
                while jj < 4 as libc::c_int {
                    j = j4 + jj;
                    if !(j < 0 as libc::c_int || j >= h) {
                        ii = 0 as libc::c_int;
                        while ii < 4 as libc::c_int {
                            i = i4 + ii;
                            if !(i < 0 as libc::c_int || i >= w) {
                                let A: libc::c_int = 40 as libc::c_int
                                    * 1024 as libc::c_int;
                                let B: libc::c_int = 20 as libc::c_int
                                    * 1024 as libc::c_int;
                                dx = ii * 10 as libc::c_int * 1024 as libc::c_int;
                                dz = jj * 10 as libc::c_int * 1024 as libc::c_int;
                                dmin = -(1 as libc::c_int) as uint64_t;
                                v = v00;
                                d = 0 as libc::c_int as uint64_t;
                                r = (x000 - 0 as libc::c_int + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y000 + B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z000 - 0 as libc::c_int + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x001 - 0 as libc::c_int + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y001 - B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z001 - 0 as libc::c_int + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x010 - A + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y010 + B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z010 - 0 as libc::c_int + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v01;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x011 - A + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y011 - B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z011 - 0 as libc::c_int + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v01;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x100 - 0 as libc::c_int + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y100 + B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z100 - A + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v10;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x101 - 0 as libc::c_int + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y101 - B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z101 - A + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v10;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x110 - A + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y110 + B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z110 - A + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v11;
                                }
                                d = 0 as libc::c_int as uint64_t;
                                r = (x111 - A + dx) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (y111 - B) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                r = (z111 - A + dz) as int64_t;
                                d = (d as libc::c_ulong)
                                    .wrapping_add((r * r) as libc::c_ulong) as uint64_t
                                    as uint64_t;
                                if d < dmin {
                                    dmin = d;
                                    v = v11;
                                }
                                *out.offset((j * w + i) as isize) = v;
                            }
                            ii += 1;
                        }
                    }
                    jj += 1;
                }
                x000 = x010;
                y000 = y010;
                z000 = z010;
                x100 = x110;
                y100 = y110;
                z100 = z110;
                x001 = x011;
                y001 = y011;
                z001 = z011;
                x101 = x111;
                y101 = y111;
                z101 = z111;
                v00 = v01;
                v10 = v11;
            }
            pi += 1;
        }
        pj += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapVoronoi(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    x -= 2 as libc::c_int;
    z -= 2 as libc::c_int;
    let mut px: libc::c_int = x >> 2 as libc::c_int;
    let mut pz: libc::c_int = z >> 2 as libc::c_int;
    let mut pw: libc::c_int = (x + w >> 2 as libc::c_int) - px + 2 as libc::c_int;
    let mut ph: libc::c_int = (z + h >> 2 as libc::c_int) - pz + 2 as libc::c_int;
    if !((*l).p).is_null() {
        let mut err: libc::c_int = ((*(*l).p).getMap)
            .expect("non-null function pointer")((*l).p, out, px, pz, pw, ph);
        if err != 0 as libc::c_int {
            return err;
        }
    }
    let mut src: *mut libc::c_int = out.offset((w * h) as isize);
    memmove(
        src as *mut libc::c_void,
        out as *const libc::c_void,
        ((pw * ph) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    mapVoronoiPlane(
        (*l).startSalt,
        out,
        src,
        x,
        z,
        w,
        h,
        0 as libc::c_int,
        px,
        pz,
        pw,
        ph,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapVoronoi114(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    x -= 2 as libc::c_int;
    z -= 2 as libc::c_int;
    let mut pX: libc::c_int = x >> 2 as libc::c_int;
    let mut pZ: libc::c_int = z >> 2 as libc::c_int;
    let mut pW: libc::c_int = (x + w >> 2 as libc::c_int) - pX + 2 as libc::c_int;
    let mut pH: libc::c_int = (z + h >> 2 as libc::c_int) - pZ + 2 as libc::c_int;
    if !((*l).p).is_null() {
        let mut err: libc::c_int = ((*(*l).p).getMap)
            .expect("non-null function pointer")((*l).p, out, pX, pZ, pW, pH);
        if err != 0 as libc::c_int {
            return err;
        }
    }
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut pj: libc::c_int = 0;
    let mut pix: libc::c_int = 0;
    let mut pjz: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut j4: libc::c_int = 0;
    let mut mi: libc::c_int = 0;
    let mut mj: libc::c_int = 0;
    let mut v00: libc::c_int = 0;
    let mut v01: libc::c_int = 0;
    let mut v10: libc::c_int = 0;
    let mut v11: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut da1: int64_t = 0;
    let mut da2: int64_t = 0;
    let mut db1: int64_t = 0;
    let mut db2: int64_t = 0;
    let mut dc1: int64_t = 0;
    let mut dc2: int64_t = 0;
    let mut dd1: int64_t = 0;
    let mut dd2: int64_t = 0;
    let mut sja: int64_t = 0;
    let mut sjb: int64_t = 0;
    let mut sjc: int64_t = 0;
    let mut sjd: int64_t = 0;
    let mut da: int64_t = 0;
    let mut db: int64_t = 0;
    let mut dc: int64_t = 0;
    let mut dd: int64_t = 0;
    let mut buf: *mut libc::c_int = out.offset((pW * pH) as isize);
    let mut st: uint64_t = (*l).startSalt;
    let mut ss: uint64_t = (*l).startSeed;
    let mut cs: uint64_t = 0;
    pj = 0 as libc::c_int;
    while pj < pH - 1 as libc::c_int {
        v00 = *out.offset(((pj + 0 as libc::c_int) * pW) as isize);
        v01 = *out.offset(((pj + 1 as libc::c_int) * pW) as isize);
        pjz = pZ + pj;
        j4 = (pjz << 2 as libc::c_int) - z;
        pi = 0 as libc::c_int;
        while pi < pW - 1 as libc::c_int {
            pix = pX + pi;
            i4 = (pix << 2 as libc::c_int) - x;
            v10 = *out
                .offset((pi + 1 as libc::c_int + (pj + 0 as libc::c_int) * pW) as isize);
            v11 = *out
                .offset((pi + 1 as libc::c_int + (pj + 1 as libc::c_int) * pW) as isize);
            if v00 == v01 && v00 == v10 && v00 == v11 {
                jj = 0 as libc::c_int;
                while jj < 4 as libc::c_int {
                    j = j4 + jj;
                    if !(j < 0 as libc::c_int || j >= h) {
                        ii = 0 as libc::c_int;
                        while ii < 4 as libc::c_int {
                            i = i4 + ii;
                            if !(i < 0 as libc::c_int || i >= w) {
                                *buf.offset((j * w + i) as isize) = v00;
                            }
                            ii += 1;
                        }
                    }
                    jj += 1;
                }
            } else {
                cs = getChunkSeed(
                    ss,
                    pi + pX << 2 as libc::c_int,
                    pj + pZ << 2 as libc::c_int,
                );
                da1 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int) as int64_t;
                cs = mcStepSeed(cs, st);
                da2 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int) as int64_t;
                cs = getChunkSeed(
                    ss,
                    (pi + pX + 1 as libc::c_int) << 2 as libc::c_int,
                    pj + pZ << 2 as libc::c_int,
                );
                db1 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int + 40 as libc::c_int * 1024 as libc::c_int)
                    as int64_t;
                cs = mcStepSeed(cs, st);
                db2 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int) as int64_t;
                cs = getChunkSeed(
                    ss,
                    pi + pX << 2 as libc::c_int,
                    (pj + pZ + 1 as libc::c_int) << 2 as libc::c_int,
                );
                dc1 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int) as int64_t;
                cs = mcStepSeed(cs, st);
                dc2 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int + 40 as libc::c_int * 1024 as libc::c_int)
                    as int64_t;
                cs = getChunkSeed(
                    ss,
                    (pi + pX + 1 as libc::c_int) << 2 as libc::c_int,
                    (pj + pZ + 1 as libc::c_int) << 2 as libc::c_int,
                );
                dd1 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int + 40 as libc::c_int * 1024 as libc::c_int)
                    as int64_t;
                cs = mcStepSeed(cs, st);
                dd2 = ((mcFirstInt(cs, 1024 as libc::c_int) - 512 as libc::c_int)
                    * 36 as libc::c_int + 40 as libc::c_int * 1024 as libc::c_int)
                    as int64_t;
                jj = 0 as libc::c_int;
                while jj < 4 as libc::c_int {
                    j = j4 + jj;
                    if !(j < 0 as libc::c_int || j >= h) {
                        mj = 10240 as libc::c_int * jj;
                        sja = (mj as libc::c_long - da2) * (mj as libc::c_long - da2);
                        sjb = (mj as libc::c_long - db2) * (mj as libc::c_long - db2);
                        sjc = (mj as libc::c_long - dc2) * (mj as libc::c_long - dc2);
                        sjd = (mj as libc::c_long - dd2) * (mj as libc::c_long - dd2);
                        ii = 0 as libc::c_int;
                        while ii < 4 as libc::c_int {
                            i = i4 + ii;
                            if !(i < 0 as libc::c_int || i >= w) {
                                mi = 10240 as libc::c_int * ii;
                                da = (mi as libc::c_long - da1) * (mi as libc::c_long - da1)
                                    + sja;
                                db = (mi as libc::c_long - db1) * (mi as libc::c_long - db1)
                                    + sjb;
                                dc = (mi as libc::c_long - dc1) * (mi as libc::c_long - dc1)
                                    + sjc;
                                dd = (mi as libc::c_long - dd1) * (mi as libc::c_long - dd1)
                                    + sjd;
                                if (da < db && da < dc && da < dd) as libc::c_int
                                    as libc::c_long != 0
                                {
                                    v = v00;
                                } else if (db < da && db < dc && db < dd) as libc::c_int
                                        as libc::c_long != 0
                                    {
                                    v = v10;
                                } else if (dc < da && dc < db && dc < dd) as libc::c_int
                                        as libc::c_long != 0
                                    {
                                    v = v01;
                                } else {
                                    v = v11;
                                }
                                *buf.offset((j * w + i) as isize) = v;
                            }
                            ii += 1;
                        }
                    }
                    jj += 1;
                }
            }
            pi += 1;
            v00 = v10;
            v01 = v11;
        }
        pj += 1;
    }
    memmove(
        out as *mut libc::c_void,
        buf as *const libc::c_void,
        ((w * h) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getVoronoiSHA(mut seed: uint64_t) -> uint64_t {
    static mut K: [uint32_t; 64] = [
        0x428a2f98 as libc::c_int as uint32_t,
        0x71374491 as libc::c_int as uint32_t,
        0xb5c0fbcf as libc::c_uint,
        0xe9b5dba5 as libc::c_uint,
        0x3956c25b as libc::c_int as uint32_t,
        0x59f111f1 as libc::c_int as uint32_t,
        0x923f82a4 as libc::c_uint,
        0xab1c5ed5 as libc::c_uint,
        0xd807aa98 as libc::c_uint,
        0x12835b01 as libc::c_int as uint32_t,
        0x243185be as libc::c_int as uint32_t,
        0x550c7dc3 as libc::c_int as uint32_t,
        0x72be5d74 as libc::c_int as uint32_t,
        0x80deb1fe as libc::c_uint,
        0x9bdc06a7 as libc::c_uint,
        0xc19bf174 as libc::c_uint,
        0xe49b69c1 as libc::c_uint,
        0xefbe4786 as libc::c_uint,
        0xfc19dc6 as libc::c_int as uint32_t,
        0x240ca1cc as libc::c_int as uint32_t,
        0x2de92c6f as libc::c_int as uint32_t,
        0x4a7484aa as libc::c_int as uint32_t,
        0x5cb0a9dc as libc::c_int as uint32_t,
        0x76f988da as libc::c_int as uint32_t,
        0x983e5152 as libc::c_uint,
        0xa831c66d as libc::c_uint,
        0xb00327c8 as libc::c_uint,
        0xbf597fc7 as libc::c_uint,
        0xc6e00bf3 as libc::c_uint,
        0xd5a79147 as libc::c_uint,
        0x6ca6351 as libc::c_int as uint32_t,
        0x14292967 as libc::c_int as uint32_t,
        0x27b70a85 as libc::c_int as uint32_t,
        0x2e1b2138 as libc::c_int as uint32_t,
        0x4d2c6dfc as libc::c_int as uint32_t,
        0x53380d13 as libc::c_int as uint32_t,
        0x650a7354 as libc::c_int as uint32_t,
        0x766a0abb as libc::c_int as uint32_t,
        0x81c2c92e as libc::c_uint,
        0x92722c85 as libc::c_uint,
        0xa2bfe8a1 as libc::c_uint,
        0xa81a664b as libc::c_uint,
        0xc24b8b70 as libc::c_uint,
        0xc76c51a3 as libc::c_uint,
        0xd192e819 as libc::c_uint,
        0xd6990624 as libc::c_uint,
        0xf40e3585 as libc::c_uint,
        0x106aa070 as libc::c_int as uint32_t,
        0x19a4c116 as libc::c_int as uint32_t,
        0x1e376c08 as libc::c_int as uint32_t,
        0x2748774c as libc::c_int as uint32_t,
        0x34b0bcb5 as libc::c_int as uint32_t,
        0x391c0cb3 as libc::c_int as uint32_t,
        0x4ed8aa4a as libc::c_int as uint32_t,
        0x5b9cca4f as libc::c_int as uint32_t,
        0x682e6ff3 as libc::c_int as uint32_t,
        0x748f82ee as libc::c_int as uint32_t,
        0x78a5636f as libc::c_int as uint32_t,
        0x84c87814 as libc::c_uint,
        0x8cc70208 as libc::c_uint,
        0x90befffa as libc::c_uint,
        0xa4506ceb as libc::c_uint,
        0xbef9a3f7 as libc::c_uint,
        0xc67178f2 as libc::c_uint,
    ];
    static mut B: [uint32_t; 8] = [
        0x6a09e667 as libc::c_int as uint32_t,
        0xbb67ae85 as libc::c_uint,
        0x3c6ef372 as libc::c_int as uint32_t,
        0xa54ff53a as libc::c_uint,
        0x510e527f as libc::c_int as uint32_t,
        0x9b05688c as libc::c_uint,
        0x1f83d9ab as libc::c_int as uint32_t,
        0x5be0cd19 as libc::c_int as uint32_t,
    ];
    let mut m: [uint32_t; 64] = [0; 64];
    let mut a0: uint32_t = 0;
    let mut a1: uint32_t = 0;
    let mut a2: uint32_t = 0;
    let mut a3: uint32_t = 0;
    let mut a4: uint32_t = 0;
    let mut a5: uint32_t = 0;
    let mut a6: uint32_t = 0;
    let mut a7: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    m[0 as libc::c_int as usize] = (seed as uint32_t).swap_bytes();
    m[1 as libc::c_int
        as usize] = ((seed >> 32 as libc::c_int) as uint32_t).swap_bytes();
    m[2 as libc::c_int as usize] = 0x80000000 as libc::c_uint;
    i = 3 as libc::c_int as uint32_t;
    while i < 15 as libc::c_int as libc::c_uint {
        m[i as usize] = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(1);
    }
    m[15 as libc::c_int as usize] = 0x40 as libc::c_int as uint32_t;
    i = 16 as libc::c_int as uint32_t;
    while i < 64 as libc::c_int as libc::c_uint {
        m[i
            as usize] = (m[i.wrapping_sub(7 as libc::c_int as libc::c_uint) as usize])
            .wrapping_add(m[i.wrapping_sub(16 as libc::c_int as libc::c_uint) as usize]);
        x = m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize];
        m[i
            as usize] = (m[i as usize] as libc::c_uint)
            .wrapping_add(
                rotr32(x, 7 as libc::c_int as uint8_t)
                    ^ rotr32(x, 18 as libc::c_int as uint8_t) ^ x >> 3 as libc::c_int,
            ) as uint32_t as uint32_t;
        x = m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize];
        m[i
            as usize] = (m[i as usize] as libc::c_uint)
            .wrapping_add(
                rotr32(x, 17 as libc::c_int as uint8_t)
                    ^ rotr32(x, 19 as libc::c_int as uint8_t) ^ x >> 10 as libc::c_int,
            ) as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
    a0 = B[0 as libc::c_int as usize];
    a1 = B[1 as libc::c_int as usize];
    a2 = B[2 as libc::c_int as usize];
    a3 = B[3 as libc::c_int as usize];
    a4 = B[4 as libc::c_int as usize];
    a5 = B[5 as libc::c_int as usize];
    a6 = B[6 as libc::c_int as usize];
    a7 = B[7 as libc::c_int as usize];
    i = 0 as libc::c_int as uint32_t;
    while i < 64 as libc::c_int as libc::c_uint {
        x = a7.wrapping_add(K[i as usize]).wrapping_add(m[i as usize]);
        x = (x as libc::c_uint)
            .wrapping_add(
                rotr32(a4, 6 as libc::c_int as uint8_t)
                    ^ rotr32(a4, 11 as libc::c_int as uint8_t)
                    ^ rotr32(a4, 25 as libc::c_int as uint8_t),
            ) as uint32_t as uint32_t;
        x = (x as libc::c_uint).wrapping_add(a4 & a5 ^ !a4 & a6) as uint32_t as uint32_t;
        y = rotr32(a0, 2 as libc::c_int as uint8_t)
            ^ rotr32(a0, 13 as libc::c_int as uint8_t)
            ^ rotr32(a0, 22 as libc::c_int as uint8_t);
        y = (y as libc::c_uint).wrapping_add(a0 & a1 ^ a0 & a2 ^ a1 & a2) as uint32_t
            as uint32_t;
        a7 = a6;
        a6 = a5;
        a5 = a4;
        a4 = a3.wrapping_add(x);
        a3 = a2;
        a2 = a1;
        a1 = a0;
        a0 = x.wrapping_add(y);
        i = i.wrapping_add(1);
    }
    a0 = (a0 as libc::c_uint).wrapping_add(B[0 as libc::c_int as usize]) as uint32_t
        as uint32_t;
    a1 = (a1 as libc::c_uint).wrapping_add(B[1 as libc::c_int as usize]) as uint32_t
        as uint32_t;
    return a0.swap_bytes() as libc::c_ulong
        | (a1.swap_bytes() as uint64_t) << 32 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn voronoiAccess3D(
    mut sha: uint64_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut x4: *mut libc::c_int,
    mut y4: *mut libc::c_int,
    mut z4: *mut libc::c_int,
) {
    x -= 2 as libc::c_int;
    y -= 2 as libc::c_int;
    z -= 2 as libc::c_int;
    let mut pX: libc::c_int = x >> 2 as libc::c_int;
    let mut pY: libc::c_int = y >> 2 as libc::c_int;
    let mut pZ: libc::c_int = z >> 2 as libc::c_int;
    let mut dx: libc::c_int = (x & 3 as libc::c_int) * 10240 as libc::c_int;
    let mut dy: libc::c_int = (y & 3 as libc::c_int) * 10240 as libc::c_int;
    let mut dz: libc::c_int = (z & 3 as libc::c_int) * 10240 as libc::c_int;
    let mut ax: libc::c_int = 0 as libc::c_int;
    let mut ay: libc::c_int = 0 as libc::c_int;
    let mut az: libc::c_int = 0 as libc::c_int;
    let mut dmin: uint64_t = -(1 as libc::c_int) as uint64_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut bx: libc::c_int = (i & 4 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        let mut by: libc::c_int = (i & 2 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        let mut bz: libc::c_int = (i & 1 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        let mut cx: libc::c_int = pX + bx;
        let mut cy: libc::c_int = pY + by;
        let mut cz: libc::c_int = pZ + bz;
        let mut rx: libc::c_int = 0;
        let mut ry: libc::c_int = 0;
        let mut rz: libc::c_int = 0;
        getVoronoiCell(sha, cx, cy, cz, &mut rx, &mut ry, &mut rz);
        rx += dx - 40 as libc::c_int * 1024 as libc::c_int * bx;
        ry += dy - 40 as libc::c_int * 1024 as libc::c_int * by;
        rz += dz - 40 as libc::c_int * 1024 as libc::c_int * bz;
        let mut d: uint64_t = (rx as libc::c_ulong)
            .wrapping_mul(rx as uint64_t)
            .wrapping_add((ry as libc::c_ulong).wrapping_mul(ry as uint64_t))
            .wrapping_add((rz as libc::c_ulong).wrapping_mul(rz as uint64_t));
        if d < dmin {
            dmin = d;
            ax = cx;
            ay = cy;
            az = cz;
        }
        i += 1;
    }
    if !x4.is_null() {
        *x4 = ax;
    }
    if !y4.is_null() {
        *y4 = ay;
    }
    if !z4.is_null() {
        *z4 = az;
    }
}
