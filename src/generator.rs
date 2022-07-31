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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getVoronoiSHA(worldSeed: uint64_t) -> uint64_t;
    fn mapVoronoi114(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapVoronoi(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapOceanMix(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapOceanTemp(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapRiverMix(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSwampRiver(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapShore(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSunflower(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSmooth(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapRiver(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapHills(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapBiomeEdge(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapNoise(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapBamboo(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapBiome(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapDeepOcean(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn setLayerSeed(layer: *mut Layer, worldSeed: uint64_t);
    fn setNetherSeed(nn: *mut NetherNoise, seed: uint64_t);
    fn genNetherScaled(
        nn: *const NetherNoise,
        out: *mut libc::c_int,
        r: Range,
        mc: libc::c_int,
        sha: uint64_t,
    ) -> libc::c_int;
    fn setEndSeed(en: *mut EndNoise, seed: uint64_t);
    fn genEndScaled(
        en: *const EndNoise,
        out: *mut libc::c_int,
        r: Range,
        mc: libc::c_int,
        sha: uint64_t,
    ) -> libc::c_int;
    fn initBiomeNoise(bn: *mut BiomeNoise, mc: libc::c_int);
    fn setBiomeSeed(bn: *mut BiomeNoise, seed: uint64_t, large: libc::c_int);
    fn genBiomeNoiseScaled(
        bn: *const BiomeNoise,
        out: *mut libc::c_int,
        r: Range,
        mc: libc::c_int,
        sha: uint64_t,
    ) -> libc::c_int;
    fn isOceanic(id: libc::c_int) -> libc::c_int;
    fn mapContinent(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapZoomFuzzy(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapZoom(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapLand(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapLand16(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapIsland(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSnow(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSnow16(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapCool(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapHeat(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapSpecial(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn mapMushroom(
        _: *const Layer,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
    pub getMap: Option<mapfunc_t>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Generator {
    pub mc: libc::c_int,
    pub dim: libc::c_int,
    pub flags: uint32_t,
    pub seed: uint64_t,
    pub sha: uint64_t,
    pub c2rust_unnamed: C2RustUnnamed,
    pub nn: NetherNoise,
    pub en: EndNoise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub bn: BiomeNoise,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ls: LayerStack,
    pub xlayer: [Layer; 5],
    pub entry: *mut Layer,
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
unsafe extern "C" fn getLayerSalt(mut salt: uint64_t) -> uint64_t {
    let mut ls: uint64_t = mcStepSeed(salt, salt);
    ls = mcStepSeed(ls, salt);
    ls = mcStepSeed(ls, salt);
    return ls;
}
#[no_mangle]
pub unsafe extern "C" fn mapOceanMixMod(
    mut l: *const Layer,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> libc::c_int {
    let mut otyp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ((*(*l).p2).getMap).expect("non-null function pointer")((*l).p2, out, x, z, w, h);
    otyp = malloc(
        ((w * h) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memcpy(
        otyp as *mut libc::c_void,
        out as *const libc::c_void,
        ((w * h) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    ((*(*l).p).getMap).expect("non-null function pointer")((*l).p, out, x, z, w, h);
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w {
            let mut landID: libc::c_int = 0;
            let mut oceanID: libc::c_int = 0;
            landID = *out.offset((i + j * w) as isize);
            if !(isOceanic(landID) == 0) {
                oceanID = *otyp.offset((i + j * w) as isize);
                if landID == deep_ocean as libc::c_int {
                    match oceanID {
                        45 => {
                            oceanID = deep_lukewarm_ocean as libc::c_int;
                        }
                        0 => {
                            oceanID = deep_ocean as libc::c_int;
                        }
                        46 => {
                            oceanID = deep_cold_ocean as libc::c_int;
                        }
                        10 => {
                            oceanID = deep_frozen_ocean as libc::c_int;
                        }
                        _ => {}
                    }
                }
                *out.offset((i + j * w) as isize) = oceanID;
            }
            i += 1;
        }
        j += 1;
    }
    free(otyp as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setupGenerator(
    mut g: *mut Generator,
    mut mc: libc::c_int,
    mut flags: uint32_t,
) {
    (*g).mc = mc;
    (*g).dim = 1000 as libc::c_int;
    (*g).flags = flags;
    (*g).seed = 0 as libc::c_int as uint64_t;
    (*g).sha = 0 as libc::c_int as uint64_t;
    if mc <= MC_1_17 as libc::c_int {
        setupLayerStack(
            &mut (*g).c2rust_unnamed.c2rust_unnamed.ls,
            mc,
            (flags & 0x1 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        let ref mut fresh0 = (*g).c2rust_unnamed.c2rust_unnamed.entry;
        *fresh0 = 0 as *mut Layer;
        if flags & 0x4 as libc::c_int as libc::c_uint != 0 && mc >= MC_1_13 as libc::c_int {
            let ref mut fresh1 = (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_16;
            *fresh1 = setupLayer(
                ((*g).c2rust_unnamed.c2rust_unnamed.xlayer)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
                Some(
                    mapOceanMixMod
                        as unsafe extern "C" fn(
                            *const Layer,
                            *mut libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                mc,
                1 as libc::c_int as int8_t,
                0 as libc::c_int as int8_t,
                0 as libc::c_int as uint64_t,
                (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_16,
                &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                    .as_mut_ptr()
                    .offset(L_ZOOM_16_OCEAN as libc::c_int as isize),
            );
            let ref mut fresh2 = (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_64;
            *fresh2 = setupLayer(
                ((*g).c2rust_unnamed.c2rust_unnamed.xlayer)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as isize),
                Some(
                    mapOceanMixMod
                        as unsafe extern "C" fn(
                            *const Layer,
                            *mut libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                mc,
                1 as libc::c_int as int8_t,
                0 as libc::c_int as int8_t,
                0 as libc::c_int as uint64_t,
                (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_64,
                &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                    .as_mut_ptr()
                    .offset(L_ZOOM_64_OCEAN as libc::c_int as isize),
            );
            let ref mut fresh3 = (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_256;
            *fresh3 = setupLayer(
                ((*g).c2rust_unnamed.c2rust_unnamed.xlayer)
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize),
                Some(
                    mapOceanMixMod
                        as unsafe extern "C" fn(
                            *const Layer,
                            *mut libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                            libc::c_int,
                        ) -> libc::c_int,
                ),
                mc,
                1 as libc::c_int as int8_t,
                0 as libc::c_int as int8_t,
                0 as libc::c_int as uint64_t,
                (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_256,
                &mut *((*g).c2rust_unnamed.c2rust_unnamed.ls.layers)
                    .as_mut_ptr()
                    .offset(L_OCEAN_TEMP_256 as libc::c_int as isize),
            );
        }
    } else {
        initBiomeNoise(&mut (*g).c2rust_unnamed.c2rust_unnamed_0.bn, mc);
    };
}
#[no_mangle]
pub unsafe extern "C" fn applySeed(
    mut g: *mut Generator,
    mut dim: libc::c_int,
    mut seed: uint64_t,
) {
    (*g).dim = dim;
    (*g).seed = seed;
    (*g).sha = 0 as libc::c_int as uint64_t;
    if dim == 0 as libc::c_int {
        if (*g).mc <= MC_1_17 as libc::c_int {
            setLayerSeed(
                if !((*g).c2rust_unnamed.c2rust_unnamed.entry).is_null() {
                    (*g).c2rust_unnamed.c2rust_unnamed.entry
                } else {
                    (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_1
                },
                seed,
            );
        } else {
            setBiomeSeed(
                &mut (*g).c2rust_unnamed.c2rust_unnamed_0.bn,
                seed,
                ((*g).flags & 0x1 as libc::c_int as libc::c_uint) as libc::c_int,
            );
        }
    } else if dim == -(1 as libc::c_int) && (*g).mc >= MC_1_16 as libc::c_int {
        setNetherSeed(&mut (*g).nn, seed);
    } else if dim == 1 as libc::c_int && (*g).mc >= MC_1_9 as libc::c_int {
        setEndSeed(&mut (*g).en, seed);
    }
    if (*g).mc >= MC_1_15 as libc::c_int {
        if (*g).mc <= MC_1_17 as libc::c_int
            && dim == 0 as libc::c_int
            && ((*g).c2rust_unnamed.c2rust_unnamed.entry).is_null()
        {
            (*g).sha = (*(*g).c2rust_unnamed.c2rust_unnamed.ls.entry_1).startSalt;
        } else {
            (*g).sha = getVoronoiSHA(seed);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn getMinCacheSize(
    mut g: *const Generator,
    mut scale: libc::c_int,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut sz: libc::c_int,
) -> size_t {
    if sy == 0 as libc::c_int {
        sy = 1 as libc::c_int;
    }
    let mut len: size_t = (sx as size_t)
        .wrapping_mul(sz as libc::c_ulong)
        .wrapping_mul(sy as libc::c_ulong);
    if (*g).mc <= MC_1_17 as libc::c_int && (*g).dim == 0 as libc::c_int {
        let mut entry: *const Layer = getLayerForScale(g, scale);
        if entry.is_null() {
            printf(
                b"getMinCacheSize(): failed to determine scaled entry\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        let mut len2d: size_t = getMinLayerCacheSize(entry, sx, sz);
        len = (len as libc::c_ulong).wrapping_add(len2d.wrapping_sub((sx * sz) as libc::c_ulong))
            as size_t as size_t;
    } else if scale <= 1 as libc::c_int {
        sx = (sx + 3 as libc::c_int >> 2 as libc::c_int) + 2 as libc::c_int;
        sy = (sy + 3 as libc::c_int >> 2 as libc::c_int) + 2 as libc::c_int;
        sz = (sz + 3 as libc::c_int >> 2 as libc::c_int) + 2 as libc::c_int;
        len = (len as libc::c_ulong).wrapping_add((sx * sy * sz) as libc::c_ulong) as size_t
            as size_t;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn allocCache(mut g: *const Generator, mut r: Range) -> *mut libc::c_int {
    let mut len: size_t = getMinCacheSize(g, r.scale, r.sx, r.sy, r.sz);
    return calloc(len, ::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn genBiomes(
    mut g: *const Generator,
    mut cache: *mut libc::c_int,
    mut r: Range,
) -> libc::c_int {
    let mut err: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*g).dim == 0 as libc::c_int {
        if (*g).mc <= MC_1_17 as libc::c_int {
            let mut entry: *const Layer = getLayerForScale(g, r.scale);
            if entry.is_null() {
                return -(1 as libc::c_int);
            }
            err = genArea(entry, cache, r.x, r.z, r.sx, r.sz);
            if err != 0 {
                return err;
            }
            k = 1 as libc::c_int;
            while k < r.sy {
                i = 0 as libc::c_int;
                while i < r.sx * r.sz {
                    *cache.offset((k * r.sx * r.sz + i) as isize) = *cache.offset(i as isize);
                    i += 1;
                }
                k += 1;
            }
            return 0 as libc::c_int;
        } else {
            return genBiomeNoiseScaled(
                &(*g).c2rust_unnamed.c2rust_unnamed_0.bn,
                cache,
                r,
                (*g).mc,
                (*g).sha,
            );
        }
    } else {
        if (*g).dim == -(1 as libc::c_int) {
            return genNetherScaled(&(*g).nn, cache, r, (*g).mc, (*g).sha);
        } else {
            if (*g).dim == 1 as libc::c_int {
                return genEndScaled(&(*g).en, cache, r, (*g).mc, (*g).sha);
            }
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn getBiomeAt(
    mut g: *const Generator,
    mut scale: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut z: libc::c_int,
) -> libc::c_int {
    let mut r: Range = {
        let mut init = Range {
            scale: scale,
            x: x,
            z: z,
            sx: 1 as libc::c_int,
            sz: 1 as libc::c_int,
            y: y,
            sy: 1 as libc::c_int,
        };
        init
    };
    let mut ids: *mut libc::c_int = allocCache(g, r);
    let mut id: libc::c_int = genBiomes(g, ids, r);
    if id == 0 as libc::c_int {
        id = *ids.offset(0 as libc::c_int as isize);
    } else {
        id = none as libc::c_int;
    }
    free(ids as *mut libc::c_void);
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn getLayerForScale(
    mut g: *const Generator,
    mut scale: libc::c_int,
) -> *const Layer {
    if (*g).mc > MC_1_17 as libc::c_int {
        return 0 as *const Layer;
    }
    match scale {
        0 => return (*g).c2rust_unnamed.c2rust_unnamed.entry,
        1 => return (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_1,
        4 => return (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_4,
        16 => return (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_16,
        64 => return (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_64,
        256 => return (*g).c2rust_unnamed.c2rust_unnamed.ls.entry_256,
        _ => return 0 as *const Layer,
    };
}
#[no_mangle]
pub unsafe extern "C" fn setupLayer(
    mut l: *mut Layer,
    mut map: Option<mapfunc_t>,
    mut mc: libc::c_int,
    mut zoom: int8_t,
    mut edge: int8_t,
    mut saltbase: uint64_t,
    mut p: *mut Layer,
    mut p2: *mut Layer,
) -> *mut Layer {
    let ref mut fresh4 = (*l).getMap;
    *fresh4 = map;
    (*l).mc = mc as int8_t;
    (*l).zoom = zoom;
    (*l).edge = edge;
    (*l).scale = 0 as libc::c_int;
    if saltbase == 0 as libc::c_int as libc::c_ulong
        || saltbase as libc::c_ulonglong == !(0 as libc::c_ulonglong)
    {
        (*l).layerSalt = saltbase;
    } else {
        (*l).layerSalt = getLayerSalt(saltbase);
    }
    (*l).startSalt = 0 as libc::c_int as uint64_t;
    (*l).startSeed = 0 as libc::c_int as uint64_t;
    let ref mut fresh5 = (*l).noise;
    *fresh5 = 0 as *mut libc::c_void;
    let ref mut fresh6 = (*l).data;
    *fresh6 = 0 as *mut libc::c_void;
    let ref mut fresh7 = (*l).p;
    *fresh7 = p;
    let ref mut fresh8 = (*l).p2;
    *fresh8 = p2;
    return l;
}
unsafe extern "C" fn setupScale(mut l: *mut Layer, mut scale: libc::c_int) {
    (*l).scale = scale;
    if !((*l).p).is_null() {
        setupScale((*l).p, scale * (*l).zoom as libc::c_int);
    }
    if !((*l).p2).is_null() {
        setupScale((*l).p2, scale * (*l).zoom as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn setupLayerStack(
    mut g: *mut LayerStack,
    mut mc: libc::c_int,
    mut largeBiomes: libc::c_int,
) {
    if mc < MC_1_3 as libc::c_int {
        largeBiomes = 0 as libc::c_int;
    }
    memset(
        g as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<LayerStack>() as libc::c_ulong,
    );
    let mut p: *mut Layer = 0 as *mut Layer;
    let mut l: *mut Layer = ((*g).layers).as_mut_ptr();
    p = setupLayer(
        l.offset(L_CONTINENT_4096 as libc::c_int as isize),
        Some(mapContinent as mapfunc_t),
        mc,
        1 as libc::c_int as int8_t,
        0 as libc::c_int as int8_t,
        1 as libc::c_int as uint64_t,
        0 as *mut Layer,
        0 as *mut Layer,
    );
    p = setupLayer(
        l.offset(L_ZOOM_2048 as libc::c_int as isize),
        Some(mapZoomFuzzy as mapfunc_t),
        mc,
        2 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        2000 as libc::c_int as uint64_t,
        p,
        0 as *mut Layer,
    );
    p = setupLayer(
        l.offset(L_LAND_2048 as libc::c_int as isize),
        Some(mapLand as mapfunc_t),
        mc,
        1 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        1 as libc::c_int as uint64_t,
        p,
        0 as *mut Layer,
    );
    p = setupLayer(
        l.offset(L_ZOOM_1024 as libc::c_int as isize),
        Some(mapZoom as mapfunc_t),
        mc,
        2 as libc::c_int as int8_t,
        3 as libc::c_int as int8_t,
        2001 as libc::c_int as uint64_t,
        p,
        0 as *mut Layer,
    );
    p = setupLayer(
        l.offset(L_LAND_1024_A as libc::c_int as isize),
        Some(mapLand as mapfunc_t),
        mc,
        1 as libc::c_int as int8_t,
        2 as libc::c_int as int8_t,
        2 as libc::c_int as uint64_t,
        p,
        0 as *mut Layer,
    );
    if mc <= MC_1_6 as libc::c_int {
        p = setupLayer(
            l.offset(L_SNOW_1024 as libc::c_int as isize),
            Some(mapSnow16 as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_512 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_512 as libc::c_int as isize),
            Some(mapLand16 as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_256 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_256 as libc::c_int as isize),
            Some(mapLand16 as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            4 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_MUSHROOM_256 as libc::c_int as isize),
            Some(mapMushroom as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            5 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_BIOME_256 as libc::c_int as isize),
            Some(mapBiome as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            200 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_128 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_NOISE_256 as libc::c_int as isize),
            Some(mapNoise as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            100 as libc::c_int as uint64_t,
            l.offset(L_MUSHROOM_256 as libc::c_int as isize),
            0 as *mut Layer,
        );
    } else {
        p = setupLayer(
            l.offset(L_LAND_1024_B as libc::c_int as isize),
            Some(mapLand as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            50 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_1024_C as libc::c_int as isize),
            Some(mapLand as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            70 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ISLAND_1024 as libc::c_int as isize),
            Some(mapIsland as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SNOW_1024 as libc::c_int as isize),
            Some(mapSnow as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_1024_D as libc::c_int as isize),
            Some(mapLand as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_COOL_1024 as libc::c_int as isize),
            Some(mapCool as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_HEAT_1024 as libc::c_int as isize),
            Some(mapHeat as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SPECIAL_1024 as libc::c_int as isize),
            Some(mapSpecial as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_512 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_256 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_256 as libc::c_int as isize),
            Some(mapLand as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            4 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_MUSHROOM_256 as libc::c_int as isize),
            Some(mapMushroom as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            5 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_DEEP_OCEAN_256 as libc::c_int as isize),
            Some(mapDeepOcean as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            4 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_BIOME_256 as libc::c_int as isize),
            Some(mapBiome as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            200 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        if mc >= MC_1_14 as libc::c_int {
            p = setupLayer(
                l.offset(L_BAMBOO_256 as libc::c_int as isize),
                Some(mapBamboo as mapfunc_t),
                mc,
                1 as libc::c_int as int8_t,
                0 as libc::c_int as int8_t,
                1001 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
        p = setupLayer(
            l.offset(L_ZOOM_128 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_BIOME_EDGE_64 as libc::c_int as isize),
            Some(mapBiomeEdge as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_RIVER_INIT_256 as libc::c_int as isize),
            Some(mapNoise as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            100 as libc::c_int as uint64_t,
            l.offset(L_DEEP_OCEAN_256 as libc::c_int as isize),
            0 as *mut Layer,
        );
    }
    if mc <= MC_1_12 as libc::c_int {
        p = setupLayer(
            l.offset(L_ZOOM_128_HILLS as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            0 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_HILLS as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            0 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    } else if mc >= MC_1_1 as libc::c_int {
        p = setupLayer(
            l.offset(L_ZOOM_128_HILLS as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_HILLS as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    }
    if mc <= MC_1_0 as libc::c_int {
        p = setupLayer(
            l.offset(L_ZOOM_32 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_ZOOM_64 as libc::c_int as isize),
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_32 as libc::c_int as isize),
            Some(mapLand16 as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SHORE_16 as libc::c_int as isize),
            Some(mapShore as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SMOOTH_4 as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_128_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_NOISE_256 as libc::c_int as isize),
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_32_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1004 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1005 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_RIVER_4 as libc::c_int as isize),
            Some(mapRiver as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SMOOTH_4_RIVER as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    } else if mc <= MC_1_6 as libc::c_int {
        p = setupLayer(
            l.offset(L_HILLS_64 as libc::c_int as isize),
            Some(mapHills as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_ZOOM_64 as libc::c_int as isize),
            l.offset(L_ZOOM_64_HILLS as libc::c_int as isize),
        );
        p = setupLayer(
            l.offset(L_ZOOM_32 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_32 as libc::c_int as isize),
            Some(mapLand16 as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SHORE_16 as libc::c_int as isize),
            Some(mapShore as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SWAMP_RIVER_16 as libc::c_int as isize),
            Some(mapSwampRiver as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        if largeBiomes != 0 {
            p = setupLayer(
                l.offset(L_ZOOM_LARGE_A as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1004 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
            p = setupLayer(
                l.offset(L_ZOOM_LARGE_B as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1005 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
        p = setupLayer(
            l.offset(L_SMOOTH_4 as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_128_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_NOISE_256 as libc::c_int as isize),
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_32_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1004 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1005 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        if largeBiomes != 0 {
            p = setupLayer(
                l.offset(L_ZOOM_L_RIVER_A as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1006 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
            p = setupLayer(
                l.offset(L_ZOOM_L_RIVER_B as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1007 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
        p = setupLayer(
            l.offset(L_RIVER_4 as libc::c_int as isize),
            Some(mapRiver as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SMOOTH_4_RIVER as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    } else {
        p = setupLayer(
            l.offset(L_HILLS_64 as libc::c_int as isize),
            Some(mapHills as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_BIOME_EDGE_64 as libc::c_int as isize),
            l.offset(L_ZOOM_64_HILLS as libc::c_int as isize),
        );
        p = setupLayer(
            l.offset(L_SUNFLOWER_64 as libc::c_int as isize),
            Some(mapSunflower as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_32 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_LAND_32 as libc::c_int as isize),
            Some(mapLand as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SHORE_16 as libc::c_int as isize),
            Some(mapShore as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4 as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        if largeBiomes != 0 {
            p = setupLayer(
                l.offset(L_ZOOM_LARGE_A as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1004 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
            p = setupLayer(
                l.offset(L_ZOOM_LARGE_B as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1005 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
        p = setupLayer(
            l.offset(L_SMOOTH_4 as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_128_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            l.offset(L_RIVER_INIT_256 as libc::c_int as isize),
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_32_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4_RIVER as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            1003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        if largeBiomes != 0 && mc == MC_1_7 as libc::c_int {
            p = setupLayer(
                l.offset(L_ZOOM_L_RIVER_A as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1004 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
            p = setupLayer(
                l.offset(L_ZOOM_L_RIVER_B as libc::c_int as isize),
                Some(mapZoom as mapfunc_t),
                mc,
                2 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                1005 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
        p = setupLayer(
            l.offset(L_RIVER_4 as libc::c_int as isize),
            Some(mapRiver as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_SMOOTH_4_RIVER as libc::c_int as isize),
            Some(mapSmooth as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            2 as libc::c_int as int8_t,
            1000 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    }
    p = setupLayer(
        l.offset(L_RIVER_MIX_4 as libc::c_int as isize),
        Some(mapRiverMix as mapfunc_t),
        mc,
        1 as libc::c_int as int8_t,
        0 as libc::c_int as int8_t,
        100 as libc::c_int as uint64_t,
        l.offset(L_SMOOTH_4 as libc::c_int as isize),
        l.offset(L_SMOOTH_4_RIVER as libc::c_int as isize),
    );
    if mc <= MC_1_12 as libc::c_int {
        p = setupLayer(
            l.offset(L_VORONOI_1 as libc::c_int as isize),
            Some(mapVoronoi114 as mapfunc_t),
            mc,
            4 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            10 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
    } else {
        p = setupLayer(
            l.offset(L_OCEAN_TEMP_256 as libc::c_int as isize),
            Some(mapOceanTemp as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            2 as libc::c_int as uint64_t,
            0 as *mut Layer,
            0 as *mut Layer,
        );
        let ref mut fresh9 = (*p).noise;
        *fresh9 = &mut (*g).oceanRnd as *mut PerlinNoise as *mut libc::c_void;
        p = setupLayer(
            l.offset(L_ZOOM_128_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2001 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_64_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2002 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_32_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2003 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_16_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2004 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_8_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2005 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_ZOOM_4_OCEAN as libc::c_int as isize),
            Some(mapZoom as mapfunc_t),
            mc,
            2 as libc::c_int as int8_t,
            3 as libc::c_int as int8_t,
            2006 as libc::c_int as uint64_t,
            p,
            0 as *mut Layer,
        );
        p = setupLayer(
            l.offset(L_OCEAN_MIX_4 as libc::c_int as isize),
            Some(mapOceanMix as mapfunc_t),
            mc,
            1 as libc::c_int as int8_t,
            17 as libc::c_int as int8_t,
            100 as libc::c_int as uint64_t,
            l.offset(L_RIVER_MIX_4 as libc::c_int as isize),
            l.offset(L_ZOOM_4_OCEAN as libc::c_int as isize),
        );
        if mc <= MC_1_14 as libc::c_int {
            p = setupLayer(
                l.offset(L_VORONOI_1 as libc::c_int as isize),
                Some(mapVoronoi114 as mapfunc_t),
                mc,
                4 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                10 as libc::c_int as uint64_t,
                p,
                0 as *mut Layer,
            );
        } else {
            p = setupLayer(
                l.offset(L_VORONOI_1 as libc::c_int as isize),
                Some(mapVoronoi as mapfunc_t),
                mc,
                4 as libc::c_int as int8_t,
                3 as libc::c_int as int8_t,
                !(0 as libc::c_ulonglong) as uint64_t,
                p,
                0 as *mut Layer,
            );
        }
    }
    let ref mut fresh10 = (*g).entry_1;
    *fresh10 = p;
    let ref mut fresh11 = (*g).entry_4;
    *fresh11 = l.offset(
        (if mc <= MC_1_12 as libc::c_int {
            L_RIVER_MIX_4 as libc::c_int
        } else {
            L_OCEAN_MIX_4 as libc::c_int
        }) as isize,
    );
    if largeBiomes != 0 {
        let ref mut fresh12 = (*g).entry_16;
        *fresh12 = l.offset(L_ZOOM_4 as libc::c_int as isize);
        let ref mut fresh13 = (*g).entry_64;
        *fresh13 = l.offset(
            (if mc <= MC_1_6 as libc::c_int {
                L_SWAMP_RIVER_16 as libc::c_int
            } else {
                L_SHORE_16 as libc::c_int
            }) as isize,
        );
        let ref mut fresh14 = (*g).entry_256;
        *fresh14 = l.offset(
            (if mc <= MC_1_7 as libc::c_int {
                L_HILLS_64 as libc::c_int
            } else {
                L_SUNFLOWER_64 as libc::c_int
            }) as isize,
        );
    } else if mc >= MC_1_1 as libc::c_int {
        let ref mut fresh15 = (*g).entry_16;
        *fresh15 = l.offset(
            (if mc <= MC_1_6 as libc::c_int {
                L_SWAMP_RIVER_16 as libc::c_int
            } else {
                L_SHORE_16 as libc::c_int
            }) as isize,
        );
        let ref mut fresh16 = (*g).entry_64;
        *fresh16 = l.offset(
            (if mc <= MC_1_7 as libc::c_int {
                L_HILLS_64 as libc::c_int
            } else {
                L_SUNFLOWER_64 as libc::c_int
            }) as isize,
        );
        let ref mut fresh17 = (*g).entry_256;
        *fresh17 = l.offset(
            (if mc <= MC_1_14 as libc::c_int {
                L_BIOME_256 as libc::c_int
            } else {
                L_BAMBOO_256 as libc::c_int
            }) as isize,
        );
    } else {
        let ref mut fresh18 = (*g).entry_16;
        *fresh18 = l.offset(L_ZOOM_16 as libc::c_int as isize);
        let ref mut fresh19 = (*g).entry_64;
        *fresh19 = l.offset(L_ZOOM_64 as libc::c_int as isize);
        let ref mut fresh20 = (*g).entry_256;
        *fresh20 = l.offset(L_BIOME_256 as libc::c_int as isize);
    }
    setupScale((*g).entry_1, 1 as libc::c_int);
}
unsafe extern "C" fn getMaxArea(
    mut layer: *const Layer,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut maxX: *mut libc::c_int,
    mut maxZ: *mut libc::c_int,
    mut siz: *mut size_t,
) {
    if layer.is_null() {
        return;
    }
    areaX += (*layer).edge as libc::c_int;
    areaZ += (*layer).edge as libc::c_int;
    if !((*layer).p2).is_null() || (*layer).zoom as libc::c_int != 1 as libc::c_int {
        *siz = (*siz as libc::c_ulong).wrapping_add((areaX * areaZ) as libc::c_ulong) as size_t
            as size_t;
    }
    if areaX > *maxX {
        *maxX = areaX;
    }
    if areaZ > *maxZ {
        *maxZ = areaZ;
    }
    if (*layer).zoom as libc::c_int == 2 as libc::c_int {
        areaX >>= 1 as libc::c_int;
        areaZ >>= 1 as libc::c_int;
    } else if (*layer).zoom as libc::c_int == 4 as libc::c_int {
        areaX >>= 2 as libc::c_int;
        areaZ >>= 2 as libc::c_int;
    }
    getMaxArea((*layer).p, areaX, areaZ, maxX, maxZ, siz);
    if !((*layer).p2).is_null() {
        getMaxArea((*layer).p2, areaX, areaZ, maxX, maxZ, siz);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getMinLayerCacheSize(
    mut layer: *const Layer,
    mut sizeX: libc::c_int,
    mut sizeZ: libc::c_int,
) -> size_t {
    let mut maxX: libc::c_int = sizeX;
    let mut maxZ: libc::c_int = sizeZ;
    let mut bufsiz: size_t = 0 as libc::c_int as size_t;
    getMaxArea(layer, sizeX, sizeZ, &mut maxX, &mut maxZ, &mut bufsiz);
    return bufsiz.wrapping_add((maxX as libc::c_ulong).wrapping_mul(maxZ as size_t));
}
#[no_mangle]
pub unsafe extern "C" fn genArea(
    mut layer: *const Layer,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> libc::c_int {
    memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        ((areaWidth * areaHeight) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return ((*layer).getMap).expect("non-null function pointer")(
        layer, out, areaX, areaZ, areaWidth, areaHeight,
    );
}
