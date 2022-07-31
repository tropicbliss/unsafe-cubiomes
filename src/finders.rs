#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn nan(_: *const libc::c_char) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mapBiome(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn isSnowy(id: libc::c_int) -> libc::c_int;
    fn isOceanic(id: libc::c_int) -> libc::c_int;
    fn isDeepOcean(id: libc::c_int) -> libc::c_int;
    fn isShallowOcean(id: libc::c_int) -> libc::c_int;
    fn isMesa(id: libc::c_int) -> libc::c_int;
    fn areSimilar(mc: libc::c_int, id1: libc::c_int, id2: libc::c_int) -> libc::c_int;
    fn getCategory(mc: libc::c_int, id: libc::c_int) -> libc::c_int;
    fn getMutated(mc: libc::c_int, id: libc::c_int) -> libc::c_int;
    fn isOverworld(mc: libc::c_int, id: libc::c_int) -> libc::c_int;
    fn sampleBiomeNoise(
        bn: *const BiomeNoise,
        np: *mut int64_t,
        x: libc::c_int,
        y: libc::c_int,
        z: libc::c_int,
        dat: *mut uint64_t,
        flags: uint32_t,
    ) -> libc::c_int;
    fn setLayerSeed(layer: *mut Layer, worldSeed: uint64_t);
    fn sampleDoublePerlin(
        noise: *const DoublePerlinNoise,
        x: libc::c_double,
        y: libc::c_double,
        z: libc::c_double,
    ) -> libc::c_double;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn rand() -> libc::c_int;
    fn allocCache(g: *const Generator, r: Range) -> *mut libc::c_int;
    fn genArea(
        layer: *const Layer,
        out: *mut libc::c_int,
        areaX: libc::c_int,
        areaZ: libc::c_int,
        areaWidth: libc::c_int,
        areaHeight: libc::c_int,
    ) -> libc::c_int;
    fn getMinLayerCacheSize(
        layer: *const Layer,
        sizeX: libc::c_int,
        sizeZ: libc::c_int,
    ) -> size_t;
    fn getLayerForScale(g: *const Generator, scale: libc::c_int) -> *const Layer;
    fn getBiomeAt(
        g: *const Generator,
        scale: libc::c_int,
        x: libc::c_int,
        y: libc::c_int,
        z: libc::c_int,
    ) -> libc::c_int;
    fn genBiomes(g: *const Generator, cache: *mut libc::c_int, r: Range) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn applySeed(g: *mut Generator, dim: libc::c_int, seed: uint64_t);
    fn mapShore(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn getSurfaceHeight(
        ncol00: *const libc::c_double,
        ncol01: *const libc::c_double,
        ncol10: *const libc::c_double,
        ncol11: *const libc::c_double,
        colymin: libc::c_int,
        colymax: libc::c_int,
        blockspercell: libc::c_int,
        dx: libc::c_double,
        dz: libc::c_double,
    ) -> libc::c_int;
    fn sampleNoiseColumnEnd(
        column: *mut libc::c_double,
        sn: *const SurfaceNoise,
        en: *const EndNoise,
        x: libc::c_int,
        z: libc::c_int,
        colymin: libc::c_int,
        colymax: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
pub type LayerId = libc::c_uint;
pub const L_NUM: LayerId = 59;
pub const L_ZOOM_L_RIVER_B: LayerId = 58;
pub const L_ZOOM_L_RIVER_A: LayerId = 57;
pub const L_ZOOM_LARGE_B: LayerId = 56;
pub const L_ZOOM_LARGE_A: LayerId = 55;
pub const L_VORONOI_ZOOM_1: LayerId = 54;
pub const L_VORONOI_1: LayerId = 54;
pub const L13_OCEAN_MIX_4: LayerId = 53;
pub const L_OCEAN_MIX_4: LayerId = 53;
pub const L13_ZOOM_4: LayerId = 52;
pub const L_ZOOM_4_OCEAN: LayerId = 52;
pub const L13_ZOOM_8: LayerId = 51;
pub const L_ZOOM_8_OCEAN: LayerId = 51;
pub const L13_ZOOM_16: LayerId = 50;
pub const L_ZOOM_16_OCEAN: LayerId = 50;
pub const L13_ZOOM_32: LayerId = 49;
pub const L_ZOOM_32_OCEAN: LayerId = 49;
pub const L13_ZOOM_64: LayerId = 48;
pub const L_ZOOM_64_OCEAN: LayerId = 48;
pub const L13_ZOOM_128: LayerId = 47;
pub const L_ZOOM_128_OCEAN: LayerId = 47;
pub const L13_OCEAN_TEMP_256: LayerId = 46;
pub const L_OCEAN_TEMP_256: LayerId = 46;
pub const L_RIVER_MIX_4: LayerId = 45;
pub const L_SMOOTH_4_RIVER: LayerId = 44;
pub const L_RIVER_4: LayerId = 43;
pub const L_ZOOM_4_RIVER: LayerId = 42;
pub const L_ZOOM_8_RIVER: LayerId = 41;
pub const L_ZOOM_16_RIVER: LayerId = 40;
pub const L_ZOOM_32_RIVER: LayerId = 39;
pub const L_ZOOM_64_RIVER: LayerId = 38;
pub const L_ZOOM_128_RIVER: LayerId = 37;
pub const L_SMOOTH_4: LayerId = 36;
pub const L_ZOOM_4: LayerId = 35;
pub const L_ZOOM_8: LayerId = 34;
pub const L_SWAMP_RIVER_16: LayerId = 33;
pub const L_SHORE_16: LayerId = 32;
pub const L_ZOOM_16: LayerId = 31;
pub const L_ADD_ISLAND_32: LayerId = 30;
pub const L_LAND_32: LayerId = 30;
pub const L_ZOOM_32: LayerId = 29;
pub const L_RARE_BIOME_64: LayerId = 28;
pub const L_SUNFLOWER_64: LayerId = 28;
pub const L_HILLS_64: LayerId = 27;
pub const L_ZOOM_64_HILLS: LayerId = 26;
pub const L_ZOOM_128_HILLS: LayerId = 25;
pub const L_RIVER_INIT_256: LayerId = 24;
pub const L_NOISE_256: LayerId = 24;
pub const L_BIOME_EDGE_64: LayerId = 23;
pub const L_ZOOM_64: LayerId = 22;
pub const L_ZOOM_128: LayerId = 21;
pub const L14_BAMBOO_256: LayerId = 20;
pub const L_BAMBOO_256: LayerId = 20;
pub const L_BIOME_256: LayerId = 19;
pub const L_DEEP_OCEAN_256: LayerId = 18;
pub const L_ADD_MUSHROOM_256: LayerId = 17;
pub const L_MUSHROOM_256: LayerId = 17;
pub const L_ADD_ISLAND_256: LayerId = 16;
pub const L_LAND_256: LayerId = 16;
pub const L_ZOOM_256: LayerId = 15;
pub const L_LAND_512: LayerId = 14;
pub const L_ZOOM_512: LayerId = 13;
pub const L_SPECIAL_1024: LayerId = 12;
pub const L_HEAT_ICE_1024: LayerId = 11;
pub const L_HEAT_1024: LayerId = 11;
pub const L_COOL_WARM_1024: LayerId = 10;
pub const L_COOL_1024: LayerId = 10;
pub const L_ADD_ISLAND_1024D: LayerId = 9;
pub const L_LAND_1024_D: LayerId = 9;
pub const L_ADD_SNOW_1024: LayerId = 8;
pub const L_SNOW_1024: LayerId = 8;
pub const L_REMOVE_OCEAN_1024: LayerId = 7;
pub const L_ISLAND_1024: LayerId = 7;
pub const L_ADD_ISLAND_1024C: LayerId = 6;
pub const L_LAND_1024_C: LayerId = 6;
pub const L_ADD_ISLAND_1024B: LayerId = 5;
pub const L_LAND_1024_B: LayerId = 5;
pub const L_ADD_ISLAND_1024A: LayerId = 4;
pub const L_LAND_1024_A: LayerId = 4;
pub const L_ZOOM_1024: LayerId = 3;
pub const L_ADD_ISLAND_2048: LayerId = 2;
pub const L_LAND_2048: LayerId = 2;
pub const L_ZOOM_2048: LayerId = 1;
pub const L_ISLAND_4096: LayerId = 0;
pub const L_CONTINENT_4096: LayerId = 0;
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
pub struct LayerStack {
    pub layers: [Layer; 59],
    pub entry_1: *mut Layer,
    pub entry_4: *mut Layer,
    pub entry_16: *mut Layer,
    pub entry_64: *mut Layer,
    pub entry_256: *mut Layer,
    pub oceanRnd: PerlinNoise,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Generator {
    pub mc: libc::c_int,
    pub dim: libc::c_int,
    pub flags: uint32_t,
    pub seed: uint64_t,
    pub sha: uint64_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub nn: NetherNoise,
    pub en: EndNoise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub bn: BiomeNoise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ls: LayerStack,
    pub xlayer: [Layer; 5],
    pub entry: *mut Layer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type thread_id_t = pthread_t;
pub type StructureType = libc::c_uint;
pub const FEATURE_NUM: StructureType = 20;
pub const End_Gateway: StructureType = 19;
pub const End_City: StructureType = 18;
pub const Bastion: StructureType = 17;
pub const Fortress: StructureType = 16;
pub const Mineshaft: StructureType = 15;
pub const Treasure: StructureType = 14;
pub const Ancient_City: StructureType = 13;
pub const Ruined_Portal_N: StructureType = 12;
pub const Ruined_Portal: StructureType = 11;
pub const Outpost: StructureType = 10;
pub const Mansion: StructureType = 9;
pub const Monument: StructureType = 8;
pub const Shipwreck: StructureType = 7;
pub const Ocean_Ruin: StructureType = 6;
pub const Village: StructureType = 5;
pub const Igloo: StructureType = 4;
pub const Swamp_Hut: StructureType = 3;
pub const Jungle_Pyramid: StructureType = 2;
pub const Jungle_Temple: StructureType = 2;
pub const Desert_Pyramid: StructureType = 1;
pub const Feature: StructureType = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const HOUSE_NUM: C2RustUnnamed_3 = 9;
pub const HouseLarge: C2RustUnnamed_3 = 8;
pub const Blacksmith: C2RustUnnamed_3 = 7;
pub const FarmSmall: C2RustUnnamed_3 = 6;
pub const FarmLarge: C2RustUnnamed_3 = 5;
pub const Butcher: C2RustUnnamed_3 = 4;
pub const WoodHut: C2RustUnnamed_3 = 3;
pub const Library: C2RustUnnamed_3 = 2;
pub const Church: C2RustUnnamed_3 = 1;
pub const HouseSmall: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructureConfig {
    pub salt: libc::c_int,
    pub regionSize: libc::c_char,
    pub chunkRange: libc::c_char,
    pub structType: libc::c_uchar,
    pub properties: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pos {
    pub x: libc::c_int,
    pub z: libc::c_int,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const CFB_APPROX: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BiomeFilter {
    pub tempsToFind: uint64_t,
    pub otempToFind: uint64_t,
    pub majorToFind: uint64_t,
    pub edgesToFind: uint64_t,
    pub raresToFind: uint64_t,
    pub raresToFindM: uint64_t,
    pub shoreToFind: uint64_t,
    pub shoreToFindM: uint64_t,
    pub riverToFind: uint64_t,
    pub riverToFindM: uint64_t,
    pub oceanToFind: uint64_t,
    pub specialCnt: libc::c_int,
    pub flags: uint32_t,
    pub tempsToExcl: uint64_t,
    pub majorToExcl: uint64_t,
    pub edgesToExcl: uint64_t,
    pub raresToExcl: uint64_t,
    pub raresToExclM: uint64_t,
    pub shoreToExcl: uint64_t,
    pub shoreToExclM: uint64_t,
    pub riverToExcl: uint64_t,
    pub riverToExclM: uint64_t,
    pub biomeToExcl: uint64_t,
    pub biomeToExclM: uint64_t,
    pub biomeToFind: uint64_t,
    pub biomeToFindM: uint64_t,
    pub biomeToPick: uint64_t,
    pub biomeToPickM: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrongholdIter {
    pub pos: Pos,
    pub nextapprox: Pos,
    pub index: libc::c_int,
    pub ringnum: libc::c_int,
    pub ringmax: libc::c_int,
    pub ringidx: libc::c_int,
    pub angle: libc::c_double,
    pub dist: libc::c_double,
    pub rnds: uint64_t,
    pub mc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructureVariant {
    pub abandoned: uint8_t,
    pub giant: uint8_t,
    pub variant: libc::c_char,
    pub name: *const libc::c_char,
    pub biome: libc::c_short,
    pub rotation: uint8_t,
    pub mirror: uint8_t,
    pub x: libc::c_char,
    pub y: libc::c_char,
    pub z: libc::c_char,
    pub sx: libc::c_char,
    pub sy: libc::c_char,
    pub sz: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct threadinfo_t {
    pub start: uint64_t,
    pub end: uint64_t,
    pub lowBits: *const uint64_t,
    pub lowBitCnt: libc::c_int,
    pub lowBitN: libc::c_int,
    pub check: Option::<
        unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> libc::c_int,
    >,
    pub data: *mut libc::c_void,
    pub path: [libc::c_char; 4096],
    pub fp: *mut FILE,
    pub ls: linked_seeds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_seeds_t {
    pub seeds: [uint64_t; 100],
    pub len: size_t,
    pub next: *mut linked_seeds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct afk_meta_t {
    pub p: *mut Pos,
    pub n: libc::c_int,
    pub buf: *mut libc::c_int,
    pub x0: libc::c_int,
    pub z0: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub ax: libc::c_int,
    pub az: libc::c_int,
    pub rsq: libc::c_double,
    pub best: libc::c_int,
    pub sumn: libc::c_int,
    pub sumx: int64_t,
    pub sumz: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct touple {
    pub i: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub z: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdt_info_t {
    pub g: *mut Generator,
    pub ids: *mut libc::c_int,
    pub r: Range,
    pub flags: uint32_t,
    pub b: uint64_t,
    pub m: uint64_t,
    pub breq: uint64_t,
    pub mreq: uint64_t,
    pub bexc: uint64_t,
    pub mexc: uint64_t,
    pub bany: uint64_t,
    pub many: uint64_t,
    pub stop: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_data_t {
    pub bf: *const BiomeFilter,
    pub map: Option::<
        unsafe extern "C" fn(
            *const Layer,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
}
pub const M_DONE: C2RustUnnamed_5 = 2;
pub const M_STOP: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
#[inline]
unsafe extern "C" fn mulInv(mut x: uint64_t, mut m: uint64_t) -> uint64_t {
    let mut t: uint64_t = 0;
    let mut q: uint64_t = 0;
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut n: uint64_t = 0;
    if m as int64_t <= 1 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as uint64_t;
    }
    n = m;
    a = 0 as libc::c_int as uint64_t;
    b = 1 as libc::c_int as uint64_t;
    while x as int64_t > 1 as libc::c_int as libc::c_long {
        if m == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as uint64_t;
        }
        q = x.wrapping_div(m);
        t = m;
        m = x.wrapping_rem(m);
        x = t;
        t = a;
        a = b.wrapping_sub(q.wrapping_mul(a));
        b = t;
    }
    if (b as int64_t) < 0 as libc::c_int as libc::c_long {
        b = (b as libc::c_ulong).wrapping_add(n) as uint64_t as uint64_t;
    }
    return b;
}
#[inline]
unsafe extern "C" fn getStartSeed(mut ws: uint64_t, mut ls: uint64_t) -> uint64_t {
    let mut ss: uint64_t = ws;
    ss = getStartSalt(ss, ls);
    ss = mcStepSeed(ss, 0 as libc::c_int as uint64_t);
    return ss;
}
#[inline]
unsafe extern "C" fn getStartSalt(mut ws: uint64_t, mut ls: uint64_t) -> uint64_t {
    let mut st: uint64_t = ws;
    st = mcStepSeed(st, ls);
    st = mcStepSeed(st, ls);
    st = mcStepSeed(st, ls);
    return st;
}
#[inline]
unsafe extern "C" fn getLayerSalt(mut salt: uint64_t) -> uint64_t {
    let mut ls: uint64_t = mcStepSeed(salt, salt);
    ls = mcStepSeed(ls, salt);
    ls = mcStepSeed(ls, salt);
    return ls;
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
unsafe extern "C" fn mcFirstIsZero(
    mut s: uint64_t,
    mut mod_0: libc::c_int,
) -> libc::c_int {
    return (((s as int64_t >> 24 as libc::c_int) % mod_0 as libc::c_long) as libc::c_int
        == 0 as libc::c_int) as libc::c_int;
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
unsafe extern "C" fn xNextIntJ(mut xr: *mut Xoroshiro, mut n: uint32_t) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let m: libc::c_int = n.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    if m as libc::c_uint & n == 0 as libc::c_int as libc::c_uint {
        let mut x: uint64_t = (n as libc::c_ulong)
            .wrapping_mul(xNextLong(xr) >> 33 as libc::c_int);
        return (x as int64_t >> 31 as libc::c_int) as libc::c_int;
    }
    loop {
        bits = (xNextLong(xr) >> 33 as libc::c_int) as libc::c_int;
        val = (bits as libc::c_uint).wrapping_rem(n) as libc::c_int;
        if !(bits - val + m < 0 as libc::c_int) {
            break;
        }
    }
    return val;
}
#[inline]
unsafe extern "C" fn xNextLongJ(mut xr: *mut Xoroshiro) -> uint64_t {
    return xNextLong(xr) & 0xffffffff00000000 as libc::c_ulong
        | xNextLong(xr) >> 32 as libc::c_int;
}
#[inline]
unsafe extern "C" fn xNextFloat(mut xr: *mut Xoroshiro) -> libc::c_float {
    return (xNextLong(xr) >> 64 as libc::c_int - 24 as libc::c_int) as libc::c_float
        * 5.9604645E-8f32;
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
unsafe extern "C" fn nextDouble(mut seed: *mut uint64_t) -> libc::c_double {
    let mut x: uint64_t = next(seed, 26 as libc::c_int) as uint64_t;
    x <<= 27 as libc::c_int;
    x = (x as libc::c_ulong).wrapping_add(next(seed, 27 as libc::c_int) as libc::c_ulong)
        as uint64_t as uint64_t;
    return x as int64_t as libc::c_double
        / ((1 as libc::c_ulonglong) << 53 as libc::c_int) as libc::c_double;
}
#[inline]
unsafe extern "C" fn nextFloat(mut seed: *mut uint64_t) -> libc::c_float {
    return next(seed, 24 as libc::c_int) as libc::c_float
        / ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_float;
}
#[inline]
unsafe extern "C" fn nextLong(mut seed: *mut uint64_t) -> uint64_t {
    return ((next(seed, 32 as libc::c_int) as uint64_t) << 32 as libc::c_int)
        .wrapping_add(next(seed, 32 as libc::c_int) as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn nextInt(mut seed: *mut uint64_t, n: libc::c_int) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let m: libc::c_int = n - 1 as libc::c_int;
    if m & n == 0 as libc::c_int {
        let mut x: uint64_t = (n as libc::c_ulong)
            .wrapping_mul(next(seed, 31 as libc::c_int) as uint64_t);
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
unsafe extern "C" fn next(mut seed: *mut uint64_t, bits: libc::c_int) -> libc::c_int {
    *seed = ((*seed)
        .wrapping_mul(0x5deece66d as libc::c_long as libc::c_ulong)
        .wrapping_add(0xb as libc::c_int as libc::c_ulong) as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 48 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)) as uint64_t;
    return (*seed as int64_t >> 48 as libc::c_int - bits) as libc::c_int;
}
#[inline]
unsafe extern "C" fn setSeed(mut seed: *mut uint64_t, mut value: uint64_t) {
    *seed = ((value ^ 0x5deece66d as libc::c_long as libc::c_ulong) as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 48 as libc::c_int)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)) as uint64_t;
}
#[inline(always)]
unsafe extern "C" fn rotl64(mut x: uint64_t, mut b: uint8_t) -> uint64_t {
    return x << b as libc::c_int | x >> 64 as libc::c_int - b as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn isQuadBaseFeature24(
    sconf: StructureConfig,
    mut seed: uint64_t,
    mut ax: libc::c_int,
    mut ay: libc::c_int,
    mut az: libc::c_int,
) -> libc::c_float {
    seed = (seed as libc::c_ulong).wrapping_add(sconf.salt as libc::c_ulong) as uint64_t
        as uint64_t;
    let mut s00: uint64_t = seed;
    let mut s11: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let K: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let mut x0: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut z1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut z2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut z3: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    s00 ^= K;
    let mut a: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s00 as libc::c_ulonglong) as uint64_t;
    c = (c as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong) as uint64_t
        as uint64_t;
    a &= c;
    s00 = a;
    a = (a as int64_t >> 17 as libc::c_int) as uint64_t;
    c = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a);
    c = (c as int64_t >> 36 as libc::c_int) as uint64_t;
    x0 = a as libc::c_int - (c << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (x0 < 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut a_0: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_0: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s00 as libc::c_ulonglong) as uint64_t;
    c_0 = (c_0 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_0 &= c_0;
    s00 = a_0;
    a_0 = (a_0 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_0 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_0);
    c_0 = (c_0 as int64_t >> 36 as libc::c_int) as uint64_t;
    z0 = a_0 as libc::c_int
        - (c_0 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (z0 < 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s11 ^= K;
    let mut a_1: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_1: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s11 as libc::c_ulonglong) as uint64_t;
    c_1 = (c_1 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_1 &= c_1;
    s11 = a_1;
    a_1 = (a_1 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_1 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_1);
    c_1 = (c_1 as int64_t >> 36 as libc::c_int) as uint64_t;
    x1 = a_1 as libc::c_int
        - (c_1 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (x1 > x0 - 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut a_2: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_2: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s11 as libc::c_ulonglong) as uint64_t;
    c_2 = (c_2 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_2 &= c_2;
    s11 = a_2;
    a_2 = (a_2 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_2 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_2);
    c_2 = (c_2 as int64_t >> 36 as libc::c_int) as uint64_t;
    z1 = a_2 as libc::c_int
        - (c_2 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (z1 > z0 - 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x = x1 + 32 as libc::c_int - x0;
    z = z1 + 32 as libc::c_int - z0;
    if x * x + z * z > 255 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut s01: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let mut s10: uint64_t = (132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    s01 ^= K;
    let mut a_3: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_3: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s01 as libc::c_ulonglong) as uint64_t;
    c_3 = (c_3 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_3 &= c_3;
    s01 = a_3;
    a_3 = (a_3 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_3 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_3);
    c_3 = (c_3 as int64_t >> 36 as libc::c_int) as uint64_t;
    x2 = a_3 as libc::c_int
        - (c_3 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (x2 >= 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut a_4: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_4: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s01 as libc::c_ulonglong) as uint64_t;
    c_4 = (c_4 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_4 &= c_4;
    s01 = a_4;
    a_4 = (a_4 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_4 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_4);
    c_4 = (c_4 as int64_t >> 36 as libc::c_int) as uint64_t;
    z2 = a_4 as libc::c_int
        - (c_4 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (z2 < 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s10 ^= K;
    let mut a_5: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_5: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s10 as libc::c_ulonglong) as uint64_t;
    c_5 = (c_5 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_5 &= c_5;
    s10 = a_5;
    a_5 = (a_5 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_5 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_5);
    c_5 = (c_5 as int64_t >> 36 as libc::c_int) as uint64_t;
    x3 = a_5 as libc::c_int
        - (c_5 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (x3 < 20 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut a_6: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let mut c_6: uint64_t = (0x5deece66d as libc::c_ulonglong)
        .wrapping_mul(s10 as libc::c_ulonglong) as uint64_t;
    c_6 = (c_6 as libc::c_ulong).wrapping_add(11 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    a_6 &= c_6;
    s10 = a_6;
    a_6 = (a_6 as int64_t >> 17 as libc::c_int) as uint64_t;
    c_6 = (0xaaaaaaab as libc::c_uint as libc::c_ulong).wrapping_mul(a_6);
    c_6 = (c_6 as int64_t >> 36 as libc::c_int) as uint64_t;
    z3 = a_6 as libc::c_int
        - (c_6 << 3 as libc::c_int) as libc::c_int * 3 as libc::c_int;
    if (z3 >= 4 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x = x2 + 32 as libc::c_int - x3;
    z = z3 + 32 as libc::c_int - z2;
    if x * x + z * z > 255 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut sqrad: libc::c_float = getEnclosingRadius(
        x0,
        z0,
        x1,
        z1,
        x2,
        z2,
        x3,
        z3,
        ax,
        ay,
        az,
        32 as libc::c_int,
        128 as libc::c_int,
    );
    return if sqrad < 128 as libc::c_int as libc::c_float {
        sqrad
    } else {
        0 as libc::c_int as libc::c_float
    };
}
#[inline(always)]
unsafe extern "C" fn isQuadBaseFeature(
    sconf: StructureConfig,
    mut seed: uint64_t,
    mut ax: libc::c_int,
    mut ay: libc::c_int,
    mut az: libc::c_int,
    mut radius: libc::c_int,
) -> libc::c_float {
    seed = (seed as libc::c_ulong).wrapping_add(sconf.salt as libc::c_ulong) as uint64_t
        as uint64_t;
    let mut s00: uint64_t = seed;
    let mut s11: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let M: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let K: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let b: uint64_t = 0xb as libc::c_int as uint64_t;
    let mut x0: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut z1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut z2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut z3: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let R: libc::c_int = sconf.regionSize as libc::c_int;
    let C: libc::c_int = sconf.chunkRange as libc::c_int;
    let mut cd: libc::c_int = radius / 8 as libc::c_int;
    let mut rm: libc::c_int = R
        - sqrtf(
            (cd * cd - (R - C + 1 as libc::c_int) * (R - C + 1 as libc::c_int))
                as libc::c_float,
        ) as libc::c_int;
    let mut s: uint64_t = 0;
    s = s00 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    x0 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (x0 <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    z0 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (z0 <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s11 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    x1 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (x1 >= x0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    z1 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (z1 >= z0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x = x1 + R - x0;
    z = z1 + R - z0;
    if (x * x + z * z > cd * cd) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut s01: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let mut s10: uint64_t = (132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    s = s01 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    x2 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (x2 >= C - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    z2 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (z2 <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s10 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    x3 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (x3 <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    z3 = (s >> 17 as libc::c_int) as libc::c_int % C;
    if (z3 >= C - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x = x2 + R - x3;
    z = z3 + R - z2;
    if (x * x + z * z > cd * cd) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    let mut sqrad: libc::c_float = getEnclosingRadius(
        x0,
        z0,
        x1,
        z1,
        x2,
        z2,
        x3,
        z3,
        ax,
        ay,
        az,
        sconf.regionSize as libc::c_int,
        radius,
    );
    return if sqrad < radius as libc::c_float {
        sqrad
    } else {
        0 as libc::c_int as libc::c_float
    };
}
unsafe extern "C" fn getEnclosingRadius(
    mut x0: libc::c_int,
    mut z0: libc::c_int,
    mut x1: libc::c_int,
    mut z1: libc::c_int,
    mut x2: libc::c_int,
    mut z2: libc::c_int,
    mut x3: libc::c_int,
    mut z3: libc::c_int,
    mut ax: libc::c_int,
    mut ay: libc::c_int,
    mut az: libc::c_int,
    mut reg: libc::c_int,
    mut gap: libc::c_int,
) -> libc::c_float {
    x0 = x0 << 4 as libc::c_int;
    z0 = z0 << 4 as libc::c_int;
    x1 = (reg + x1 << 4 as libc::c_int) + ax;
    z1 = (reg + z1 << 4 as libc::c_int) + az;
    x2 = (reg + x2 << 4 as libc::c_int) + ax;
    z2 = z2 << 4 as libc::c_int;
    x3 = x3 << 4 as libc::c_int;
    z3 = (reg + z3 << 4 as libc::c_int) + az;
    let mut sqrad: libc::c_int = 0x7fffffff as libc::c_int;
    let mut cbx0: libc::c_int = (if x1 > x2 { x1 } else { x2 }) - gap;
    let mut cbz0: libc::c_int = (if z1 > z3 { z1 } else { z3 }) - gap;
    let mut cbx1: libc::c_int = (if x0 < x3 { x0 } else { x3 }) + gap;
    let mut cbz1: libc::c_int = (if z0 < z2 { z0 } else { z2 }) + gap;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    z = cbz0;
    while z <= cbz1 {
        x = cbx0;
        while x <= cbx1 {
            let mut sq: libc::c_int = 0 as libc::c_int;
            let mut s: libc::c_int = 0;
            s = (x - x0) * (x - x0) + (z - z0) * (z - z0);
            if s > sq {
                sq = s;
            }
            s = (x - x1) * (x - x1) + (z - z1) * (z - z1);
            if s > sq {
                sq = s;
            }
            s = (x - x2) * (x - x2) + (z - z2) * (z - z2);
            if s > sq {
                sq = s;
            }
            s = (x - x3) * (x - x3) + (z - z3) * (z - z3);
            if s > sq {
                sq = s;
            }
            if sq < sqrad {
                sqrad = sq;
            }
            x += 1;
        }
        z += 1;
    }
    return if sqrad < 0x7fffffff as libc::c_int {
        sqrtf(sqrad as libc::c_float + (ay * ay) as libc::c_float / 4.0f32)
    } else {
        0xffff as libc::c_int as libc::c_float
    };
}
#[inline(always)]
unsafe extern "C" fn isQuadBaseLarge(
    sconf: StructureConfig,
    mut seed: uint64_t,
    mut ax: libc::c_int,
    mut ay: libc::c_int,
    mut az: libc::c_int,
    mut radius: libc::c_int,
) -> libc::c_float {
    let M: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let K: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let b: uint64_t = 0xb as libc::c_int as uint64_t;
    seed = (seed as libc::c_ulong).wrapping_add(sconf.salt as libc::c_ulong) as uint64_t
        as uint64_t;
    let mut s00: uint64_t = seed;
    let mut s01: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let mut s10: uint64_t = (132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let mut s11: uint64_t = (341873128712 as libc::c_ulonglong)
        .wrapping_add(132897987541 as libc::c_ulonglong)
        .wrapping_add(seed as libc::c_ulonglong) as uint64_t;
    let R: libc::c_int = sconf.regionSize as libc::c_int;
    let C: libc::c_int = sconf.chunkRange as libc::c_int;
    let mut rm: libc::c_int = 2 as libc::c_int * R
        + ((if ax < az { ax } else { az }) - 2 as libc::c_int * radius
            + 7 as libc::c_int) / 8 as libc::c_int;
    let mut s: uint64_t = 0;
    let mut p: libc::c_int = 0;
    let mut x0: libc::c_int = 0;
    let mut z0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut z1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut z2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut z3: libc::c_int = 0;
    s = s00 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x0 = p;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    z0 = p;
    s = s11 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p > x0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x1 = p;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p > z0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    z1 = p;
    s = ((x1 - x0 >> 1 as libc::c_int) * (x1 - x0 >> 1 as libc::c_int)
        + (z1 - z0 >> 1 as libc::c_int) * (z1 - z0 >> 1 as libc::c_int)) as uint64_t;
    if s
        > (4 as libc::c_int as uint64_t)
            .wrapping_mul(radius as libc::c_ulong)
            .wrapping_mul(radius as libc::c_ulong)
    {
        return 0 as libc::c_int as libc::c_float;
    }
    s = s01 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p > x0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x2 = p;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    z2 = p;
    s = s10 ^ K;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p <= rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    x3 = p;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p = (s >> 17 as libc::c_int) as libc::c_int % C;
    s = s.wrapping_mul(K).wrapping_add(b) & M;
    p += (s >> 17 as libc::c_int) as libc::c_int % C;
    if (p > z0 - rm) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    z3 = p;
    let mut sqrad: libc::c_float = getEnclosingRadius(
        x0 >> 1 as libc::c_int,
        z0 >> 1 as libc::c_int,
        x1 >> 1 as libc::c_int,
        z1 >> 1 as libc::c_int,
        x2 >> 1 as libc::c_int,
        z2 >> 1 as libc::c_int,
        x3 >> 1 as libc::c_int,
        z3 >> 1 as libc::c_int,
        ax,
        ay,
        az,
        sconf.regionSize as libc::c_int,
        radius,
    );
    return if sqrad < radius as libc::c_float {
        sqrad
    } else {
        0 as libc::c_int as libc::c_float
    };
}
#[inline]
unsafe extern "C" fn isQuadBase(
    sconf: StructureConfig,
    mut seed: uint64_t,
    mut radius: libc::c_int,
) -> libc::c_float {
    match sconf.structType as libc::c_int {
        3 => {
            if radius == 128 as libc::c_int {
                return isQuadBaseFeature24(
                    sconf,
                    seed,
                    7 as libc::c_int + 1 as libc::c_int,
                    7 as libc::c_int + 1 as libc::c_int,
                    9 as libc::c_int + 1 as libc::c_int,
                )
            } else {
                return isQuadBaseFeature(
                    sconf,
                    seed,
                    7 as libc::c_int + 1 as libc::c_int,
                    7 as libc::c_int + 1 as libc::c_int,
                    9 as libc::c_int + 1 as libc::c_int,
                    radius,
                )
            }
        }
        1 | 2 | 4 | 5 => {
            if radius == 128 as libc::c_int {
                return isQuadBaseFeature24(
                    sconf,
                    seed,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                )
            } else {
                return isQuadBaseFeature(
                    sconf,
                    seed,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    radius,
                )
            }
        }
        10 => {
            return isQuadBaseFeature(
                sconf,
                seed,
                72 as libc::c_int,
                54 as libc::c_int,
                72 as libc::c_int,
                radius,
            );
        }
        8 => {
            return isQuadBaseLarge(
                sconf,
                seed,
                58 as libc::c_int,
                23 as libc::c_int,
                58 as libc::c_int,
                radius,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"isQuadBase: not implemented for structure type %d\n\0" as *const u8
                    as *const libc::c_char,
                sconf.structType as libc::c_int,
            );
            exit(-(1 as libc::c_int));
        }
    };
}
#[inline]
unsafe extern "C" fn getLargeStructureChunkInRegion(
    mut config: StructureConfig,
    mut seed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
) -> Pos {
    let mut pos: Pos = Pos { x: 0, z: 0 };
    let K: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let M: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let b: uint64_t = 0xb as libc::c_int as uint64_t;
    seed = (seed as libc::c_ulonglong)
        .wrapping_add(
            (regX as libc::c_ulonglong).wrapping_mul(341873128712 as libc::c_ulonglong),
        )
        .wrapping_add(
            (regZ as libc::c_ulonglong).wrapping_mul(132897987541 as libc::c_ulonglong),
        )
        .wrapping_add(config.salt as libc::c_ulonglong) as uint64_t;
    seed = seed ^ K;
    seed = seed.wrapping_mul(K).wrapping_add(b) & M;
    pos
        .x = (seed >> 17 as libc::c_int) as libc::c_int
        % config.chunkRange as libc::c_int;
    seed = seed.wrapping_mul(K).wrapping_add(b) & M;
    pos.x
        += (seed >> 17 as libc::c_int) as libc::c_int % config.chunkRange as libc::c_int;
    seed = seed.wrapping_mul(K).wrapping_add(b) & M;
    pos
        .z = (seed >> 17 as libc::c_int) as libc::c_int
        % config.chunkRange as libc::c_int;
    seed = seed.wrapping_mul(K).wrapping_add(b) & M;
    pos.z
        += (seed >> 17 as libc::c_int) as libc::c_int % config.chunkRange as libc::c_int;
    pos.x >>= 1 as libc::c_int;
    pos.z >>= 1 as libc::c_int;
    return pos;
}
#[inline]
unsafe extern "C" fn getLargeStructurePos(
    mut config: StructureConfig,
    mut seed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
) -> Pos {
    let mut pos: Pos = getLargeStructureChunkInRegion(config, seed, regX, regZ);
    pos
        .x = ((regX as uint64_t)
        .wrapping_mul(config.regionSize as libc::c_ulong)
        .wrapping_add(pos.x as libc::c_ulong) << 4 as libc::c_int) as libc::c_int;
    pos
        .z = ((regZ as uint64_t)
        .wrapping_mul(config.regionSize as libc::c_ulong)
        .wrapping_add(pos.z as libc::c_ulong) << 4 as libc::c_int) as libc::c_int;
    return pos;
}
#[inline]
unsafe extern "C" fn getFeatureChunkInRegion(
    mut config: StructureConfig,
    mut seed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
) -> Pos {
    let mut pos: Pos = Pos { x: 0, z: 0 };
    let K: uint64_t = 0x5deece66d as libc::c_ulonglong as uint64_t;
    let M: uint64_t = ((1 as libc::c_ulonglong) << 48 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    let b: uint64_t = 0xb as libc::c_int as uint64_t;
    seed = (seed as libc::c_ulonglong)
        .wrapping_add(
            (regX as libc::c_ulonglong).wrapping_mul(341873128712 as libc::c_ulonglong),
        )
        .wrapping_add(
            (regZ as libc::c_ulonglong).wrapping_mul(132897987541 as libc::c_ulonglong),
        )
        .wrapping_add(config.salt as libc::c_ulonglong) as uint64_t;
    seed = seed ^ K;
    seed = seed.wrapping_mul(K).wrapping_add(b) & M;
    let mut r: uint64_t = config.chunkRange as uint64_t;
    if r & r.wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0 {
        pos
            .x = ((seed >> 17 as libc::c_int) as libc::c_int as libc::c_ulong)
            .wrapping_rem(r) as libc::c_int;
        seed = seed.wrapping_mul(K).wrapping_add(b) & M;
        pos
            .z = ((seed >> 17 as libc::c_int) as libc::c_int as libc::c_ulong)
            .wrapping_rem(r) as libc::c_int;
    } else {
        pos
            .x = (r.wrapping_mul(seed >> 17 as libc::c_int) >> 31 as libc::c_int)
            as libc::c_int;
        seed = seed.wrapping_mul(K).wrapping_add(b) & M;
        pos
            .z = (r.wrapping_mul(seed >> 17 as libc::c_int) >> 31 as libc::c_int)
            as libc::c_int;
    }
    return pos;
}
#[inline]
unsafe extern "C" fn getFeaturePos(
    mut config: StructureConfig,
    mut seed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
) -> Pos {
    let mut pos: Pos = getFeatureChunkInRegion(config, seed, regX, regZ);
    pos
        .x = ((regX as uint64_t)
        .wrapping_mul(config.regionSize as libc::c_ulong)
        .wrapping_add(pos.x as libc::c_ulong) << 4 as libc::c_int) as libc::c_int;
    pos
        .z = ((regZ as uint64_t)
        .wrapping_mul(config.regionSize as libc::c_ulong)
        .wrapping_add(pos.z as libc::c_ulong) << 4 as libc::c_int) as libc::c_int;
    return pos;
}
#[inline]
unsafe extern "C" fn chunkGenerateRnd(
    mut worldSeed: uint64_t,
    mut chunkX: libc::c_int,
    mut chunkZ: libc::c_int,
) -> uint64_t {
    let mut rnd: uint64_t = 0;
    setSeed(&mut rnd, worldSeed);
    rnd = (nextLong(&mut rnd)).wrapping_mul(chunkX as libc::c_ulong)
        ^ (nextLong(&mut rnd)).wrapping_mul(chunkZ as libc::c_ulong) ^ worldSeed;
    setSeed(&mut rnd, rnd);
    return rnd;
}
#[inline]
unsafe extern "C" fn moveStructure(
    mut baseSeed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
) -> uint64_t {
    return baseSeed
        .wrapping_sub(
            (regX as libc::c_long * 341873128712 as libc::c_long) as libc::c_ulong,
        )
        .wrapping_sub(
            (regZ as libc::c_long * 132897987541 as libc::c_long) as libc::c_ulong,
        ) & 0xffffffffffff as libc::c_long as libc::c_ulong;
}
static mut END_GATEWAY_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 40013 as libc::c_int,
        regionSize: 1 as libc::c_int as libc::c_char,
        chunkRange: 1 as libc::c_int as libc::c_char,
        structType: End_Gateway as libc::c_int as libc::c_uchar,
        properties: 2 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut END_GATEWAY_CONFIG_115: StructureConfig = {
    let mut init = StructureConfig {
        salt: 30000 as libc::c_int,
        regionSize: 1 as libc::c_int as libc::c_char,
        chunkRange: 1 as libc::c_int as libc::c_char,
        structType: End_Gateway as libc::c_int as libc::c_uchar,
        properties: 2 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut END_CITY_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387313 as libc::c_int,
        regionSize: 20 as libc::c_int as libc::c_char,
        chunkRange: 9 as libc::c_int as libc::c_char,
        structType: End_City as libc::c_int as libc::c_uchar,
        properties: 1 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut BASTION_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 30084232 as libc::c_int,
        regionSize: 27 as libc::c_int as libc::c_char,
        chunkRange: 23 as libc::c_int as libc::c_char,
        structType: Bastion as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut FORTRESS_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 30084232 as libc::c_int,
        regionSize: 27 as libc::c_int as libc::c_char,
        chunkRange: 23 as libc::c_int as libc::c_char,
        structType: Fortress as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut FEATURE_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Feature as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut IGLOO_CONFIG_112: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Igloo as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut SWAMP_HUT_CONFIG_112: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Swamp_Hut as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut DESERT_PYRAMID_CONFIG_112: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Desert_Pyramid as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut JUNGLE_PYRAMID_CONFIG_112: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Jungle_Pyramid as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut OCEAN_RUIN_CONFIG_115: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357621 as libc::c_int,
        regionSize: 16 as libc::c_int as libc::c_char,
        chunkRange: 8 as libc::c_int as libc::c_char,
        structType: Ocean_Ruin as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut SHIPWRECK_CONFIG_115: StructureConfig = {
    let mut init = StructureConfig {
        salt: 165745295 as libc::c_int,
        regionSize: 15 as libc::c_int as libc::c_char,
        chunkRange: 7 as libc::c_int as libc::c_char,
        structType: Shipwreck as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut DESERT_PYRAMID_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357617 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Desert_Pyramid as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut IGLOO_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357618 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Igloo as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut JUNGLE_PYRAMID_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357619 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Jungle_Pyramid as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut SWAMP_HUT_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357620 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Swamp_Hut as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut OUTPOST_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 165745296 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Outpost as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut VILLAGE_CONFIG_117: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387312 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 24 as libc::c_int as libc::c_char,
        structType: Village as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut VILLAGE_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387312 as libc::c_int,
        regionSize: 34 as libc::c_int as libc::c_char,
        chunkRange: 26 as libc::c_int as libc::c_char,
        structType: Village as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut OCEAN_RUIN_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 14357621 as libc::c_int,
        regionSize: 20 as libc::c_int as libc::c_char,
        chunkRange: 12 as libc::c_int as libc::c_char,
        structType: Ocean_Ruin as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut SHIPWRECK_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 165745295 as libc::c_int,
        regionSize: 24 as libc::c_int as libc::c_char,
        chunkRange: 20 as libc::c_int as libc::c_char,
        structType: Shipwreck as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut MONUMENT_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387313 as libc::c_int,
        regionSize: 32 as libc::c_int as libc::c_char,
        chunkRange: 27 as libc::c_int as libc::c_char,
        structType: Monument as libc::c_int as libc::c_uchar,
        properties: 1 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut MANSION_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387319 as libc::c_int,
        regionSize: 80 as libc::c_int as libc::c_char,
        chunkRange: 60 as libc::c_int as libc::c_char,
        structType: Mansion as libc::c_int as libc::c_uchar,
        properties: 1 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut RUINED_PORTAL_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 34222645 as libc::c_int,
        regionSize: 40 as libc::c_int as libc::c_char,
        chunkRange: 25 as libc::c_int as libc::c_char,
        structType: Ruined_Portal as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut RUINED_PORTAL_N_CONFIG_117: StructureConfig = {
    let mut init = StructureConfig {
        salt: 34222645 as libc::c_int,
        regionSize: 25 as libc::c_int as libc::c_char,
        chunkRange: 15 as libc::c_int as libc::c_char,
        structType: Ruined_Portal_N as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut ANCIENT_CITY_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 20083232 as libc::c_int,
        regionSize: 24 as libc::c_int as libc::c_char,
        chunkRange: 16 as libc::c_int as libc::c_char,
        structType: Ancient_City as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut TREASURE_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 10387320 as libc::c_int,
        regionSize: 1 as libc::c_int as libc::c_char,
        chunkRange: 1 as libc::c_int as libc::c_char,
        structType: Treasure as libc::c_int as libc::c_uchar,
        properties: 2 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut MINESHAFT_CONFIG: StructureConfig = {
    let mut init = StructureConfig {
        salt: 0 as libc::c_int,
        regionSize: 1 as libc::c_int as libc::c_char,
        chunkRange: 1 as libc::c_int as libc::c_char,
        structType: Mineshaft as libc::c_int as libc::c_uchar,
        properties: 2 as libc::c_int as libc::c_uchar,
    };
    init
};
static mut FORTRESS_CONFIG_115: StructureConfig = {
    let mut init = StructureConfig {
        salt: 0 as libc::c_int,
        regionSize: 16 as libc::c_int as libc::c_char,
        chunkRange: 8 as libc::c_int as libc::c_char,
        structType: Fortress as libc::c_int as libc::c_uchar,
        properties: 0 as libc::c_int as libc::c_uchar,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn loadSavedSeeds(
    mut fnam: *const libc::c_char,
    mut scnt: *mut uint64_t,
) -> *mut uint64_t {
    let mut fp: *mut FILE = fopen(fnam, b"r\0" as *const u8 as *const libc::c_char);
    let mut seed: uint64_t = 0;
    let mut i: uint64_t = 0;
    let mut baseSeeds: *mut uint64_t = 0 as *mut uint64_t;
    if fp.is_null() {
        return 0 as *mut uint64_t;
    }
    *scnt = 0 as libc::c_int as uint64_t;
    while feof(fp) == 0 {
        if fscanf(
            fp,
            b"%ld\0" as *const u8 as *const libc::c_char,
            &mut seed as *mut uint64_t as *mut int64_t,
        ) == 1 as libc::c_int
        {
            *scnt = (*scnt).wrapping_add(1);
        } else {
            while feof(fp) == 0 && fgetc(fp) != '\n' as i32 {}
        }
    }
    if *scnt == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut uint64_t;
    }
    baseSeeds = calloc(*scnt, ::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        as *mut uint64_t;
    rewind(fp);
    i = 0 as libc::c_int as uint64_t;
    while i < *scnt && feof(fp) == 0 {
        if fscanf(
            fp,
            b"%ld\0" as *const u8 as *const libc::c_char,
            &mut *baseSeeds.offset(i as isize) as *mut uint64_t as *mut int64_t,
        ) == 1 as libc::c_int
        {
            i = i.wrapping_add(1);
        } else {
            while feof(fp) == 0 && fgetc(fp) != '\n' as i32 {}
        }
    }
    fclose(fp);
    return baseSeeds;
}
#[no_mangle]
pub unsafe extern "C" fn setAttemptSeed(
    mut s: *mut uint64_t,
    mut cx: libc::c_int,
    mut cz: libc::c_int,
) {
    *s
        ^= (cx >> 4 as libc::c_int) as uint64_t
            ^ ((cz >> 4 as libc::c_int) as uint64_t) << 4 as libc::c_int;
    setSeed(s, *s);
    next(s, 31 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn getPopulationSeed(
    mut mc: libc::c_int,
    mut ws: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> uint64_t {
    let mut xr: Xoroshiro = Xoroshiro { lo: 0, hi: 0 };
    let mut s: uint64_t = 0;
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    if mc >= MC_1_18 as libc::c_int {
        xSetSeed(&mut xr, ws);
        a = xNextLongJ(&mut xr);
        b = xNextLongJ(&mut xr);
    } else {
        setSeed(&mut s, ws);
        a = nextLong(&mut s);
        b = nextLong(&mut s);
    }
    if mc >= MC_1_13 as libc::c_int {
        a |= 1 as libc::c_int as libc::c_ulong;
        b |= 1 as libc::c_int as libc::c_ulong;
    } else {
        a = (a as int64_t / 2 as libc::c_int as libc::c_long
            * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long)
            as uint64_t;
        b = (b as int64_t / 2 as libc::c_int as libc::c_long
            * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long)
            as uint64_t;
    }
    return (x as libc::c_ulong)
        .wrapping_mul(a)
        .wrapping_add((z as libc::c_ulong).wrapping_mul(b)) ^ ws;
}
#[no_mangle]
pub unsafe extern "C" fn getStructureConfig(
    mut structureType: libc::c_int,
    mut mc: libc::c_int,
    mut sconf: *mut StructureConfig,
) -> libc::c_int {
    match structureType {
        0 => {
            *sconf = FEATURE_CONFIG;
            return (mc <= MC_1_12 as libc::c_int) as libc::c_int;
        }
        1 => {
            *sconf = if mc <= MC_1_12 as libc::c_int {
                DESERT_PYRAMID_CONFIG_112
            } else {
                DESERT_PYRAMID_CONFIG
            };
            return (mc >= MC_1_3 as libc::c_int) as libc::c_int;
        }
        2 => {
            *sconf = if mc <= MC_1_12 as libc::c_int {
                JUNGLE_PYRAMID_CONFIG_112
            } else {
                JUNGLE_PYRAMID_CONFIG
            };
            return (mc >= MC_1_3 as libc::c_int) as libc::c_int;
        }
        3 => {
            *sconf = if mc <= MC_1_12 as libc::c_int {
                SWAMP_HUT_CONFIG_112
            } else {
                SWAMP_HUT_CONFIG
            };
            return (mc >= MC_1_4 as libc::c_int) as libc::c_int;
        }
        4 => {
            *sconf = if mc <= MC_1_12 as libc::c_int {
                IGLOO_CONFIG_112
            } else {
                IGLOO_CONFIG
            };
            return (mc >= MC_1_9 as libc::c_int) as libc::c_int;
        }
        5 => {
            *sconf = if mc <= MC_1_17 as libc::c_int {
                VILLAGE_CONFIG_117
            } else {
                VILLAGE_CONFIG
            };
            return 1 as libc::c_int;
        }
        6 => {
            *sconf = if mc <= MC_1_15 as libc::c_int {
                OCEAN_RUIN_CONFIG_115
            } else {
                OCEAN_RUIN_CONFIG
            };
            return (mc >= MC_1_13 as libc::c_int) as libc::c_int;
        }
        7 => {
            *sconf = if mc <= MC_1_15 as libc::c_int {
                SHIPWRECK_CONFIG_115
            } else {
                SHIPWRECK_CONFIG
            };
            return (mc >= MC_1_13 as libc::c_int) as libc::c_int;
        }
        11 => {
            *sconf = RUINED_PORTAL_CONFIG;
            return (mc >= MC_1_16 as libc::c_int) as libc::c_int;
        }
        12 => {
            *sconf = if mc <= MC_1_17 as libc::c_int {
                RUINED_PORTAL_N_CONFIG_117
            } else {
                RUINED_PORTAL_CONFIG
            };
            return (mc >= MC_1_16 as libc::c_int) as libc::c_int;
        }
        8 => {
            *sconf = MONUMENT_CONFIG;
            return (mc >= MC_1_8 as libc::c_int) as libc::c_int;
        }
        18 => {
            *sconf = END_CITY_CONFIG;
            return (mc >= MC_1_9 as libc::c_int) as libc::c_int;
        }
        9 => {
            *sconf = MANSION_CONFIG;
            return (mc >= MC_1_11 as libc::c_int) as libc::c_int;
        }
        10 => {
            *sconf = OUTPOST_CONFIG;
            return (mc >= MC_1_14 as libc::c_int) as libc::c_int;
        }
        13 => {
            *sconf = ANCIENT_CITY_CONFIG;
            return (mc >= MC_1_19 as libc::c_int) as libc::c_int;
        }
        14 => {
            *sconf = TREASURE_CONFIG;
            return (mc >= MC_1_13 as libc::c_int) as libc::c_int;
        }
        15 => {
            *sconf = MINESHAFT_CONFIG;
            return 1 as libc::c_int;
        }
        16 => {
            *sconf = if mc <= MC_1_15 as libc::c_int {
                FORTRESS_CONFIG_115
            } else {
                FORTRESS_CONFIG
            };
            return 1 as libc::c_int;
        }
        17 => {
            *sconf = BASTION_CONFIG;
            return (mc >= MC_1_16 as libc::c_int) as libc::c_int;
        }
        19 => {
            *sconf = if mc <= MC_1_15 as libc::c_int {
                END_GATEWAY_CONFIG_115
            } else {
                END_GATEWAY_CONFIG
            };
            return (mc >= MC_1_13 as libc::c_int) as libc::c_int;
        }
        _ => {
            memset(
                sconf as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<StructureConfig>() as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
    };
}
#[inline]
unsafe extern "C" fn getRegPos(
    mut p: *mut Pos,
    mut s: *mut uint64_t,
    mut rx: libc::c_int,
    mut rz: libc::c_int,
    mut sc: StructureConfig,
) {
    setSeed(
        s,
        (rx as libc::c_ulonglong)
            .wrapping_mul(341873128712 as libc::c_ulonglong)
            .wrapping_add(
                (rz as libc::c_ulonglong).wrapping_mul(132897987541 as libc::c_ulonglong),
            )
            .wrapping_add(*s as libc::c_ulonglong)
            .wrapping_add(sc.salt as libc::c_ulonglong) as uint64_t,
    );
    (*p)
        .x = ((rx as uint64_t)
        .wrapping_mul(sc.regionSize as libc::c_ulong)
        .wrapping_add(nextInt(s, sc.chunkRange as libc::c_int) as libc::c_ulong)
        << 4 as libc::c_int) as libc::c_int;
    (*p)
        .z = ((rz as uint64_t)
        .wrapping_mul(sc.regionSize as libc::c_ulong)
        .wrapping_add(nextInt(s, sc.chunkRange as libc::c_int) as libc::c_ulong)
        << 4 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getStructurePos(
    mut structureType: libc::c_int,
    mut mc: libc::c_int,
    mut seed: uint64_t,
    mut regX: libc::c_int,
    mut regZ: libc::c_int,
    mut pos: *mut Pos,
) -> libc::c_int {
    let mut sconf: StructureConfig = StructureConfig {
        salt: 0,
        regionSize: 0,
        chunkRange: 0,
        structType: 0,
        properties: 0,
    };
    if getStructureConfig(structureType, mc, &mut sconf) == 0 {
        return 0 as libc::c_int;
    }
    match structureType {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 12 | 13 => {
            *pos = getFeaturePos(sconf, seed, regX, regZ);
            return 1 as libc::c_int;
        }
        8 | 9 => {
            *pos = getLargeStructurePos(sconf, seed, regX, regZ);
            return 1 as libc::c_int;
        }
        18 => {
            *pos = getLargeStructurePos(sconf, seed, regX, regZ);
            return (((*pos).x as libc::c_long * (*pos).x as int64_t
                + (*pos).z as libc::c_long * (*pos).z as int64_t) as libc::c_longlong
                >= 1008 as libc::c_int as libc::c_longlong * 1008 as libc::c_longlong)
                as libc::c_int;
        }
        10 => {
            *pos = getFeaturePos(sconf, seed, regX, regZ);
            setAttemptSeed(
                &mut seed,
                (*pos).x >> 4 as libc::c_int,
                (*pos).z >> 4 as libc::c_int,
            );
            return (nextInt(&mut seed, 5 as libc::c_int) == 0 as libc::c_int)
                as libc::c_int;
        }
        14 => {
            (*pos)
                .x = ((regX as uint32_t) << 4 as libc::c_int)
                .wrapping_add(9 as libc::c_int as libc::c_uint) as libc::c_int;
            (*pos)
                .z = ((regZ as uint32_t) << 4 as libc::c_int)
                .wrapping_add(9 as libc::c_int as libc::c_uint) as libc::c_int;
            seed = (regX as libc::c_ulonglong)
                .wrapping_mul(341873128712 as libc::c_ulonglong)
                .wrapping_add(
                    (regZ as libc::c_ulonglong)
                        .wrapping_mul(132897987541 as libc::c_ulonglong),
                )
                .wrapping_add(seed as libc::c_ulonglong)
                .wrapping_add(sconf.salt as libc::c_ulonglong) as uint64_t;
            setSeed(&mut seed, seed);
            return ((nextFloat(&mut seed) as libc::c_double) < 0.01f64) as libc::c_int;
        }
        15 => {
            return getMineshafts(mc, seed, regX, regZ, regX, regZ, pos, 1 as libc::c_int);
        }
        16 => {
            if mc >= MC_1_18 as libc::c_int {
                *pos = getFeaturePos(sconf, seed, regX, regZ);
                return 1 as libc::c_int;
            } else if mc >= MC_1_16 as libc::c_int {
                getRegPos(pos, &mut seed, regX, regZ, sconf);
                return (nextInt(&mut seed, 5 as libc::c_int) < 2 as libc::c_int)
                    as libc::c_int;
            } else {
                setAttemptSeed(
                    &mut seed,
                    regX << 4 as libc::c_int,
                    regZ << 4 as libc::c_int,
                );
                let mut valid: libc::c_int = (nextInt(&mut seed, 3 as libc::c_int)
                    == 0 as libc::c_int) as libc::c_int;
                (*pos)
                    .x = (((regX as uint64_t) << 4 as libc::c_int)
                    .wrapping_add(nextInt(&mut seed, 8 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) << 4 as libc::c_int)
                    as libc::c_int;
                (*pos)
                    .z = (((regZ as uint64_t) << 4 as libc::c_int)
                    .wrapping_add(nextInt(&mut seed, 8 as libc::c_int) as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong) << 4 as libc::c_int)
                    as libc::c_int;
                return valid;
            }
        }
        17 => {
            if mc >= MC_1_18 as libc::c_int {
                *pos = getFeaturePos(sconf, seed, regX, regZ);
                seed = chunkGenerateRnd(
                    seed,
                    (*pos).x >> 4 as libc::c_int,
                    (*pos).z >> 4 as libc::c_int,
                );
                return (nextInt(&mut seed, 5 as libc::c_int) >= 2 as libc::c_int)
                    as libc::c_int;
            } else {
                getRegPos(pos, &mut seed, regX, regZ, sconf);
                return (nextInt(&mut seed, 5 as libc::c_int) >= 2 as libc::c_int)
                    as libc::c_int;
            }
        }
        19 => {
            (*pos).x = ((regX as uint32_t) << 4 as libc::c_int) as libc::c_int;
            (*pos).z = ((regZ as uint32_t) << 4 as libc::c_int) as libc::c_int;
            seed = getPopulationSeed(mc, seed, (*pos).x, (*pos).z);
            if mc >= MC_1_18 as libc::c_int {
                let mut xr: Xoroshiro = Xoroshiro { lo: 0, hi: 0 };
                seed = (seed as libc::c_ulong)
                    .wrapping_add(
                        (10000 as libc::c_int * 4 as libc::c_int) as libc::c_ulong,
                    ) as uint64_t as uint64_t;
                xSetSeed(&mut xr, seed);
                if xNextFloat(&mut xr) as libc::c_double
                    >= 1.0f64 / 700 as libc::c_int as libc::c_double
                {
                    return 0 as libc::c_int;
                }
                (*pos).x += xNextIntJ(&mut xr, 16 as libc::c_int as uint32_t);
                (*pos).z += xNextIntJ(&mut xr, 16 as libc::c_int as uint32_t);
            } else {
                setSeed(&mut seed, seed.wrapping_add(sconf.salt as libc::c_ulong));
                if mc >= MC_1_17 as libc::c_int {
                    if nextFloat(&mut seed) as libc::c_double
                        >= 1.0f64 / 700 as libc::c_int as libc::c_double
                    {
                        return 0 as libc::c_int;
                    }
                } else if nextInt(&mut seed, 700 as libc::c_int) != 0 as libc::c_int {
                    return 0 as libc::c_int
                }
                (*pos).x += nextInt(&mut seed, 16 as libc::c_int);
                (*pos).z += nextInt(&mut seed, 16 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"ERR getStructurePos: unsupported structure type %d\n\0" as *const u8
                    as *const libc::c_char,
                structureType,
            );
            exit(-(1 as libc::c_int));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getMineshafts(
    mut mc: libc::c_int,
    mut seed: uint64_t,
    mut cx0: libc::c_int,
    mut cz0: libc::c_int,
    mut cx1: libc::c_int,
    mut cz1: libc::c_int,
    mut out: *mut Pos,
    mut nout: libc::c_int,
) -> libc::c_int {
    let mut s: uint64_t = 0;
    setSeed(&mut s, seed);
    let mut a: uint64_t = nextLong(&mut s);
    let mut b: uint64_t = nextLong(&mut s);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    i = cx0;
    while i <= cx1 {
        let mut aix: uint64_t = (i as libc::c_ulong).wrapping_mul(a) ^ seed;
        j = cz0;
        while j <= cz1 {
            setSeed(&mut s, aix ^ (j as libc::c_ulong).wrapping_mul(b));
            if mc >= MC_1_13 as libc::c_int {
                if (nextDouble(&mut s) < 0.004f64) as libc::c_int as libc::c_long != 0 {
                    if !out.is_null() && n < nout {
                        (*out.offset(n as isize))
                            .x = ((i as uint32_t) << 4 as libc::c_int) as libc::c_int;
                        (*out.offset(n as isize))
                            .z = ((j as uint32_t) << 4 as libc::c_int) as libc::c_int;
                    }
                    n += 1;
                }
            } else {
                skipNextN(&mut s, 1 as libc::c_int as uint64_t);
                if (nextDouble(&mut s) < 0.004f64) as libc::c_int as libc::c_long != 0 {
                    let mut d: libc::c_int = i;
                    if -i > d {
                        d = -i;
                    }
                    if j > d {
                        d = j;
                    }
                    if -j > d {
                        d = -j;
                    }
                    if d >= 80 as libc::c_int || nextInt(&mut s, 80 as libc::c_int) < d {
                        if !out.is_null() && n < nout {
                            (*out.offset(n as isize))
                                .x = ((i as uint32_t) << 4 as libc::c_int) as libc::c_int;
                            (*out.offset(n as isize))
                                .z = ((j as uint32_t) << 4 as libc::c_int) as libc::c_int;
                        }
                        n += 1;
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    return n;
}
unsafe extern "C" fn blocksInRange(
    mut p: *mut Pos,
    mut n: libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut ax: libc::c_int,
    mut az: libc::c_int,
    mut rsq: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    cnt = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        let mut dx: libc::c_double = ((*p.offset(i as isize)).x - x) as libc::c_double;
        let mut dz: libc::c_double = ((*p.offset(i as isize)).z - z) as libc::c_double;
        let mut px: libc::c_int = 0;
        let mut pz: libc::c_int = 0;
        px = 0 as libc::c_int;
        while px < ax {
            pz = 0 as libc::c_int;
            while pz < az {
                let mut ddx: libc::c_double = px as libc::c_double + dx;
                let mut ddz: libc::c_double = pz as libc::c_double + dz;
                cnt += (ddx * ddx + ddz * ddz <= rsq) as libc::c_int;
                pz += 1;
            }
            px += 1;
        }
        i += 1;
    }
    return cnt;
}
unsafe extern "C" fn checkAfkDist(
    mut d: *mut afk_meta_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
) {
    if x < 0 as libc::c_int || z < 0 as libc::c_int || x >= (*d).w || z >= (*d).h {
        return;
    }
    if *((*d).buf).offset((z * (*d).w + x) as isize) != 0 {
        return;
    }
    let mut q: libc::c_int = blocksInRange(
        (*d).p,
        (*d).n,
        x + (*d).x0,
        z + (*d).z0,
        (*d).ax,
        (*d).az,
        (*d).rsq,
    );
    *((*d).buf).offset((z * (*d).w + x) as isize) = q;
    if q >= (*d).best {
        if q > (*d).best {
            (*d).best = q;
            (*d).sumn = 1 as libc::c_int;
            (*d).sumx = ((*d).x0 + x) as int64_t;
            (*d).sumz = ((*d).z0 + z) as int64_t;
        } else {
            (*d).sumn += 1 as libc::c_int;
            let ref mut fresh0 = (*d).sumx;
            *fresh0 += ((*d).x0 + x) as libc::c_long;
            let ref mut fresh1 = (*d).sumz;
            *fresh1 += ((*d).z0 + z) as libc::c_long;
        }
        checkAfkDist(d, x, z - 1 as libc::c_int);
        checkAfkDist(d, x, z + 1 as libc::c_int);
        checkAfkDist(d, x - 1 as libc::c_int, z);
        checkAfkDist(d, x + 1 as libc::c_int, z);
        checkAfkDist(d, x - 1 as libc::c_int, z - 1 as libc::c_int);
        checkAfkDist(d, x - 1 as libc::c_int, z + 1 as libc::c_int);
        checkAfkDist(d, x + 1 as libc::c_int, z - 1 as libc::c_int);
        checkAfkDist(d, x + 1 as libc::c_int, z + 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getOptimalAfk(
    mut p: *mut Pos,
    mut ax: libc::c_int,
    mut ay: libc::c_int,
    mut az: libc::c_int,
    mut spcnt: *mut libc::c_int,
) -> Pos {
    let mut minX: int64_t = 2147483647 as libc::c_int as int64_t;
    let mut minZ: int64_t = 2147483647 as libc::c_int as int64_t;
    let mut maxX: int64_t = (-(2147483647 as libc::c_int) - 1 as libc::c_int) as int64_t;
    let mut maxZ: int64_t = (-(2147483647 as libc::c_int) - 1 as libc::c_int) as int64_t;
    let mut w: int64_t = 0;
    let mut h: int64_t = 0;
    let mut i: int64_t = 0;
    i = 0 as libc::c_int as int64_t;
    while i < 4 as libc::c_int as libc::c_long {
        if ((*p.offset(i as isize)).x as libc::c_long) < minX {
            minX = (*p.offset(i as isize)).x as int64_t;
        }
        if ((*p.offset(i as isize)).z as libc::c_long) < minZ {
            minZ = (*p.offset(i as isize)).z as int64_t;
        }
        if (*p.offset(i as isize)).x as libc::c_long > maxX {
            maxX = (*p.offset(i as isize)).x as int64_t;
        }
        if (*p.offset(i as isize)).z as libc::c_long > maxZ {
            maxZ = (*p.offset(i as isize)).z as int64_t;
        }
        i += 1;
    }
    minX += (ax / 2 as libc::c_int) as libc::c_long;
    minZ += (az / 2 as libc::c_int) as libc::c_long;
    maxX += (ax / 2 as libc::c_int) as libc::c_long;
    maxZ += (az / 2 as libc::c_int) as libc::c_long;
    let mut rsq: libc::c_double = 128.0f64 * 128.0f64
        - (ay * ay) as libc::c_double / 4.0f64;
    w = maxX - minX;
    h = maxZ - minZ;
    let mut afk: Pos = {
        let mut init = Pos {
            x: (*p.offset(0 as libc::c_int as isize)).x + ax / 2 as libc::c_int,
            z: (*p.offset(0 as libc::c_int as isize)).z + az / 2 as libc::c_int,
        };
        init
    };
    let mut cnt: libc::c_int = ax * az;
    let mut d: afk_meta_t = afk_meta_t {
        p: 0 as *mut Pos,
        n: 0,
        buf: 0 as *mut libc::c_int,
        x0: 0,
        z0: 0,
        w: 0,
        h: 0,
        ax: 0,
        az: 0,
        rsq: 0.,
        best: 0,
        sumn: 0,
        sumx: 0,
        sumz: 0,
    };
    d.p = p;
    d.n = 4 as libc::c_int;
    d
        .buf = calloc(
        (w * h) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    d.x0 = minX as libc::c_int;
    d.z0 = minZ as libc::c_int;
    d.w = w as libc::c_int;
    d.h = h as libc::c_int;
    d.ax = ax;
    d.az = az;
    d.rsq = rsq;
    let mut v: [libc::c_int; 6] = [0; 6];
    let mut dsp: [Pos; 6] = [
        {
            let mut init = Pos {
                x: ((*p.offset(0 as libc::c_int as isize)).x
                    + (*p.offset(2 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(0 as libc::c_int as isize)).z
                    + (*p.offset(2 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = Pos {
                x: ((*p.offset(1 as libc::c_int as isize)).x
                    + (*p.offset(3 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(1 as libc::c_int as isize)).z
                    + (*p.offset(3 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = Pos {
                x: ((*p.offset(0 as libc::c_int as isize)).x
                    + (*p.offset(1 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(0 as libc::c_int as isize)).z
                    + (*p.offset(1 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = Pos {
                x: ((*p.offset(2 as libc::c_int as isize)).x
                    + (*p.offset(3 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(2 as libc::c_int as isize)).z
                    + (*p.offset(3 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = Pos {
                x: ((*p.offset(0 as libc::c_int as isize)).x
                    + (*p.offset(3 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(0 as libc::c_int as isize)).z
                    + (*p.offset(3 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = Pos {
                x: ((*p.offset(1 as libc::c_int as isize)).x
                    + (*p.offset(2 as libc::c_int as isize)).x) / 2 as libc::c_int,
                z: ((*p.offset(1 as libc::c_int as isize)).z
                    + (*p.offset(2 as libc::c_int as isize)).z) / 2 as libc::c_int,
            };
            init
        },
    ];
    i = 0 as libc::c_int as int64_t;
    while i < 6 as libc::c_int as libc::c_long {
        v[i
            as usize] = blocksInRange(
            p,
            4 as libc::c_int,
            dsp[i as usize].x,
            dsp[i as usize].z,
            ax,
            az,
            rsq,
        );
        i += 1;
    }
    i = 0 as libc::c_int as int64_t;
    while i < 6 as libc::c_int as libc::c_long {
        let mut j: libc::c_int = 0;
        let mut jmax: libc::c_int = 0 as libc::c_int;
        let mut vmax: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            if v[j as usize] > vmax {
                jmax = j;
                vmax = v[j as usize];
            }
            j += 1;
        }
        if vmax <= ax * az {
            break;
        }
        d.best = vmax;
        d.sumn = 0 as libc::c_int;
        d.sumx = 0 as libc::c_int as int64_t;
        d.sumz = 0 as libc::c_int as int64_t;
        checkAfkDist(&mut d, dsp[jmax as usize].x - d.x0, dsp[jmax as usize].z - d.z0);
        if d.best > cnt {
            cnt = d.best;
            afk
                .x = round(d.sumx as libc::c_double / d.sumn as libc::c_double)
                as libc::c_int;
            afk
                .z = round(d.sumz as libc::c_double / d.sumn as libc::c_double)
                as libc::c_int;
            if cnt >= 3 as libc::c_int * ax * az {
                break;
            }
        }
        v[jmax as usize] = 0 as libc::c_int;
        i += 1;
    }
    if !spcnt.is_null() {
        *spcnt = cnt;
    }
    free(d.buf as *mut libc::c_void);
    return afk;
}
unsafe extern "C" fn mkdirp(mut path: *mut libc::c_char) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = strlen(path) as libc::c_int;
    let mut p: *mut libc::c_char = path;
    while *p as libc::c_int == '/' as i32 {
        p = p.offset(1);
    }
    while err == 0 && p < path.offset(len as isize) {
        let mut q: *mut libc::c_char = p;
        while *q as libc::c_int != 0 && !(*q as libc::c_int == '/' as i32) {
            q = q.offset(1);
        }
        if p != path {
            *p.offset(-(1 as libc::c_int) as isize) = '/' as i32 as libc::c_char;
        }
        *q = 0 as libc::c_int as libc::c_char;
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if stat(path, &mut st) == -(1 as libc::c_int) {
            err = mkdir(path, 0o773 as libc::c_int as __mode_t);
        } else if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
            err = 1 as libc::c_int;
        }
        p = q.offset(1 as libc::c_int as isize);
    }
    return err;
}
unsafe extern "C" fn searchAll48Thread(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut info: *mut threadinfo_t = data as *mut threadinfo_t;
    let mut seed: uint64_t = (*info).start;
    let mut end: uint64_t = (*info).end;
    let mut lp: *mut linked_seeds_t = &mut (*info).ls;
    (*lp).len = 0 as libc::c_int as size_t;
    let ref mut fresh2 = (*lp).next;
    *fresh2 = 0 as *mut linked_seeds_t;
    if !((*info).lowBits).is_null() {
        let mut hstep: uint64_t = ((1 as libc::c_ulonglong) << (*info).lowBitN)
            as uint64_t;
        let mut hmask: uint64_t = !hstep.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut mid: uint64_t = 0;
        let mut idx: libc::c_int = 0;
        mid = (*info).start & hmask;
        idx = 0 as libc::c_int;
        loop {
            seed = mid | *((*info).lowBits).offset(idx as isize);
            if !(seed < (*info).start) {
                break;
            }
            idx += 1;
        }
        while seed <= end {
            if ((*info).check).expect("non-null function pointer")(seed, (*info).data)
                as libc::c_long != 0
            {
                if !((*info).fp).is_null() {
                    fprintf(
                        (*info).fp,
                        b"%ld\n\0" as *const u8 as *const libc::c_char,
                        seed as int64_t,
                    );
                    fflush((*info).fp);
                } else {
                    (*lp).seeds[(*lp).len as usize] = seed;
                    let ref mut fresh3 = (*lp).len;
                    *fresh3 = (*fresh3).wrapping_add(1);
                    if (*lp).len
                        >= (::std::mem::size_of::<[uint64_t; 100]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
                            )
                    {
                        let mut n: *mut linked_seeds_t = malloc(
                            ::std::mem::size_of::<linked_seeds_t>() as libc::c_ulong,
                        ) as *mut linked_seeds_t;
                        if n.is_null() {
                            exit(1 as libc::c_int);
                        }
                        let ref mut fresh4 = (*lp).next;
                        *fresh4 = n;
                        lp = n;
                        (*lp).len = 0 as libc::c_int as size_t;
                        let ref mut fresh5 = (*lp).next;
                        *fresh5 = 0 as *mut linked_seeds_t;
                    }
                }
            }
            idx += 1;
            if idx >= (*info).lowBitCnt {
                idx = 0 as libc::c_int;
                mid = (mid as libc::c_ulong).wrapping_add(hstep) as uint64_t as uint64_t;
            }
            seed = mid | *((*info).lowBits).offset(idx as isize);
        }
    } else {
        while seed <= end {
            if ((*info).check).expect("non-null function pointer")(seed, (*info).data)
                as libc::c_long != 0
            {
                if !((*info).fp).is_null() {
                    fprintf(
                        (*info).fp,
                        b"%ld\n\0" as *const u8 as *const libc::c_char,
                        seed as int64_t,
                    );
                    fflush((*info).fp);
                } else {
                    (*lp).seeds[(*lp).len as usize] = seed;
                    let ref mut fresh6 = (*lp).len;
                    *fresh6 = (*fresh6).wrapping_add(1);
                    if (*lp).len
                        >= (::std::mem::size_of::<[uint64_t; 100]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
                            )
                    {
                        let mut n_0: *mut linked_seeds_t = malloc(
                            ::std::mem::size_of::<linked_seeds_t>() as libc::c_ulong,
                        ) as *mut linked_seeds_t;
                        if n_0.is_null() {
                            exit(1 as libc::c_int);
                        }
                        let ref mut fresh7 = (*lp).next;
                        *fresh7 = n_0;
                        lp = n_0;
                        (*lp).len = 0 as libc::c_int as size_t;
                        let ref mut fresh8 = (*lp).next;
                        *fresh8 = 0 as *mut linked_seeds_t;
                    }
                }
            }
            seed = seed.wrapping_add(1);
        }
    }
    pthread_exit(0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn searchAll48(
    mut seedbuf: *mut *mut uint64_t,
    mut buflen: *mut uint64_t,
    mut path: *const libc::c_char,
    mut threads: libc::c_int,
    mut lowBits: *const uint64_t,
    mut lowBitCnt: libc::c_int,
    mut lowBitN: libc::c_int,
    mut check: Option::<
        unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> libc::c_int,
    >,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut info: *mut threadinfo_t = malloc(
        (threads as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<threadinfo_t>() as libc::c_ulong),
    ) as *mut threadinfo_t;
    let mut tids: *mut thread_id_t = malloc(
        (threads as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<thread_id_t>() as libc::c_ulong),
    ) as *mut thread_id_t;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    if !path.is_null() {
        let mut pathlen: size_t = strlen(path);
        let mut dpath: [libc::c_char; 4096] = [0; 4096];
        if pathlen.wrapping_add(8 as libc::c_int as libc::c_ulong)
            >= ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        {
            current_block = 18318163450978097180;
        } else {
            strcpy(dpath.as_mut_ptr(), path);
            i = pathlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            loop {
                if !(i >= 0 as libc::c_int) {
                    current_block = 13056961889198038528;
                    break;
                }
                if dpath[i as usize] as libc::c_int == '/' as i32 {
                    dpath[i as usize] = 0 as libc::c_int as libc::c_char;
                    if mkdirp(dpath.as_mut_ptr()) != 0 {
                        current_block = 18318163450978097180;
                        break;
                    } else {
                        current_block = 13056961889198038528;
                        break;
                    }
                } else {
                    i -= 1;
                }
            }
        }
    } else if seedbuf.is_null() || buflen.is_null() {
        current_block = 18318163450978097180;
    } else {
        current_block = 13056961889198038528;
    }
    match current_block {
        13056961889198038528 => {
            t = 0 as libc::c_int;
            loop {
                if !(t < threads) {
                    current_block = 1345366029464561491;
                    break;
                }
                (*info.offset(t as isize))
                    .start = (t as libc::c_long
                    * (((1 as libc::c_int as int64_t) << 48 as libc::c_int)
                        - 1 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long) / threads as libc::c_long)
                    as uint64_t;
                (*info.offset(t as isize))
                    .end = ((t + 1 as libc::c_int) as libc::c_long
                    * (((1 as libc::c_int as int64_t) << 48 as libc::c_int)
                        - 1 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long) / threads as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as uint64_t;
                let ref mut fresh9 = (*info.offset(t as isize)).lowBits;
                *fresh9 = lowBits;
                (*info.offset(t as isize)).lowBitCnt = lowBitCnt;
                (*info.offset(t as isize)).lowBitN = lowBitN;
                let ref mut fresh10 = (*info.offset(t as isize)).check;
                *fresh10 = check;
                let ref mut fresh11 = (*info.offset(t as isize)).data;
                *fresh11 = data;
                if !path.is_null() {
                    snprintf(
                        ((*info.offset(t as isize)).path).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                        b"%s.part%d\0" as *const u8 as *const libc::c_char,
                        path,
                        t,
                    );
                    let mut fp: *mut FILE = fopen(
                        ((*info.offset(t as isize)).path).as_mut_ptr(),
                        b"a+\0" as *const u8 as *const libc::c_char,
                    );
                    if fp.is_null() {
                        current_block = 18318163450978097180;
                        break;
                    }
                    let mut c: libc::c_int = 0;
                    let mut nnl: libc::c_int = 0 as libc::c_int;
                    let mut buf: [libc::c_char; 32] = [0; 32];
                    i = 1 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if fseek(fp, -i as libc::c_long, 2 as libc::c_int) != 0 {
                            break;
                        }
                        c = fgetc(fp);
                        if c <= 0 as libc::c_int || nnl != 0 && c == '\n' as i32 {
                            break;
                        }
                        nnl |= (c != '\n' as i32) as libc::c_int;
                        i += 1;
                    }
                    if i < 32 as libc::c_int
                        && fseek(
                            fp,
                            (1 as libc::c_int - i) as libc::c_long,
                            2 as libc::c_int,
                        ) == 0
                        && fread(
                            buf.as_mut_ptr() as *mut libc::c_void,
                            (i - 1 as libc::c_int) as libc::c_ulong,
                            1 as libc::c_int as libc::c_ulong,
                            fp,
                        ) > 0 as libc::c_int as libc::c_ulong
                    {
                        let mut lentry: int64_t = 0;
                        if sscanf(
                            buf.as_mut_ptr(),
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            &mut lentry as *mut int64_t,
                        ) == 1 as libc::c_int
                        {
                            (*info.offset(t as isize)).start = lentry as uint64_t;
                            printf(
                                b"Continuing thread %d at seed %ld\n\0" as *const u8
                                    as *const libc::c_char,
                                t,
                                lentry,
                            );
                        }
                    }
                    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
                    let ref mut fresh12 = (*info.offset(t as isize)).fp;
                    *fresh12 = fp;
                } else {
                    (*info.offset(t as isize))
                        .path[0 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_char;
                    let ref mut fresh13 = (*info.offset(t as isize)).fp;
                    *fresh13 = 0 as *mut FILE;
                }
                t += 1;
            }
            match current_block {
                18318163450978097180 => {}
                _ => {
                    t = 0 as libc::c_int;
                    while t < threads {
                        pthread_create(
                            &mut *tids.offset(t as isize),
                            0 as *const pthread_attr_t,
                            Some(
                                searchAll48Thread
                                    as unsafe extern "C" fn(
                                        *mut libc::c_void,
                                    ) -> *mut libc::c_void,
                            ),
                            &mut *info.offset(t as isize) as *mut threadinfo_t
                                as *mut libc::c_void,
                        );
                        t += 1;
                    }
                    t = 0 as libc::c_int;
                    while t < threads {
                        pthread_join(
                            *tids.offset(t as isize),
                            0 as *mut *mut libc::c_void,
                        );
                        t += 1;
                    }
                    if !path.is_null() {
                        let mut fp_0: *mut FILE = fopen(
                            path,
                            b"w\0" as *const u8 as *const libc::c_char,
                        );
                        if fp_0.is_null() {
                            current_block = 18318163450978097180;
                        } else {
                            t = 0 as libc::c_int;
                            's_291: loop {
                                if !(t < threads) {
                                    current_block = 10067844863897285902;
                                    break;
                                }
                                rewind((*info.offset(t as isize)).fp);
                                let mut buffer: [libc::c_char; 4097] = [0; 4097];
                                let mut n: size_t = 0;
                                loop {
                                    n = fread(
                                        buffer.as_mut_ptr() as *mut libc::c_void,
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        4096 as libc::c_int as libc::c_ulong,
                                        (*info.offset(t as isize)).fp,
                                    );
                                    if !(n != 0) {
                                        break;
                                    }
                                    if !(fwrite(
                                        buffer.as_mut_ptr() as *const libc::c_void,
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        n,
                                        fp_0,
                                    ) == 0)
                                    {
                                        continue;
                                    }
                                    fclose(fp_0);
                                    current_block = 18318163450978097180;
                                    break 's_291;
                                }
                                fclose((*info.offset(t as isize)).fp);
                                remove(((*info.offset(t as isize)).path).as_mut_ptr());
                                t += 1;
                            }
                            match current_block {
                                18318163450978097180 => {}
                                _ => {
                                    fclose(fp_0);
                                    if !seedbuf.is_null() && !buflen.is_null() {
                                        *seedbuf = loadSavedSeeds(path, buflen);
                                    }
                                    current_block = 6712462580143783635;
                                }
                            }
                        }
                    } else {
                        *buflen = 0 as libc::c_int as uint64_t;
                        t = 0 as libc::c_int;
                        while t < threads {
                            let mut lp: *mut linked_seeds_t = &mut (*info
                                .offset(t as isize))
                                .ls;
                            loop {
                                *buflen = (*buflen as libc::c_ulong).wrapping_add((*lp).len)
                                    as uint64_t as uint64_t;
                                lp = (*lp).next;
                                if lp.is_null() {
                                    break;
                                }
                            }
                            t += 1;
                        }
                        *seedbuf = malloc(
                            (*buflen)
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint64_t;
                        if (*seedbuf).is_null() {
                            exit(1 as libc::c_int);
                        }
                        i = 0 as libc::c_int;
                        t = 0 as libc::c_int;
                        while t < threads {
                            let mut lp_0: *mut linked_seeds_t = &mut (*info
                                .offset(t as isize))
                                .ls;
                            loop {
                                memcpy(
                                    (*seedbuf).offset(i as isize) as *mut libc::c_void,
                                    ((*lp_0).seeds).as_mut_ptr() as *const libc::c_void,
                                    ((*lp_0).len)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
                                        ),
                                );
                                i = (i as libc::c_ulong).wrapping_add((*lp_0).len)
                                    as libc::c_int as libc::c_int;
                                let mut tmp: *mut linked_seeds_t = lp_0;
                                lp_0 = (*lp_0).next;
                                if tmp
                                    != &mut (*info.offset(t as isize)).ls as *mut linked_seeds_t
                                {
                                    free(tmp as *mut libc::c_void);
                                }
                                if lp_0.is_null() {
                                    break;
                                }
                            }
                            t += 1;
                        }
                        current_block = 6712462580143783635;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        18318163450978097180 => {
            err = 1 as libc::c_int;
        }
        _ => {}
    }
    free(tids as *mut libc::c_void);
    free(info as *mut libc::c_void);
    return err;
}
#[inline]
unsafe extern "C" fn scanForQuadBits(
    sconf: StructureConfig,
    mut radius: libc::c_int,
    mut s48: uint64_t,
    mut lbit: uint64_t,
    mut lbitn: libc::c_int,
    mut invB: uint64_t,
    mut x: int64_t,
    mut z: int64_t,
    mut w: int64_t,
    mut h: int64_t,
    mut qplist: *mut Pos,
    mut n: libc::c_int,
) -> libc::c_int {
    let m: uint64_t = ((1 as libc::c_ulonglong) << lbitn) as uint64_t;
    let A: uint64_t = 341873128712 as libc::c_ulonglong as uint64_t;
    if n < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    lbit &= m.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut i: int64_t = 0;
    let mut j: int64_t = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    i = x;
    while i <= x + w {
        let mut sx: uint64_t = s48.wrapping_add(A.wrapping_mul(i as libc::c_ulong));
        j = (z as libc::c_ulong & !m.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            | lbit.wrapping_sub(sx).wrapping_mul(invB)
                & m.wrapping_sub(1 as libc::c_int as libc::c_ulong)) as int64_t;
        if j < z {
            j = (j as libc::c_ulong).wrapping_add(m) as int64_t as int64_t;
        }
        while j <= z + h {
            let mut sp: uint64_t = moveStructure(
                s48,
                -i as libc::c_int,
                -j as libc::c_int,
            );
            if !(sp & m.wrapping_sub(1 as libc::c_int as libc::c_ulong) != lbit) {
                if isQuadBase(sconf, sp, radius) != 0. {
                    (*qplist.offset(cnt as isize)).x = i as libc::c_int;
                    (*qplist.offset(cnt as isize)).z = j as libc::c_int;
                    cnt += 1;
                    if cnt >= n {
                        return cnt;
                    }
                }
            }
            j = (j as libc::c_ulong).wrapping_add(m) as int64_t as int64_t;
        }
        i += 1;
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn scanForQuads(
    sconf: StructureConfig,
    mut radius: libc::c_int,
    mut s48: uint64_t,
    mut lowBits: *const uint64_t,
    mut lowBitCnt: libc::c_int,
    mut lowBitN: libc::c_int,
    mut salt: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut qplist: *mut Pos,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut invB: uint64_t = 0;
    if lowBitN == 20 as libc::c_int {
        invB = 132477 as libc::c_ulonglong as uint64_t;
    } else if lowBitN == 48 as libc::c_int {
        invB = 211541297333629 as libc::c_ulonglong as uint64_t;
    } else {
        invB = mulInv(
            132897987541 as libc::c_ulonglong as uint64_t,
            ((1 as libc::c_ulonglong) << lowBitN) as uint64_t,
        );
    }
    i = 0 as libc::c_int;
    while i < lowBitCnt {
        cnt
            += scanForQuadBits(
                sconf,
                radius,
                s48,
                (*lowBits.offset(i as isize)).wrapping_sub(salt),
                lowBitN,
                invB,
                x as int64_t,
                z as int64_t,
                w as int64_t,
                h as int64_t,
                qplist.offset(cnt as isize),
                n - cnt,
            );
        if cnt >= n {
            break;
        }
        i += 1;
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn locateBiome(
    mut g: *const Generator,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut radius: libc::c_int,
    mut validBiomes: *const libc::c_char,
    mut rng: *mut uint64_t,
    mut passes: *mut libc::c_int,
) -> Pos {
    let mut out: Pos = {
        let mut init = Pos { x: x, z: z };
        init
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    found = 0 as libc::c_int;
    if (*g).mc >= MC_1_18 as libc::c_int {
        x >>= 2 as libc::c_int;
        z >>= 2 as libc::c_int;
        radius >>= 2 as libc::c_int;
        let mut dat: uint64_t = 0 as libc::c_int as uint64_t;
        j = -radius;
        while j <= radius {
            i = -radius;
            while i <= radius {
                let mut id: libc::c_int = 0;
                let mut xi: libc::c_int = x + i;
                let mut zj: libc::c_int = z + j;
                id = sampleBiomeNoise(
                    &(*g).c2rust_unnamed.c2rust_unnamed_0.bn,
                    0 as *mut int64_t,
                    xi,
                    y,
                    zj,
                    &mut dat,
                    0 as libc::c_int as uint32_t,
                );
                if !(*validBiomes.offset(id as isize) == 0) {
                    if found == 0 as libc::c_int
                        || nextInt(rng, found + 1 as libc::c_int) == 0 as libc::c_int
                    {
                        out.x = x + i << 2 as libc::c_int;
                        out.z = z + j << 2 as libc::c_int;
                    }
                    found += 1;
                }
                i += 1;
            }
            j += 1;
        }
    } else {
        let mut x1: libc::c_int = x - radius >> 2 as libc::c_int;
        let mut z1: libc::c_int = z - radius >> 2 as libc::c_int;
        let mut x2: libc::c_int = x + radius >> 2 as libc::c_int;
        let mut z2: libc::c_int = z + radius >> 2 as libc::c_int;
        let mut width: libc::c_int = x2 - x1 + 1 as libc::c_int;
        let mut height: libc::c_int = z2 - z1 + 1 as libc::c_int;
        let mut r: Range = {
            let mut init = Range {
                scale: 4 as libc::c_int,
                x: x1,
                z: z1,
                sx: width,
                sz: height,
                y: y,
                sy: 1 as libc::c_int,
            };
            init
        };
        let mut ids: *mut libc::c_int = allocCache(g, r);
        genBiomes(g, ids, r);
        if (*g).mc >= MC_1_13 as libc::c_int {
            i = 0 as libc::c_int;
            j = 2 as libc::c_int;
            while i < width * height {
                if !(*validBiomes.offset(*ids.offset(i as isize) as isize) == 0) {
                    if found == 0 as libc::c_int
                        || {
                            let fresh14 = j;
                            j = j + 1;
                            nextInt(rng, fresh14) == 0 as libc::c_int
                        }
                    {
                        out.x = x1 + i % width << 2 as libc::c_int;
                        out.z = z1 + i / width << 2 as libc::c_int;
                        found = 1 as libc::c_int;
                    }
                }
                i += 1;
            }
            found = j - 2 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
            while i < width * height {
                if !(*validBiomes.offset(*ids.offset(i as isize) as isize) == 0) {
                    if found == 0 as libc::c_int
                        || nextInt(rng, found + 1 as libc::c_int) == 0 as libc::c_int
                    {
                        out.x = x1 + i % width << 2 as libc::c_int;
                        out.z = z1 + i / width << 2 as libc::c_int;
                        found += 1;
                    }
                }
                i += 1;
            }
        }
        free(ids as *mut libc::c_void);
    }
    if !passes.is_null() {
        *passes = found;
    }
    return out;
}
#[inline]
unsafe extern "C" fn valid_1x1(
    mut g: *const Generator,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut r: Range,
    mut buf: *mut libc::c_int,
    mut valid: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_int = buf
        .offset((x - r.x) as isize)
        .offset(((z - r.z) * r.sx) as isize)
        .offset(((y - r.y) * (r.sx * r.sz)) as isize);
    if *p != 0 {
        return 1 as libc::c_int;
    }
    *p = -(1 as libc::c_int);
    let mut id: libc::c_int = getBiomeAt(g, 4 as libc::c_int, x, y, z);
    return *valid.offset(id as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn areBiomesViable(
    mut g: *const Generator,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
    mut rad: libc::c_int,
    mut validBiomes: *const libc::c_char,
    mut approx: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut x1: libc::c_int = x - rad >> 2 as libc::c_int;
    let mut x2: libc::c_int = x + rad >> 2 as libc::c_int;
    let mut sx: libc::c_int = x2 - x1 + 1 as libc::c_int;
    let mut z1: libc::c_int = z - rad >> 2 as libc::c_int;
    let mut z2: libc::c_int = z + rad >> 2 as libc::c_int;
    let mut sz: libc::c_int = z2 - z1 + 1 as libc::c_int;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    if (*g).mc >= MC_1_18 as libc::c_int {
        y1 = y - rad >> 2 as libc::c_int;
        y2 = y + rad >> 2 as libc::c_int;
        sy = y2 - y1 + 1 as libc::c_int;
    } else {
        y2 = 0 as libc::c_int;
        y1 = y2;
        sy = 1 as libc::c_int;
    }
    let mut r: Range = {
        let mut init = Range {
            scale: 4 as libc::c_int,
            x: x1,
            z: z1,
            sx: sx,
            sz: sz,
            y: y1,
            sy: sy,
        };
        init
    };
    let mut ids: *mut libc::c_int = allocCache(g, r);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut viable: libc::c_int = 1 as libc::c_int;
    let mut v: *const libc::c_char = validBiomes;
    if valid_1x1(g, x1, y1, z1, r, ids, v) == 0 {
        current_block = 6101713967990833207;
    } else if valid_1x1(g, x2, y2, z2, r, ids, v) == 0 {
        current_block = 6101713967990833207;
    } else if valid_1x1(g, x1, y1, z2, r, ids, v) == 0 {
        current_block = 6101713967990833207;
    } else if valid_1x1(g, x2, y2, z1, r, ids, v) == 0 {
        current_block = 6101713967990833207;
    } else {
        if (*g).mc >= MC_1_18 as libc::c_int {
            if valid_1x1(g, x1, y2, z1, r, ids, v) == 0 {
                current_block = 6101713967990833207;
            } else if valid_1x1(g, x2, y1, z2, r, ids, v) == 0 {
                current_block = 6101713967990833207;
            } else if valid_1x1(g, x1, y2, z2, r, ids, v) == 0 {
                current_block = 6101713967990833207;
            } else if valid_1x1(g, x2, y1, z1, r, ids, v) == 0 {
                current_block = 6101713967990833207;
            } else {
                current_block = 26972500619410423;
            }
        } else {
            current_block = 26972500619410423;
        }
        match current_block {
            6101713967990833207 => {}
            _ => {
                if approx >= 1 as libc::c_int {
                    viable = 1 as libc::c_int;
                    current_block = 3222590281903869779;
                } else if (*g).mc >= MC_1_18 as libc::c_int {
                    i = 0 as libc::c_int;
                    's_117: loop {
                        if !(i < sx) {
                            current_block = 3222590281903869779;
                            break;
                        }
                        j = 0 as libc::c_int;
                        while j < sy {
                            k = 0 as libc::c_int;
                            while k < sz {
                                if valid_1x1(g, x1 + i, y1 + j, z1 + k, r, ids, v) == 0 {
                                    current_block = 6101713967990833207;
                                    break 's_117;
                                }
                                k += 1;
                            }
                            j += 1;
                        }
                        i += 1;
                    }
                } else {
                    viable = (genBiomes(g, ids, r) == 0) as libc::c_int;
                    if viable != 0 {
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < sx * sy * sz) {
                                current_block = 3222590281903869779;
                                break;
                            }
                            if *v.offset(*ids.offset(i as isize) as isize) == 0 {
                                current_block = 6101713967990833207;
                                break;
                            }
                            i += 1;
                        }
                    } else {
                        current_block = 3222590281903869779;
                    }
                }
            }
        }
    }
    match current_block {
        6101713967990833207 => {
            viable = 0 as libc::c_int;
        }
        _ => {}
    }
    free(ids as *mut libc::c_void);
    return viable;
}
#[no_mangle]
pub unsafe extern "C" fn getValidStrongholdBiomes(
    mut mc: libc::c_int,
) -> *const libc::c_char {
    static mut strongholdBiomes: [libc::c_int; 60] = [
        plains as libc::c_int,
        desert as libc::c_int,
        mountains as libc::c_int,
        forest as libc::c_int,
        taiga as libc::c_int,
        snowy_tundra as libc::c_int,
        snowy_mountains as libc::c_int,
        mushroom_fields as libc::c_int,
        mushroom_field_shore as libc::c_int,
        desert_hills as libc::c_int,
        wooded_hills as libc::c_int,
        taiga_hills as libc::c_int,
        mountain_edge as libc::c_int,
        jungle as libc::c_int,
        jungle_hills as libc::c_int,
        jungle_edge as libc::c_int,
        stone_shore as libc::c_int,
        birch_forest as libc::c_int,
        birch_forest_hills as libc::c_int,
        dark_forest as libc::c_int,
        snowy_taiga as libc::c_int,
        snowy_taiga_hills as libc::c_int,
        giant_tree_taiga as libc::c_int,
        giant_tree_taiga_hills as libc::c_int,
        wooded_mountains as libc::c_int,
        savanna as libc::c_int,
        savanna_plateau as libc::c_int,
        badlands as libc::c_int,
        wooded_badlands_plateau as libc::c_int,
        badlands_plateau as libc::c_int,
        sunflower_plains as libc::c_int,
        desert_lakes as libc::c_int,
        gravelly_mountains as libc::c_int,
        flower_forest as libc::c_int,
        taiga_mountains as libc::c_int,
        ice_spikes as libc::c_int,
        modified_jungle as libc::c_int,
        modified_jungle_edge as libc::c_int,
        tall_birch_forest as libc::c_int,
        tall_birch_hills as libc::c_int,
        dark_forest_hills as libc::c_int,
        snowy_taiga_mountains as libc::c_int,
        giant_spruce_taiga as libc::c_int,
        giant_spruce_taiga_hills as libc::c_int,
        modified_gravelly_mountains as libc::c_int,
        shattered_savanna as libc::c_int,
        shattered_savanna_plateau as libc::c_int,
        eroded_badlands as libc::c_int,
        modified_wooded_badlands_plateau as libc::c_int,
        modified_badlands_plateau as libc::c_int,
        bamboo_jungle as libc::c_int,
        bamboo_jungle_hills as libc::c_int,
        dripstone_caves as libc::c_int,
        lush_caves as libc::c_int,
        meadow as libc::c_int,
        grove as libc::c_int,
        snowy_slopes as libc::c_int,
        stony_peaks as libc::c_int,
        jagged_peaks as libc::c_int,
        frozen_peaks as libc::c_int,
    ];
    let mut i: libc::c_uint = 0;
    static mut v15: [libc::c_char; 256] = [0; 256];
    static mut v17: [libc::c_char; 256] = [0; 256];
    static mut v18: [libc::c_char; 256] = [0; 256];
    let mut valid: *mut libc::c_char = if mc <= MC_1_15 as libc::c_int {
        v15.as_mut_ptr()
    } else if mc <= MC_1_17 as libc::c_int {
        v17.as_mut_ptr()
    } else {
        v18.as_mut_ptr()
    };
    if *valid.offset(strongholdBiomes[0 as libc::c_int as usize] as isize) == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 60]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            *valid
                .offset(
                    strongholdBiomes[i as usize] as isize,
                ) = 1 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1);
        }
        if mc >= MC_1_18 as libc::c_int {
            *valid
                .offset(
                    stone_shore as libc::c_int as isize,
                ) = 0 as libc::c_int as libc::c_char;
        } else if mc >= MC_1_16 as libc::c_int {
            *valid
                .offset(
                    bamboo_jungle as libc::c_int as isize,
                ) = 0 as libc::c_int as libc::c_char;
            *valid
                .offset(
                    bamboo_jungle_hills as libc::c_int as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
    }
    return valid;
}
#[no_mangle]
pub unsafe extern "C" fn initFirstStronghold(
    mut sh: *mut StrongholdIter,
    mut mc: libc::c_int,
    mut s48: uint64_t,
) -> Pos {
    let mut dist: libc::c_double = 0.;
    let mut angle: libc::c_double = 0.;
    let mut rnds: uint64_t = 0;
    let mut p: Pos = Pos { x: 0, z: 0 };
    setSeed(&mut rnds, s48);
    angle = 2.0f64 * 3.141592653589793f64 * nextDouble(&mut rnds);
    if mc >= MC_1_9 as libc::c_int {
        dist = 4.0f64 * 32.0f64
            + (nextDouble(&mut rnds) - 0.5f64) * 32 as libc::c_int as libc::c_double
                * 2.5f64;
    } else {
        dist = (1.25f64 + nextDouble(&mut rnds)) * 32.0f64;
    }
    p
        .x = ((round(cos(angle) * dist) as libc::c_int) << 4 as libc::c_int)
        + 8 as libc::c_int;
    p
        .z = ((round(sin(angle) * dist) as libc::c_int) << 4 as libc::c_int)
        + 8 as libc::c_int;
    if !sh.is_null() {
        let ref mut fresh15 = (*sh).pos.z;
        *fresh15 = 0 as libc::c_int;
        (*sh).pos.x = *fresh15;
        (*sh).nextapprox = p;
        (*sh).index = 0 as libc::c_int;
        (*sh).ringnum = 0 as libc::c_int;
        (*sh).ringmax = 3 as libc::c_int;
        (*sh).ringidx = 0 as libc::c_int;
        (*sh).angle = angle;
        (*sh).dist = dist;
        (*sh).rnds = rnds;
        (*sh).mc = mc;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn nextStronghold(
    mut sh: *mut StrongholdIter,
    mut g: *const Generator,
) -> libc::c_int {
    (*sh)
        .pos = locateBiome(
        g,
        (*sh).nextapprox.x,
        0 as libc::c_int,
        (*sh).nextapprox.z,
        112 as libc::c_int,
        getValidStrongholdBiomes((*sh).mc),
        &mut (*sh).rnds,
        0 as *mut libc::c_int,
    );
    let ref mut fresh16 = (*sh).ringidx;
    *fresh16 += 1;
    (*sh).angle
        += 2 as libc::c_int as libc::c_double * 3.141592653589793f64
            / (*sh).ringmax as libc::c_double;
    if (*sh).ringidx == (*sh).ringmax {
        let ref mut fresh17 = (*sh).ringnum;
        *fresh17 += 1;
        (*sh).ringidx = 0 as libc::c_int;
        (*sh)
            .ringmax = (*sh).ringmax
            + 2 as libc::c_int * (*sh).ringmax / ((*sh).ringnum + 1 as libc::c_int);
        if (*sh).ringmax > 128 as libc::c_int - (*sh).index {
            (*sh).ringmax = 128 as libc::c_int - (*sh).index;
        }
        (*sh).angle += nextDouble(&mut (*sh).rnds) * 3.141592653589793f64 * 2.0f64;
    }
    if (*sh).mc >= MC_1_9 as libc::c_int {
        (*sh)
            .dist = 4.0f64 * 32.0f64 + 6.0f64 * (*sh).ringnum as libc::c_double * 32.0f64
            + (nextDouble(&mut (*sh).rnds) - 0.5f64)
                * 32 as libc::c_int as libc::c_double * 2.5f64;
    } else {
        (*sh).dist = (1.25f64 + nextDouble(&mut (*sh).rnds)) * 32.0f64;
    }
    (*sh)
        .nextapprox
        .x = ((round(cos((*sh).angle) * (*sh).dist) as libc::c_int) << 4 as libc::c_int)
        + 8 as libc::c_int;
    (*sh)
        .nextapprox
        .z = ((round(sin((*sh).angle) * (*sh).dist) as libc::c_int) << 4 as libc::c_int)
        + 8 as libc::c_int;
    let ref mut fresh18 = (*sh).index;
    *fresh18 += 1;
    return (if (*sh).mc >= MC_1_9 as libc::c_int {
        128 as libc::c_int
    } else {
        3 as libc::c_int
    }) - ((*sh).index - 1 as libc::c_int);
}
unsafe extern "C" fn getGrassProbability(
    mut seed: uint64_t,
    mut biome: libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_double {
    match biome {
        1 => return 1.0f64,
        3 => return 0.8f64,
        4 => return 1.0f64,
        5 => return 1.0f64,
        6 => return 0.3f64,
        7 => return 0.15f64,
        16 => return 0.0f64,
        12 => return 0.02f64,
        13 => return 0.02f64,
        18 => return 1.0f64,
        19 => return 1.0f64,
        20 => return 1.0f64,
        21 => return 1.0f64,
        22 => return 1.0f64,
        23 => return 1.0f64,
        27 => return 1.0f64,
        28 => return 1.0f64,
        29 => return 0.9f64,
        30 => return 0.1f64,
        31 => return 0.1f64,
        32 => return 0.6f64,
        33 => return 0.6f64,
        34 => return 0.2f64,
        35 => return 1.0f64,
        36 => return 0.9f64,
        38 => return 0.0f64,
        39 => return 0.0f64,
        129 => return 1.0f64,
        131 => return 0.2f64,
        132 => return 1.0f64,
        133 => return 1.0f64,
        134 => return 0.9f64,
        149 => return 1.0f64,
        151 => return 1.0f64,
        155 => return 1.0f64,
        156 => return 1.0f64,
        157 => return 0.9f64,
        158 => return 0.1f64,
        160 => return 0.6f64,
        161 => return 0.6f64,
        162 => return 0.2f64,
        163 => return 1.0f64,
        164 => return 1.0f64,
        168 => return 0.4f64,
        169 => return 0.4f64,
        _ => return 0 as libc::c_int as libc::c_double,
    };
}
unsafe extern "C" fn getValidSpawnBiomes() -> *const libc::c_char {
    static mut biomesToSpawnIn: [libc::c_int; 7] = [
        forest as libc::c_int,
        plains as libc::c_int,
        taiga as libc::c_int,
        taiga_hills as libc::c_int,
        wooded_hills as libc::c_int,
        jungle as libc::c_int,
        jungle_hills as libc::c_int,
    ];
    static mut isValid: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_uint = 0;
    if isValid[biomesToSpawnIn[0 as libc::c_int as usize] as usize] == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isValid[biomesToSpawnIn[i as usize]
                as usize] = 1 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1);
        }
    }
    return isValid.as_mut_ptr();
}
unsafe extern "C" fn findServerSpawn(
    mut g: *const Generator,
    mut chunkX: libc::c_int,
    mut chunkZ: libc::c_int,
    mut bx: *mut libc::c_double,
    mut bz: *mut libc::c_double,
    mut bn: *mut libc::c_double,
    mut accum: *mut libc::c_double,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    if (*g).mc >= MC_1_18 as libc::c_int {
        x = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            z = 0 as libc::c_int;
            while z < 4 as libc::c_int {
                let mut x4: libc::c_int = (chunkX << 2 as libc::c_int) + x;
                let mut z4: libc::c_int = (chunkZ << 2 as libc::c_int) + z;
                let mut id: libc::c_int = getBiomeAt(
                    g,
                    4 as libc::c_int,
                    x4,
                    16 as libc::c_int,
                    z4,
                );
                if isOceanic(id) != 0 || id == river as libc::c_int {
                    z += 1;
                } else {
                    *bx = (x4 << 2 as libc::c_int) as libc::c_double;
                    *bz = (z4 << 2 as libc::c_int) as libc::c_double;
                    *bn = 1 as libc::c_int as libc::c_double;
                    return 1 as libc::c_int;
                }
            }
            x += 1;
        }
        return 0 as libc::c_int;
    } else {
        let mut r: Range = {
            let mut init = Range {
                scale: 1 as libc::c_int,
                x: chunkX << 4 as libc::c_int,
                z: chunkZ << 4 as libc::c_int,
                sx: 16 as libc::c_int,
                sz: 16 as libc::c_int,
                y: 0 as libc::c_int,
                sy: 1 as libc::c_int,
            };
            init
        };
        let mut area: *mut libc::c_int = allocCache(g, r);
        genBiomes(g, area, r);
        x = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            z = 0 as libc::c_int;
            while z < 16 as libc::c_int {
                let mut pos: Pos = {
                    let mut init = Pos { x: r.x + x, z: r.z + z };
                    init
                };
                let mut id_0: libc::c_int = *area
                    .offset((z * 16 as libc::c_int + x) as isize);
                let mut gp: libc::c_double = getGrassProbability(
                    (*g).mc as uint64_t,
                    id_0,
                    pos.x,
                    pos.z,
                );
                if !(gp == 0 as libc::c_int as libc::c_double) {
                    *bx += *accum * gp * pos.x as libc::c_double;
                    *bz += *accum * gp * pos.z as libc::c_double;
                    *bn += *accum * gp;
                    *accum *= 1 as libc::c_int as libc::c_double - gp;
                    if *accum < 0.001f64 {
                        free(area as *mut libc::c_void);
                        return 1 as libc::c_int;
                    }
                }
                z += 1;
            }
            x += 1;
        }
        free(area as *mut libc::c_void);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn getSpawnDist(
    mut g: *const Generator,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> uint64_t {
    let mut np: [int64_t; 6] = [0; 6];
    let mut flags: uint32_t = (SAMPLE_NO_DEPTH as libc::c_int
        | SAMPLE_NO_BIOME as libc::c_int) as uint32_t;
    sampleBiomeNoise(
        &(*g).c2rust_unnamed.c2rust_unnamed_0.bn,
        np.as_mut_ptr(),
        x >> 2 as libc::c_int,
        0 as libc::c_int,
        z >> 2 as libc::c_int,
        0 as *mut uint64_t,
        flags,
    );
    let spawn_np: [[int64_t; 2]; 7] = [
        [-(10000 as libc::c_int) as int64_t, 10000 as libc::c_int as int64_t],
        [-(10000 as libc::c_int) as int64_t, 10000 as libc::c_int as int64_t],
        [-(1100 as libc::c_int) as int64_t, 10000 as libc::c_int as int64_t],
        [-(10000 as libc::c_int) as int64_t, 10000 as libc::c_int as int64_t],
        [0 as libc::c_int as int64_t, 0 as libc::c_int as int64_t],
        [-(10000 as libc::c_int) as int64_t, -(1600 as libc::c_int) as int64_t],
        [1600 as libc::c_int as int64_t, 10000 as libc::c_int as int64_t],
    ];
    let mut ds: uint64_t = 0 as libc::c_int as uint64_t;
    let mut ds1: uint64_t = 0 as libc::c_int as uint64_t;
    let mut ds2: uint64_t = 0 as libc::c_int as uint64_t;
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut q: uint64_t = 0;
    let mut i: uint64_t = 0;
    i = 0 as libc::c_int as uint64_t;
    while i < 5 as libc::c_int as libc::c_ulong {
        a = (np[i as usize] as libc::c_ulong)
            .wrapping_sub(spawn_np[i as usize][1 as libc::c_int as usize] as uint64_t);
        b = (-np[i as usize] as libc::c_ulong)
            .wrapping_add(spawn_np[i as usize][0 as libc::c_int as usize] as uint64_t);
        q = if a as int64_t > 0 as libc::c_int as libc::c_long {
            a
        } else if b as int64_t > 0 as libc::c_int as libc::c_long {
            b
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        ds = (ds as libc::c_ulong).wrapping_add(q.wrapping_mul(q)) as uint64_t
            as uint64_t;
        i = i.wrapping_add(1);
    }
    a = (np[5 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_sub(
            spawn_np[5 as libc::c_int as usize][1 as libc::c_int as usize] as uint64_t,
        );
    b = (-np[5 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            spawn_np[5 as libc::c_int as usize][0 as libc::c_int as usize] as uint64_t,
        );
    q = if a as int64_t > 0 as libc::c_int as libc::c_long {
        a
    } else if b as int64_t > 0 as libc::c_int as libc::c_long {
        b
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    ds1 = ds.wrapping_add(q.wrapping_mul(q));
    a = (np[5 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_sub(
            spawn_np[6 as libc::c_int as usize][1 as libc::c_int as usize] as uint64_t,
        );
    b = (-np[5 as libc::c_int as usize] as libc::c_ulong)
        .wrapping_add(
            spawn_np[6 as libc::c_int as usize][0 as libc::c_int as usize] as uint64_t,
        );
    q = if a as int64_t > 0 as libc::c_int as libc::c_long {
        a
    } else if b as int64_t > 0 as libc::c_int as libc::c_long {
        b
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    ds2 = ds.wrapping_add(q.wrapping_mul(q));
    return if ds1 <= ds2 { ds1 } else { ds2 };
}
unsafe extern "C" fn findFittest(
    mut g: *const Generator,
    mut pos: *mut Pos,
    mut fitness: *mut uint64_t,
    mut maxrad: libc::c_double,
    mut step: libc::c_double,
) {
    let mut rad: libc::c_double = step;
    let mut ang: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut p: Pos = *pos;
    while rad <= maxrad {
        let mut x: libc::c_int = p.x + (sin(ang) * rad) as libc::c_int;
        let mut z: libc::c_int = p.z + (cos(ang) * rad) as libc::c_int;
        let mut d: libc::c_double = (x as libc::c_double * x as libc::c_double
            + z as libc::c_double * z as libc::c_double)
            / (2500 as libc::c_int * 2500 as libc::c_int) as libc::c_double;
        let mut fit: uint64_t = (d * d * 1e8f64) as uint64_t;
        fit = (fit as libc::c_ulong).wrapping_add(getSpawnDist(g, x, z)) as uint64_t
            as uint64_t;
        if fit < *fitness {
            (*pos).x = x;
            (*pos).z = z;
            *fitness = fit;
        }
        ang += step / rad;
        if ang <= 3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double {
            continue;
        }
        ang = 0 as libc::c_int as libc::c_double;
        rad += step;
    }
}
unsafe extern "C" fn findFittestPos(mut g: *const Generator) -> Pos {
    let mut spawn: Pos = {
        let mut init = Pos {
            x: 0 as libc::c_int,
            z: 0 as libc::c_int,
        };
        init
    };
    let mut fitness: uint64_t = getSpawnDist(g, 0 as libc::c_int, 0 as libc::c_int);
    findFittest(g, &mut spawn, &mut fitness, 2048.0f64, 512.0f64);
    findFittest(g, &mut spawn, &mut fitness, 512.0f64, 32.0f64);
    spawn.x = ((spawn.x >> 4 as libc::c_int) << 4 as libc::c_int) + 8 as libc::c_int;
    spawn.z = ((spawn.z >> 4 as libc::c_int) << 4 as libc::c_int) + 8 as libc::c_int;
    return spawn;
}
#[no_mangle]
pub unsafe extern "C" fn getSpawn(mut g: *const Generator) -> Pos {
    let mut isSpawnBiome: *const libc::c_char = getValidSpawnBiomes();
    let mut spawn: Pos = Pos { x: 0, z: 0 };
    let mut found: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rnd: uint64_t = 0 as libc::c_int as uint64_t;
    if (*g).mc <= MC_1_17 as libc::c_int {
        setSeed(&mut rnd, (*g).seed);
        spawn = locateBiome(
            g,
            0 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            256 as libc::c_int,
            isSpawnBiome,
            &mut rnd,
            &mut found,
        );
        if found == 0 {
            spawn.z = 8 as libc::c_int;
            spawn.x = spawn.z;
        }
    } else {
        spawn = findFittestPos(g);
    }
    let mut accum: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut bx: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bz: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bn: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut gp: libc::c_double = 0.;
    if (*g).mc >= MC_1_13 as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut u: libc::c_int = 0;
        let mut v: libc::c_int = 0;
        v = 0 as libc::c_int;
        u = v;
        k = u;
        j = k;
        i = 0 as libc::c_int;
        while i < 1024 as libc::c_int {
            if j > -(16 as libc::c_int) && j <= 16 as libc::c_int
                && k > -(16 as libc::c_int) && k <= 16 as libc::c_int
            {
                if findServerSpawn(
                    g,
                    (spawn.x >> 4 as libc::c_int) + j,
                    (spawn.z >> 4 as libc::c_int) + k,
                    &mut bx,
                    &mut bz,
                    &mut bn,
                    &mut accum,
                ) != 0
                {
                    spawn.x = round(bx / bn) as libc::c_int;
                    spawn.z = round(bz / bn) as libc::c_int;
                    return spawn;
                }
            }
            if j == k || j < 0 as libc::c_int && j == -k
                || j > 0 as libc::c_int && j == 1 as libc::c_int - k
            {
                let mut tmp: libc::c_int = u;
                u = -v;
                v = tmp;
            }
            j += u;
            k += v;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 1000 as libc::c_int {
            let mut biome: libc::c_int = getBiomeAt(
                g,
                1 as libc::c_int,
                spawn.x,
                0 as libc::c_int,
                spawn.z,
            );
            gp = getGrassProbability((*g).seed, biome, spawn.x, spawn.z);
            bx += accum * gp * spawn.x as libc::c_double;
            bz += accum * gp * spawn.z as libc::c_double;
            bn += accum * gp;
            accum *= 1 as libc::c_int as libc::c_double - gp;
            if accum < 0.001f64 {
                spawn.x = round(bx / bn) as libc::c_int;
                spawn.z = round(bz / bn) as libc::c_int;
                break;
            } else {
                spawn.x
                    += nextInt(&mut rnd, 64 as libc::c_int)
                        - nextInt(&mut rnd, 64 as libc::c_int);
                spawn.z
                    += nextInt(&mut rnd, 64 as libc::c_int)
                        - nextInt(&mut rnd, 64 as libc::c_int);
                i += 1;
            }
        }
    }
    return spawn;
}
#[no_mangle]
pub unsafe extern "C" fn estimateSpawn(mut g: *const Generator) -> Pos {
    let mut isSpawnBiome: *const libc::c_char = getValidSpawnBiomes();
    let mut spawn: Pos = Pos { x: 0, z: 0 };
    if (*g).mc <= MC_1_17 as libc::c_int {
        let mut found: libc::c_int = 0;
        let mut rnd: uint64_t = 0;
        setSeed(&mut rnd, (*g).seed);
        spawn = locateBiome(
            g,
            0 as libc::c_int,
            63 as libc::c_int,
            0 as libc::c_int,
            256 as libc::c_int,
            isSpawnBiome,
            &mut rnd,
            &mut found,
        );
        if found == 0 {
            spawn.z = 8 as libc::c_int;
            spawn.x = spawn.z;
        }
        if (*g).mc >= MC_1_13 as libc::c_int {
            spawn.x &= !(0xf as libc::c_int);
            spawn.z &= !(0xf as libc::c_int);
        }
    } else {
        spawn = findFittestPos(g);
    }
    return spawn;
}
#[no_mangle]
pub unsafe extern "C" fn isViableFeatureBiome(
    mut mc: libc::c_int,
    mut structureType: libc::c_int,
    mut biomeID: libc::c_int,
) -> libc::c_int {
    match structureType {
        1 => {
            return (biomeID == desert as libc::c_int
                || biomeID == desert_hills as libc::c_int) as libc::c_int;
        }
        2 => {
            return (biomeID == jungle as libc::c_int
                || biomeID == jungle_hills as libc::c_int
                || biomeID == bamboo_jungle as libc::c_int
                || biomeID == bamboo_jungle_hills as libc::c_int) as libc::c_int;
        }
        3 => return (biomeID == swamp as libc::c_int) as libc::c_int,
        4 => {
            if mc < MC_1_9 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == snowy_tundra as libc::c_int
                || biomeID == snowy_taiga as libc::c_int
                || biomeID == snowy_slopes as libc::c_int) as libc::c_int;
        }
        6 => {
            if mc < MC_1_13 as libc::c_int {
                return 0 as libc::c_int;
            }
            return isOceanic(biomeID);
        }
        7 => {
            if mc < MC_1_13 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (isOceanic(biomeID) != 0 || biomeID == beach as libc::c_int
                || biomeID == snowy_beach as libc::c_int) as libc::c_int;
        }
        11 | 12 => return (mc >= MC_1_16 as libc::c_int) as libc::c_int,
        13 => {
            if mc < MC_1_19 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == deep_dark as libc::c_int) as libc::c_int;
        }
        14 => {
            if mc < MC_1_13 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == beach as libc::c_int
                || biomeID == snowy_beach as libc::c_int) as libc::c_int;
        }
        15 => return isOverworld(mc, biomeID),
        8 => {
            if mc < MC_1_8 as libc::c_int {
                return 0 as libc::c_int;
            }
            return isDeepOcean(biomeID);
        }
        10 => {
            if mc < MC_1_14 as libc::c_int {
                return 0 as libc::c_int;
            }
            if mc >= MC_1_18 as libc::c_int {
                match biomeID {
                    2 | 1 | 35 | 12 | 5 | 177 | 181 | 180 | 182 | 179 | 178 => {
                        return 1 as libc::c_int;
                    }
                    _ => return 0 as libc::c_int,
                }
            }
        }
        5 => {}
        9 => {
            if mc < MC_1_11 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == dark_forest as libc::c_int
                || biomeID == dark_forest_hills as libc::c_int) as libc::c_int;
        }
        16 => {
            return (biomeID == nether_wastes as libc::c_int
                || biomeID == soul_sand_valley as libc::c_int
                || biomeID == warped_forest as libc::c_int
                || biomeID == crimson_forest as libc::c_int
                || biomeID == basalt_deltas as libc::c_int) as libc::c_int;
        }
        17 => {
            if mc < MC_1_16 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == nether_wastes as libc::c_int
                || biomeID == soul_sand_valley as libc::c_int
                || biomeID == warped_forest as libc::c_int
                || biomeID == crimson_forest as libc::c_int) as libc::c_int;
        }
        18 => {
            if mc < MC_1_9 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == end_midlands as libc::c_int
                || biomeID == end_highlands as libc::c_int) as libc::c_int;
        }
        19 => {
            if mc < MC_1_13 as libc::c_int {
                return 0 as libc::c_int;
            }
            return (biomeID == end_highlands as libc::c_int) as libc::c_int;
        }
        _ => {
            fprintf(
                stderr,
                b"isViableFeatureBiome: not implemented for structure type %d.\n\0"
                    as *const u8 as *const libc::c_char,
                structureType,
            );
            exit(1 as libc::c_int);
        }
    }
    if biomeID == plains as libc::c_int || biomeID == desert as libc::c_int
        || biomeID == savanna as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if mc >= MC_1_10 as libc::c_int && biomeID == taiga as libc::c_int {
        return 1 as libc::c_int;
    }
    if mc >= MC_1_14 as libc::c_int && biomeID == snowy_tundra as libc::c_int {
        return 1 as libc::c_int;
    }
    if mc >= MC_1_18 as libc::c_int && biomeID == meadow as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getValidMonumentBiomes1() -> *const libc::c_char {
    static mut oceanMonumentBiomeList1: [libc::c_int; 12] = [
        ocean as libc::c_int,
        deep_ocean as libc::c_int,
        river as libc::c_int,
        frozen_river as libc::c_int,
        frozen_ocean as libc::c_int,
        deep_frozen_ocean as libc::c_int,
        cold_ocean as libc::c_int,
        deep_cold_ocean as libc::c_int,
        lukewarm_ocean as libc::c_int,
        deep_lukewarm_ocean as libc::c_int,
        warm_ocean as libc::c_int,
        deep_warm_ocean as libc::c_int,
    ];
    static mut isValid: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_uint = 0;
    if isValid[oceanMonumentBiomeList1[0 as libc::c_int as usize] as usize] == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isValid[oceanMonumentBiomeList1[i as usize]
                as usize] = 1 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1);
        }
    }
    return isValid.as_mut_ptr();
}
unsafe extern "C" fn getValidMonumentBiomes2() -> *const libc::c_char {
    static mut oceanMonumentBiomeList2: [libc::c_int; 5] = [
        deep_frozen_ocean as libc::c_int,
        deep_cold_ocean as libc::c_int,
        deep_ocean as libc::c_int,
        deep_lukewarm_ocean as libc::c_int,
        deep_warm_ocean as libc::c_int,
    ];
    static mut isValid: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_uint = 0;
    if isValid[oceanMonumentBiomeList2[0 as libc::c_int as usize] as usize] == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isValid[oceanMonumentBiomeList2[i as usize]
                as usize] = 1 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1);
        }
    }
    return isValid.as_mut_ptr();
}
unsafe extern "C" fn getValidMansionBiomes() -> *const libc::c_char {
    static mut mansionBiomeList: [libc::c_int; 2] = [
        dark_forest as libc::c_int,
        dark_forest as libc::c_int + 128 as libc::c_int,
    ];
    static mut isValid: [libc::c_char; 256] = [0; 256];
    let mut i: libc::c_uint = 0;
    if isValid[mansionBiomeList[0 as libc::c_int as usize] as usize] == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isValid[mansionBiomeList[i as usize]
                as usize] = 1 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1);
        }
    }
    return isValid.as_mut_ptr();
}
unsafe extern "C" fn mapViableBiome(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = mapBiome(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut styp: libc::c_int = *((*l).data as *const libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut biomeID: libc::c_int = *out.offset((i + w * j) as isize);
            match styp {
                1 => {
                    if biomeID == desert as libc::c_int || isMesa(biomeID) != 0 {
                        return 0 as libc::c_int;
                    }
                }
                2 => {
                    if biomeID == jungle as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                3 => {
                    if biomeID == swamp as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                4 => {
                    if biomeID == snowy_tundra as libc::c_int
                        || biomeID == snowy_taiga as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                }
                14 => {
                    if isOceanic(biomeID) != 0 {
                        return 0 as libc::c_int;
                    }
                }
                6 | 7 | 8 => {
                    if isOceanic(biomeID) != 0 {
                        return 0 as libc::c_int;
                    }
                }
                9 => {
                    if biomeID == dark_forest as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                _ => return 0 as libc::c_int,
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mapViableShore(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = mapShore(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    let mut styp: libc::c_int = *((*l).data as *const libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut mc: libc::c_int = *((*l).data as *const libc::c_int)
        .offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut biomeID: libc::c_int = *out.offset((i + w * j) as isize);
            match styp {
                1 | 2 | 3 | 4 | 6 | 7 | 5 | 8 | 9 | 14 => {
                    if isViableFeatureBiome(mc, styp, biomeID) != 0 {
                        return 0 as libc::c_int;
                    }
                }
                _ => return 0 as libc::c_int,
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isViableStructurePos(
    mut structureType: libc::c_int,
    mut g: *mut Generator,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut flags: uint32_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut approx: libc::c_int = 0 as libc::c_int;
    let mut viable: libc::c_int = 0 as libc::c_int;
    let mut chunkX: int64_t = (x >> 4 as libc::c_int) as int64_t;
    let mut chunkZ: int64_t = (z >> 4 as libc::c_int) as int64_t;
    let mut sampleX: libc::c_int = 0;
    let mut sampleZ: libc::c_int = 0;
    let mut sampleY: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    if (*g).dim == -(1 as libc::c_int) {
        if structureType == Fortress as libc::c_int && (*g).mc <= MC_1_17 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (*g).mc < MC_1_16 as libc::c_int {
            return 0 as libc::c_int;
        }
        if structureType == Ruined_Portal_N as libc::c_int {
            return 1 as libc::c_int;
        }
        if structureType == Fortress as libc::c_int {
            let mut sc: StructureConfig = StructureConfig {
                salt: 0,
                regionSize: 0,
                chunkRange: 0,
                structType: 0,
                properties: 0,
            };
            if getStructureConfig(Fortress as libc::c_int, (*g).mc, &mut sc) == 0 {
                return 0 as libc::c_int;
            }
            let mut rp: Pos = {
                let mut init = Pos {
                    x: (chunkX / sc.regionSize as libc::c_long
                        - (x < 0 as libc::c_int) as libc::c_int as libc::c_long)
                        as libc::c_int,
                    z: (chunkZ / sc.regionSize as libc::c_long
                        - (z < 0 as libc::c_int) as libc::c_int as libc::c_long)
                        as libc::c_int,
                };
                init
            };
            if getStructurePos(
                Bastion as libc::c_int,
                (*g).mc,
                (*g).seed,
                rp.x,
                rp.z,
                &mut rp,
            ) == 0
            {
                return 1 as libc::c_int;
            }
            return (isViableStructurePos(Bastion as libc::c_int, g, x, z, flags) == 0)
                as libc::c_int;
        }
        sampleY = 0 as libc::c_int;
        if (*g).mc >= MC_1_18 as libc::c_int && structureType == Bastion as libc::c_int {
            let mut sv: StructureVariant = StructureVariant {
                abandoned: 0,
                giant: 0,
                variant: 0,
                name: 0 as *const libc::c_char,
                biome: 0,
                rotation: 0,
                mirror: 0,
                x: 0,
                y: 0,
                z: 0,
                sx: 0,
                sy: 0,
                sz: 0,
            };
            getVariant(
                &mut sv,
                Bastion as libc::c_int,
                (*g).mc,
                (*g).seed,
                x,
                z,
                -(1 as libc::c_int),
            );
            sampleX = (((chunkX << 5 as libc::c_int)
                + (2 as libc::c_int * sv.x as libc::c_int) as libc::c_long
                + sv.sx as libc::c_long) / 2 as libc::c_int as libc::c_long
                >> 2 as libc::c_int) as libc::c_int;
            sampleZ = (((chunkZ << 5 as libc::c_int)
                + (2 as libc::c_int * sv.z as libc::c_int) as libc::c_long
                + sv.sz as libc::c_long) / 2 as libc::c_int as libc::c_long
                >> 2 as libc::c_int) as libc::c_int;
            if (*g).mc >= MC_1_19 as libc::c_int {
                sampleY = 33 as libc::c_int >> 2 as libc::c_int;
            }
        } else {
            sampleX = ((chunkX << 2 as libc::c_int) + 2 as libc::c_int as libc::c_long)
                as libc::c_int;
            sampleZ = ((chunkZ << 2 as libc::c_int) + 2 as libc::c_int as libc::c_long)
                as libc::c_int;
        }
        id = getBiomeAt(g, 4 as libc::c_int, sampleX, sampleY, sampleZ);
        return isViableFeatureBiome((*g).mc, structureType, id);
    } else {
        if (*g).dim == 1 as libc::c_int {
            match structureType {
                18 => {
                    if (*g).mc < MC_1_9 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                19 => {
                    if (*g).mc < MC_1_13 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                _ => return 0 as libc::c_int,
            }
            id = getBiomeAt(
                g,
                16 as libc::c_int,
                chunkX as libc::c_int,
                0 as libc::c_int,
                chunkZ as libc::c_int,
            );
            return if isViableFeatureBiome((*g).mc, structureType, id) != 0 {
                id
            } else {
                0 as libc::c_int
            };
        }
    }
    let mut lbiome: Layer = Layer {
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
    let mut lshore: Layer = Layer {
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
    let mut entry: *mut Layer = 0 as *mut Layer;
    let mut data: [libc::c_int; 2] = [structureType, (*g).mc];
    if (*g).mc <= MC_1_17 as libc::c_int {
        lbiome = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_BIOME_256 as libc::c_int as usize];
        lshore = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_SHORE_16 as libc::c_int as usize];
        entry = (*g).c2rust_unnamed.c2rust_unnamed.entry;
        let ref mut fresh19 = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_BIOME_256 as libc::c_int as usize]
            .data;
        *fresh19 = data.as_mut_ptr() as *mut libc::c_void;
        let ref mut fresh20 = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_BIOME_256 as libc::c_int as usize]
            .getMap;
        *fresh20 = Some(
            mapViableBiome
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        );
        let ref mut fresh21 = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_SHORE_16 as libc::c_int as usize]
            .data;
        *fresh21 = data.as_mut_ptr() as *mut libc::c_void;
        let ref mut fresh22 = (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_SHORE_16 as libc::c_int as usize]
            .getMap;
        *fresh22 = Some(
            mapViableShore
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        );
    }
    match structureType {
        6 | 7 | 14 => {
            if (*g).mc < MC_1_13 as libc::c_int {
                current_block = 14993526094991803608;
            } else {
                current_block = 17514367958679633418;
            }
        }
        4 => {
            if (*g).mc < MC_1_9 as libc::c_int {
                current_block = 14993526094991803608;
            } else {
                current_block = 17514367958679633418;
            }
        }
        1 | 2 | 3 => {
            current_block = 17514367958679633418;
        }
        5 => {
            if (*g).mc <= MC_1_17 as libc::c_int {
                if (*g).mc == MC_1_15 as libc::c_int {
                    let ref mut fresh25 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
                    *fresh25 = &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                        .as_mut_ptr()
                        .offset(L_VORONOI_1 as libc::c_int as isize) as *mut Layer;
                    sampleX = ((chunkX << 4 as libc::c_int)
                        + 9 as libc::c_int as libc::c_long) as libc::c_int;
                    sampleZ = ((chunkZ << 4 as libc::c_int)
                        + 9 as libc::c_int as libc::c_long) as libc::c_int;
                } else {
                    let ref mut fresh26 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
                    *fresh26 = &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                        .as_mut_ptr()
                        .offset(L_RIVER_MIX_4 as libc::c_int as isize) as *mut Layer;
                    sampleX = ((chunkX << 2 as libc::c_int)
                        + 2 as libc::c_int as libc::c_long) as libc::c_int;
                    sampleZ = ((chunkZ << 2 as libc::c_int)
                        + 2 as libc::c_int as libc::c_long) as libc::c_int;
                }
                id = getBiomeAt(g, 0 as libc::c_int, sampleX, 0 as libc::c_int, sampleZ);
                if id < 0 as libc::c_int
                    || isViableFeatureBiome((*g).mc, structureType, id) == 0
                {
                    current_block = 14993526094991803608;
                } else if flags != 0 && id as uint32_t != flags {
                    current_block = 14993526094991803608;
                } else {
                    if (*g).mc <= MC_1_9 as libc::c_int {
                        sampleX = ((chunkX << 4 as libc::c_int)
                            + 2 as libc::c_int as libc::c_long) as libc::c_int;
                        sampleZ = ((chunkZ << 4 as libc::c_int)
                            + 2 as libc::c_int as libc::c_long) as libc::c_int;
                        id = getBiomeAt(
                            g,
                            1 as libc::c_int,
                            sampleX,
                            0 as libc::c_int,
                            sampleZ,
                        );
                        if id < 0 as libc::c_int
                            || isViableFeatureBiome((*g).mc, structureType, id) == 0
                        {
                            current_block = 14993526094991803608;
                        } else {
                            current_block = 14648606000749551097;
                        }
                    } else {
                        current_block = 14648606000749551097;
                    }
                    match current_block {
                        14993526094991803608 => {}
                        _ => {
                            viable = id;
                            current_block = 4996352559381616237;
                        }
                    }
                }
            } else {
                let vv: [libc::c_int; 5] = [
                    plains as libc::c_int,
                    desert as libc::c_int,
                    savanna as libc::c_int,
                    taiga as libc::c_int,
                    snowy_tundra as libc::c_int,
                ];
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                loop {
                    if !(i
                        < (::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ))
                    {
                        current_block = 14993526094991803608;
                        break;
                    }
                    if !(flags != 0 && flags != vv[i as usize] as uint32_t) {
                        let mut sv_0: StructureVariant = StructureVariant {
                            abandoned: 0,
                            giant: 0,
                            variant: 0,
                            name: 0 as *const libc::c_char,
                            biome: 0,
                            rotation: 0,
                            mirror: 0,
                            x: 0,
                            y: 0,
                            z: 0,
                            sx: 0,
                            sy: 0,
                            sz: 0,
                        };
                        getVariant(
                            &mut sv_0,
                            Village as libc::c_int,
                            (*g).mc,
                            (*g).seed,
                            x,
                            z,
                            vv[i as usize],
                        );
                        sampleX = (((chunkX << 5 as libc::c_int)
                            + (2 as libc::c_int * sv_0.x as libc::c_int) as libc::c_long
                            + sv_0.sx as libc::c_long) / 2 as libc::c_int as libc::c_long
                            >> 2 as libc::c_int) as libc::c_int;
                        sampleZ = (((chunkZ << 5 as libc::c_int)
                            + (2 as libc::c_int * sv_0.z as libc::c_int) as libc::c_long
                            + sv_0.sz as libc::c_long) / 2 as libc::c_int as libc::c_long
                            >> 2 as libc::c_int) as libc::c_int;
                        sampleY = 319 as libc::c_int >> 2 as libc::c_int;
                        id = getBiomeAt(g, 0 as libc::c_int, sampleX, sampleY, sampleZ);
                        if id == vv[i as usize]
                            || id == meadow as libc::c_int
                                && vv[i as usize] == plains as libc::c_int
                        {
                            viable = id;
                            current_block = 4996352559381616237;
                            break;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        10 => {
            if (*g).mc < MC_1_14 as libc::c_int {
                current_block = 14993526094991803608;
            } else {
                let mut rng: uint64_t = (*g).seed;
                setAttemptSeed(&mut rng, chunkX as libc::c_int, chunkZ as libc::c_int);
                if nextInt(&mut rng, 5 as libc::c_int) != 0 as libc::c_int {
                    current_block = 14993526094991803608;
                } else {
                    let mut vilconf: StructureConfig = StructureConfig {
                        salt: 0,
                        regionSize: 0,
                        chunkRange: 0,
                        structType: 0,
                        properties: 0,
                    };
                    if getStructureConfig(Village as libc::c_int, (*g).mc, &mut vilconf)
                        == 0
                    {
                        current_block = 14993526094991803608;
                    } else {
                        let mut cx0: libc::c_int = (chunkX
                            - 10 as libc::c_int as libc::c_long) as libc::c_int;
                        let mut cx1: libc::c_int = (chunkX
                            + 10 as libc::c_int as libc::c_long) as libc::c_int;
                        let mut cz0: libc::c_int = (chunkZ
                            - 10 as libc::c_int as libc::c_long) as libc::c_int;
                        let mut cz1: libc::c_int = (chunkZ
                            + 10 as libc::c_int as libc::c_long) as libc::c_int;
                        let mut rx0: libc::c_int = cx0
                            / vilconf.regionSize as libc::c_int
                            - (cx0 < 0 as libc::c_int) as libc::c_int;
                        let mut rx1: libc::c_int = cx1
                            / vilconf.regionSize as libc::c_int
                            - (cx1 < 0 as libc::c_int) as libc::c_int;
                        let mut rz0: libc::c_int = cz0
                            / vilconf.regionSize as libc::c_int
                            - (cz0 < 0 as libc::c_int) as libc::c_int;
                        let mut rz1: libc::c_int = cz1
                            / vilconf.regionSize as libc::c_int
                            - (cz1 < 0 as libc::c_int) as libc::c_int;
                        let mut rx: libc::c_int = 0;
                        let mut rz: libc::c_int = 0;
                        rz = rz0;
                        's_548: loop {
                            if !(rz <= rz1) {
                                current_block = 14652688882591975137;
                                break;
                            }
                            rx = rx0;
                            while rx <= rx1 {
                                let mut p: Pos = getFeaturePos(vilconf, (*g).seed, rx, rz);
                                let mut cx: libc::c_int = p.x >> 4 as libc::c_int;
                                let mut cz: libc::c_int = p.z >> 4 as libc::c_int;
                                if cx >= cx0 && cx <= cx1 && cz >= cz0 && cz <= cz1 {
                                    if (*g).mc >= MC_1_16 as libc::c_int {
                                        current_block = 14993526094991803608;
                                        break 's_548;
                                    }
                                    if isViableStructurePos(
                                        Village as libc::c_int,
                                        g,
                                        p.x,
                                        p.z,
                                        0 as libc::c_int as uint32_t,
                                    ) != 0
                                    {
                                        current_block = 14993526094991803608;
                                        break 's_548;
                                    } else {
                                        current_block = 4996352559381616237;
                                        break 's_548;
                                    }
                                } else {
                                    rx += 1;
                                }
                            }
                            rz += 1;
                        }
                        match current_block {
                            14993526094991803608 => {}
                            4996352559381616237 => {}
                            _ => {
                                if (*g).mc >= MC_1_18 as libc::c_int {
                                    rng = chunkGenerateRnd(
                                        (*g).seed,
                                        chunkX as libc::c_int,
                                        chunkZ as libc::c_int,
                                    );
                                    match nextInt(&mut rng, 4 as libc::c_int) {
                                        0 => {
                                            sampleX = 15 as libc::c_int;
                                            sampleZ = 15 as libc::c_int;
                                        }
                                        1 => {
                                            sampleX = -(15 as libc::c_int);
                                            sampleZ = 15 as libc::c_int;
                                        }
                                        2 => {
                                            sampleX = -(15 as libc::c_int);
                                            sampleZ = -(15 as libc::c_int);
                                        }
                                        3 => {
                                            sampleX = 15 as libc::c_int;
                                            sampleZ = -(15 as libc::c_int);
                                        }
                                        _ => return 0 as libc::c_int,
                                    }
                                    sampleX = (((chunkX << 5 as libc::c_int)
                                        + sampleX as libc::c_long)
                                        / 2 as libc::c_int as libc::c_long >> 2 as libc::c_int)
                                        as libc::c_int;
                                    sampleZ = (((chunkZ << 5 as libc::c_int)
                                        + sampleZ as libc::c_long)
                                        / 2 as libc::c_int as libc::c_long >> 2 as libc::c_int)
                                        as libc::c_int;
                                } else if (*g).mc >= MC_1_16 as libc::c_int {
                                    let ref mut fresh27 = (*g)
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .entry;
                                    *fresh27 = &mut *((*g)
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .ls
                                        .layers)
                                        .as_mut_ptr()
                                        .offset(L_RIVER_MIX_4 as libc::c_int as isize)
                                        as *mut Layer;
                                    sampleX = ((chunkX << 2 as libc::c_int)
                                        + 2 as libc::c_int as libc::c_long) as libc::c_int;
                                    sampleZ = ((chunkZ << 2 as libc::c_int)
                                        + 2 as libc::c_int as libc::c_long) as libc::c_int;
                                } else {
                                    let ref mut fresh28 = (*g)
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .entry;
                                    *fresh28 = &mut *((*g)
                                        .c2rust_unnamed
                                        .c2rust_unnamed
                                        .ls
                                        .layers)
                                        .as_mut_ptr()
                                        .offset(L_VORONOI_1 as libc::c_int as isize) as *mut Layer;
                                    sampleX = ((chunkX << 4 as libc::c_int)
                                        + 9 as libc::c_int as libc::c_long) as libc::c_int;
                                    sampleZ = ((chunkZ << 4 as libc::c_int)
                                        + 9 as libc::c_int as libc::c_long) as libc::c_int;
                                }
                                id = getBiomeAt(
                                    g,
                                    0 as libc::c_int,
                                    sampleX,
                                    319 as libc::c_int >> 2 as libc::c_int,
                                    sampleZ,
                                );
                                if id < 0 as libc::c_int
                                    || isViableFeatureBiome((*g).mc, structureType, id) == 0
                                {
                                    current_block = 14993526094991803608;
                                } else {
                                    current_block = 4996352559381616237;
                                }
                            }
                        }
                    }
                }
            }
        }
        8 => {
            if (*g).mc < MC_1_8 as libc::c_int {
                current_block = 14993526094991803608;
            } else {
                if (*g).mc == MC_1_8 as libc::c_int {
                    id = getBiomeAt(
                        g,
                        1 as libc::c_int,
                        ((chunkX << 4 as libc::c_int) + 8 as libc::c_int as libc::c_long)
                            as libc::c_int,
                        0 as libc::c_int,
                        ((chunkZ << 4 as libc::c_int) + 8 as libc::c_int as libc::c_long)
                            as libc::c_int,
                    );
                    if id < 0 as libc::c_int || isDeepOcean(id) == 0 {
                        current_block = 14993526094991803608;
                    } else {
                        current_block = 2103801789718498838;
                    }
                } else if (*g).mc <= MC_1_17 as libc::c_int {
                    let ref mut fresh29 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
                    *fresh29 = &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                        .as_mut_ptr()
                        .offset(L_SHORE_16 as libc::c_int as isize) as *mut Layer;
                    id = getBiomeAt(
                        g,
                        0 as libc::c_int,
                        chunkX as libc::c_int,
                        0 as libc::c_int,
                        chunkZ as libc::c_int,
                    );
                    if id < 0 as libc::c_int || isDeepOcean(id) == 0 {
                        current_block = 14993526094991803608;
                    } else {
                        current_block = 2103801789718498838;
                    }
                } else {
                    current_block = 2103801789718498838;
                }
                match current_block {
                    14993526094991803608 => {}
                    _ => {
                        sampleX = ((chunkX << 4 as libc::c_int)
                            + 8 as libc::c_int as libc::c_long) as libc::c_int;
                        sampleZ = ((chunkZ << 4 as libc::c_int)
                            + 8 as libc::c_int as libc::c_long) as libc::c_int;
                        if (*g).mc >= MC_1_9 as libc::c_int
                            && (*g).mc <= MC_1_17 as libc::c_int
                        {
                            if areBiomesViable(
                                g,
                                sampleX,
                                63 as libc::c_int,
                                sampleZ,
                                16 as libc::c_int,
                                getValidMonumentBiomes2(),
                                approx,
                            ) == 0
                            {
                                current_block = 14993526094991803608;
                            } else {
                                current_block = 2089914658669629659;
                            }
                        } else if (*g).mc >= MC_1_18 as libc::c_int {
                            id = getBiomeAt(
                                g,
                                4 as libc::c_int,
                                sampleX >> 2 as libc::c_int,
                                36 as libc::c_int >> 2 as libc::c_int,
                                sampleZ >> 2 as libc::c_int,
                            );
                            if isDeepOcean(id) == 0 {
                                current_block = 14993526094991803608;
                            } else {
                                current_block = 2089914658669629659;
                            }
                        } else {
                            current_block = 2089914658669629659;
                        }
                        match current_block {
                            14993526094991803608 => {}
                            _ => {
                                if areBiomesViable(
                                    g,
                                    sampleX,
                                    63 as libc::c_int,
                                    sampleZ,
                                    29 as libc::c_int,
                                    getValidMonumentBiomes1(),
                                    approx,
                                ) != 0
                                {
                                    current_block = 4996352559381616237;
                                } else {
                                    current_block = 14993526094991803608;
                                }
                            }
                        }
                    }
                }
            }
        }
        9 => {
            if (*g).mc < MC_1_11 as libc::c_int {
                current_block = 14993526094991803608;
            } else if (*g).mc <= MC_1_17 as libc::c_int {
                sampleX = ((chunkX << 4 as libc::c_int)
                    + 8 as libc::c_int as libc::c_long) as libc::c_int;
                sampleZ = ((chunkZ << 4 as libc::c_int)
                    + 8 as libc::c_int as libc::c_long) as libc::c_int;
                if areBiomesViable(
                    g,
                    sampleX,
                    0 as libc::c_int,
                    sampleZ,
                    32 as libc::c_int,
                    getValidMansionBiomes(),
                    approx,
                ) == 0
                {
                    current_block = 14993526094991803608;
                } else {
                    current_block = 4996352559381616237;
                }
            } else {
                sampleX = ((chunkX << 4 as libc::c_int)
                    + 7 as libc::c_int as libc::c_long) as libc::c_int;
                sampleZ = ((chunkZ << 4 as libc::c_int)
                    + 7 as libc::c_int as libc::c_long) as libc::c_int;
                id = getBiomeAt(
                    g,
                    4 as libc::c_int,
                    sampleX >> 2 as libc::c_int,
                    319 as libc::c_int >> 4 as libc::c_int,
                    sampleZ >> 2 as libc::c_int,
                );
                if id < 0 as libc::c_int
                    || isViableFeatureBiome((*g).mc, structureType, id) == 0
                {
                    current_block = 14993526094991803608;
                } else {
                    current_block = 4996352559381616237;
                }
            }
        }
        11 | 12 => {
            if (*g).mc >= MC_1_16 as libc::c_int {
                current_block = 4996352559381616237;
            } else {
                current_block = 14993526094991803608;
            }
        }
        13 => {
            if (*g).mc < MC_1_19 as libc::c_int {
                current_block = 14993526094991803608;
            } else {
                let mut sv_1: StructureVariant = StructureVariant {
                    abandoned: 0,
                    giant: 0,
                    variant: 0,
                    name: 0 as *const libc::c_char,
                    biome: 0,
                    rotation: 0,
                    mirror: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    sx: 0,
                    sy: 0,
                    sz: 0,
                };
                getVariant(
                    &mut sv_1,
                    Ancient_City as libc::c_int,
                    (*g).mc,
                    (*g).seed,
                    x,
                    z,
                    -(1 as libc::c_int),
                );
                sampleX = (((chunkX << 5 as libc::c_int)
                    + (2 as libc::c_int * sv_1.x as libc::c_int) as libc::c_long
                    + sv_1.sx as libc::c_long) / 2 as libc::c_int as libc::c_long
                    >> 2 as libc::c_int) as libc::c_int;
                sampleZ = (((chunkZ << 5 as libc::c_int)
                    + (2 as libc::c_int * sv_1.z as libc::c_int) as libc::c_long
                    + sv_1.sz as libc::c_long) / 2 as libc::c_int as libc::c_long
                    >> 2 as libc::c_int) as libc::c_int;
                sampleY = -(27 as libc::c_int) >> 2 as libc::c_int;
                id = getBiomeAt(g, 4 as libc::c_int, sampleX, sampleY, sampleZ);
                if id < 0 as libc::c_int
                    || isViableFeatureBiome((*g).mc, structureType, id) == 0
                {
                    current_block = 14993526094991803608;
                } else {
                    current_block = 4996352559381616237;
                }
            }
        }
        15 => {
            current_block = 4996352559381616237;
        }
        _ => {
            fprintf(
                stderr,
                b"isViableStructurePos: validation for structure type %d not implemented\n\0"
                    as *const u8 as *const libc::c_char,
                structureType,
            );
            current_block = 14993526094991803608;
        }
    }
    match current_block {
        17514367958679633418 => {
            if (*g).mc >= MC_1_16 as libc::c_int {
                if (*g).mc < MC_1_18 as libc::c_int {
                    let ref mut fresh23 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
                    *fresh23 = &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                        .as_mut_ptr()
                        .offset(L_RIVER_MIX_4 as libc::c_int as isize) as *mut Layer;
                }
                sampleX = ((chunkX << 2 as libc::c_int)
                    + 2 as libc::c_int as libc::c_long) as libc::c_int;
                sampleZ = ((chunkZ << 2 as libc::c_int)
                    + 2 as libc::c_int as libc::c_long) as libc::c_int;
            } else {
                let ref mut fresh24 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
                *fresh24 = &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                    .as_mut_ptr()
                    .offset(L_VORONOI_1 as libc::c_int as isize) as *mut Layer;
                sampleX = ((chunkX << 4 as libc::c_int)
                    + 9 as libc::c_int as libc::c_long) as libc::c_int;
                sampleZ = ((chunkZ << 4 as libc::c_int)
                    + 9 as libc::c_int as libc::c_long) as libc::c_int;
            }
            id = getBiomeAt(
                g,
                0 as libc::c_int,
                sampleX,
                319 as libc::c_int >> 2 as libc::c_int,
                sampleZ,
            );
            if id < 0 as libc::c_int
                || isViableFeatureBiome((*g).mc, structureType, id) == 0
            {
                current_block = 14993526094991803608;
            } else {
                current_block = 4996352559381616237;
            }
        }
        _ => {}
    }
    match current_block {
        4996352559381616237 => {
            if viable == 0 {
                viable = 1 as libc::c_int;
            }
        }
        _ => {}
    }
    if (*g).mc <= MC_1_17 as libc::c_int {
        (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_BIOME_256 as libc::c_int as usize] = lbiome;
        (*g)
            .c2rust_unnamed
            .c2rust_unnamed
            .ls
            .layers[L_SHORE_16 as libc::c_int as usize] = lshore;
        let ref mut fresh30 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
        *fresh30 = entry;
    }
    return viable;
}
#[no_mangle]
pub unsafe extern "C" fn isViableStructureTerrain(
    mut structType: libc::c_int,
    mut g: *mut Generator,
    mut x: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_int {
    let mut id: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    if (*g).mc < MC_1_18 as libc::c_int {
        return 1 as libc::c_int;
    }
    if structType == Desert_Pyramid as libc::c_int
        || structType == Jungle_Temple as libc::c_int
    {
        sx = if structType == Desert_Pyramid as libc::c_int {
            21 as libc::c_int
        } else {
            12 as libc::c_int
        };
        sz = if structType == Desert_Pyramid as libc::c_int {
            21 as libc::c_int
        } else {
            15 as libc::c_int
        };
    } else if structType == Mansion as libc::c_int {
        let mut cx: libc::c_int = x >> 4 as libc::c_int;
        let mut cz: libc::c_int = z >> 4 as libc::c_int;
        let mut rng: uint64_t = chunkGenerateRnd((*g).seed, cx, cz);
        let mut rot: libc::c_int = nextInt(&mut rng, 4 as libc::c_int);
        sx = 5 as libc::c_int;
        sz = 5 as libc::c_int;
        if rot == 0 as libc::c_int {
            sx = -(5 as libc::c_int);
        }
        if rot == 1 as libc::c_int {
            sx = -(5 as libc::c_int);
            sz = -(5 as libc::c_int);
        }
        if rot == 2 as libc::c_int {
            sz = -(5 as libc::c_int);
        }
        x = (cx << 4 as libc::c_int) + 7 as libc::c_int;
        z = (cz << 4 as libc::c_int) + 7 as libc::c_int;
    } else {
        return 1 as libc::c_int
    }
    id = getBiomeAt(
        g,
        4 as libc::c_int,
        x + sx >> 2 as libc::c_int,
        15 as libc::c_int,
        z >> 2 as libc::c_int,
    );
    if isOceanic(id) != 0 || id == river as libc::c_int
        || id == frozen_river as libc::c_int
    {
        return 0 as libc::c_int;
    }
    id = getBiomeAt(
        g,
        4 as libc::c_int,
        x >> 2 as libc::c_int,
        15 as libc::c_int,
        z + sz >> 2 as libc::c_int,
    );
    if isOceanic(id) != 0 || id == river as libc::c_int
        || id == frozen_river as libc::c_int
    {
        return 0 as libc::c_int;
    }
    id = getBiomeAt(
        g,
        4 as libc::c_int,
        x + sx >> 2 as libc::c_int,
        15 as libc::c_int,
        z + sz >> 2 as libc::c_int,
    );
    if isOceanic(id) != 0 || id == river as libc::c_int
        || id == frozen_river as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isViableEndCityTerrain(
    mut en: *const EndNoise,
    mut sn: *const SurfaceNoise,
    mut blockX: libc::c_int,
    mut blockZ: libc::c_int,
) -> libc::c_int {
    let mut chunkX: libc::c_int = blockX >> 4 as libc::c_int;
    let mut chunkZ: libc::c_int = blockZ >> 4 as libc::c_int;
    blockX = (chunkX << 4 as libc::c_int) + 7 as libc::c_int;
    blockZ = (chunkZ << 4 as libc::c_int) + 7 as libc::c_int;
    let mut cellx: libc::c_int = blockX >> 3 as libc::c_int;
    let mut cellz: libc::c_int = blockZ >> 3 as libc::c_int;
    let y0: libc::c_int = 15 as libc::c_int;
    let y1: libc::c_int = 18 as libc::c_int;
    let mut ncol: [[[libc::c_double; 4]; 3]; 3] = [[[0.; 4]; 3]; 3];
    sampleNoiseColumnEnd(
        (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr(),
        sn,
        en,
        cellx,
        cellz,
        y0,
        y1,
    );
    sampleNoiseColumnEnd(
        (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr(),
        sn,
        en,
        cellx,
        cellz + 1 as libc::c_int,
        y0,
        y1,
    );
    sampleNoiseColumnEnd(
        (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr(),
        sn,
        en,
        cellx + 1 as libc::c_int,
        cellz,
        y0,
        y1,
    );
    sampleNoiseColumnEnd(
        (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr(),
        sn,
        en,
        cellx + 1 as libc::c_int,
        cellz + 1 as libc::c_int,
        y0,
        y1,
    );
    let mut h00: libc::c_int = 0;
    let mut h01: libc::c_int = 0;
    let mut h10: libc::c_int = 0;
    let mut h11: libc::c_int = 0;
    h00 = getSurfaceHeight(
        (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
            as *const libc::c_double,
        (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
            as *const libc::c_double,
        (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
            as *const libc::c_double,
        (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
            as *const libc::c_double,
        y0,
        y1,
        4 as libc::c_int,
        (blockX & 7 as libc::c_int) as libc::c_double / 8.0f64,
        (blockZ & 7 as libc::c_int) as libc::c_double / 8.0f64,
    );
    let mut cs: uint64_t = 0;
    setSeed(
        &mut cs,
        (chunkX as libc::c_ulonglong)
            .wrapping_add(
                (chunkZ as libc::c_ulonglong).wrapping_mul(10387313 as libc::c_ulonglong),
            ) as uint64_t,
    );
    match nextInt(&mut cs, 4 as libc::c_int) {
        0 => {
            sampleNoiseColumnEnd(
                (ncol[0 as libc::c_int as usize][2 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 0 as libc::c_int,
                cellz + 2 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 1 as libc::c_int,
                cellz + 2 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[2 as libc::c_int as usize][0 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 2 as libc::c_int,
                cellz + 0 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 2 as libc::c_int,
                cellz + 1 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[2 as libc::c_int as usize][2 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 2 as libc::c_int,
                cellz + 2 as libc::c_int,
                y0,
                y1,
            );
            h01 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX & 7 as libc::c_int) as libc::c_double / 8.0f64,
                (blockZ + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h10 = getSurfaceHeight(
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h11 = getSurfaceHeight(
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
        }
        1 => {
            sampleNoiseColumnEnd(
                (ncol[0 as libc::c_int as usize][2 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 0 as libc::c_int,
                cellz + 2 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 1 as libc::c_int,
                cellz + 2 as libc::c_int,
                y0,
                y1,
            );
            h01 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX & 7 as libc::c_int) as libc::c_double / 8.0f64,
                (blockZ + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h10 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h11 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][2 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
        }
        2 => {
            h01 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX & 7 as libc::c_int) as libc::c_double / 8.0f64,
                (blockZ - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h10 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h11 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
        }
        3 => {
            sampleNoiseColumnEnd(
                (ncol[2 as libc::c_int as usize][0 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 2 as libc::c_int,
                cellz + 0 as libc::c_int,
                y0,
                y1,
            );
            sampleNoiseColumnEnd(
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize])
                    .as_mut_ptr(),
                sn,
                en,
                cellx + 2 as libc::c_int,
                cellz + 1 as libc::c_int,
                y0,
                y1,
            );
            h01 = getSurfaceHeight(
                (ncol[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[0 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX & 7 as libc::c_int) as libc::c_double / 8.0f64,
                (blockZ - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h10 = getSurfaceHeight(
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
            h11 = getSurfaceHeight(
                (ncol[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[1 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                (ncol[2 as libc::c_int as usize][1 as libc::c_int as usize]).as_mut_ptr()
                    as *const libc::c_double,
                y0,
                y1,
                4 as libc::c_int,
                (blockX + 5 as libc::c_int & 7 as libc::c_int) as libc::c_double
                    / 8.0f64,
                (blockZ - 5 as libc::c_int & 7 as libc::c_int) as libc::c_double / 8.0f64,
            );
        }
        _ => return 0 as libc::c_int,
    }
    if h01 < h00 {
        h00 = h01;
    }
    if h10 < h00 {
        h00 = h10;
    }
    if h11 < h00 {
        h00 = h11;
    }
    return (h00 >= 60 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getVariant(
    mut r: *mut StructureVariant,
    mut structType: libc::c_int,
    mut mc: libc::c_int,
    mut seed: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut biomeID: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut sx: libc::c_char = 0;
    let mut sy: libc::c_char = 0;
    let mut sz: libc::c_char = 0;
    let mut rng: uint64_t = chunkGenerateRnd(
        seed,
        x >> 4 as libc::c_int,
        z >> 4 as libc::c_int,
    );
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<StructureVariant>() as libc::c_ulong,
    );
    (*r).variant = -(1 as libc::c_int) as libc::c_char;
    (*r).biome = -(1 as libc::c_int) as libc::c_short;
    match structType {
        5 => {
            if mc <= MC_1_9 as libc::c_int {
                return 0 as libc::c_int;
            }
            if isViableFeatureBiome(mc, Village as libc::c_int, biomeID) == 0 {
                return 0 as libc::c_int;
            }
            if mc <= MC_1_13 as libc::c_int {
                skipNextN(
                    &mut rng,
                    (if mc == MC_1_13 as libc::c_int {
                        10 as libc::c_int
                    } else {
                        11 as libc::c_int
                    }) as uint64_t,
                );
                (*r)
                    .abandoned = (nextInt(&mut rng, 50 as libc::c_int)
                    == 0 as libc::c_int) as libc::c_int as uint8_t;
                return 1 as libc::c_int;
            }
            (*r).biome = biomeID as libc::c_short;
            (*r).rotation = nextInt(&mut rng, 4 as libc::c_int) as uint8_t;
            let mut current_block_228: u64;
            match biomeID {
                177 => {
                    (*r).biome = plains as libc::c_int as libc::c_short;
                    current_block_228 = 11712972921842978987;
                }
                1 => {
                    current_block_228 = 11712972921842978987;
                }
                2 => {
                    t = nextInt(&mut rng, 250 as libc::c_int);
                    if t < 98 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 17 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                    } else if t < 196 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 12 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 12 as libc::c_int as libc::c_char;
                    } else if t < 245 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 15 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 15 as libc::c_int as libc::c_char;
                    } else if t < 247 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 17 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 249 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 12 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 12 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 250 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 15 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 15 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    }
                    current_block_228 = 13434751124187322381;
                }
                35 => {
                    t = nextInt(&mut rng, 459 as libc::c_int);
                    if t < 100 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 14 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 12 as libc::c_int as libc::c_char;
                    } else if t < 150 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                    } else if t < 300 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                    } else if t < 450 as libc::c_int {
                        (*r).variant = 4 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                    } else if t < 452 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 14 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 12 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 453 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 456 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 459 as libc::c_int {
                        (*r).variant = 4 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 6 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    }
                    current_block_228 = 13434751124187322381;
                }
                5 => {
                    t = nextInt(&mut rng, 100 as libc::c_int);
                    if t < 49 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 22 as libc::c_int as libc::c_char;
                        sy = 3 as libc::c_int as libc::c_char;
                        sz = 18 as libc::c_int as libc::c_char;
                    } else if t < 98 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                    } else if t < 99 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 22 as libc::c_int as libc::c_char;
                        sy = 3 as libc::c_int as libc::c_char;
                        sz = 18 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 100 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    }
                    current_block_228 = 13434751124187322381;
                }
                12 => {
                    t = nextInt(&mut rng, 306 as libc::c_int);
                    if t < 100 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 12 as libc::c_int as libc::c_char;
                        sy = 8 as libc::c_int as libc::c_char;
                        sz = 8 as libc::c_int as libc::c_char;
                    } else if t < 150 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                    } else if t < 300 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 7 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 7 as libc::c_int as libc::c_char;
                    } else if t < 302 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 12 as libc::c_int as libc::c_char;
                        sy = 8 as libc::c_int as libc::c_char;
                        sz = 8 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 303 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 306 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 7 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 7 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    }
                    current_block_228 = 13434751124187322381;
                }
                _ => return 0 as libc::c_int,
            }
            match current_block_228 {
                11712972921842978987 => {
                    t = nextInt(&mut rng, 204 as libc::c_int);
                    if t < 50 as libc::c_int {
                        (*r).variant = 0 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 4 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                    } else if t < 100 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 10 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 10 as libc::c_int as libc::c_char;
                    } else if t < 150 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 8 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 15 as libc::c_int as libc::c_char;
                    } else if t < 200 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 9 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                    } else if t < 201 as libc::c_int {
                        (*r).variant = 0 as libc::c_int as libc::c_char;
                        sx = 9 as libc::c_int as libc::c_char;
                        sy = 4 as libc::c_int as libc::c_char;
                        sz = 9 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 202 as libc::c_int {
                        (*r).variant = 1 as libc::c_int as libc::c_char;
                        sx = 10 as libc::c_int as libc::c_char;
                        sy = 7 as libc::c_int as libc::c_char;
                        sz = 10 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 203 as libc::c_int {
                        (*r).variant = 2 as libc::c_int as libc::c_char;
                        sx = 8 as libc::c_int as libc::c_char;
                        sy = 5 as libc::c_int as libc::c_char;
                        sz = 15 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    } else if t < 204 as libc::c_int {
                        (*r).variant = 3 as libc::c_int as libc::c_char;
                        sx = 11 as libc::c_int as libc::c_char;
                        sy = 9 as libc::c_int as libc::c_char;
                        sz = 11 as libc::c_int as libc::c_char;
                        (*r).abandoned = 1 as libc::c_int as uint8_t;
                    }
                }
                _ => {}
            }
        }
        17 => {
            (*r).rotation = nextInt(&mut rng, 4 as libc::c_int) as uint8_t;
            (*r).variant = nextInt(&mut rng, 4 as libc::c_int) as libc::c_char;
            match (*r).variant as libc::c_int {
                0 => {
                    sx = 46 as libc::c_int as libc::c_char;
                    sy = 24 as libc::c_int as libc::c_char;
                    sz = 46 as libc::c_int as libc::c_char;
                }
                1 => {
                    sx = 30 as libc::c_int as libc::c_char;
                    sy = 24 as libc::c_int as libc::c_char;
                    sz = 48 as libc::c_int as libc::c_char;
                }
                2 => {
                    sx = 38 as libc::c_int as libc::c_char;
                    sy = 48 as libc::c_int as libc::c_char;
                    sz = 38 as libc::c_int as libc::c_char;
                }
                3 => {
                    sx = 16 as libc::c_int as libc::c_char;
                    sy = 32 as libc::c_int as libc::c_char;
                    sz = 32 as libc::c_int as libc::c_char;
                }
                _ => {}
            }
        }
        13 => {
            (*r).rotation = nextInt(&mut rng, 4 as libc::c_int) as uint8_t;
            (*r)
                .variant = (1 as libc::c_int + nextInt(&mut rng, 3 as libc::c_int))
                as libc::c_char;
            sx = 18 as libc::c_int as libc::c_char;
            sy = 31 as libc::c_int as libc::c_char;
            sz = 41 as libc::c_int as libc::c_char;
        }
        11 => {
            (*r).giant = (nextFloat(&mut rng) < 0.05f32) as libc::c_int as uint8_t;
            if (*r).giant != 0 {
                (*r)
                    .variant = (1 as libc::c_int + nextInt(&mut rng, 3 as libc::c_int))
                    as libc::c_char;
            } else {
                (*r)
                    .variant = (1 as libc::c_int + nextInt(&mut rng, 10 as libc::c_int))
                    as libc::c_char;
            }
            (*r).rotation = nextInt(&mut rng, 4 as libc::c_int) as uint8_t;
            (*r).mirror = (nextFloat(&mut rng) < 0.05f32) as libc::c_int as uint8_t;
            return 1 as libc::c_int;
        }
        8 => {
            let ref mut fresh31 = (*r).z;
            *fresh31 = -(29 as libc::c_int) as libc::c_char;
            (*r).x = *fresh31;
            let ref mut fresh32 = (*r).sz;
            *fresh32 = 58 as libc::c_int as libc::c_char;
            (*r).sx = *fresh32;
            return 1 as libc::c_int;
        }
        1 => {
            (*r).sx = 21 as libc::c_int as libc::c_char;
            (*r).sz = 21 as libc::c_int as libc::c_char;
            return 1 as libc::c_int;
        }
        2 => {
            (*r).sx = 12 as libc::c_int as libc::c_char;
            (*r).sz = 15 as libc::c_int as libc::c_char;
            return 1 as libc::c_int;
        }
        3 => {
            (*r).sx = 7 as libc::c_int as libc::c_char;
            (*r).sz = 9 as libc::c_int as libc::c_char;
            return 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    }
    (*r).y = 0 as libc::c_int as libc::c_char;
    (*r).sy = sy;
    match (*r).rotation as libc::c_int {
        0 => {
            (*r).x = -((x < 0 as libc::c_int) as libc::c_int) as libc::c_char;
            (*r).z = -((z < 0 as libc::c_int) as libc::c_int) as libc::c_char;
            (*r).sx = sx;
            (*r).sz = sz;
        }
        1 => {
            (*r)
                .x = ((x > 0 as libc::c_int) as libc::c_int - sz as libc::c_int)
                as libc::c_char;
            (*r).z = -((z < 0 as libc::c_int) as libc::c_int) as libc::c_char;
            (*r).sx = sz;
            (*r).sz = sx;
        }
        2 => {
            (*r)
                .x = ((x > 0 as libc::c_int) as libc::c_int - sx as libc::c_int)
                as libc::c_char;
            (*r)
                .z = ((z > 0 as libc::c_int) as libc::c_int - sz as libc::c_int)
                as libc::c_char;
            (*r).sx = sx;
            (*r).sz = sz;
        }
        3 => {
            (*r).x = -((x < 0 as libc::c_int) as libc::c_int) as libc::c_char;
            (*r)
                .z = ((z > 0 as libc::c_int) as libc::c_int - sx as libc::c_int)
                as libc::c_char;
            (*r).sx = sz;
            (*r).sz = sx;
        }
        _ => return 0 as libc::c_int,
    }
    if structType == Ancient_City as libc::c_int {
        sx = 13 as libc::c_int as libc::c_char;
        sz = 20 as libc::c_int as libc::c_char;
        match (*r).rotation as libc::c_int {
            0 => {
                let ref mut fresh33 = (*r).x;
                *fresh33 = (*fresh33 as libc::c_int - sx as libc::c_int) as libc::c_char;
                let ref mut fresh34 = (*r).z;
                *fresh34 = (*fresh34 as libc::c_int - sz as libc::c_int) as libc::c_char;
            }
            1 => {
                let ref mut fresh35 = (*r).x;
                *fresh35 = (*fresh35 as libc::c_int + sz as libc::c_int) as libc::c_char;
                let ref mut fresh36 = (*r).z;
                *fresh36 = (*fresh36 as libc::c_int - sx as libc::c_int) as libc::c_char;
            }
            2 => {
                let ref mut fresh37 = (*r).x;
                *fresh37 = (*fresh37 as libc::c_int + sx as libc::c_int) as libc::c_char;
                let ref mut fresh38 = (*r).z;
                *fresh38 = (*fresh38 as libc::c_int + sz as libc::c_int) as libc::c_char;
            }
            3 => {
                let ref mut fresh39 = (*r).x;
                *fresh39 = (*fresh39 as libc::c_int - sz as libc::c_int) as libc::c_char;
                let ref mut fresh40 = (*r).z;
                *fresh40 = (*fresh40 as libc::c_int + sx as libc::c_int) as libc::c_char;
            }
            _ => {}
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getHouseList(
    mut worldSeed: uint64_t,
    mut chunkX: libc::c_int,
    mut chunkZ: libc::c_int,
    mut out: *mut libc::c_int,
) -> uint64_t {
    let mut rng: uint64_t = chunkGenerateRnd(worldSeed, chunkX, chunkZ);
    skipNextN(&mut rng, 1 as libc::c_int as uint64_t);
    *out
        .offset(
            HouseSmall as libc::c_int as isize,
        ) = nextInt(&mut rng, 4 as libc::c_int - 2 as libc::c_int + 1 as libc::c_int)
        + 2 as libc::c_int;
    *out
        .offset(
            Church as libc::c_int as isize,
        ) = nextInt(&mut rng, 1 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int)
        + 0 as libc::c_int;
    *out
        .offset(
            Library as libc::c_int as isize,
        ) = nextInt(&mut rng, 2 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int)
        + 0 as libc::c_int;
    *out
        .offset(
            WoodHut as libc::c_int as isize,
        ) = nextInt(&mut rng, 5 as libc::c_int - 2 as libc::c_int + 1 as libc::c_int)
        + 2 as libc::c_int;
    *out
        .offset(
            Butcher as libc::c_int as isize,
        ) = nextInt(&mut rng, 2 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int)
        + 0 as libc::c_int;
    *out
        .offset(
            FarmLarge as libc::c_int as isize,
        ) = nextInt(&mut rng, 4 as libc::c_int - 1 as libc::c_int + 1 as libc::c_int)
        + 1 as libc::c_int;
    *out
        .offset(
            FarmSmall as libc::c_int as isize,
        ) = nextInt(&mut rng, 4 as libc::c_int - 2 as libc::c_int + 1 as libc::c_int)
        + 2 as libc::c_int;
    *out
        .offset(
            Blacksmith as libc::c_int as isize,
        ) = nextInt(&mut rng, 1 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int)
        + 0 as libc::c_int;
    *out
        .offset(
            HouseLarge as libc::c_int as isize,
        ) = nextInt(&mut rng, 3 as libc::c_int - 0 as libc::c_int + 1 as libc::c_int)
        + 0 as libc::c_int;
    return rng;
}
#[no_mangle]
pub unsafe extern "C" fn setupBiomeFilter(
    mut bf: *mut BiomeFilter,
    mut mc: libc::c_int,
    mut flags: uint32_t,
    mut required: *const libc::c_int,
    mut requiredLen: libc::c_int,
    mut excluded: *const libc::c_int,
    mut excludedLen: libc::c_int,
    mut matchany: *const libc::c_int,
    mut matchanyLen: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    memset(
        bf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<BiomeFilter>() as libc::c_ulong,
    );
    (*bf).flags = flags;
    i = 0 as libc::c_int;
    while i < matchanyLen {
        id = *matchany.offset(i as isize);
        if id < 128 as libc::c_int {
            let ref mut fresh41 = (*bf).biomeToPick;
            *fresh41 = (*fresh41 as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
                as uint64_t;
        } else {
            let ref mut fresh42 = (*bf).biomeToPickM;
            *fresh42 = (*fresh42 as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
        }
        let mut ibf: BiomeFilter = BiomeFilter {
            tempsToFind: 0,
            otempToFind: 0,
            majorToFind: 0,
            edgesToFind: 0,
            raresToFind: 0,
            raresToFindM: 0,
            shoreToFind: 0,
            shoreToFindM: 0,
            riverToFind: 0,
            riverToFindM: 0,
            oceanToFind: 0,
            specialCnt: 0,
            flags: 0,
            tempsToExcl: 0,
            majorToExcl: 0,
            edgesToExcl: 0,
            raresToExcl: 0,
            raresToExclM: 0,
            shoreToExcl: 0,
            shoreToExclM: 0,
            riverToExcl: 0,
            riverToExclM: 0,
            biomeToExcl: 0,
            biomeToExclM: 0,
            biomeToFind: 0,
            biomeToFindM: 0,
            biomeToPick: 0,
            biomeToPickM: 0,
        };
        setupBiomeFilter(
            &mut ibf,
            mc,
            0 as libc::c_int as uint32_t,
            &mut id,
            1 as libc::c_int,
            0 as *const libc::c_int,
            0 as libc::c_int,
            0 as *const libc::c_int,
            0 as libc::c_int,
        );
        if i == 0 as libc::c_int {
            (*bf).tempsToFind = ibf.tempsToFind;
            (*bf).otempToFind = ibf.otempToFind;
            (*bf).majorToFind = ibf.majorToFind;
            (*bf).edgesToFind = ibf.edgesToFind;
            (*bf).raresToFind = ibf.raresToFind;
            (*bf).raresToFindM = ibf.raresToFindM;
            (*bf).shoreToFind = ibf.shoreToFind;
            (*bf).shoreToFindM = ibf.shoreToFindM;
            (*bf).riverToFind = ibf.riverToFind;
            (*bf).riverToFindM = ibf.riverToFindM;
            (*bf).oceanToFind = ibf.oceanToFind;
        } else {
            let ref mut fresh43 = (*bf).tempsToFind;
            *fresh43 &= ibf.tempsToFind;
            let ref mut fresh44 = (*bf).otempToFind;
            *fresh44 &= ibf.otempToFind;
            let ref mut fresh45 = (*bf).majorToFind;
            *fresh45 &= ibf.majorToFind;
            let ref mut fresh46 = (*bf).edgesToFind;
            *fresh46 &= ibf.edgesToFind;
            let ref mut fresh47 = (*bf).raresToFind;
            *fresh47 &= ibf.raresToFind;
            let ref mut fresh48 = (*bf).raresToFindM;
            *fresh48 &= ibf.raresToFindM;
            let ref mut fresh49 = (*bf).shoreToFind;
            *fresh49 &= ibf.shoreToFind;
            let ref mut fresh50 = (*bf).shoreToFindM;
            *fresh50 &= ibf.shoreToFindM;
            let ref mut fresh51 = (*bf).riverToFind;
            *fresh51 &= ibf.riverToFind;
            let ref mut fresh52 = (*bf).riverToFindM;
            *fresh52 &= ibf.riverToFindM;
            let ref mut fresh53 = (*bf).oceanToFind;
            *fresh53 &= ibf.oceanToFind;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < excludedLen {
        id = *excluded.offset(i as isize);
        if id & !(0xbf as libc::c_int) != 0 {
            fprintf(
                stderr,
                b"setupBiomeFilter: biomeID=%d not supported.\n\0" as *const u8
                    as *const libc::c_char,
                id,
            );
            exit(-(1 as libc::c_int));
        }
        if id < 128 as libc::c_int {
            let ref mut fresh54 = (*bf).biomeToExcl;
            *fresh54 = (*fresh54 as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
                as uint64_t;
        } else {
            let ref mut fresh55 = (*bf).biomeToExclM;
            *fresh55 = (*fresh55 as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
        }
        i += 1;
    }
    if excludedLen != 0 && mc >= MC_1_7 as libc::c_int {
        let mut b: uint64_t = 0;
        let mut m: uint64_t = 0;
        let mut j: libc::c_int = 0;
        j = Oceanic as libc::c_int;
        while j <= Freezing as libc::c_int + Special as libc::c_int {
            m = 0 as libc::c_int as uint64_t;
            b = m;
            let mut temp: libc::c_int = if j <= Freezing as libc::c_int {
                j
            } else {
                j - Special as libc::c_int | 0xf00 as libc::c_int
            };
            genPotential(&mut b, &mut m, L_SPECIAL_1024 as libc::c_int, mc, temp);
            if (*bf).biomeToExcl & b != 0 || (*bf).biomeToExclM & m != 0 {
                let ref mut fresh56 = (*bf).tempsToExcl;
                *fresh56 = (*fresh56 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << j) as uint64_t;
            }
            j += 1;
        }
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            if !(isOverworld(mc, j) == 0) {
                if j < 128 as libc::c_int {
                    m = 0 as libc::c_int as uint64_t;
                    b = m;
                    genPotential(&mut b, &mut m, L_BIOME_256 as libc::c_int, mc, j);
                    if !(*bf).biomeToExcl & b != 0 || !(*bf).biomeToExclM & m != 0 {
                        let ref mut fresh57 = (*bf).majorToExcl;
                        *fresh57 = (*fresh57 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j) as uint64_t;
                    }
                }
                m = 0 as libc::c_int as uint64_t;
                b = m;
                genPotential(&mut b, &mut m, L_BIOME_EDGE_64 as libc::c_int, mc, j);
                if !(*bf).biomeToExcl & b != 0 || !(*bf).biomeToExclM & m != 0 {
                    if j < 128 as libc::c_int {
                        let ref mut fresh58 = (*bf).edgesToExcl;
                        *fresh58 = (*fresh58 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j) as uint64_t;
                    } else {
                        let ref mut fresh59 = (*bf).edgesToExcl;
                        *fresh59 = (*fresh59 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j - 128 as libc::c_int)
                            as uint64_t;
                    }
                }
                m = 0 as libc::c_int as uint64_t;
                b = m;
                genPotential(&mut b, &mut m, L_SUNFLOWER_64 as libc::c_int, mc, j);
                if !(*bf).biomeToExcl & b != 0 || !(*bf).biomeToExclM & m != 0 {
                    if j < 128 as libc::c_int {
                        let ref mut fresh60 = (*bf).raresToExcl;
                        *fresh60 = (*fresh60 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j) as uint64_t;
                    } else {
                        let ref mut fresh61 = (*bf).raresToExclM;
                        *fresh61 = (*fresh61 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j - 128 as libc::c_int)
                            as uint64_t;
                    }
                }
                m = 0 as libc::c_int as uint64_t;
                b = m;
                genPotential(&mut b, &mut m, L_SHORE_16 as libc::c_int, mc, j);
                if !(*bf).biomeToExcl & b != 0 || !(*bf).biomeToExclM & m != 0 {
                    if j < 128 as libc::c_int {
                        let ref mut fresh62 = (*bf).shoreToExcl;
                        *fresh62 = (*fresh62 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j) as uint64_t;
                    } else {
                        let ref mut fresh63 = (*bf).shoreToExclM;
                        *fresh63 = (*fresh63 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j - 128 as libc::c_int)
                            as uint64_t;
                    }
                }
                m = 0 as libc::c_int as uint64_t;
                b = m;
                genPotential(&mut b, &mut m, L_RIVER_MIX_4 as libc::c_int, mc, j);
                if !(*bf).biomeToExcl & b != 0 || !(*bf).biomeToExclM & m != 0 {
                    if j < 128 as libc::c_int {
                        let ref mut fresh64 = (*bf).riverToExcl;
                        *fresh64 = (*fresh64 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j) as uint64_t;
                    } else {
                        let ref mut fresh65 = (*bf).riverToExclM;
                        *fresh65 = (*fresh65 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << j - 128 as libc::c_int)
                            as uint64_t;
                    }
                }
            }
            j += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < requiredLen {
        id = *required.offset(i as isize);
        if id & !(0xbf as libc::c_int) != 0 {
            fprintf(
                stderr,
                b"setupBiomeFilter: biomeID=%d not supported.\n\0" as *const u8
                    as *const libc::c_char,
                id,
            );
            exit(-(1 as libc::c_int));
        }
        let mut current_block_235: u64;
        match id {
            14 => {
                let ref mut fresh66 = (*bf).raresToFind;
                *fresh66 = (*fresh66 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << mushroom_fields as libc::c_int)
                    as uint64_t;
                current_block_235 = 966939755071783605;
            }
            15 => {
                current_block_235 = 966939755071783605;
            }
            39 | 38 | 37 | 165 | 167 | 166 => {
                let ref mut fresh70 = (*bf).tempsToFind;
                *fresh70 = (*fresh70 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong)
                        << Warm as libc::c_int + Special as libc::c_int) as uint64_t;
                if id == badlands_plateau as libc::c_int
                    || id == modified_badlands_plateau as libc::c_int
                {
                    let ref mut fresh71 = (*bf).majorToFind;
                    *fresh71 = (*fresh71 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << badlands_plateau as libc::c_int)
                        as uint64_t;
                }
                if id == wooded_badlands_plateau as libc::c_int
                    || id == modified_wooded_badlands_plateau as libc::c_int
                {
                    let ref mut fresh72 = (*bf).majorToFind;
                    *fresh72 = (*fresh72 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong)
                            << wooded_badlands_plateau as libc::c_int) as uint64_t;
                }
                if id < 128 as libc::c_int {
                    let ref mut fresh73 = (*bf).raresToFind;
                    *fresh73 = (*fresh73 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    let ref mut fresh74 = (*bf).riverToFind;
                    *fresh74 = (*fresh74 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh75 = (*bf).raresToFindM;
                    *fresh75 = (*fresh75 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh76 = (*bf).riverToFindM;
                    *fresh76 = (*fresh76 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
            21 | 23 | 22 | 149 | 151 | 168 | 169 => {
                let ref mut fresh77 = (*bf).tempsToFind;
                *fresh77 = (*fresh77 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong)
                        << Lush as libc::c_int + Special as libc::c_int) as uint64_t;
                let ref mut fresh78 = (*bf).majorToFind;
                *fresh78 = (*fresh78 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << jungle as libc::c_int) as uint64_t;
                if id == bamboo_jungle as libc::c_int
                    || id == bamboo_jungle_hills as libc::c_int
                {
                    let ref mut fresh79 = (*bf).edgesToFind;
                    *fresh79 = (*fresh79 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong)
                            << (bamboo_jungle as libc::c_int & 0x3f as libc::c_int))
                        as uint64_t;
                    let ref mut fresh80 = (*bf).raresToFindM;
                    *fresh80 = (*fresh80 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh81 = (*bf).riverToFindM;
                    *fresh81 = (*fresh81 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                } else if id == jungle_edge as libc::c_int {
                    let ref mut fresh82 = (*bf).riverToFind;
                    *fresh82 = (*fresh82 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << jungle_edge as libc::c_int)
                        as uint64_t;
                } else {
                    if id == modified_jungle_edge as libc::c_int {
                        let ref mut fresh83 = (*bf).edgesToFind;
                        *fresh83 = (*fresh83 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << jungle_edge as libc::c_int)
                            as uint64_t;
                    } else {
                        let ref mut fresh84 = (*bf).edgesToFind;
                        *fresh84 = (*fresh84 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << jungle as libc::c_int)
                            as uint64_t;
                    }
                    if id < 128 as libc::c_int {
                        let ref mut fresh85 = (*bf).raresToFind;
                        *fresh85 = (*fresh85 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id) as uint64_t;
                        let ref mut fresh86 = (*bf).riverToFind;
                        *fresh86 = (*fresh86 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id) as uint64_t;
                    } else {
                        let ref mut fresh87 = (*bf).raresToFindM;
                        *fresh87 = (*fresh87 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                            as uint64_t;
                        let ref mut fresh88 = (*bf).riverToFindM;
                        *fresh88 = (*fresh88 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                            as uint64_t;
                    }
                }
                current_block_235 = 16988252441985098516;
            }
            32 | 33 | 160 | 161 => {
                let ref mut fresh89 = (*bf).tempsToFind;
                *fresh89 = (*fresh89 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong)
                        << Cold as libc::c_int + Special as libc::c_int) as uint64_t;
                let ref mut fresh90 = (*bf).majorToFind;
                *fresh90 = (*fresh90 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << giant_tree_taiga as libc::c_int)
                    as uint64_t;
                let ref mut fresh91 = (*bf).edgesToFind;
                *fresh91 = (*fresh91 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << giant_tree_taiga as libc::c_int)
                    as uint64_t;
                if id < 128 as libc::c_int {
                    let ref mut fresh92 = (*bf).raresToFind;
                    *fresh92 = (*fresh92 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    let ref mut fresh93 = (*bf).riverToFind;
                    *fresh93 = (*fresh93 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh94 = (*bf).raresToFindM;
                    *fresh94 = (*fresh94 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh95 = (*bf).riverToFindM;
                    *fresh95 = (*fresh95 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
            35 | 36 | 163 | 164 | 17 | 130 => {
                let ref mut fresh96 = (*bf).tempsToFind;
                *fresh96 = (*fresh96 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << Warm as libc::c_int) as uint64_t;
                if id == desert_hills as libc::c_int || id == desert_lakes as libc::c_int
                {
                    let ref mut fresh97 = (*bf).majorToFind;
                    *fresh97 = (*fresh97 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << desert as libc::c_int) as uint64_t;
                    let ref mut fresh98 = (*bf).edgesToFind;
                    *fresh98 = (*fresh98 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << desert as libc::c_int) as uint64_t;
                } else {
                    let ref mut fresh99 = (*bf).majorToFind;
                    *fresh99 = (*fresh99 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << savanna as libc::c_int)
                        as uint64_t;
                    let ref mut fresh100 = (*bf).edgesToFind;
                    *fresh100 = (*fresh100 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << savanna as libc::c_int)
                        as uint64_t;
                }
                if id < 128 as libc::c_int {
                    let ref mut fresh101 = (*bf).raresToFind;
                    *fresh101 = (*fresh101 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    let ref mut fresh102 = (*bf).riverToFind;
                    *fresh102 = (*fresh102 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh103 = (*bf).raresToFindM;
                    *fresh103 = (*fresh103 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh104 = (*bf).riverToFindM;
                    *fresh104 = (*fresh104 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
            29 | 157 | 27 | 28 | 155 | 156 | 6 | 134 => {
                let ref mut fresh105 = (*bf).tempsToFind;
                *fresh105 = (*fresh105 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << Lush as libc::c_int) as uint64_t;
                if id == dark_forest as libc::c_int
                    || id == dark_forest_hills as libc::c_int
                {
                    let ref mut fresh106 = (*bf).majorToFind;
                    *fresh106 = (*fresh106 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << dark_forest as libc::c_int)
                        as uint64_t;
                    let ref mut fresh107 = (*bf).edgesToFind;
                    *fresh107 = (*fresh107 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << dark_forest as libc::c_int)
                        as uint64_t;
                } else if id == birch_forest as libc::c_int
                        || id == birch_forest_hills as libc::c_int
                        || id == tall_birch_forest as libc::c_int
                        || id == tall_birch_hills as libc::c_int
                    {
                    let ref mut fresh108 = (*bf).majorToFind;
                    *fresh108 = (*fresh108 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << birch_forest as libc::c_int)
                        as uint64_t;
                    let ref mut fresh109 = (*bf).edgesToFind;
                    *fresh109 = (*fresh109 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << birch_forest as libc::c_int)
                        as uint64_t;
                } else if id == swamp as libc::c_int || id == swamp_hills as libc::c_int
                    {
                    let ref mut fresh110 = (*bf).majorToFind;
                    *fresh110 = (*fresh110 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << swamp as libc::c_int) as uint64_t;
                    let ref mut fresh111 = (*bf).edgesToFind;
                    *fresh111 = (*fresh111 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << swamp as libc::c_int) as uint64_t;
                }
                if id < 128 as libc::c_int {
                    let ref mut fresh112 = (*bf).raresToFind;
                    *fresh112 = (*fresh112 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    let ref mut fresh113 = (*bf).riverToFind;
                    *fresh113 = (*fresh113 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh114 = (*bf).raresToFindM;
                    *fresh114 = (*fresh114 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh115 = (*bf).riverToFindM;
                    *fresh115 = (*fresh115 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
            30 | 31 | 158 | 12 | 13 | 140 | 11 => {
                let ref mut fresh116 = (*bf).tempsToFind;
                *fresh116 = (*fresh116 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << Freezing as libc::c_int) as uint64_t;
                if id == snowy_taiga as libc::c_int
                    || id == snowy_taiga_hills as libc::c_int
                    || id == snowy_taiga_mountains as libc::c_int
                {
                    let ref mut fresh117 = (*bf).edgesToFind;
                    *fresh117 = (*fresh117 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << snowy_taiga as libc::c_int)
                        as uint64_t;
                } else {
                    let ref mut fresh118 = (*bf).edgesToFind;
                    *fresh118 = (*fresh118 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << snowy_tundra as libc::c_int)
                        as uint64_t;
                }
                if id == frozen_river as libc::c_int {
                    let ref mut fresh119 = (*bf).raresToFind;
                    *fresh119 = (*fresh119 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << snowy_tundra as libc::c_int)
                        as uint64_t;
                    let ref mut fresh120 = (*bf).riverToFind;
                    *fresh120 = (*fresh120 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else if id < 128 as libc::c_int {
                    let ref mut fresh121 = (*bf).raresToFind;
                    *fresh121 = (*fresh121 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    let ref mut fresh122 = (*bf).riverToFind;
                    *fresh122 = (*fresh122 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh123 = (*bf).raresToFindM;
                    *fresh123 = (*fresh123 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                    let ref mut fresh124 = (*bf).riverToFindM;
                    *fresh124 = (*fresh124 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
            129 => {
                let ref mut fresh125 = (*bf).raresToFindM;
                *fresh125 = (*fresh125 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                let ref mut fresh126 = (*bf).riverToFindM;
                *fresh126 = (*fresh126 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            26 => {
                let ref mut fresh127 = (*bf).tempsToFind;
                *fresh127 = (*fresh127 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << Freezing as libc::c_int) as uint64_t;
                current_block_235 = 14496385620556043821;
            }
            16 | 25 => {
                current_block_235 = 14496385620556043821;
            }
            3 => {
                let ref mut fresh129 = (*bf).majorToFind;
                *fresh129 = (*fresh129 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << mountains as libc::c_int) as uint64_t;
                current_block_235 = 18266276585208273763;
            }
            34 => {
                current_block_235 = 18266276585208273763;
            }
            131 => {
                let ref mut fresh132 = (*bf).majorToFind;
                *fresh132 = (*fresh132 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << mountains as libc::c_int) as uint64_t;
                current_block_235 = 15482313706129935342;
            }
            162 => {
                current_block_235 = 15482313706129935342;
            }
            5 | 19 => {
                let ref mut fresh135 = (*bf).edgesToFind;
                *fresh135 = (*fresh135 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << taiga as libc::c_int) as uint64_t;
                let ref mut fresh136 = (*bf).raresToFind;
                *fresh136 = (*fresh136 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                let ref mut fresh137 = (*bf).riverToFind;
                *fresh137 = (*fresh137 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            133 => {
                let ref mut fresh138 = (*bf).edgesToFind;
                *fresh138 = (*fresh138 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << taiga as libc::c_int) as uint64_t;
                let ref mut fresh139 = (*bf).raresToFindM;
                *fresh139 = (*fresh139 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                let ref mut fresh140 = (*bf).riverToFindM;
                *fresh140 = (*fresh140 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            1 | 4 | 18 => {
                let ref mut fresh141 = (*bf).raresToFind;
                *fresh141 = (*fresh141 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                let ref mut fresh142 = (*bf).riverToFind;
                *fresh142 = (*fresh142 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            132 => {
                let ref mut fresh143 = (*bf).raresToFindM;
                *fresh143 = (*fresh143 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                let ref mut fresh144 = (*bf).riverToFindM;
                *fresh144 = (*fresh144 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            2 => {
                let ref mut fresh145 = (*bf).riverToFind;
                *fresh145 = (*fresh145 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                current_block_235 = 16988252441985098516;
            }
            _ => {
                if isOceanic(id) != 0 {
                    let ref mut fresh146 = (*bf).tempsToFind;
                    *fresh146 = (*fresh146 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << Oceanic as libc::c_int)
                        as uint64_t;
                    let ref mut fresh147 = (*bf).oceanToFind;
                    *fresh147 = (*fresh147 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                    if isShallowOcean(id) != 0 {
                        if id != lukewarm_ocean as libc::c_int
                            && id != cold_ocean as libc::c_int
                        {
                            let ref mut fresh148 = (*bf).otempToFind;
                            *fresh148 = (*fresh148 as libc::c_ulonglong
                                | (1 as libc::c_ulonglong) << id) as uint64_t;
                        }
                    } else {
                        let ref mut fresh149 = (*bf).raresToFind;
                        *fresh149 = (*fresh149 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << deep_ocean as libc::c_int)
                            as uint64_t;
                        let ref mut fresh150 = (*bf).riverToFind;
                        *fresh150 = (*fresh150 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << deep_ocean as libc::c_int)
                            as uint64_t;
                        if id == deep_warm_ocean as libc::c_int {
                            let ref mut fresh151 = (*bf).otempToFind;
                            *fresh151 = (*fresh151 as libc::c_ulonglong
                                | (1 as libc::c_ulonglong) << warm_ocean as libc::c_int)
                                as uint64_t;
                        } else if id == deep_ocean as libc::c_int {
                            let ref mut fresh152 = (*bf).otempToFind;
                            *fresh152 = (*fresh152 as libc::c_ulonglong
                                | (1 as libc::c_ulonglong) << ocean as libc::c_int)
                                as uint64_t;
                        } else if id == deep_frozen_ocean as libc::c_int {
                            let ref mut fresh153 = (*bf).otempToFind;
                            *fresh153 = (*fresh153 as libc::c_ulonglong
                                | (1 as libc::c_ulonglong) << frozen_ocean as libc::c_int)
                                as uint64_t;
                        }
                    }
                } else if id < 64 as libc::c_int {
                    let ref mut fresh154 = (*bf).riverToFind;
                    *fresh154 = (*fresh154 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id) as uint64_t;
                } else {
                    let ref mut fresh155 = (*bf).riverToFindM;
                    *fresh155 = (*fresh155 as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                        as uint64_t;
                }
                current_block_235 = 16988252441985098516;
            }
        }
        match current_block_235 {
            15482313706129935342 => {
                let ref mut fresh133 = (*bf).raresToFindM;
                *fresh133 = (*fresh133 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
                let ref mut fresh134 = (*bf).riverToFindM;
                *fresh134 = (*fresh134 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
            }
            18266276585208273763 => {
                let ref mut fresh130 = (*bf).raresToFind;
                *fresh130 = (*fresh130 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
                let ref mut fresh131 = (*bf).riverToFind;
                *fresh131 = (*fresh131 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
            }
            966939755071783605 => {
                let ref mut fresh67 = (*bf).tempsToFind;
                *fresh67 = (*fresh67 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << Oceanic as libc::c_int) as uint64_t;
                let ref mut fresh68 = (*bf).majorToFind;
                *fresh68 = (*fresh68 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << mushroom_fields as libc::c_int)
                    as uint64_t;
                let ref mut fresh69 = (*bf).riverToFind;
                *fresh69 = (*fresh69 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
            }
            14496385620556043821 => {
                let ref mut fresh128 = (*bf).riverToFind;
                *fresh128 = (*fresh128 as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id) as uint64_t;
            }
            _ => {}
        }
        i += 1;
    }
    (*bf).biomeToFind = (*bf).riverToFind;
    let ref mut fresh156 = (*bf).biomeToFind;
    *fresh156 = (*fresh156 as libc::c_ulonglong
        & !((1 as libc::c_ulonglong) << ocean as libc::c_int
            | (1 as libc::c_ulonglong) << deep_ocean as libc::c_int)) as uint64_t;
    let ref mut fresh157 = (*bf).biomeToFind;
    *fresh157 |= (*bf).oceanToFind;
    (*bf).biomeToFindM = (*bf).riverToFindM;
    (*bf).shoreToFind = (*bf).riverToFind;
    let ref mut fresh158 = (*bf).shoreToFind;
    *fresh158 = (*fresh158 as libc::c_ulonglong
        & !((1 as libc::c_ulonglong) << river as libc::c_int
            | (1 as libc::c_ulonglong) << frozen_river as libc::c_int)) as uint64_t;
    (*bf).shoreToFindM = (*bf).riverToFindM;
    (*bf).specialCnt = 0 as libc::c_int;
    (*bf).specialCnt
        += ((*bf).tempsToFind as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << Warm as libc::c_int + Special as libc::c_int
            != 0) as libc::c_int;
    (*bf).specialCnt
        += ((*bf).tempsToFind as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << Lush as libc::c_int + Special as libc::c_int
            != 0) as libc::c_int;
    (*bf).specialCnt
        += ((*bf).tempsToFind as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << Cold as libc::c_int + Special as libc::c_int
            != 0) as libc::c_int;
}
unsafe extern "C" fn f_graddesc_test(
    mut data: *mut libc::c_void,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut p: libc::c_double,
) -> libc::c_int {
    let mut info: *mut gdt_info_t = data as *mut gdt_info_t;
    if !((*info).stop).is_null() && *(*info).stop as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    let mut idx: libc::c_int = (z - (*info).r.z) * (*info).r.sx + (x - (*info).r.x);
    if *((*info).ids).offset(idx as isize) != -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut id: libc::c_int = getBiomeAt((*info).g, (*info).r.scale, x, (*info).r.y, z);
    *((*info).ids).offset(idx as isize) = id;
    if id < 128 as libc::c_int {
        let ref mut fresh159 = (*info).b;
        *fresh159 = (*fresh159 as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
            as uint64_t;
    } else {
        let ref mut fresh160 = (*info).m;
        *fresh160 = (*fresh160 as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
    }
    let mut match_exc: libc::c_int = ((*info).bexc | (*info).mexc
        == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut match_any: libc::c_int = ((*info).bany | (*info).many
        == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut match_req: libc::c_int = ((*info).breq | (*info).mreq
        == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    match_exc
        |= (((*info).b & (*info).bexc != 0 || (*info).m & (*info).mexc != 0)
            as libc::c_int == 0 as libc::c_int) as libc::c_int;
    match_any
        |= ((*info).b & (*info).bany != 0 || (*info).m & (*info).many != 0)
            as libc::c_int;
    match_req
        |= ((*info).b & (*info).breq == (*info).breq
            && (*info).m & (*info).mreq == (*info).mreq) as libc::c_int;
    if match_exc != 0 && match_any != 0 && match_req != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkForBiomes(
    mut g: *mut Generator,
    mut cache: *mut libc::c_int,
    mut r: Range,
    mut dim: libc::c_int,
    mut seed: uint64_t,
    mut filter: *const BiomeFilter,
    mut stop: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    if !stop.is_null() && *stop as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if (*g).mc <= MC_1_17 as libc::c_int && dim == 0 as libc::c_int {
        let mut entry: *mut Layer = getLayerForScale(g, r.scale) as *mut Layer;
        ret = checkForBiomesAtLayer(
            &mut (*g).c2rust_unnamed.c2rust_unnamed.ls,
            entry,
            cache,
            seed,
            r.x,
            r.z,
            r.sx as libc::c_uint,
            r.sz as libc::c_uint,
            filter,
        );
        if ret == 0 as libc::c_int && r.sy > 1 as libc::c_int && !cache.is_null() {
            i = 0 as libc::c_int;
            while i < r.sy {
                j = 0 as libc::c_int;
                while j < r.sx * r.sz {
                    *cache
                        .offset(
                            (i * r.sx * r.sz + j) as isize,
                        ) = *cache.offset(j as isize);
                    j += 1;
                }
                i += 1;
            }
        }
        return ret;
    }
    let mut ids: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut id: libc::c_int = 0;
    if !cache.is_null() {
        ids = cache;
    } else {
        ids = allocCache(g, r);
    }
    if (*g).dim != dim || (*g).seed != seed {
        applySeed(g, dim, seed);
    }
    let mut info: [gdt_info_t; 1] = [gdt_info_t {
        g: 0 as *mut Generator,
        ids: 0 as *mut libc::c_int,
        r: Range {
            scale: 0,
            x: 0,
            z: 0,
            sx: 0,
            sz: 0,
            y: 0,
            sy: 0,
        },
        flags: 0,
        b: 0,
        m: 0,
        breq: 0,
        mreq: 0,
        bexc: 0,
        mexc: 0,
        bany: 0,
        many: 0,
        stop: 0 as *mut libc::c_char,
    }; 1];
    let ref mut fresh161 = (*info.as_mut_ptr()).g;
    *fresh161 = g;
    let ref mut fresh162 = (*info.as_mut_ptr()).ids;
    *fresh162 = ids;
    (*info.as_mut_ptr()).r = r;
    (*info.as_mut_ptr()).flags = (*filter).flags;
    let ref mut fresh163 = (*info.as_mut_ptr()).m;
    *fresh163 = 0 as libc::c_int as uint64_t;
    (*info.as_mut_ptr()).b = *fresh163;
    (*info.as_mut_ptr()).breq = (*filter).biomeToFind;
    (*info.as_mut_ptr()).mreq = (*filter).biomeToFindM;
    (*info.as_mut_ptr()).bexc = (*filter).biomeToExcl;
    (*info.as_mut_ptr()).mexc = (*filter).biomeToExclM;
    (*info.as_mut_ptr()).bany = (*filter).biomeToPick;
    (*info.as_mut_ptr()).many = (*filter).biomeToPickM;
    let ref mut fresh164 = (*info.as_mut_ptr()).stop;
    *fresh164 = stop;
    ret = 0 as libc::c_int;
    memset(
        ids as *mut libc::c_void,
        -(1 as libc::c_int),
        ((r.sx * r.sz) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    let mut n: libc::c_int = r.sx * r.sy * r.sz;
    let mut trials: libc::c_int = n;
    let mut buf: *mut touple = 0 as *mut touple;
    if r.scale == 4 as libc::c_int && r.sx * r.sz > 64 as libc::c_int {
        let mut tmin: libc::c_double = 0.;
        let mut tmax: libc::c_double = 0.;
        let mut err: libc::c_int = 0 as libc::c_int;
        err = getParaRange(
            &mut (*g).c2rust_unnamed.c2rust_unnamed_0.bn.temperature,
            &mut tmin,
            &mut tmax,
            r.x,
            r.z,
            r.sx,
            r.sz,
            info.as_mut_ptr() as *mut libc::c_void,
            Some(
                f_graddesc_test
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
        );
        if !(err != 0) {
            err = getParaRange(
                &mut (*g).c2rust_unnamed.c2rust_unnamed_0.bn.humidity,
                &mut tmin,
                &mut tmax,
                r.x,
                r.z,
                r.sx,
                r.sz,
                info.as_mut_ptr() as *mut libc::c_void,
                Some(
                    f_graddesc_test
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            libc::c_int,
                            libc::c_int,
                            libc::c_double,
                        ) -> libc::c_int,
                ),
            );
            if !(err != 0) {
                err = getParaRange(
                    &mut (*g).c2rust_unnamed.c2rust_unnamed_0.bn.erosion,
                    &mut tmin,
                    &mut tmax,
                    r.x,
                    r.z,
                    r.sx,
                    r.sz,
                    info.as_mut_ptr() as *mut libc::c_void,
                    Some(
                        f_graddesc_test
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                libc::c_int,
                                libc::c_int,
                                libc::c_double,
                            ) -> libc::c_int,
                    ),
                );
                err != 0;
            }
        }
        if err != 0 || !stop.is_null() && *stop as libc::c_int != 0
            || (*filter).flags & CFB_APPROX as libc::c_int as libc::c_uint != 0
        {
            current_block = 544707285801045047;
        } else {
            current_block = 6717214610478484138;
        }
    } else {
        current_block = 6717214610478484138;
    }
    match current_block {
        6717214610478484138 => {
            buf = malloc(
                (n as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<touple>() as libc::c_ulong),
            ) as *mut touple;
            id = 0 as libc::c_int;
            k = 0 as libc::c_int;
            while k < r.sy {
                j = 0 as libc::c_int;
                while j < r.sz {
                    i = 0 as libc::c_int;
                    while i < r.sx {
                        (*buf.offset(id as isize)).i = id;
                        (*buf.offset(id as isize)).x = i;
                        (*buf.offset(id as isize)).y = k;
                        (*buf.offset(id as isize)).z = j;
                        id += 1;
                        i += 1;
                    }
                    j += 1;
                }
                k += 1;
            }
            if (*filter).flags & CFB_APPROX as libc::c_int as libc::c_uint != 0 {
                let mut t: libc::c_int = 400 as libc::c_int
                    + sqrt(n as libc::c_double) as libc::c_int;
                if trials > t {
                    trials = t;
                }
            }
            i = 0 as libc::c_int;
            while i < trials {
                let mut t_0: touple = touple { i: 0, x: 0, y: 0, z: 0 };
                j = n - i;
                k = rand() % j;
                t_0 = *buf.offset(k as isize);
                if k != j - 1 as libc::c_int {
                    *buf
                        .offset(
                            k as isize,
                        ) = *buf.offset((j - 1 as libc::c_int) as isize);
                    *buf.offset((j - 1 as libc::c_int) as isize) = t_0;
                }
                if !stop.is_null() && *stop as libc::c_int != 0 {
                    break;
                }
                if !(t_0.y == 0 as libc::c_int
                    && *((*info.as_mut_ptr()).ids).offset(t_0.i as isize)
                        != -(1 as libc::c_int))
                {
                    id = getBiomeAt(g, r.scale, r.x + t_0.x, r.y + t_0.y, r.z + t_0.z);
                    *((*info.as_mut_ptr()).ids).offset(t_0.i as isize) = id;
                    if id < 128 as libc::c_int {
                        let ref mut fresh165 = (*info.as_mut_ptr()).b;
                        *fresh165 = (*fresh165 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id) as uint64_t;
                    } else {
                        let ref mut fresh166 = (*info.as_mut_ptr()).m;
                        *fresh166 = (*fresh166 as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << id - 128 as libc::c_int)
                            as uint64_t;
                    }
                    let mut match_exc: libc::c_int = ((*info.as_mut_ptr()).bexc
                        | (*info.as_mut_ptr()).mexc == 0 as libc::c_int as libc::c_ulong)
                        as libc::c_int;
                    let mut match_any: libc::c_int = ((*info.as_mut_ptr()).bany
                        | (*info.as_mut_ptr()).many == 0 as libc::c_int as libc::c_ulong)
                        as libc::c_int;
                    let mut match_req: libc::c_int = ((*info.as_mut_ptr()).breq
                        | (*info.as_mut_ptr()).mreq == 0 as libc::c_int as libc::c_ulong)
                        as libc::c_int;
                    match_exc
                        |= (((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).bexc != 0
                            || (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).mexc != 0)
                            as libc::c_int == 0 as libc::c_int) as libc::c_int;
                    match_any
                        |= ((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).bany != 0
                            || (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).many != 0)
                            as libc::c_int;
                    match_req
                        |= ((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).breq
                            == (*info.as_mut_ptr()).breq
                            && (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).mreq
                                == (*info.as_mut_ptr()).mreq) as libc::c_int;
                    if match_exc != 0 && match_any != 0 && match_req != 0 {
                        break;
                    }
                }
                i += 1;
            }
        }
        _ => {}
    }
    if !stop.is_null() && *stop as libc::c_int != 0 {
        ret = 0 as libc::c_int;
    } else {
        let mut match_exc_0: libc::c_int = ((*info.as_mut_ptr()).bexc
            | (*info.as_mut_ptr()).mexc == 0 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        let mut match_any_0: libc::c_int = ((*info.as_mut_ptr()).bany
            | (*info.as_mut_ptr()).many == 0 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        let mut match_req_0: libc::c_int = ((*info.as_mut_ptr()).breq
            | (*info.as_mut_ptr()).mreq == 0 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        match_exc_0
            |= (((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).bexc != 0
                || (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).mexc != 0)
                as libc::c_int == 0 as libc::c_int) as libc::c_int;
        match_any_0
            |= ((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).bany != 0
                || (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).many != 0)
                as libc::c_int;
        match_req_0
            |= ((*info.as_mut_ptr()).b & (*info.as_mut_ptr()).breq
                == (*info.as_mut_ptr()).breq
                && (*info.as_mut_ptr()).m & (*info.as_mut_ptr()).mreq
                    == (*info.as_mut_ptr()).mreq) as libc::c_int;
        ret = (match_exc_0 != 0 && match_any_0 != 0 && match_req_0 != 0) as libc::c_int;
    }
    if !buf.is_null() {
        free(buf as *mut libc::c_void);
    }
    if ids != cache {
        free(ids as *mut libc::c_void);
    }
    return ret;
}
unsafe extern "C" fn mapFilterSpecial(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut temps: uint64_t = 0;
    let mut specialcnt: libc::c_int = (*(*f).bf).specialCnt;
    if specialcnt > 0 as libc::c_int {
        let mut ss: uint64_t = (*l).startSeed;
        let mut cs: uint64_t = 0;
        j = 0 as libc::c_int;
        while j < h {
            i = 0 as libc::c_int;
            while i < w {
                cs = getChunkSeed(ss, x + i, z + j);
                if mcFirstIsZero(cs, 13 as libc::c_int) != 0 {
                    specialcnt -= 1;
                }
                i += 1;
            }
            j += 1;
        }
        if specialcnt > 0 as libc::c_int {
            return M_STOP as libc::c_int;
        }
    }
    let mut err: libc::c_int = ((*f).map)
        .expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    temps = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut id: libc::c_int = *out.offset((i + w * j) as isize);
            let mut isspecial: libc::c_int = id & 0xf00 as libc::c_int;
            id &= !(0xf00 as libc::c_int);
            if isspecial != 0 && id != Freezing as libc::c_int {
                temps = (temps as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id + Special as libc::c_int)
                    as uint64_t;
            } else {
                temps = (temps as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
                    as uint64_t;
            }
            i += 1;
        }
        j += 1;
    }
    if (*(*f).bf).biomeToExcl != 0 && (*(*f).bf).tempsToFind == 0 {
        if 0 as libc::c_int != 0
            && temps & (*(*f).bf).tempsToExcl == 0 as libc::c_int as libc::c_ulong
        {
            return M_STOP as libc::c_int;
        }
    }
    if temps & (*(*f).bf).tempsToFind ^ (*(*f).bf).tempsToFind != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterMushroom(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    if w * h < 100 as libc::c_int
        && (*(*f).bf).majorToFind as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << mushroom_fields as libc::c_int != 0
    {
        let mut ss: uint64_t = (*l).startSeed;
        let mut cs: uint64_t = 0;
        j = 0 as libc::c_int;
        's_22: loop {
            if !(j < h) {
                current_block = 17965632435239708295;
                break;
            }
            i = 0 as libc::c_int;
            while i < w {
                cs = getChunkSeed(ss, i + x, j + z);
                if mcFirstIsZero(cs, 100 as libc::c_int) != 0 {
                    current_block = 14423782878026222453;
                    break 's_22;
                }
                i += 1;
            }
            j += 1;
        }
        match current_block {
            14423782878026222453 => {}
            _ => return M_STOP as libc::c_int,
        }
    }
    err = ((*f).map).expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    if (*(*f).bf).majorToFind as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << mushroom_fields as libc::c_int != 0
    {
        i = 0 as libc::c_int;
        while i < w * h {
            if *out.offset(i as isize) == mushroom_fields as libc::c_int {
                return 0 as libc::c_int;
            }
            i += 1;
        }
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterBiome(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: uint64_t = 0;
    let mut err: libc::c_int = ((*f).map)
        .expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut id: libc::c_int = *out.offset((i + w * j) as isize);
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
            i += 1;
        }
        j += 1;
    }
    if (*(*f).bf).biomeToExcl != 0 && (*(*f).bf).majorToFind == 0 {
        if !b & (*(*f).bf).majorToExcl != 0 {
            return M_STOP as libc::c_int;
        }
    }
    if b & (*(*f).bf).majorToFind ^ (*(*f).bf).majorToFind != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterOceanTemp(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: uint64_t = 0;
    let mut err: libc::c_int = ((*f).map)
        .expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut id: libc::c_int = *out.offset((i + w * j) as isize);
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
            i += 1;
        }
        j += 1;
    }
    if b & (*(*f).bf).otempToFind ^ (*(*f).bf).otempToFind != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterBiomeEdge(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut b: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    err = ((*f).map).expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < w * h {
        b = (b as libc::c_ulonglong
            | (1 as libc::c_ulonglong)
                << (*out.offset(i as isize) & 0x3f as libc::c_int)) as uint64_t;
        i += 1;
    }
    if (*(*f).bf).edgesToExcl != 0 && (*(*f).bf).edgesToFind == 0 {
        if 0 as libc::c_int != 0
            && b & (*(*f).bf).edgesToExcl == 0 as libc::c_int as libc::c_ulong
        {
            return M_STOP as libc::c_int;
        }
    }
    if b & (*(*f).bf).edgesToFind ^ (*(*f).bf).edgesToFind != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterRareBiome(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut b: uint64_t = 0;
    let mut bm: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    err = ((*f).map).expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    bm = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < w * h {
        let mut id: libc::c_int = *out.offset(i as isize);
        if id < 128 as libc::c_int {
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
        } else {
            bm = (bm as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
        }
        i += 1;
    }
    if ((*(*f).bf).raresToExcl != 0 || (*(*f).bf).raresToExclM != 0)
        && !((*(*f).bf).raresToFind != 0 || (*(*f).bf).raresToFindM != 0)
    {
        if 0 as libc::c_int != 0
            && b & (*(*f).bf).raresToExcl == 0 as libc::c_int as libc::c_ulong
            && bm & (*(*f).bf).raresToExclM == 0 as libc::c_int as libc::c_ulong
        {
            return M_DONE as libc::c_int;
        }
    }
    if b & (*(*f).bf).raresToFind ^ (*(*f).bf).raresToFind != 0 {
        return M_STOP as libc::c_int;
    }
    if bm & (*(*f).bf).raresToFindM ^ (*(*f).bf).raresToFindM != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterShore(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut b: uint64_t = 0;
    let mut bm: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = ((*f).map)
        .expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    bm = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < w * h {
        let mut id: libc::c_int = *out.offset(i as isize);
        if id < 128 as libc::c_int {
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
        } else {
            bm = (bm as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
        }
        i += 1;
    }
    if ((*(*f).bf).shoreToExcl != 0 || (*(*f).bf).shoreToExclM != 0)
        && !((*(*f).bf).shoreToFind != 0 || (*(*f).bf).shoreToFindM != 0)
    {
        if 0 as libc::c_int != 0
            && b & (*(*f).bf).shoreToExcl == 0 as libc::c_int as libc::c_ulong
            && bm & (*(*f).bf).shoreToExclM == 0 as libc::c_int as libc::c_ulong
        {
            return M_DONE as libc::c_int;
        }
    }
    if b & (*(*f).bf).shoreToFind ^ (*(*f).bf).shoreToFind != 0 {
        return M_STOP as libc::c_int;
    }
    if bm & (*(*f).bf).shoreToFindM ^ (*(*f).bf).shoreToFindM != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterRiverMix(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut b: uint64_t = 0;
    let mut bm: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = ((*f).map)
        .expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    bm = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < w * h {
        let mut id: libc::c_int = *out.offset(i as isize);
        if id < 128 as libc::c_int {
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
        } else {
            bm = (bm as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
        }
        i += 1;
    }
    if ((*(*f).bf).riverToExcl != 0 || (*(*f).bf).riverToExclM != 0)
        && !((*(*f).bf).riverToFind != 0 || (*(*f).bf).riverToFindM != 0)
    {
        if 0 as libc::c_int != 0
            && b & (*(*f).bf).riverToExcl == 0 as libc::c_int as libc::c_ulong
            && bm & (*(*f).bf).riverToExclM == 0 as libc::c_int as libc::c_ulong
        {
            return M_DONE as libc::c_int;
        }
    }
    if b & (*(*f).bf).riverToFind ^ (*(*f).bf).riverToFind != 0 {
        return M_STOP as libc::c_int;
    }
    if bm & (*(*f).bf).riverToFindM ^ (*(*f).bf).riverToFindM != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn mapFilterOceanMix(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut f: *const filter_data_t = (*l).data as *const filter_data_t;
    let mut b: uint64_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    if (*(*f).bf).riverToFind != 0 {
        err = ((*(*l).p).getMap)
            .expect("non-null function pointer")((*l).p, out, x, z, w, h);
        if err != 0 {
            return err;
        }
    }
    err = ((*f).map).expect("non-null function pointer")(l, out, x, z, w, h);
    if (err != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return err;
    }
    b = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < w * h {
        let mut id: libc::c_int = *out.offset(i as isize);
        if id < 128 as libc::c_int {
            b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id) as uint64_t;
        }
        i += 1;
    }
    if b & (*(*f).bf).oceanToFind ^ (*(*f).bf).oceanToFind != 0 {
        return M_STOP as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn swapMap(
    mut fd: *mut filter_data_t,
    mut bf: *const BiomeFilter,
    mut l: *mut Layer,
    mut map: Option::<
        unsafe extern "C" fn(
            *const Layer,
            *mut libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
) {
    let ref mut fresh167 = (*fd).bf;
    *fresh167 = bf;
    let ref mut fresh168 = (*fd).map;
    *fresh168 = (*l).getMap;
    let ref mut fresh169 = (*l).data;
    *fresh169 = fd as *mut libc::c_void;
    let ref mut fresh170 = (*l).getMap;
    *fresh170 = map;
}
unsafe extern "C" fn restoreMap(mut fd: *mut filter_data_t, mut l: *mut Layer) {
    let ref mut fresh171 = (*l).getMap;
    *fresh171 = (*fd).map;
    let ref mut fresh172 = (*l).data;
    *fresh172 = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn checkForBiomesAtLayer(
    mut g: *mut LayerStack,
    mut entry: *mut Layer,
    mut cache: *mut libc::c_int,
    mut seed: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_uint,
    mut h: libc::c_uint,
    mut filter: *const BiomeFilter,
) -> libc::c_int {
    let mut l: *mut Layer = 0 as *mut Layer;
    if (*filter).flags & CFB_APPROX as libc::c_int as libc::c_uint != 0 {
        let mut current_block_70: u64;
        l = entry;
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut bx: libc::c_int = x * (*l).scale;
        let mut bz: libc::c_int = z * (*l).scale;
        let mut bw: libc::c_int = w.wrapping_mul((*l).scale as libc::c_uint)
            as libc::c_int;
        let mut bh: libc::c_int = h.wrapping_mul((*l).scale as libc::c_uint)
            as libc::c_int;
        let mut x0: libc::c_int = 0;
        let mut z0: libc::c_int = 0;
        let mut x1: libc::c_int = 0;
        let mut z1: libc::c_int = 0;
        let mut ss: uint64_t = 0;
        let mut cs: uint64_t = 0;
        let mut potential: uint64_t = 0;
        let mut required: uint64_t = 0;
        let mut specialcnt: libc::c_int = (*filter).specialCnt;
        if specialcnt > 0 as libc::c_int {
            l = &mut *((*g).layers)
                .as_mut_ptr()
                .offset(L_SPECIAL_1024 as libc::c_int as isize) as *mut Layer;
            x0 = bx / (*l).scale;
            if x < 0 as libc::c_int {
                x0 -= 1;
            }
            z0 = bz / (*l).scale;
            if z < 0 as libc::c_int {
                z0 -= 1;
            }
            x1 = (bx + bw) / (*l).scale;
            if x + w as libc::c_int >= 0 as libc::c_int {
                x1 += 1;
            }
            z1 = (bz + bh) / (*l).scale;
            if z + h as libc::c_int >= 0 as libc::c_int {
                z1 += 1;
            }
            ss = getStartSeed(seed, (*l).layerSalt);
            j = z0;
            while j <= z1 {
                i = x0;
                while i <= x1 {
                    cs = getChunkSeed(ss, i, j);
                    if mcFirstIsZero(cs, 13 as libc::c_int) != 0 {
                        specialcnt -= 1;
                    }
                    i += 1;
                }
                j += 1;
            }
            if specialcnt > 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        l = &mut *((*g).layers).as_mut_ptr().offset(L_BIOME_256 as libc::c_int as isize)
            as *mut Layer;
        x0 = bx / (*l).scale;
        if x < 0 as libc::c_int {
            x0 -= 1;
        }
        z0 = bz / (*l).scale;
        if z < 0 as libc::c_int {
            z0 -= 1;
        }
        x1 = (bx + bw) / (*l).scale;
        if x + w as libc::c_int >= 0 as libc::c_int {
            x1 += 1;
        }
        z1 = (bz + bh) / (*l).scale;
        if z + h as libc::c_int >= 0 as libc::c_int {
            z1 += 1;
        }
        if (*filter).majorToFind as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << mushroom_fields as libc::c_int != 0
        {
            ss = getStartSeed(
                seed,
                (*g).layers[L_MUSHROOM_256 as libc::c_int as usize].layerSalt,
            );
            j = z0;
            's_228: loop {
                if !(j <= z1) {
                    current_block_70 = 2516253395664191498;
                    break;
                }
                i = x0;
                while i <= x1 {
                    cs = getChunkSeed(ss, i, j);
                    if mcFirstIsZero(cs, 100 as libc::c_int) != 0 {
                        current_block_70 = 1308292425695195108;
                        break 's_228;
                    }
                    i += 1;
                }
                j += 1;
            }
            match current_block_70 {
                1308292425695195108 => {}
                _ => return 0 as libc::c_int,
            }
        }
        potential = 0 as libc::c_int as uint64_t;
        required = ((*filter).majorToFind as libc::c_ulonglong
            & ((1 as libc::c_ulonglong) << badlands_plateau as libc::c_int
                | (1 as libc::c_ulonglong) << wooded_badlands_plateau as libc::c_int
                | (1 as libc::c_ulonglong) << desert as libc::c_int
                | (1 as libc::c_ulonglong) << savanna as libc::c_int
                | (1 as libc::c_ulonglong) << plains as libc::c_int
                | (1 as libc::c_ulonglong) << forest as libc::c_int
                | (1 as libc::c_ulonglong) << dark_forest as libc::c_int
                | (1 as libc::c_ulonglong) << mountains as libc::c_int
                | (1 as libc::c_ulonglong) << birch_forest as libc::c_int
                | (1 as libc::c_ulonglong) << swamp as libc::c_int)) as uint64_t;
        ss = getStartSeed(seed, (*l).layerSalt);
        j = z0;
        while j <= z1 {
            i = x0;
            while i <= x1 {
                cs = getChunkSeed(ss, i, j);
                let mut cs6: libc::c_int = mcFirstInt(cs, 6 as libc::c_int);
                let mut cs3: libc::c_int = mcFirstInt(cs, 3 as libc::c_int);
                let mut cs4: libc::c_int = mcFirstInt(cs, 4 as libc::c_int);
                if cs3 != 0 {
                    potential = (potential as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << badlands_plateau as libc::c_int)
                        as uint64_t;
                } else {
                    potential = (potential as libc::c_ulonglong
                        | (1 as libc::c_ulonglong)
                            << wooded_badlands_plateau as libc::c_int) as uint64_t;
                }
                match cs6 {
                    0 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << desert as libc::c_int
                                | (1 as libc::c_ulonglong) << forest as libc::c_int))
                            as uint64_t;
                    }
                    1 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << desert as libc::c_int
                                | (1 as libc::c_ulonglong) << dark_forest as libc::c_int))
                            as uint64_t;
                    }
                    2 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << desert as libc::c_int
                                | (1 as libc::c_ulonglong) << mountains as libc::c_int))
                            as uint64_t;
                    }
                    3 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << savanna as libc::c_int
                                | (1 as libc::c_ulonglong) << plains as libc::c_int))
                            as uint64_t;
                    }
                    4 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << savanna as libc::c_int
                                | (1 as libc::c_ulonglong) << birch_forest as libc::c_int))
                            as uint64_t;
                    }
                    5 => {
                        potential = (potential as libc::c_ulonglong
                            | ((1 as libc::c_ulonglong) << plains as libc::c_int
                                | (1 as libc::c_ulonglong) << swamp as libc::c_int))
                            as uint64_t;
                    }
                    _ => {}
                }
                if cs4 == 3 as libc::c_int {
                    potential = (potential as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << snowy_taiga as libc::c_int)
                        as uint64_t;
                } else {
                    potential = (potential as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << snowy_tundra as libc::c_int)
                        as uint64_t;
                }
                i += 1;
            }
            j += 1;
        }
        if potential & required ^ required != 0 {
            return 0 as libc::c_int;
        }
    }
    l = ((*g).layers).as_mut_ptr();
    let mut ids: *mut libc::c_int = 0 as *mut libc::c_int;
    if !cache.is_null() {
        ids = cache;
    } else {
        ids = calloc(
            getMinLayerCacheSize(entry, w as libc::c_int, h as libc::c_int),
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
    }
    let mut fd: [filter_data_t; 9] = [filter_data_t {
        bf: 0 as *const BiomeFilter,
        map: None,
    }; 9];
    swapMap(
        fd.as_mut_ptr().offset(0 as libc::c_int as isize),
        filter,
        l.offset(L_OCEAN_MIX_4 as libc::c_int as isize),
        Some(
            mapFilterOceanMix
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(1 as libc::c_int as isize),
        filter,
        l.offset(L_RIVER_MIX_4 as libc::c_int as isize),
        Some(
            mapFilterRiverMix
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(2 as libc::c_int as isize),
        filter,
        l.offset(L_SHORE_16 as libc::c_int as isize),
        Some(
            mapFilterShore
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(3 as libc::c_int as isize),
        filter,
        l.offset(L_SUNFLOWER_64 as libc::c_int as isize),
        Some(
            mapFilterRareBiome
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(4 as libc::c_int as isize),
        filter,
        l.offset(L_BIOME_EDGE_64 as libc::c_int as isize),
        Some(
            mapFilterBiomeEdge
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(5 as libc::c_int as isize),
        filter,
        l.offset(L_OCEAN_TEMP_256 as libc::c_int as isize),
        Some(
            mapFilterOceanTemp
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(6 as libc::c_int as isize),
        filter,
        l.offset(L_BIOME_256 as libc::c_int as isize),
        Some(
            mapFilterBiome
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(7 as libc::c_int as isize),
        filter,
        l.offset(L_MUSHROOM_256 as libc::c_int as isize),
        Some(
            mapFilterMushroom
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    swapMap(
        fd.as_mut_ptr().offset(8 as libc::c_int as isize),
        filter,
        l.offset(L_SPECIAL_1024 as libc::c_int as isize),
        Some(
            mapFilterSpecial
                as unsafe extern "C" fn(
                    *const Layer,
                    *mut libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
    setLayerSeed(entry, seed);
    let mut err: libc::c_int = ((*entry).getMap)
        .expect(
            "non-null function pointer",
        )(entry, ids, x, z, w as libc::c_int, h as libc::c_int);
    let mut ret: libc::c_int = 0 as libc::c_int;
    if err == 0 as libc::c_int {
        let mut b: uint64_t = 0 as libc::c_int as uint64_t;
        let mut m: uint64_t = 0 as libc::c_int as uint64_t;
        let mut i_0: libc::c_uint = 0;
        i_0 = 0 as libc::c_int as libc::c_uint;
        while i_0 < w.wrapping_mul(h) {
            let mut id: libc::c_int = *ids.offset(i_0 as isize);
            if id < 128 as libc::c_int {
                b = (b as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
                    as uint64_t;
            } else {
                m = (m as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
            }
            i_0 = i_0.wrapping_add(1);
        }
        let mut match_exc: libc::c_int = ((*filter).biomeToExcl | (*filter).biomeToExclM
            == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        let mut match_any: libc::c_int = ((*filter).biomeToPick | (*filter).biomeToPickM
            == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        let mut match_req: libc::c_int = ((*filter).biomeToFind | (*filter).biomeToFindM
            == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
        match_exc
            |= !(b & (*filter).biomeToExcl != 0 || m & (*filter).biomeToExclM != 0)
                as libc::c_int;
        match_any
            |= (b & (*filter).biomeToPick != 0 || m & (*filter).biomeToPickM != 0)
                as libc::c_int;
        match_req
            |= (b & (*filter).biomeToFind == (*filter).biomeToFind
                && m & (*filter).biomeToFindM == (*filter).biomeToFindM) as libc::c_int;
        if match_exc != 0 && match_any != 0 && match_req != 0 {
            ret = 1 as libc::c_int;
        }
    } else if err == M_STOP as libc::c_int {
        ret = 0 as libc::c_int;
    } else if err == M_DONE as libc::c_int {
        ret = 2 as libc::c_int;
    }
    restoreMap(
        fd.as_mut_ptr().offset(8 as libc::c_int as isize),
        l.offset(L_SPECIAL_1024 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(7 as libc::c_int as isize),
        l.offset(L_MUSHROOM_256 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(6 as libc::c_int as isize),
        l.offset(L_BIOME_256 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(5 as libc::c_int as isize),
        l.offset(L_OCEAN_TEMP_256 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(4 as libc::c_int as isize),
        l.offset(L_BIOME_EDGE_64 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(3 as libc::c_int as isize),
        l.offset(L_SUNFLOWER_64 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(2 as libc::c_int as isize),
        l.offset(L_SHORE_16 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(1 as libc::c_int as isize),
        l.offset(L_RIVER_MIX_4 as libc::c_int as isize),
    );
    restoreMap(
        fd.as_mut_ptr().offset(0 as libc::c_int as isize),
        l.offset(L_OCEAN_MIX_4 as libc::c_int as isize),
    );
    if cache.is_null() {
        free(ids as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn checkForTemps(
    mut g: *mut LayerStack,
    mut seed: uint64_t,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut tc: *const libc::c_int,
) -> libc::c_int {
    let mut ls: uint64_t = getLayerSalt(3 as libc::c_int as uint64_t);
    let mut ss: uint64_t = getStartSeed(seed, ls);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut scnt: libc::c_int = 0 as libc::c_int;
    if *tc.offset((Special as libc::c_int + Warm as libc::c_int) as isize)
        > 0 as libc::c_int
    {
        scnt += *tc.offset((Special as libc::c_int + Warm as libc::c_int) as isize);
    }
    if *tc.offset((Special as libc::c_int + Lush as libc::c_int) as isize)
        > 0 as libc::c_int
    {
        scnt += *tc.offset((Special as libc::c_int + Lush as libc::c_int) as isize);
    }
    if *tc.offset((Special as libc::c_int + Cold as libc::c_int) as isize)
        > 0 as libc::c_int
    {
        scnt += *tc.offset((Special as libc::c_int + Cold as libc::c_int) as isize);
    }
    if scnt > 0 as libc::c_int {
        j = 0 as libc::c_int;
        while j < h {
            i = 0 as libc::c_int;
            while i < w {
                if mcFirstIsZero(getChunkSeed(ss, x + i, z + j), 13 as libc::c_int) != 0
                {
                    scnt -= 1;
                }
                i += 1;
            }
            j += 1;
        }
        if scnt > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    let mut l: *mut Layer = &mut *((*g).layers)
        .as_mut_ptr()
        .offset(L_SPECIAL_1024 as libc::c_int as isize) as *mut Layer;
    let mut ccnt: [libc::c_int; 9] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut area: *mut libc::c_int = calloc(
        getMinLayerCacheSize(l, w, h),
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut ret: libc::c_int = 1 as libc::c_int;
    setLayerSeed(l, seed);
    genArea(l, area, x, z, w, h);
    i = 0 as libc::c_int;
    while i < w * h {
        let mut id: libc::c_int = *area.offset(i as isize);
        let mut t: libc::c_int = id & 0xff as libc::c_int;
        if id != t && t != Freezing as libc::c_int {
            t += Special as libc::c_int;
        }
        ccnt[t as usize] += 1;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if ccnt[i as usize] < *tc.offset(i as isize)
            || ccnt[i as usize] != 0 && *tc.offset(i as isize) < 0 as libc::c_int
        {
            ret = 0 as libc::c_int;
            break;
        } else {
            i += 1;
        }
    }
    free(area as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn canBiomeGenerate(
    mut layerId: libc::c_int,
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> libc::c_int {
    let mut dofilter: libc::c_int = 0 as libc::c_int;
    if dofilter != 0 || layerId == L_BIOME_256 as libc::c_int {
        dofilter = 1 as libc::c_int;
        if id >= 64 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if dofilter != 0
        || layerId == L_BAMBOO_256 as libc::c_int && mc >= MC_1_14 as libc::c_int
    {
        dofilter = 1 as libc::c_int;
        match id {
            23 | 34 | 37 => return 0 as libc::c_int,
            _ => {}
        }
    }
    if dofilter != 0
        || layerId == L_BIOME_EDGE_64 as libc::c_int && mc >= MC_1_7 as libc::c_int
    {
        dofilter = 1 as libc::c_int;
        if id >= 64 as libc::c_int && id != bamboo_jungle as libc::c_int {
            return 0 as libc::c_int;
        }
        match id {
            13 | 17 | 18 | 19 | 22 | 28 | 31 | 33 | 36 => return 0 as libc::c_int,
            _ => {}
        }
    }
    if dofilter != 0 || layerId == L_HILLS_64 as libc::c_int {
        dofilter = 1 as libc::c_int;
    }
    if dofilter != 0
        || layerId == L_SUNFLOWER_64 as libc::c_int && mc >= MC_1_7 as libc::c_int
    {
        dofilter = 1 as libc::c_int;
        match id {
            10 | 15 | 16 | 25 | 26 => return 0 as libc::c_int,
            _ => {}
        }
    }
    if dofilter != 0 || layerId == L_SHORE_16 as libc::c_int {
        dofilter = 1 as libc::c_int;
        if id == river as libc::c_int || id == frozen_river as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if dofilter != 0 || layerId == L_RIVER_MIX_4 as libc::c_int {
        dofilter = 1 as libc::c_int;
        if id == frozen_ocean as libc::c_int && mc >= MC_1_7 as libc::c_int {
            return 0 as libc::c_int;
        }
        if isDeepOcean(id) != 0 && id != deep_ocean as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if dofilter != 0
        || layerId == L_OCEAN_MIX_4 as libc::c_int && mc >= MC_1_13 as libc::c_int
    {
        dofilter = 1 as libc::c_int;
    }
    if dofilter == 0 && layerId != L_VORONOI_1 as libc::c_int {
        printf(
            b"canBiomeGenerate(): unsupported layer (%d) or version (%d)\n\0"
                as *const u8 as *const libc::c_char,
            layerId,
            mc,
        );
        return 0 as libc::c_int;
    }
    return isOverworld(mc, id);
}
#[no_mangle]
pub unsafe extern "C" fn getAvailableBiomes(
    mut mL: *mut uint64_t,
    mut mM: *mut uint64_t,
    mut layerId: libc::c_int,
    mut mc: libc::c_int,
) {
    *mM = 0 as libc::c_int as uint64_t;
    *mL = *mM;
    let mut i: libc::c_int = 0;
    if mc >= MC_1_18 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if isOverworld(mc, i) != 0 {
                *mL = (*mL as libc::c_ulonglong | (1 as libc::c_ulonglong) << i)
                    as uint64_t;
            }
            if isOverworld(mc, i + 128 as libc::c_int) != 0 {
                *mM = (*mM as libc::c_ulonglong | (1 as libc::c_ulonglong) << i)
                    as uint64_t;
            }
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if canBiomeGenerate(layerId, mc, i) != 0 {
                *mL = (*mL as libc::c_ulonglong | (1 as libc::c_ulonglong) << i)
                    as uint64_t;
            }
            if canBiomeGenerate(layerId, mc, i + 128 as libc::c_int) != 0 {
                *mM = (*mM as libc::c_ulonglong | (1 as libc::c_ulonglong) << i)
                    as uint64_t;
            }
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn genPotential(
    mut mL: *mut uint64_t,
    mut mM: *mut uint64_t,
    mut layer: libc::c_int,
    mut mc: libc::c_int,
    mut id: libc::c_int,
) {
    let mut current_block: u64;
    if layer >= L_BIOME_256 as libc::c_int && canBiomeGenerate(layer, mc, id) == 0 {
        return;
    }
    match layer {
        12 => {
            if mc <= MC_1_6 as libc::c_int {
                current_block = 2900234161786722821;
            } else {
                if id == Oceanic as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_MUSHROOM_256 as libc::c_int,
                        mc,
                        mushroom_fields as libc::c_int,
                    );
                }
                if id & !(0xf00 as libc::c_int) >= Oceanic as libc::c_int
                    && id & !(0xf00 as libc::c_int) <= Freezing as libc::c_int
                {
                    genPotential(mL, mM, L_MUSHROOM_256 as libc::c_int, mc, id);
                }
                current_block = 2167674248514145403;
            }
        }
        17 => {
            if mc >= MC_1_7 as libc::c_int {
                if id == Oceanic as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_DEEP_OCEAN_256 as libc::c_int,
                        mc,
                        deep_ocean as libc::c_int,
                    );
                }
                if id == mushroom_fields as libc::c_int {
                    genPotential(mL, mM, L_DEEP_OCEAN_256 as libc::c_int, mc, id);
                }
                if id & !(0xf00 as libc::c_int) >= Oceanic as libc::c_int
                    && id & !(0xf00 as libc::c_int) <= Freezing as libc::c_int
                {
                    genPotential(mL, mM, L_DEEP_OCEAN_256 as libc::c_int, mc, id);
                }
            } else if id == ocean as libc::c_int || id == mushroom_fields as libc::c_int
                {
                genPotential(mL, mM, L_BIOME_256 as libc::c_int, mc, id);
            } else {
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    desert as libc::c_int,
                );
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    forest as libc::c_int,
                );
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    mountains as libc::c_int,
                );
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    swamp as libc::c_int,
                );
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    plains as libc::c_int,
                );
                genPotential(
                    mL,
                    mM,
                    L_BIOME_256 as libc::c_int,
                    mc,
                    taiga as libc::c_int,
                );
                if mc >= MC_1_2 as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_256 as libc::c_int,
                        mc,
                        jungle as libc::c_int,
                    );
                }
                if id != plains as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_256 as libc::c_int,
                        mc,
                        snowy_tundra as libc::c_int,
                    );
                }
            }
            current_block = 2167674248514145403;
        }
        18 => {
            if mc <= MC_1_6 as libc::c_int {
                current_block = 2900234161786722821;
            } else {
                match id & !(0xf00 as libc::c_int) {
                    1 => {
                        if id & 0xf00 as libc::c_int != 0 {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                badlands_plateau as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                wooded_badlands_plateau as libc::c_int,
                            );
                        } else {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                desert as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                savanna as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                plains as libc::c_int,
                            );
                        }
                    }
                    2 => {
                        if id & 0xf00 as libc::c_int != 0 {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                jungle as libc::c_int,
                            );
                        } else {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                forest as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                dark_forest as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                mountains as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                plains as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                birch_forest as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                swamp as libc::c_int,
                            );
                        }
                    }
                    3 => {
                        if id & 0xf00 as libc::c_int != 0 {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                giant_tree_taiga as libc::c_int,
                            );
                        } else {
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                forest as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                mountains as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                taiga as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_BIOME_256 as libc::c_int,
                                mc,
                                plains as libc::c_int,
                            );
                        }
                    }
                    4 => {
                        genPotential(
                            mL,
                            mM,
                            L_BIOME_256 as libc::c_int,
                            mc,
                            snowy_tundra as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_BIOME_256 as libc::c_int,
                            mc,
                            snowy_taiga as libc::c_int,
                        );
                    }
                    _ => {
                        id &= !(0xf00 as libc::c_int);
                        genPotential(mL, mM, L_BIOME_256 as libc::c_int, mc, id);
                    }
                }
                current_block = 2167674248514145403;
            }
        }
        19 | 20 => {
            if mc < MC_1_14 as libc::c_int && layer == L_BAMBOO_256 as libc::c_int {
                current_block = 2900234161786722821;
            } else if mc >= MC_1_7 as libc::c_int {
                if mc >= MC_1_14 as libc::c_int && id == jungle as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        bamboo_jungle as libc::c_int,
                    );
                }
                if id == wooded_badlands_plateau as libc::c_int
                    || id == badlands_plateau as libc::c_int
                {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        badlands as libc::c_int,
                    );
                } else if id == giant_tree_taiga as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        taiga as libc::c_int,
                    );
                } else if id == desert as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        wooded_mountains as libc::c_int,
                    );
                } else if id == swamp as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        jungle_edge as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_BIOME_EDGE_64 as libc::c_int,
                        mc,
                        plains as libc::c_int,
                    );
                }
                genPotential(mL, mM, L_BIOME_EDGE_64 as libc::c_int, mc, id);
                current_block = 2167674248514145403;
            } else {
                current_block = 18039362937860286515;
            }
        }
        23 => {
            current_block = 18039362937860286515;
        }
        27 => {
            if mc <= MC_1_6 as libc::c_int {
                if id == mushroom_fields as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        mushroom_field_shore as libc::c_int,
                    );
                } else if id == mountains as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        mountain_edge as libc::c_int,
                    );
                } else if id != ocean as libc::c_int && id != river as libc::c_int
                        && id != swamp as libc::c_int
                    {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        beach as libc::c_int,
                    );
                }
                genPotential(mL, mM, L_SHORE_16 as libc::c_int, mc, id);
            } else {
                if id == plains as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_SUNFLOWER_64 as libc::c_int,
                        mc,
                        sunflower_plains as libc::c_int,
                    );
                }
                genPotential(mL, mM, L_SUNFLOWER_64 as libc::c_int, mc, id);
            }
            current_block = 2167674248514145403;
        }
        28 => {
            if mc <= MC_1_6 as libc::c_int {
                current_block = 2900234161786722821;
            } else {
                if id == mushroom_fields as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        mushroom_field_shore as libc::c_int,
                    );
                } else if getCategory(mc, id) == jungle as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        beach as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        jungle_edge as libc::c_int,
                    );
                } else if id == mountains as libc::c_int
                        || id == wooded_mountains as libc::c_int
                        || id == mountain_edge as libc::c_int
                    {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        stone_shore as libc::c_int,
                    );
                } else if isSnowy(id) != 0 {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        snowy_beach as libc::c_int,
                    );
                } else if id == badlands as libc::c_int
                        || id == wooded_badlands_plateau as libc::c_int
                    {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        desert as libc::c_int,
                    );
                } else if id != ocean as libc::c_int && id != deep_ocean as libc::c_int
                        && id != river as libc::c_int && id != swamp as libc::c_int
                    {
                    genPotential(
                        mL,
                        mM,
                        L_SHORE_16 as libc::c_int,
                        mc,
                        beach as libc::c_int,
                    );
                }
                genPotential(mL, mM, L_SHORE_16 as libc::c_int, mc, id);
                current_block = 2167674248514145403;
            }
        }
        32 => {
            if id == snowy_tundra as libc::c_int {
                genPotential(
                    mL,
                    mM,
                    L_RIVER_MIX_4 as libc::c_int,
                    mc,
                    frozen_river as libc::c_int,
                );
            } else if id == mushroom_fields as libc::c_int
                    || id == mushroom_field_shore as libc::c_int
                {
                genPotential(
                    mL,
                    mM,
                    L_RIVER_MIX_4 as libc::c_int,
                    mc,
                    mushroom_field_shore as libc::c_int,
                );
            } else if id != ocean as libc::c_int
                    && (mc < MC_1_7 as libc::c_int || isOceanic(id) == 0)
                {
                genPotential(
                    mL,
                    mM,
                    L_RIVER_MIX_4 as libc::c_int,
                    mc,
                    river as libc::c_int,
                );
            }
            genPotential(mL, mM, L_RIVER_MIX_4 as libc::c_int, mc, id);
            current_block = 2167674248514145403;
        }
        45 => {
            if mc >= MC_1_13 as libc::c_int && isOceanic(id) != 0 {
                if id == ocean as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        warm_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        lukewarm_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        cold_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        frozen_ocean as libc::c_int,
                    );
                    current_block = 2408932541243239002;
                } else if id == deep_ocean as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        deep_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        deep_lukewarm_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        deep_cold_ocean as libc::c_int,
                    );
                    genPotential(
                        mL,
                        mM,
                        L_VORONOI_1 as libc::c_int,
                        mc,
                        deep_frozen_ocean as libc::c_int,
                    );
                    current_block = 2408932541243239002;
                } else {
                    current_block = 2167674248514145403;
                }
            } else {
                current_block = 2408932541243239002;
            }
            match current_block {
                2167674248514145403 => {}
                _ => {
                    genPotential(mL, mM, L_VORONOI_1 as libc::c_int, mc, id);
                    current_block = 2167674248514145403;
                }
            }
        }
        53 => {
            if mc <= MC_1_12 as libc::c_int {
                current_block = 2900234161786722821;
            } else {
                current_block = 1916527840833100240;
            }
        }
        54 => {
            current_block = 1916527840833100240;
        }
        _ => {
            printf(
                b"genPotential() not implemented for layer %d\n\0" as *const u8
                    as *const libc::c_char,
                layer,
            );
            current_block = 2167674248514145403;
        }
    }
    match current_block {
        18039362937860286515 => {
            if mc <= MC_1_6 as libc::c_int && layer == L_BIOME_EDGE_64 as libc::c_int {
                current_block = 2900234161786722821;
            } else {
                if isShallowOcean(id) == 0 && getMutated(mc, id) > 0 as libc::c_int {
                    genPotential(
                        mL,
                        mM,
                        L_HILLS_64 as libc::c_int,
                        mc,
                        getMutated(mc, id),
                    );
                }
                match id {
                    2 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            desert_hills as libc::c_int,
                        );
                    }
                    4 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            wooded_hills as libc::c_int,
                        );
                    }
                    27 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            birch_forest_hills as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            getMutated(mc, birch_forest_hills as libc::c_int),
                        );
                    }
                    29 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            plains as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            getMutated(mc, plains as libc::c_int),
                        );
                    }
                    5 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            taiga_hills as libc::c_int,
                        );
                    }
                    32 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            giant_tree_taiga_hills as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            getMutated(mc, giant_tree_taiga_hills as libc::c_int),
                        );
                    }
                    30 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            snowy_taiga_hills as libc::c_int,
                        );
                    }
                    1 => {
                        if mc >= MC_1_7 as libc::c_int {
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                wooded_hills as libc::c_int,
                            );
                        }
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            forest as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            getMutated(mc, forest as libc::c_int),
                        );
                    }
                    12 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            snowy_mountains as libc::c_int,
                        );
                    }
                    21 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            jungle_hills as libc::c_int,
                        );
                    }
                    168 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            bamboo_jungle_hills as libc::c_int,
                        );
                    }
                    0 => {
                        if mc >= MC_1_7 as libc::c_int {
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                deep_ocean as libc::c_int,
                            );
                        }
                    }
                    3 => {
                        if mc >= MC_1_7 as libc::c_int {
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                wooded_mountains as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                getMutated(mc, wooded_mountains as libc::c_int),
                            );
                        }
                    }
                    35 => {
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            savanna_plateau as libc::c_int,
                        );
                        genPotential(
                            mL,
                            mM,
                            L_HILLS_64 as libc::c_int,
                            mc,
                            getMutated(mc, savanna_plateau as libc::c_int),
                        );
                    }
                    _ => {
                        if areSimilar(mc, id, wooded_badlands_plateau as libc::c_int)
                            != 0
                        {
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                badlands as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                getMutated(mc, badlands as libc::c_int),
                            );
                        } else if isDeepOcean(id) != 0 {
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                plains as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                forest as libc::c_int,
                            );
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                getMutated(mc, plains as libc::c_int),
                            );
                            genPotential(
                                mL,
                                mM,
                                L_HILLS_64 as libc::c_int,
                                mc,
                                getMutated(mc, forest as libc::c_int),
                            );
                        }
                    }
                }
                genPotential(mL, mM, L_HILLS_64 as libc::c_int, mc, id);
                current_block = 2167674248514145403;
            }
        }
        1916527840833100240 => {
            if id < 128 as libc::c_int {
                *mL = (*mL as libc::c_ulonglong | (1 as libc::c_ulonglong) << id)
                    as uint64_t;
            } else {
                *mM = (*mM as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << id - 128 as libc::c_int) as uint64_t;
            }
            current_block = 2167674248514145403;
        }
        _ => {}
    }
    match current_block {
        2900234161786722821 => {
            printf(
                b"genPotential() bad layer %d for version\n\0" as *const u8
                    as *const libc::c_char,
                layer,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn getParaDescent(
    mut para: *const DoublePerlinNoise,
    mut factor: libc::c_double,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut i0: libc::c_int,
    mut j0: libc::c_int,
    mut maxrad: libc::c_int,
    mut maxiter: libc::c_int,
    mut alpha: libc::c_double,
    mut data: *mut libc::c_void,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_double,
        ) -> libc::c_int,
    >,
) -> libc::c_double {
    let mut dirx: libc::c_int = 0 as libc::c_int;
    let mut dirz: libc::c_int = 0 as libc::c_int;
    let mut dira: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: libc::c_double = 0.;
    let mut vd: libc::c_double = 0.;
    let mut va: libc::c_double = 0.;
    v = factor
        * sampleDoublePerlin(
            para,
            (x + i0) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            (z + j0) as libc::c_double,
        );
    if func.is_some() {
        if func
            .expect(
                "non-null function pointer",
            )(
            data,
            x + i0,
            z + j0,
            if factor < 0 as libc::c_int as libc::c_double { -v } else { v },
        ) != 0
        {
            return nan(b"\0" as *const u8 as *const libc::c_char);
        }
    }
    i = i0;
    j = j0;
    k = 0 as libc::c_int;
    while k < maxiter {
        if dirx == 0 as libc::c_int {
            dirx = 1 as libc::c_int;
        }
        if i + dirx >= 0 as libc::c_int && i + dirx < w {
            vd = factor
                * sampleDoublePerlin(
                    para,
                    (x + i + dirx) as libc::c_double,
                    0 as libc::c_int as libc::c_double,
                    (z + j) as libc::c_double,
                );
        } else {
            vd = v;
        }
        if vd >= v {
            dirx *= -(1 as libc::c_int);
            if i + dirx >= 0 as libc::c_int && i + dirx < w {
                vd = factor
                    * sampleDoublePerlin(
                        para,
                        (x + i + dirx) as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        (z + j) as libc::c_double,
                    );
            } else {
                vd = v;
            }
            if vd >= v {
                dirx = 0 as libc::c_int;
            }
        }
        if dirx != 0 {
            let mut current_block_30: u64;
            dira = (dirx as libc::c_double * alpha * (v - vd)) as libc::c_int;
            if abs(dira) > 2 as libc::c_int && i + dira >= 0 as libc::c_int
                && i + dira < w
            {
                va = factor
                    * sampleDoublePerlin(
                        para,
                        (x + i + dira) as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        (z + j) as libc::c_double,
                    );
                if va < vd {
                    i += dira;
                    v = va;
                    current_block_30 = 3610016944583890659;
                } else {
                    current_block_30 = 1608152415753874203;
                }
            } else {
                current_block_30 = 1608152415753874203;
            }
            match current_block_30 {
                1608152415753874203 => {
                    v = vd;
                    i += dirx;
                }
                _ => {}
            }
            if func.is_some() {
                if func
                    .expect(
                        "non-null function pointer",
                    )(
                    data,
                    x + i,
                    z + j,
                    if factor < 0 as libc::c_int as libc::c_double { -v } else { v },
                ) != 0
                {
                    return nan(b"\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        if dirz == 0 as libc::c_int {
            dirz = 1 as libc::c_int;
        }
        if j + dirz >= 0 as libc::c_int && j + dirz < h {
            vd = factor
                * sampleDoublePerlin(
                    para,
                    (x + i) as libc::c_double,
                    0 as libc::c_int as libc::c_double,
                    (z + j + dirz) as libc::c_double,
                );
        } else {
            vd = v;
        }
        if vd >= v {
            dirz *= -(1 as libc::c_int);
            if j + dirz >= 0 as libc::c_int && j + dirz < h {
                vd = factor
                    * sampleDoublePerlin(
                        para,
                        (x + i) as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        (z + j + dirz) as libc::c_double,
                    );
            } else {
                vd = v;
            }
            if vd >= v {
                dirz = 0 as libc::c_int;
            }
        }
        if dirz != 0 {
            let mut current_block_55: u64;
            dira = (dirz as libc::c_double * alpha * (v - vd)) as libc::c_int;
            if abs(dira) > 2 as libc::c_int && j + dira >= 0 as libc::c_int
                && j + dira < h
            {
                va = factor
                    * sampleDoublePerlin(
                        para,
                        (x + i) as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        (z + j + dira) as libc::c_double,
                    );
                if va < vd {
                    j += dira;
                    v = va;
                    current_block_55 = 1217889855984862520;
                } else {
                    current_block_55 = 13763002826403452995;
                }
            } else {
                current_block_55 = 13763002826403452995;
            }
            match current_block_55 {
                13763002826403452995 => {
                    j += dirz;
                    v = vd;
                }
                _ => {}
            }
            if func.is_some() {
                if func
                    .expect(
                        "non-null function pointer",
                    )(
                    data,
                    x + i,
                    z + j,
                    if factor < 0 as libc::c_int as libc::c_double { -v } else { v },
                ) != 0
                {
                    return nan(b"\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        if dirx == 0 as libc::c_int && dirz == 0 as libc::c_int {
            let mut c: libc::c_int = 0;
            c = 0 as libc::c_int;
            while c < 4 as libc::c_int {
                dirx = if c & 1 as libc::c_int != 0 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
                dirz = if c & 2 as libc::c_int != 0 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
                if !(i + dirx < 0 as libc::c_int || i + dirx >= w
                    || j + dirz < 0 as libc::c_int || j + dirz >= h)
                {
                    vd = factor
                        * sampleDoublePerlin(
                            para,
                            (x + i + dirx) as libc::c_double,
                            0 as libc::c_int as libc::c_double,
                            (z + j + dirz) as libc::c_double,
                        );
                    if vd < v {
                        v = vd;
                        i += dirx;
                        j += dirz;
                        break;
                    }
                }
                c += 1;
            }
            if c >= 4 as libc::c_int {
                break;
            }
        }
        if abs(i - i0) > maxrad || abs(j - j0) > maxrad {
            break;
        }
        k += 1;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn getParaRange(
    mut para: *const DoublePerlinNoise,
    mut pmin: *mut libc::c_double,
    mut pmax: *mut libc::c_double,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut data: *mut libc::c_void,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_double,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut current_block: u64;
    let beta: libc::c_double = 1.5f64;
    let factor: libc::c_double = 10000 as libc::c_int as libc::c_double;
    let perlin_grad: libc::c_double = 2.0f64 * 1.875f64;
    let mut v: libc::c_double = 0.;
    let mut lmin: libc::c_double = 0.;
    let mut lmax: libc::c_double = 0.;
    let mut dr: libc::c_double = 0.;
    let mut vdif: libc::c_double = 0.;
    let mut small_regime: libc::c_double = 0.;
    let mut skip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    let mut ww: libc::c_int = 0;
    let mut hh: libc::c_int = 0;
    let mut skipsiz: libc::c_int = 0;
    let mut maxrad: libc::c_int = 0;
    let mut maxiter: libc::c_int = 0;
    let mut err: libc::c_int = 1 as libc::c_int;
    *pmin = 1.7976931348623157e+308f64;
    *pmax = -1.7976931348623157e+308f64;
    lmin = 1.7976931348623157e+308f64;
    lmax = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*para).octA.octcnt {
        let mut lac: libc::c_double = (*((*para).octA.octaves).offset(i as isize))
            .lacunarity;
        if lac < lmin {
            lmin = lac;
        }
        if lac > lmax {
            lmax = lac;
        }
        i += 1;
    }
    small_regime = 1e3f64 * sqrt(lmax);
    if ((w * h) as libc::c_double) < small_regime {
        j = 0 as libc::c_int;
        while j < h {
            i = 0 as libc::c_int;
            while i < w {
                v = factor
                    * sampleDoublePerlin(
                        para,
                        (x + i) as libc::c_double,
                        0 as libc::c_int as libc::c_double,
                        (z + j) as libc::c_double,
                    );
                if func.is_some() {
                    err = func
                        .expect("non-null function pointer")(data, x + i, z + j, v);
                    if err != 0 {
                        return err;
                    }
                }
                if v < *pmin {
                    *pmin = v;
                }
                if v > *pmax {
                    *pmax = v;
                }
                i += 1;
            }
            j += 1;
        }
        return 0 as libc::c_int;
    }
    step = (0.5f64 / lmin - 1.19209290e-7f32 as libc::c_double) as libc::c_int
        + 1 as libc::c_int;
    dr = lmax / lmin * beta;
    j = 0 as libc::c_int;
    's_176: loop {
        if !(j < h) {
            current_block = 10150597327160359210;
            break;
        }
        i = 0 as libc::c_int;
        while i < w {
            v = getParaDescent(
                para,
                factor,
                x,
                z,
                w,
                h,
                i,
                j,
                step,
                step,
                dr,
                data,
                func,
            );
            if v != v {
                current_block = 6442418240192079606;
                break 's_176;
            }
            if v < *pmin {
                *pmin = v;
            }
            v = -getParaDescent(
                para,
                -factor,
                x,
                z,
                w,
                h,
                i,
                j,
                step,
                step,
                dr,
                data,
                func,
            );
            if v != v {
                current_block = 6442418240192079606;
                break 's_176;
            }
            if v > *pmax {
                *pmax = v;
            }
            i += step;
        }
        j += step;
    }
    match current_block {
        10150597327160359210 => {
            if lmin == lmax {
                return 0 as libc::c_int;
            }
            step = (1.0f64 / (perlin_grad * lmax + 1.19209290e-7f32 as libc::c_double))
                as libc::c_int + 1 as libc::c_int;
            vdif = 0 as libc::c_int as libc::c_double;
            i = 0 as libc::c_int;
            while i < (*para).octA.octcnt {
                let mut p: *const PerlinNoise = ((*para).octA.octaves)
                    .offset(i as isize);
                let mut contrib: libc::c_double = step as libc::c_double
                    * (*p).lacunarity * 1.0f64;
                if contrib > 1.0f64 {
                    contrib = 1 as libc::c_int as libc::c_double;
                }
                vdif += contrib * (*p).amplitude;
                i += 1;
            }
            i = 0 as libc::c_int;
            while i < (*para).octB.octcnt {
                let lac_factB: libc::c_double = 337.0f64 / 331.0f64;
                let mut p_0: *const PerlinNoise = ((*para).octB.octaves)
                    .offset(i as isize);
                let mut contrib_0: libc::c_double = step as libc::c_double
                    * (*p_0).lacunarity * lac_factB;
                if contrib_0 > 1.0f64 {
                    contrib_0 = 1 as libc::c_int as libc::c_double;
                }
                vdif += contrib_0 * (*p_0).amplitude;
                i += 1;
            }
            vdif = fabs(factor * vdif * (*para).amplitude);
            maxrad = step;
            maxiter = step * 2 as libc::c_int;
            ww = (w + step - 1 as libc::c_int) / step;
            hh = (h + step - 1 as libc::c_int) / step;
            skipsiz = (((ww + 1 as libc::c_int) * (hh + 1 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int;
            skip = malloc(skipsiz as libc::c_ulong) as *mut libc::c_char;
            memset(
                skip as *mut libc::c_void,
                0 as libc::c_int,
                skipsiz as libc::c_ulong,
            );
            jj = 0 as libc::c_int;
            's_363: loop {
                if !(jj <= hh) {
                    current_block = 11064061988481400464;
                    break;
                }
                j = jj * step;
                if j >= h {
                    j = h - 1 as libc::c_int;
                }
                ii = 0 as libc::c_int;
                while ii <= ww {
                    i = ii * step;
                    if i >= w {
                        i = w - 1 as libc::c_int;
                    }
                    if !(*skip.offset((jj * ww + ii) as isize) != 0) {
                        v = factor
                            * sampleDoublePerlin(
                                para,
                                (x + i) as libc::c_double,
                                0 as libc::c_int as libc::c_double,
                                (z + j) as libc::c_double,
                            );
                        if func.is_some() {
                            let mut e: libc::c_int = func
                                .expect("non-null function pointer")(data, x + i, z + j, v);
                            if e != 0 {
                                err = e;
                                current_block = 6442418240192079606;
                                break 's_363;
                            }
                        }
                        if v > *pmax {
                            *pmax = v;
                        }
                        dr = beta * (v - *pmin) / vdif;
                        if dr > 1.0f64 {
                            let mut a: libc::c_int = 0;
                            let mut b: libc::c_int = 0;
                            let mut r: libc::c_int = dr as libc::c_int;
                            b = 0 as libc::c_int;
                            while b < r {
                                if !(b + jj < 0 as libc::c_int || b + jj >= hh) {
                                    a = -r + 1 as libc::c_int;
                                    while a < r {
                                        if !(a + ii < 0 as libc::c_int || a + ii >= ww) {
                                            *skip
                                                .offset(
                                                    ((b + jj) * ww + (a + ii)) as isize,
                                                ) = 1 as libc::c_int as libc::c_char;
                                        }
                                        a += 1;
                                    }
                                }
                                b += 1;
                            }
                        } else {
                            v = getParaDescent(
                                para,
                                factor,
                                x,
                                z,
                                w,
                                h,
                                i,
                                j,
                                maxrad,
                                maxiter,
                                dr,
                                data,
                                func,
                            );
                            if v != v {
                                current_block = 6442418240192079606;
                                break 's_363;
                            }
                            if v < *pmin {
                                *pmin = v;
                            }
                        }
                    }
                    ii += 1;
                }
                jj += 1;
            }
            match current_block {
                6442418240192079606 => {}
                _ => {
                    memset(
                        skip as *mut libc::c_void,
                        0 as libc::c_int,
                        skipsiz as libc::c_ulong,
                    );
                    jj = 0 as libc::c_int;
                    's_545: loop {
                        if !(jj <= hh) {
                            current_block = 16974974966130203269;
                            break;
                        }
                        j = jj * step;
                        if j >= h {
                            j = h - 1 as libc::c_int;
                        }
                        ii = 0 as libc::c_int;
                        while ii <= ww {
                            i = ii * step;
                            if i >= w {
                                i = w - 1 as libc::c_int;
                            }
                            if !(*skip.offset((jj * ww + ii) as isize) != 0) {
                                v = -factor
                                    * sampleDoublePerlin(
                                        para,
                                        (x + i) as libc::c_double,
                                        0 as libc::c_int as libc::c_double,
                                        (z + j) as libc::c_double,
                                    );
                                if func.is_some() {
                                    let mut e_0: libc::c_int = func
                                        .expect(
                                            "non-null function pointer",
                                        )(data, x + i, z + j, -v);
                                    if e_0 != 0 {
                                        err = e_0;
                                        current_block = 6442418240192079606;
                                        break 's_545;
                                    }
                                }
                                dr = beta * (v + *pmax) / vdif;
                                if dr > 1.0f64 {
                                    let mut a_0: libc::c_int = 0;
                                    let mut b_0: libc::c_int = 0;
                                    let mut r_0: libc::c_int = dr as libc::c_int;
                                    b_0 = 0 as libc::c_int;
                                    while b_0 < r_0 {
                                        if !(b_0 + jj < 0 as libc::c_int || b_0 + jj >= hh) {
                                            a_0 = -r_0 + 1 as libc::c_int;
                                            while a_0 < r_0 {
                                                if !(a_0 + ii < 0 as libc::c_int || a_0 + ii >= ww) {
                                                    *skip
                                                        .offset(
                                                            ((b_0 + jj) * ww + (a_0 + ii)) as isize,
                                                        ) = 1 as libc::c_int as libc::c_char;
                                                }
                                                a_0 += 1;
                                            }
                                        }
                                        b_0 += 1;
                                    }
                                } else {
                                    v = -getParaDescent(
                                        para,
                                        -factor,
                                        x,
                                        z,
                                        w,
                                        h,
                                        i,
                                        j,
                                        maxrad,
                                        maxiter,
                                        dr,
                                        data,
                                        func,
                                    );
                                    if v != v {
                                        current_block = 6442418240192079606;
                                        break 's_545;
                                    }
                                    if v > *pmax {
                                        *pmax = v;
                                    }
                                }
                            }
                            ii += 1;
                        }
                        jj += 1;
                    }
                    match current_block {
                        6442418240192079606 => {}
                        _ => {
                            err = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !skip.is_null() {
        free(skip as *mut libc::c_void);
    }
    return err;
}
static mut g_biome_para_range_18: [[libc::c_int; 13]; 50] = [
    [
        ocean as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(4550 as libc::c_int),
        -(1900 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        plains as libc::c_int,
        -(4500 as libc::c_int),
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        1000 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        desert as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        windswept_hills as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        1000 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        4500 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        forest as libc::c_int,
        -(4500 as libc::c_int),
        5500 as libc::c_int,
        -(1000 as libc::c_int),
        3000 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        taiga as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1500 as libc::c_int),
        1000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        swamp as libc::c_int,
        -(4500 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1100 as libc::c_int),
        2147483647 as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        river as libc::c_int,
        -(4500 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        500 as libc::c_int,
    ],
    [
        frozen_ocean as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4501 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(4550 as libc::c_int),
        -(1900 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        frozen_river as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4501 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        500 as libc::c_int,
    ],
    [
        snowy_plains as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4500 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        1000 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        mushroom_fields as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(10500 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        beach as libc::c_int,
        -(4500 as libc::c_int),
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        -(1100 as libc::c_int),
        -(2225 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2666 as libc::c_int,
    ],
    [
        jungle as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        1000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        sparse_jungle as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        1000 as libc::c_int,
        3000 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        deep_ocean as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(10500 as libc::c_int),
        -(4551 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        stony_shore as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        -(1100 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(2225 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        snowy_beach as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4500 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        -(1100 as libc::c_int),
        -(2225 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2666 as libc::c_int,
    ],
    [
        birch_forest as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        1000 as libc::c_int,
        3000 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        dark_forest as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        3000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        snowy_taiga as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4500 as libc::c_int),
        -(1000 as libc::c_int),
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        old_growth_pine_taiga as libc::c_int,
        -(4500 as libc::c_int),
        -(1500 as libc::c_int),
        3000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        windswept_forest as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        1000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        4500 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        savanna as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        savanna_plateau as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1100 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        badlands as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        1000 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        wooded_badlands as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        1000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        warm_ocean as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(10500 as libc::c_int),
        -(1900 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        lukewarm_ocean as libc::c_int,
        2001 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(4550 as libc::c_int),
        -(1900 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        cold_ocean as libc::c_int,
        -(4500 as libc::c_int),
        -(1501 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(4550 as libc::c_int),
        -(1900 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        deep_lukewarm_ocean as libc::c_int,
        2001 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(10500 as libc::c_int),
        -(4551 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        deep_cold_ocean as libc::c_int,
        -(4500 as libc::c_int),
        -(1501 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(10500 as libc::c_int),
        -(4551 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        deep_frozen_ocean as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4501 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(10500 as libc::c_int),
        -(4551 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        sunflower_plains as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3500 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        windswept_gravelly_hills as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1500 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        4500 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        flower_forest as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3500 as libc::c_int),
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(500 as libc::c_int),
    ],
    [
        ice_spikes as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(4500 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3500 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        old_growth_birch_forest as libc::c_int,
        -(1500 as libc::c_int),
        2000 as libc::c_int,
        1000 as libc::c_int,
        3000 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        old_growth_spruce_taiga as libc::c_int,
        -(4500 as libc::c_int),
        -(1500 as libc::c_int),
        3000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1900 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(500 as libc::c_int),
    ],
    [
        windswept_savanna as libc::c_int,
        -(1500 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        3000 as libc::c_int,
        -(1899 as libc::c_int),
        300 as libc::c_int,
        4500 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        501 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        eroded_badlands as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        bamboo_jungle as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        3000 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        dripstone_caves as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        6999 as libc::c_int,
        3001 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        1000 as libc::c_int,
        9500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        lush_caves as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        2001 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        1000 as libc::c_int,
        9500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        meadow as libc::c_int,
        -(4500 as libc::c_int),
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        3000 as libc::c_int,
        300 as libc::c_int,
        2147483647 as libc::c_int,
        -(7799 as libc::c_int),
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        grove as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(1000 as libc::c_int),
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        snowy_slopes as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        jagged_peaks as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(9333 as libc::c_int),
        -(4001 as libc::c_int),
    ],
    [
        frozen_peaks as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        4000 as libc::c_int,
        9333 as libc::c_int,
    ],
    [
        stony_peaks as libc::c_int,
        2000 as libc::c_int,
        5500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(9333 as libc::c_int),
        9333 as libc::c_int,
    ],
];
static mut g_biome_para_range_19_diff: [[libc::c_int; 13]; 6] = [
    [
        eroded_badlands as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        500 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(500 as libc::c_int),
        2147483647 as libc::c_int,
    ],
    [
        grove as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(1000 as libc::c_int),
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        10499 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        snowy_slopes as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(1000 as libc::c_int),
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        10499 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        jagged_peaks as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2000 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1899 as libc::c_int),
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        -(3750 as libc::c_int),
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        10499 as libc::c_int,
        -(9333 as libc::c_int),
        -(4001 as libc::c_int),
    ],
    [
        deep_dark as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        1818 as libc::c_int,
        10500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
    [
        mangrove_swamp as libc::c_int,
        2000 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(1100 as libc::c_int),
        2147483647 as libc::c_int,
        5500 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
        -(2147483647 as libc::c_int) - 1 as libc::c_int,
        2147483647 as libc::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn getBiomeParaLimits(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> *const libc::c_int {
    if mc < MC_1_18 as libc::c_int {
        return 0 as *const libc::c_int;
    }
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if mc >= MC_1_19 as libc::c_int {
        n = (::std::mem::size_of::<[[libc::c_int; 13]; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<[libc::c_int; 13]>() as libc::c_ulong)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < n {
            if g_biome_para_range_19_diff[i as usize][0 as libc::c_int as usize] == id {
                return &*(*g_biome_para_range_19_diff.as_ptr().offset(i as isize))
                    .as_ptr()
                    .offset(1 as libc::c_int as isize) as *const libc::c_int;
            }
            i += 1;
        }
    }
    n = (::std::mem::size_of::<[[libc::c_int; 13]; 50]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[libc::c_int; 13]>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if g_biome_para_range_18[i as usize][0 as libc::c_int as usize] == id {
            return &*(*g_biome_para_range_18.as_ptr().offset(i as isize))
                .as_ptr()
                .offset(1 as libc::c_int as isize) as *const libc::c_int;
        }
        i += 1;
    }
    return 0 as *const libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPossibleBiomesForLimits(
    mut ids: *mut libc::c_char,
    mut mc: libc::c_int,
    mut limits: *mut [libc::c_int; 2],
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    memset(
        ids as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(isOverworld(mc, i) == 0) {
            let mut bp: *const libc::c_int = getBiomeParaLimits(mc, i);
            if !bp.is_null() {
                j = 0 as libc::c_int;
                while j < 6 as libc::c_int {
                    if (*limits.offset(j as isize))[0 as libc::c_int as usize]
                        > *bp.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize)
                        || (*limits.offset(j as isize))[1 as libc::c_int as usize]
                            < *bp
                                .offset((2 as libc::c_int * j + 0 as libc::c_int) as isize)
                    {
                        break;
                    }
                    j += 1;
                }
                if j >= 6 as libc::c_int {
                    *ids
                        .offset(
                            *bp.offset(-(1 as libc::c_int) as isize) as isize,
                        ) = 1 as libc::c_int as libc::c_char;
                }
            }
        }
        i += 1;
    }
}
