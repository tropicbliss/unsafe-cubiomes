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
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
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
pub const mangrove_swamp: BiomeID = 184;
pub const deep_dark: BiomeID = 183;
pub const frozen_peaks: BiomeID = 181;
pub const jagged_peaks: BiomeID = 180;
pub const stony_peaks: BiomeID = 182;
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
pub const deep_frozen_ocean: BiomeID = 50;
pub const deep_cold_ocean: BiomeID = 49;
pub const deep_lukewarm_ocean: BiomeID = 48;
pub const deep_warm_ocean: BiomeID = 47;
pub const cold_ocean: BiomeID = 46;
pub const lukewarm_ocean: BiomeID = 45;
pub const warm_ocean: BiomeID = 44;
pub const end_barrens: BiomeID = 43;
pub const end_highlands: BiomeID = 42;
pub const end_midlands: BiomeID = 41;
pub const small_end_islands: BiomeID = 40;
pub const badlands_plateau: BiomeID = 39;
pub const wooded_badlands_plateau: BiomeID = 38;
pub const badlands: BiomeID = 37;
pub const savanna_plateau: BiomeID = 36;
pub const savanna: BiomeID = 35;
pub const wooded_mountains: BiomeID = 34;
pub const giant_tree_taiga_hills: BiomeID = 33;
pub const giant_tree_taiga: BiomeID = 32;
pub const snowy_taiga_hills: BiomeID = 31;
pub const snowy_taiga: BiomeID = 30;
pub const dark_forest: BiomeID = 29;
pub const birch_forest_hills: BiomeID = 28;
pub const birch_forest: BiomeID = 27;
pub const snowy_beach: BiomeID = 26;
pub const stone_shore: BiomeID = 25;
pub const deep_ocean: BiomeID = 24;
pub const jungle_edge: BiomeID = 23;
pub const jungle_hills: BiomeID = 22;
pub const jungle: BiomeID = 21;
pub const mountain_edge: BiomeID = 20;
pub const taiga_hills: BiomeID = 19;
pub const wooded_hills: BiomeID = 18;
pub const desert_hills: BiomeID = 17;
pub const beach: BiomeID = 16;
pub const mushroom_field_shore: BiomeID = 15;
pub const mushroom_fields: BiomeID = 14;
pub const snowy_mountains: BiomeID = 13;
pub const snowy_tundra: BiomeID = 12;
pub const frozen_river: BiomeID = 11;
pub const frozen_ocean: BiomeID = 10;
pub const the_end: BiomeID = 9;
pub const nether_wastes: BiomeID = 8;
pub const river: BiomeID = 7;
pub const swamp: BiomeID = 6;
pub const taiga: BiomeID = 5;
pub const forest: BiomeID = 4;
pub const mountains: BiomeID = 3;
pub const desert: BiomeID = 2;
pub const plains: BiomeID = 1;
pub const ocean: BiomeID = 0;
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
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
pub const MC_NEWEST: MCVersion = 19;
pub type size_t = libc::c_ulong;
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type MCVersion = libc::c_uint;
pub type BiomeID = libc::c_int;
pub const BIOME_NUM: BiomeID = 51;
pub const frozenDeepOcean: BiomeID = 50;
pub const coldDeepOcean: BiomeID = 49;
pub const lukewarmDeepOcean: BiomeID = 48;
pub const warmDeepOcean: BiomeID = 47;
pub const coldOcean: BiomeID = 46;
pub const lukewarmOcean: BiomeID = 45;
pub const warmOcean: BiomeID = 44;
pub const mesaPlateau: BiomeID = 39;
pub const mesaPlateau_F: BiomeID = 38;
pub const mesa: BiomeID = 37;
pub const savannaPlateau: BiomeID = 36;
pub const extremeHillsPlus: BiomeID = 34;
pub const megaTaigaHills: BiomeID = 33;
pub const megaTaiga: BiomeID = 32;
pub const coldTaigaHills: BiomeID = 31;
pub const coldTaiga: BiomeID = 30;
pub const roofedForest: BiomeID = 29;
pub const birchForestHills: BiomeID = 28;
pub const birchForest: BiomeID = 27;
pub const coldBeach: BiomeID = 26;
pub const stoneBeach: BiomeID = 25;
pub const deepOcean: BiomeID = 24;
pub const jungleEdge: BiomeID = 23;
pub const jungleHills: BiomeID = 22;
pub const extremeHillsEdge: BiomeID = 20;
pub const taigaHills: BiomeID = 19;
pub const forestHills: BiomeID = 18;
pub const desertHills: BiomeID = 17;
pub const mushroomIslandShore: BiomeID = 15;
pub const mushroomIsland: BiomeID = 14;
pub const iceMountains: BiomeID = 13;
pub const icePlains: BiomeID = 12;
pub const frozenRiver: BiomeID = 11;
pub const frozenOcean: BiomeID = 10;
pub const sky: BiomeID = 9;
pub const hell: BiomeID = 8;
pub const swampland: BiomeID = 6;
pub const extremeHills: BiomeID = 3;
pub const none: BiomeID = -1;
pub type BiomeTempCategory = libc::c_uint;
pub const Special: BiomeTempCategory = 5;
#[no_mangle]
pub unsafe extern "C" fn mc2str(mut mc: libc::c_int) -> *const libc::c_char {
    match mc {
        0 => return b"1.0\0" as *const u8 as *const libc::c_char,
        1 => return b"1.1\0" as *const u8 as *const libc::c_char,
        2 => return b"1.2\0" as *const u8 as *const libc::c_char,
        3 => return b"1.3\0" as *const u8 as *const libc::c_char,
        4 => return b"1.4\0" as *const u8 as *const libc::c_char,
        5 => return b"1.5\0" as *const u8 as *const libc::c_char,
        6 => return b"1.6\0" as *const u8 as *const libc::c_char,
        7 => return b"1.7\0" as *const u8 as *const libc::c_char,
        8 => return b"1.8\0" as *const u8 as *const libc::c_char,
        9 => return b"1.9\0" as *const u8 as *const libc::c_char,
        10 => return b"1.10\0" as *const u8 as *const libc::c_char,
        11 => return b"1.11\0" as *const u8 as *const libc::c_char,
        12 => return b"1.12\0" as *const u8 as *const libc::c_char,
        13 => return b"1.13\0" as *const u8 as *const libc::c_char,
        14 => return b"1.14\0" as *const u8 as *const libc::c_char,
        15 => return b"1.15\0" as *const u8 as *const libc::c_char,
        16 => return b"1.16\0" as *const u8 as *const libc::c_char,
        17 => return b"1.17\0" as *const u8 as *const libc::c_char,
        18 => return b"1.18\0" as *const u8 as *const libc::c_char,
        19 => return b"1.19\0" as *const u8 as *const libc::c_char,
        _ => return 0 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn str2mc(mut s: *const libc::c_char) -> libc::c_int {
    if strcmp(s, b"1.19\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_19 as libc::c_int;
    }
    if strcmp(s, b"1.18\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_18 as libc::c_int;
    }
    if strcmp(s, b"1.17\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_17 as libc::c_int;
    }
    if strcmp(s, b"1.16\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_16 as libc::c_int;
    }
    if strcmp(s, b"1.15\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_15 as libc::c_int;
    }
    if strcmp(s, b"1.14\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_14 as libc::c_int;
    }
    if strcmp(s, b"1.13\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_13 as libc::c_int;
    }
    if strcmp(s, b"1.12\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_12 as libc::c_int;
    }
    if strcmp(s, b"1.11\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_11 as libc::c_int;
    }
    if strcmp(s, b"1.10\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_10 as libc::c_int;
    }
    if strcmp(s, b"1.9\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_9 as libc::c_int;
    }
    if strcmp(s, b"1.8\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_8 as libc::c_int;
    }
    if strcmp(s, b"1.7\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_7 as libc::c_int;
    }
    if strcmp(s, b"1.6\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_6 as libc::c_int;
    }
    if strcmp(s, b"1.5\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_5 as libc::c_int;
    }
    if strcmp(s, b"1.4\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_4 as libc::c_int;
    }
    if strcmp(s, b"1.3\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_3 as libc::c_int;
    }
    if strcmp(s, b"1.2\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_2 as libc::c_int;
    }
    if strcmp(s, b"1.1\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_1 as libc::c_int;
    }
    if strcmp(s, b"1.0\0" as *const u8 as *const libc::c_char) == 0 {
        return MC_1_0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn biome2str(
    mut mc: libc::c_int,
    mut id: libc::c_int,
) -> *const libc::c_char {
    if mc >= MC_1_18 as libc::c_int {
        match id {
            155 => {
                return b"old_growth_birch_forest\0" as *const u8 as *const libc::c_char;
            }
            32 => return b"old_growth_pine_taiga\0" as *const u8 as *const libc::c_char,
            160 => {
                return b"old_growth_spruce_taiga\0" as *const u8 as *const libc::c_char;
            }
            12 => return b"snowy_plains\0" as *const u8 as *const libc::c_char,
            23 => return b"sparse_jungle\0" as *const u8 as *const libc::c_char,
            25 => return b"stony_shore\0" as *const u8 as *const libc::c_char,
            3 => return b"windswept_hills\0" as *const u8 as *const libc::c_char,
            34 => return b"windswept_forest\0" as *const u8 as *const libc::c_char,
            131 => {
                return b"windswept_gravelly_hills\0" as *const u8 as *const libc::c_char;
            }
            163 => return b"windswept_savanna\0" as *const u8 as *const libc::c_char,
            38 => return b"wooded_badlands\0" as *const u8 as *const libc::c_char,
            _ => {}
        }
    }
    match id {
        0 => return b"ocean\0" as *const u8 as *const libc::c_char,
        1 => return b"plains\0" as *const u8 as *const libc::c_char,
        2 => return b"desert\0" as *const u8 as *const libc::c_char,
        3 => return b"mountains\0" as *const u8 as *const libc::c_char,
        4 => return b"forest\0" as *const u8 as *const libc::c_char,
        5 => return b"taiga\0" as *const u8 as *const libc::c_char,
        6 => return b"swamp\0" as *const u8 as *const libc::c_char,
        7 => return b"river\0" as *const u8 as *const libc::c_char,
        8 => return b"nether_wastes\0" as *const u8 as *const libc::c_char,
        9 => return b"the_end\0" as *const u8 as *const libc::c_char,
        10 => return b"frozen_ocean\0" as *const u8 as *const libc::c_char,
        11 => return b"frozen_river\0" as *const u8 as *const libc::c_char,
        12 => return b"snowy_tundra\0" as *const u8 as *const libc::c_char,
        13 => return b"snowy_mountains\0" as *const u8 as *const libc::c_char,
        14 => return b"mushroom_fields\0" as *const u8 as *const libc::c_char,
        15 => return b"mushroom_field_shore\0" as *const u8 as *const libc::c_char,
        16 => return b"beach\0" as *const u8 as *const libc::c_char,
        17 => return b"desert_hills\0" as *const u8 as *const libc::c_char,
        18 => return b"wooded_hills\0" as *const u8 as *const libc::c_char,
        19 => return b"taiga_hills\0" as *const u8 as *const libc::c_char,
        20 => return b"mountain_edge\0" as *const u8 as *const libc::c_char,
        21 => return b"jungle\0" as *const u8 as *const libc::c_char,
        22 => return b"jungle_hills\0" as *const u8 as *const libc::c_char,
        23 => return b"jungle_edge\0" as *const u8 as *const libc::c_char,
        24 => return b"deep_ocean\0" as *const u8 as *const libc::c_char,
        25 => return b"stone_shore\0" as *const u8 as *const libc::c_char,
        26 => return b"snowy_beach\0" as *const u8 as *const libc::c_char,
        27 => return b"birch_forest\0" as *const u8 as *const libc::c_char,
        28 => return b"birch_forest_hills\0" as *const u8 as *const libc::c_char,
        29 => return b"dark_forest\0" as *const u8 as *const libc::c_char,
        30 => return b"snowy_taiga\0" as *const u8 as *const libc::c_char,
        31 => return b"snowy_taiga_hills\0" as *const u8 as *const libc::c_char,
        32 => return b"giant_tree_taiga\0" as *const u8 as *const libc::c_char,
        33 => return b"giant_tree_taiga_hills\0" as *const u8 as *const libc::c_char,
        34 => return b"wooded_mountains\0" as *const u8 as *const libc::c_char,
        35 => return b"savanna\0" as *const u8 as *const libc::c_char,
        36 => return b"savanna_plateau\0" as *const u8 as *const libc::c_char,
        37 => return b"badlands\0" as *const u8 as *const libc::c_char,
        38 => return b"wooded_badlands_plateau\0" as *const u8 as *const libc::c_char,
        39 => return b"badlands_plateau\0" as *const u8 as *const libc::c_char,
        40 => return b"small_end_islands\0" as *const u8 as *const libc::c_char,
        41 => return b"end_midlands\0" as *const u8 as *const libc::c_char,
        42 => return b"end_highlands\0" as *const u8 as *const libc::c_char,
        43 => return b"end_barrens\0" as *const u8 as *const libc::c_char,
        44 => return b"warm_ocean\0" as *const u8 as *const libc::c_char,
        45 => return b"lukewarm_ocean\0" as *const u8 as *const libc::c_char,
        46 => return b"cold_ocean\0" as *const u8 as *const libc::c_char,
        47 => return b"deep_warm_ocean\0" as *const u8 as *const libc::c_char,
        48 => return b"deep_lukewarm_ocean\0" as *const u8 as *const libc::c_char,
        49 => return b"deep_cold_ocean\0" as *const u8 as *const libc::c_char,
        50 => return b"deep_frozen_ocean\0" as *const u8 as *const libc::c_char,
        127 => return b"the_void\0" as *const u8 as *const libc::c_char,
        129 => return b"sunflower_plains\0" as *const u8 as *const libc::c_char,
        130 => return b"desert_lakes\0" as *const u8 as *const libc::c_char,
        131 => return b"gravelly_mountains\0" as *const u8 as *const libc::c_char,
        132 => return b"flower_forest\0" as *const u8 as *const libc::c_char,
        133 => return b"taiga_mountains\0" as *const u8 as *const libc::c_char,
        134 => return b"swamp_hills\0" as *const u8 as *const libc::c_char,
        140 => return b"ice_spikes\0" as *const u8 as *const libc::c_char,
        149 => return b"modified_jungle\0" as *const u8 as *const libc::c_char,
        151 => return b"modified_jungle_edge\0" as *const u8 as *const libc::c_char,
        155 => return b"tall_birch_forest\0" as *const u8 as *const libc::c_char,
        156 => return b"tall_birch_hills\0" as *const u8 as *const libc::c_char,
        157 => return b"dark_forest_hills\0" as *const u8 as *const libc::c_char,
        158 => return b"snowy_taiga_mountains\0" as *const u8 as *const libc::c_char,
        160 => return b"giant_spruce_taiga\0" as *const u8 as *const libc::c_char,
        161 => return b"giant_spruce_taiga_hills\0" as *const u8 as *const libc::c_char,
        162 => {
            return b"modified_gravelly_mountains\0" as *const u8 as *const libc::c_char;
        }
        163 => return b"shattered_savanna\0" as *const u8 as *const libc::c_char,
        164 => return b"shattered_savanna_plateau\0" as *const u8 as *const libc::c_char,
        165 => return b"eroded_badlands\0" as *const u8 as *const libc::c_char,
        166 => {
            return b"modified_wooded_badlands_plateau\0" as *const u8 as *const libc::c_char;
        }
        167 => return b"modified_badlands_plateau\0" as *const u8 as *const libc::c_char,
        168 => return b"bamboo_jungle\0" as *const u8 as *const libc::c_char,
        169 => return b"bamboo_jungle_hills\0" as *const u8 as *const libc::c_char,
        170 => return b"soul_sand_valley\0" as *const u8 as *const libc::c_char,
        171 => return b"crimson_forest\0" as *const u8 as *const libc::c_char,
        172 => return b"warped_forest\0" as *const u8 as *const libc::c_char,
        173 => return b"basalt_deltas\0" as *const u8 as *const libc::c_char,
        174 => return b"dripstone_caves\0" as *const u8 as *const libc::c_char,
        175 => return b"lush_caves\0" as *const u8 as *const libc::c_char,
        177 => return b"meadow\0" as *const u8 as *const libc::c_char,
        178 => return b"grove\0" as *const u8 as *const libc::c_char,
        179 => return b"snowy_slopes\0" as *const u8 as *const libc::c_char,
        182 => return b"stony_peaks\0" as *const u8 as *const libc::c_char,
        180 => return b"jagged_peaks\0" as *const u8 as *const libc::c_char,
        181 => return b"frozen_peaks\0" as *const u8 as *const libc::c_char,
        183 => return b"deep_dark\0" as *const u8 as *const libc::c_char,
        184 => return b"mangrove_swamp\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn setBiomeColor(
    mut biomeColor: *mut [libc::c_uchar; 3],
    mut id: libc::c_int,
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
) {
    (*biomeColor.offset(id as isize))[0 as libc::c_int as usize] = r;
    (*biomeColor.offset(id as isize))[1 as libc::c_int as usize] = g;
    (*biomeColor.offset(id as isize))[2 as libc::c_int as usize] = b;
}
#[no_mangle]
pub unsafe extern "C" fn setMutationColor(
    mut biomeColor: *mut [libc::c_uchar; 3],
    mut mutated: libc::c_int,
    mut parent: libc::c_int,
) {
    let mut c: libc::c_uint = 0;
    c = ((*biomeColor.offset(parent as isize))[0 as libc::c_int as usize] as libc::c_int
        + 40 as libc::c_int) as libc::c_uint;
    (*biomeColor.offset(mutated as isize))[0 as libc::c_int as usize] =
        (if c > 255 as libc::c_int as libc::c_uint {
            255 as libc::c_int as libc::c_uint
        } else {
            c
        }) as libc::c_uchar;
    c = ((*biomeColor.offset(parent as isize))[1 as libc::c_int as usize] as libc::c_int
        + 40 as libc::c_int) as libc::c_uint;
    (*biomeColor.offset(mutated as isize))[1 as libc::c_int as usize] =
        (if c > 255 as libc::c_int as libc::c_uint {
            255 as libc::c_int as libc::c_uint
        } else {
            c
        }) as libc::c_uchar;
    c = ((*biomeColor.offset(parent as isize))[2 as libc::c_int as usize] as libc::c_int
        + 40 as libc::c_int) as libc::c_uint;
    (*biomeColor.offset(mutated as isize))[2 as libc::c_int as usize] =
        (if c > 255 as libc::c_int as libc::c_uint {
            255 as libc::c_int as libc::c_uint
        } else {
            c
        }) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn initBiomeColors(mut biomeColors: *mut [libc::c_uchar; 3]) {
    memset(
        biomeColors as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int * 3 as libc::c_int) as libc::c_ulong,
    );
    setBiomeColor(
        biomeColors,
        ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        plains as libc::c_int,
        141 as libc::c_int as libc::c_uchar,
        179 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        desert as libc::c_int,
        250 as libc::c_int as libc::c_uchar,
        148 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        mountains as libc::c_int,
        96 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        forest as libc::c_int,
        5 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        taiga as libc::c_int,
        11 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        swamp as libc::c_int,
        7 as libc::c_int as libc::c_uchar,
        249 as libc::c_int as libc::c_uchar,
        178 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        river as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        nether_wastes as libc::c_int,
        87 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        the_end as libc::c_int,
        128 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        frozen_ocean as libc::c_int,
        112 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        frozen_river as libc::c_int,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_tundra as libc::c_int,
        255 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_mountains as libc::c_int,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        mushroom_fields as libc::c_int,
        255 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        mushroom_field_shore as libc::c_int,
        160 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        beach as libc::c_int,
        250 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        desert_hills as libc::c_int,
        210 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        wooded_hills as libc::c_int,
        34 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        taiga_hills as libc::c_int,
        22 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        mountain_edge as libc::c_int,
        114 as libc::c_int as libc::c_uchar,
        120 as libc::c_int as libc::c_uchar,
        154 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        jungle as libc::c_int,
        80 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        jungle_hills as libc::c_int,
        44 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        jungle_edge as libc::c_int,
        96 as libc::c_int as libc::c_uchar,
        147 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        stone_shore as libc::c_int,
        162 as libc::c_int as libc::c_uchar,
        162 as libc::c_int as libc::c_uchar,
        132 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_beach as libc::c_int,
        250 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        birch_forest as libc::c_int,
        48 as libc::c_int as libc::c_uchar,
        116 as libc::c_int as libc::c_uchar,
        68 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        birch_forest_hills as libc::c_int,
        31 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        dark_forest as libc::c_int,
        64 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_taiga as libc::c_int,
        49 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
        74 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_taiga_hills as libc::c_int,
        36 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        giant_tree_taiga as libc::c_int,
        89 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        giant_tree_taiga_hills as libc::c_int,
        69 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        wooded_mountains as libc::c_int,
        91 as libc::c_int as libc::c_uchar,
        115 as libc::c_int as libc::c_uchar,
        82 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        savanna as libc::c_int,
        189 as libc::c_int as libc::c_uchar,
        178 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        savanna_plateau as libc::c_int,
        167 as libc::c_int as libc::c_uchar,
        157 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        badlands as libc::c_int,
        217 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        wooded_badlands_plateau as libc::c_int,
        176 as libc::c_int as libc::c_uchar,
        151 as libc::c_int as libc::c_uchar,
        101 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        badlands_plateau as libc::c_int,
        202 as libc::c_int as libc::c_uchar,
        140 as libc::c_int as libc::c_uchar,
        101 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        small_end_islands as libc::c_int,
        75 as libc::c_int as libc::c_uchar,
        75 as libc::c_int as libc::c_uchar,
        171 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        end_midlands as libc::c_int,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        end_highlands as libc::c_int,
        181 as libc::c_int as libc::c_uchar,
        181 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        end_barrens as libc::c_int,
        112 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        warm_ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        lukewarm_ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        cold_ocean as libc::c_int,
        32 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_warm_ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_lukewarm_ocean as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_cold_ocean as libc::c_int,
        32 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_frozen_ocean as libc::c_int,
        64 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        the_void as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    setMutationColor(
        biomeColors,
        sunflower_plains as libc::c_int,
        plains as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        desert_lakes as libc::c_int,
        desert as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        gravelly_mountains as libc::c_int,
        mountains as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        flower_forest as libc::c_int,
        forest as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        taiga_mountains as libc::c_int,
        taiga as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        swamp_hills as libc::c_int,
        swamp as libc::c_int,
    );
    setBiomeColor(
        biomeColors,
        ice_spikes as libc::c_int,
        180 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    );
    setMutationColor(
        biomeColors,
        modified_jungle as libc::c_int,
        jungle as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        modified_jungle_edge as libc::c_int,
        jungle_edge as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        tall_birch_forest as libc::c_int,
        birch_forest as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        tall_birch_hills as libc::c_int,
        birch_forest_hills as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        dark_forest_hills as libc::c_int,
        dark_forest as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        snowy_taiga_mountains as libc::c_int,
        snowy_taiga as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        giant_spruce_taiga as libc::c_int,
        giant_tree_taiga as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        giant_spruce_taiga_hills as libc::c_int,
        giant_tree_taiga_hills as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        modified_gravelly_mountains as libc::c_int,
        wooded_mountains as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        shattered_savanna as libc::c_int,
        savanna as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        shattered_savanna_plateau as libc::c_int,
        savanna_plateau as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        eroded_badlands as libc::c_int,
        badlands as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        modified_wooded_badlands_plateau as libc::c_int,
        wooded_badlands_plateau as libc::c_int,
    );
    setMutationColor(
        biomeColors,
        modified_badlands_plateau as libc::c_int,
        badlands_plateau as libc::c_int,
    );
    setBiomeColor(
        biomeColors,
        bamboo_jungle as libc::c_int,
        132 as libc::c_int as libc::c_uchar,
        149 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        bamboo_jungle_hills as libc::c_int,
        92 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        soul_sand_valley as libc::c_int,
        77 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        crimson_forest as libc::c_int,
        152 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        warped_forest as libc::c_int,
        73 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        basalt_deltas as libc::c_int,
        100 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
        99 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        dripstone_caves as libc::c_int,
        78 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        lush_caves as libc::c_int,
        40 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        meadow as libc::c_int,
        96 as libc::c_int as libc::c_uchar,
        164 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        grove as libc::c_int,
        71 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        snowy_slopes as libc::c_int,
        196 as libc::c_int as libc::c_uchar,
        196 as libc::c_int as libc::c_uchar,
        196 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        stony_peaks as libc::c_int,
        123 as libc::c_int as libc::c_uchar,
        143 as libc::c_int as libc::c_uchar,
        116 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        jagged_peaks as libc::c_int,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        200 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        frozen_peaks as libc::c_int,
        176 as libc::c_int as libc::c_uchar,
        179 as libc::c_int as libc::c_uchar,
        206 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        deep_dark as libc::c_int,
        3 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        mangrove_swamp as libc::c_int,
        44 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        142 as libc::c_int as libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn initBiomeTypeColors(mut biomeColors: *mut [libc::c_uchar; 3]) {
    memset(
        biomeColors as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int * 3 as libc::c_int) as libc::c_ulong,
    );
    setBiomeColor(
        biomeColors,
        Oceanic as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        Warm as libc::c_int,
        0xff as libc::c_int as libc::c_uchar,
        0xc0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        Lush as libc::c_int,
        0 as libc::c_int as libc::c_uchar,
        0xa0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        Cold as libc::c_int,
        0x60 as libc::c_int as libc::c_uchar,
        0x60 as libc::c_int as libc::c_uchar,
        0x60 as libc::c_int as libc::c_uchar,
    );
    setBiomeColor(
        biomeColors,
        Freezing as libc::c_int,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
        0xff as libc::c_int as libc::c_uchar,
    );
}
unsafe extern "C" fn _str2id(mut s: *const libc::c_char) -> libc::c_int {
    if *s as libc::c_int == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut id: libc::c_int = 0;
    id = 0 as libc::c_int;
    while id < 256 as libc::c_int {
        let mut p: *const libc::c_char = biome2str(MC_NEWEST as libc::c_int, id);
        if !p.is_null() && (f.is_null() || strlen(f) < strlen(p)) {
            if !(strstr(s, p)).is_null() {
                f = p;
                ret = id;
            }
        }
        let mut t: *const libc::c_char = biome2str(MC_1_17 as libc::c_int, id);
        if !t.is_null() && t != p && (f.is_null() || strlen(f) < strlen(t)) {
            if !(strstr(s, t)).is_null() {
                f = t;
                ret = id;
            }
        }
        id += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn parseBiomeColors(
    mut biomeColors: *mut [libc::c_uchar; 3],
    mut buf: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = buf;
    let mut bstr: [libc::c_char; 64] = [0; 64];
    let mut id: libc::c_int = 0;
    let mut col: [libc::c_int; 4] = [0; 4];
    let mut n: libc::c_int = 0;
    let mut ib: libc::c_int = 0;
    let mut ic: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *p != 0 {
        ic = 0 as libc::c_int;
        ib = ic;
        while *p as libc::c_int != 0
            && *p as libc::c_int != '\n' as i32
            && *p as libc::c_int != ';' as i32
        {
            if (ib as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong)
                < ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
            {
                if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
                    || *p as libc::c_int == '_' as i32
                {
                    let fresh0 = ib;
                    ib = ib + 1;
                    bstr[fresh0 as usize] = *p;
                } else if *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32 {
                    let fresh1 = ib;
                    ib = ib + 1;
                    bstr[fresh1 as usize] =
                        (*p as libc::c_int - 'A' as i32 + 'a' as i32) as libc::c_char;
                }
            }
            if ic < 4 as libc::c_int
                && (*p as libc::c_int == '#' as i32
                    || *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32)
            {
                let fresh2 = ic;
                ic = ic + 1;
                col[fresh2 as usize] = strtol(
                    p.offset(1 as libc::c_int as isize)
                        .offset((*p as libc::c_int == '0' as i32) as libc::c_int as isize),
                    &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                    16 as libc::c_int,
                ) as libc::c_int;
            } else if ic < 4 as libc::c_int
                && *p as libc::c_int >= '0' as i32
                && *p as libc::c_int <= '9' as i32
            {
                let fresh3 = ic;
                ic = ic + 1;
                col[fresh3 as usize] = strtol(
                    p,
                    &mut p as *mut *const libc::c_char as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
            }
            if *p as libc::c_int == '\n' as i32 || *p as libc::c_int == ';' as i32 {
                break;
            }
            p = p.offset(1);
        }
        while *p as libc::c_int != 0 && *p as libc::c_int != '\n' as i32 {
            p = p.offset(1);
        }
        while *p as libc::c_int == '\n' as i32 {
            p = p.offset(1);
        }
        bstr[ib as usize] = 0 as libc::c_int as libc::c_char;
        id = _str2id(bstr.as_mut_ptr());
        if id >= 0 as libc::c_int && id < 256 as libc::c_int {
            if ic == 3 as libc::c_int {
                (*biomeColors.offset(id as isize))[0 as libc::c_int as usize] =
                    (col[0 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
                (*biomeColors.offset(id as isize))[1 as libc::c_int as usize] =
                    (col[1 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
                (*biomeColors.offset(id as isize))[2 as libc::c_int as usize] =
                    (col[2 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
                n += 1;
            } else if ic == 1 as libc::c_int {
                (*biomeColors.offset(id as isize))[0 as libc::c_int as usize] =
                    (col[0 as libc::c_int as usize] >> 16 as libc::c_int & 0xff as libc::c_int)
                        as libc::c_uchar;
                (*biomeColors.offset(id as isize))[1 as libc::c_int as usize] =
                    (col[0 as libc::c_int as usize] >> 8 as libc::c_int & 0xff as libc::c_int)
                        as libc::c_uchar;
                (*biomeColors.offset(id as isize))[2 as libc::c_int as usize] =
                    (col[0 as libc::c_int as usize] >> 0 as libc::c_int & 0xff as libc::c_int)
                        as libc::c_uchar;
                n += 1;
            }
        } else if ic == 4 as libc::c_int {
            id = col[0 as libc::c_int as usize] & 0xff as libc::c_int;
            (*biomeColors.offset(id as isize))[0 as libc::c_int as usize] =
                (col[1 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
            (*biomeColors.offset(id as isize))[1 as libc::c_int as usize] =
                (col[2 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
            (*biomeColors.offset(id as isize))[2 as libc::c_int as usize] =
                (col[3 as libc::c_int as usize] & 0xff as libc::c_int) as libc::c_uchar;
            n += 1;
        } else if ic == 2 as libc::c_int {
            id = col[0 as libc::c_int as usize] & 0xff as libc::c_int;
            (*biomeColors.offset(id as isize))[0 as libc::c_int as usize] =
                (col[1 as libc::c_int as usize] >> 16 as libc::c_int & 0xff as libc::c_int)
                    as libc::c_uchar;
            (*biomeColors.offset(id as isize))[1 as libc::c_int as usize] =
                (col[1 as libc::c_int as usize] >> 8 as libc::c_int & 0xff as libc::c_int)
                    as libc::c_uchar;
            (*biomeColors.offset(id as isize))[2 as libc::c_int as usize] =
                (col[1 as libc::c_int as usize] >> 0 as libc::c_int & 0xff as libc::c_int)
                    as libc::c_uchar;
            n += 1;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn biomesToImage(
    mut pixels: *mut libc::c_uchar,
    mut biomeColors: *mut [libc::c_uchar; 3],
    mut biomes: *const libc::c_int,
    sx: libc::c_uint,
    sy: libc::c_uint,
    pixscale: libc::c_uint,
    flip: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut containsInvalidBiomes: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int as libc::c_uint;
    while j < sy {
        i = 0 as libc::c_int as libc::c_uint;
        while i < sx {
            let mut id: libc::c_int = *biomes.offset(j.wrapping_mul(sx).wrapping_add(i) as isize);
            let mut r: libc::c_uint = 0;
            let mut g: libc::c_uint = 0;
            let mut b: libc::c_uint = 0;
            if id < 0 as libc::c_int || id >= 256 as libc::c_int {
                containsInvalidBiomes = 1 as libc::c_int;
                r = ((*biomeColors.offset((id & 0x7f as libc::c_int) as isize))
                    [0 as libc::c_int as usize] as libc::c_int
                    - 40 as libc::c_int) as libc::c_uint;
                r = if r > 0xff as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    r & 0xff as libc::c_int as libc::c_uint
                };
                g = ((*biomeColors.offset((id & 0x7f as libc::c_int) as isize))
                    [1 as libc::c_int as usize] as libc::c_int
                    - 40 as libc::c_int) as libc::c_uint;
                g = if g > 0xff as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    g & 0xff as libc::c_int as libc::c_uint
                };
                b = ((*biomeColors.offset((id & 0x7f as libc::c_int) as isize))
                    [2 as libc::c_int as usize] as libc::c_int
                    - 40 as libc::c_int) as libc::c_uint;
                b = if b > 0xff as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    b & 0xff as libc::c_int as libc::c_uint
                };
            } else {
                r = (*biomeColors.offset(id as isize))[0 as libc::c_int as usize] as libc::c_uint;
                g = (*biomeColors.offset(id as isize))[1 as libc::c_int as usize] as libc::c_uint;
                b = (*biomeColors.offset(id as isize))[2 as libc::c_int as usize] as libc::c_uint;
            }
            let mut m: libc::c_uint = 0;
            let mut n: libc::c_uint = 0;
            m = 0 as libc::c_int as libc::c_uint;
            while m < pixscale {
                n = 0 as libc::c_int as libc::c_uint;
                while n < pixscale {
                    let mut idx: libc::c_int =
                        pixscale.wrapping_mul(i).wrapping_add(n) as libc::c_int;
                    if flip != 0 {
                        idx = (idx as libc::c_uint).wrapping_add(
                            sx.wrapping_mul(pixscale)
                                .wrapping_mul(pixscale.wrapping_mul(j).wrapping_add(m)),
                        ) as libc::c_int as libc::c_int;
                    } else {
                        idx = (idx as libc::c_uint).wrapping_add(
                            sx.wrapping_mul(pixscale).wrapping_mul(
                                pixscale
                                    .wrapping_mul(
                                        sy.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            .wrapping_sub(j),
                                    )
                                    .wrapping_add(m),
                            ),
                        ) as libc::c_int as libc::c_int;
                    }
                    let mut pix: *mut libc::c_uchar =
                        pixels.offset((3 as libc::c_int * idx) as isize);
                    *pix.offset(0 as libc::c_int as isize) = r as libc::c_uchar;
                    *pix.offset(1 as libc::c_int as isize) = g as libc::c_uchar;
                    *pix.offset(2 as libc::c_int as isize) = b as libc::c_uchar;
                    n = n.wrapping_add(1);
                }
                m = m.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    return containsInvalidBiomes;
}
#[no_mangle]
pub unsafe extern "C" fn savePPM(
    mut path: *const libc::c_char,
    mut pixels: *const libc::c_uchar,
    sx: libc::c_uint,
    sy: libc::c_uint,
) -> libc::c_int {
    let mut fp: *mut FILE = fopen(path, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    fprintf(
        fp,
        b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char,
        sx,
        sy,
    );
    let mut pixelsLen: size_t = (3 as libc::c_int as libc::c_uint)
        .wrapping_mul(sx)
        .wrapping_mul(sy) as size_t;
    let mut written: size_t = fwrite(
        pixels as *const libc::c_void,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        pixelsLen,
        fp,
    );
    fclose(fp);
    return (written != pixelsLen) as libc::c_int;
}
