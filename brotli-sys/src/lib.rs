#![allow(bad_style)]

extern crate libc;

use libc::{c_void, size_t, c_int};

#[cfg(target_env = "msvc")]
#[doc(hidden)]
pub type __enum_ty = libc::c_int;
#[cfg(not(target_env = "msvc"))]
#[doc(hidden)]
pub type __enum_ty = libc::c_uint;

pub type brotli_alloc_func = extern fn(*mut c_void, size_t) -> *mut c_void;
pub type brotli_free_func = extern fn(*mut c_void, *mut c_void);

// ========== Decoder functionality ==========

pub type BrotliResult = __enum_ty;
pub type BrotliRunningState = __enum_ty;
pub type BrotliRunningMetablockHeaderState = __enum_ty;
pub type BrotliRunningUncompressedState = __enum_ty;
pub type BrotliRunningTreeGroupState = __enum_ty;
pub type BrotliRunningContextMapState = __enum_ty;
pub type BrotliRunningHuffmanState = __enum_ty;
pub type BrotliRunningDecodeUint8State = __enum_ty;
pub type BrotliRunningReadBlockLengthState = __enum_ty;

pub type BrotliState = BrotliStateStruct;

#[repr(C)]
pub struct BrotliStateStruct {
    pub state: BrotliRunningState,
    pub loop_counter: c_int,
    pub br: BrotliBitReader,
    pub alloc_func: Option<brotli_alloc_func>,
    pub free_func: Option<brotli_free_func>,
    pub memory_manager_opaque: *mut c_void,
    buffer: u64,
    pub buffer_length: u32,
    pub pos: c_int,
    pub max_backward_distance: c_int,
    pub max_backward_distance_minus_custom_dict_size: c_int,
    pub max_distance: c_int,
    pub ringbuffer_size: c_int,
    pub ringbuffer_mask: c_int,
    pub dist_rb_idx: c_int,
    pub dist_rb: [c_int; 4],
    pub error_code: c_int,
    pub sub_loop_counter: u32,
    pub ringbuffer: *mut u8,
    pub ringbuffer_end: *mut u8,
    pub htree_command: *mut HuffmanCode,
    pub context_lookup1: *const u8,
    pub context_lookup2: *const u8,
    pub context_map_slice: *mut u8,
    pub dist_context_map_slice: *mut u8,
    pub literal_hgroup: HuffmanTreeGroup,
    pub insert_copy_hgroup: HuffmanTreeGroup,
    pub distance_hgroup: HuffmanTreeGroup,
    pub block_type_trees: *mut HuffmanCode,
    pub block_len_trees: *mut HuffmanCode,
    pub trivial_literal_context: c_int,
    pub distance_context: c_int,
    pub meta_block_remaining_len: c_int,
    pub block_length_index: u32,
    pub block_length: [u32; 3],
    pub num_block_types: [u32; 3],
    pub block_type_rb: [u32; 6],
    pub distance_postfix_bits: u32,
    pub num_direct_distance_codes: u32,
    pub distance_postfix_mask: c_int,
    pub num_dist_htrees: u32,
    pub dist_context_map: *mut u8,
    pub literal_htree: *mut HuffmanCode,
    pub dist_htree_index: u8,
    pub repeat_code_len: u32,
    pub prev_code_len: u32,
    pub copy_length: c_int,
    pub distance_code: c_int,
    pub rb_roundtrips: size_t,
    pub partial_pos_out: size_t,
    pub symbol: u32,
    pub repeat: u32,
    pub space: u32,
    pub table: [HuffmanCode; 32],
    pub symbol_lists: *mut u16,
    pub symbols_lists_array: [u16; 720],
    pub next_symbol: [c_int; 32],
    pub code_length_code_lengths: [u8; 18],
    pub code_length_histo: [u16; 16],
    pub htree_index: c_int,
    pub next: *mut HuffmanCode,
    pub context_index: u32,
    pub max_run_length_prefix: u32,
    pub code: u32,
    pub context_map_table: [HuffmanCode; 646],
    pub mtf_upper_bound: u32,
    pub mtf: [u8; 260],
    pub custom_dict: *const u8,
    pub custom_dict_size: c_int,
    pub substate_metablock_header: BrotliRunningMetablockHeaderState,
    pub substate_tree_group: BrotliRunningTreeGroupState,
    pub substate_context_map: BrotliRunningContextMapState,
    pub substate_uncompressed: BrotliRunningUncompressedState,
    pub substate_huffman: BrotliRunningHuffmanState,
    pub substate_decode_uint8: BrotliRunningDecodeUint8State,
    pub substate_read_block_length: BrotliRunningReadBlockLengthState,
    pub is_last_metablock: u8,
    pub is_uncompressed: u8,
    pub is_metadata: u8,
    pub size_nibbles: u8,
    pub window_bits: u32,
    pub num_literal_htrees: u32,
    pub context_map: *mut u8,
    pub context_modes: *mut u8,
    pub trivial_literal_contexts: [u32; 8],
}

#[cfg(target_pointer_width = "32")]
pub type reg_t = u32;
#[cfg(target_pointer_width = "64")]
pub type reg_t = u64;

#[repr(C)]
pub struct BrotliBitReader {
    pub val_: reg_t,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: size_t,
}

#[repr(C)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}

#[repr(C)]
pub struct HuffmanTreeGroup {
    pub htrees: *mut *mut HuffmanCode,
    pub codes: *mut HuffmanCode,
    pub alphabet_size: u16,
    pub num_htrees: u16,
}

pub const BROTLI_RESULT_ERROR: BrotliResult = 0;
pub const BROTLI_RESULT_SUCCESS: BrotliResult = 1;
pub const BROTLI_RESULT_NEEDS_MORE_INPUT: BrotliResult = 2;
pub const BROTLI_RESULT_NEEDS_MORE_OUTPUT: BrotliResult = 3;

extern {
    // BrotliState
    pub fn BrotliCreateState(alloc_func: Option<brotli_alloc_func>,
                             free_func: Option<brotli_free_func>,
                             opaque: *mut c_void) -> *mut BrotliState;
    pub fn BrotliDestroyState(state: *mut BrotliState);
    pub fn BrotliDecompressedSize(encoded_size: size_t,
                                  encoded_buff: *const u8,
                                  decoded_size: *mut size_t) -> c_int;
    pub fn BrotliDecompressBuffer(encoded_size: size_t,
                                  encoded_buffer: *const u8,
                                  decoded_size: *mut size_t,
                                  decoded_buffer: *mut u8) -> BrotliResult;
    pub fn BrotliDecompressStream(available_in: *mut size_t,
                                  next_in: *mut *const u8,
                                  available_out: *mut size_t,
                                  next_out: *mut *mut u8,
                                  total_out: *mut size_t,
                                  s: *mut BrotliState) -> BrotliResult;
    pub fn BrotliSetCustomDictionary(size: size_t,
                                     dict: *const u8,
                                     s: *mut BrotliState);

    // raw state
    pub fn BrotliStateInit(s: *mut BrotliState);
    pub fn BrotliStateInitWithCustomAllocators(s: *mut BrotliState,
                                               alloc_func: brotli_alloc_func,
                                               free_func: brotli_free_func,
                                               opaque: *mut c_void);
    pub fn BrotliStateCleanup(s: *mut BrotliState);
    pub fn BrotliStateMetablockBegin(s: *mut BrotliState);
    pub fn BrotliStateCleanupAfterMetablock(s: *mut BrotliState);
    pub fn BrotliHuffmanTreeGroupInit(s: *mut BrotliState,
                                      group: *mut HuffmanTreeGroup,
                                      alphabet_size: u32,
                                      ntrees: u32);
    pub fn BrotliHuffmanTreeGroupRelease(s: *mut BrotliState,
                                         group: *mut HuffmanTreeGroup);
    pub fn BrotliStateIsStreamStart(s: *const BrotliState) -> c_int;
    pub fn BrotliStateIsStreamEnd(s: *const BrotliState) -> c_int;

    // huffman
    pub fn BrotliBuildCodeLengthsHuffmanTable(root_table: *mut HuffmanCode,
                                              code_lengths: *const u8,
                                              count: *mut u16);
    pub fn BrotliBuildHuffmanTable(root_table: *mut HuffmanCode,
                                   root_bits: c_int,
                                   symbol_lists: *const u16,
                                   count_arg: *mut u16) -> u32;
    pub fn BrotliBuildSimpleHuffmanTable(table: *mut HuffmanCode,
                                         root_bits: c_int,
                                         symbols: *mut u16,
                                         num_symbols: u32) -> u32;
}



// ========== Encoder functionality ==========

pub type BrotliEncoderMode = __enum_ty;
pub type BrotliEncoderParameter = __enum_ty;
pub type BrotliEncoderOperation = __enum_ty;
pub type BrotliEncoderStreamState = __enum_ty;

pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;

pub const BROTLI_PARAM_MODE: BrotliEncoderParameter = 0;
pub const BROTLI_PARAM_QUALITY: BrotliEncoderParameter = 1;
pub const BROTLI_PARAM_LGWIN: BrotliEncoderParameter = 2;
pub const BROTLI_PARAM_LGBLOCK: BrotliEncoderParameter = 3;

pub const BROTLI_OPERATION_PROCESS: BrotliEncoderOperation = 0;
pub const BROTLI_OPERATION_FLUSH: BrotliEncoderOperation = 1;
pub const BROTLI_OPERATION_FINISH: BrotliEncoderOperation = 2;

pub const BROTLI_DEFAULT_QUALITY: u32 = 11;
pub const BROTLI_DEFAULT_WINDOW: u32 = 22;

pub type BrotliEncoderState = BrotliEncoderStateStruct;

#[repr(C)]
pub struct BrotliEncoderStateStruct {
    pub mode: BrotliEncoderMode,
    pub quality: c_int,
    pub lgwin: c_int,
    pub lgblock: c_int,
    pub memory_manager_: MemoryManager,
    pub hashers_: Hashers,
    pub hash_type_: c_int,
    pub input_pos_: u64,
    pub ringbuffer_: RingBuffer,
    pub cmd_alloc_size_: size_t,
    pub commands_: *mut Command,
    pub num_commands_: size_t,
    pub num_literals_: size_t,
    pub last_insert_len_: size_t,
    pub last_flush_pos_: u64,
    pub last_processed_pos_: u64,
    pub dist_cache_: [c_int; 4],
    pub saved_dist_cache_: [c_int; 4],
    pub last_byte_: u8,
    pub last_byte_bits_: u8,
    pub prev_byte_: u8,
    pub prev_byte2_: u8,
    pub storage_size_: size_t,
    pub storage_: *mut u8,
    pub small_table_: [c_int; 1024],  // 1 << 10
    pub large_table_: *mut c_int,
    pub large_table_size_: size_t,
    pub cmd_depths_: [u8; 128],
    pub cmd_bits_: [u16; 128],
    pub cmd_code_: [u8; 512],
    pub cmd_code_numbits_: size_t,
    pub command_buf_: *mut u32,
    pub literal_buf_: *mut u8,
    pub next_out_: *mut u8,
    pub available_out_: size_t,
    pub total_out_: size_t,
    pub flush_buf_: [u8; 2],
    pub stream_state_: BrotliEncoderStreamState,
    pub is_last_block_emitted_: c_int,
    pub is_initialized_: c_int,
}

#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: Option<brotli_alloc_func>,
    pub free_func: Option<brotli_free_func>,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct Hashers {
    pub hash_h2: *mut H2,
    pub hash_h3: *mut H3,
    pub hash_h4: *mut H4,
    pub hash_h5: *mut H5,
    pub hash_h6: *mut H6,
    pub hash_h7: *mut H7,
    pub hash_h8: *mut H8,
    pub hash_h9: *mut H9,
    pub hash_h10: *mut H10,
}

#[repr(C)]
pub struct H2 {
    pub buckets_: [u32; 65537],  // (1 << 16) + 1
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H3 {
    pub buckets_: [u32; 65538],  // (1 << 16) + 2
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H4 {
    pub buckets_: [u32; 131076],  // (1 << 17) + 4
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H5 {
    pub num_: [u16; 16384],  // 1 << 14
    pub buckets_: [u32; 262144],  // (1 << 4) << 14
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H6 {
    pub num_: [u16; 16384],  // 1 << 14
    pub buckets_: [u32; 524288],  // (1 << 5) << 14
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H7 {
    pub num_: [u16; 32768],  // 1 << 15
    pub buckets_: [u32; 2097152],  // (1 << 6) << 15
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H8 {
    pub num_: [u16; 32768],  // 1 << 15
    pub buckets_: [u32; 4194304],  // (1 << 7) << 15
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H9 {
    pub num_: [u16; 32768],  // 1 << 15
    pub buckets_: [u32; 8388608],  // (1 << 8) << 15
    pub is_dirty_: c_int,
    pub num_dict_lookups_: size_t,
    pub num_dict_matches_: size_t,
}

#[repr(C)]
pub struct H10 {
    pub window_mask_: size_t,
    pub buckets_: [u32; 131072],  // 1 << 17
    pub forest_: *mut u32,
    pub invalid_pos_: u32,
    pub is_dirty_: c_int,
}

#[repr(C)]
pub struct RingBuffer {
    pub size_: u32,
    pub mask_: u32,
    pub tail_size_: u32,
    pub total_size_: u32,
    pub cur_size_: u32,
    pub pos_: u32,
    pub data_: *mut u8,
    pub buffer_: *mut u8,
}

#[repr(C)]
pub struct Command {
    pub insert_len_: u32,
    pub copy_len_: u32,
    pub dist_extra_: u32,
    pub cmd_prefix_: u16,
    pub dist_prefix_: u16,
}

extern {
    pub fn BrotliEncoderSetParameter(state: *mut BrotliEncoderState,
                                     p: BrotliEncoderParameter,
                                     value: u32)
                                     -> c_int;
    pub fn BrotliEncoderCreateInstance(alloc_func: Option<brotli_alloc_func>,
                                       free_func: Option<brotli_free_func>,
                                       opaque: *mut c_void)
                                       -> *mut BrotliEncoderState;
    pub fn BrotliEncoderDestroyInstance(state: *mut BrotliEncoderState);
    pub fn BrotliEncoderInputBlockSize(state: *mut BrotliEncoderState) -> size_t;
    pub fn BrotliEncoderWriteMetaBlock(state: *mut BrotliEncoderState,
                                       input_size: size_t,
                                       input_buffer: *const u8,
                                       is_last: c_int,
                                       encoded_size: *mut size_t,
                                       encoded_buffer: *mut u8)
                                       -> c_int;
    pub fn BrotliEncoderWriteMetadata(state: *mut BrotliEncoderState,
                                      input_size: size_t,
                                      input_buffer: *const u8,
                                      is_last: c_int,
                                      encoded_size: *mut size_t,
                                      encoded_buffer: *mut u8)
                                      -> c_int;
    pub fn BrotliEncoderFinishStream(state: *mut BrotliEncoderState,
                                     encoded_size: *mut size_t,
                                     encoded_buffer: *mut u8)
                                     -> c_int;
    pub fn BrotliEncoderCopyInputToRingBuffer(state: *mut BrotliEncoderState,
                                              input_size: size_t,
                                              input_buffer: *const u8);
    pub fn BrotliEncoderWriteData(state: *mut BrotliEncoderState,
                                  is_last: c_int,
                                  force_flush: c_int,
                                  out_size: *mut size_t,
                                  output: *mut *mut u8)
                                  -> c_int;
    pub fn BrotliEncoderSetCustomDictionary(state: *mut BrotliEncoderState,
                                            size: size_t,
                                            dict: *const u8);
    pub fn BrotliEncoderMaxCompressedSize(input_size: size_t) -> size_t;
    pub fn BrotliEncoderCompress(quality: c_int,
                                 lgwin: c_int,
                                 mode: BrotliEncoderMode,
                                 input_size: size_t,
                                 input_buffer: *const u8,
                                 encoded_size: *mut size_t,
                                 encoded_buffer: *mut u8)
                                 -> c_int;
    pub fn BrotliEncoderCompressStream(s: *mut BrotliEncoderState,
                                       op: BrotliEncoderOperation,
                                       available_in: *mut size_t,
                                       next_in: *mut *const u8,
                                       available_out: *mut size_t,
                                       next_out: *mut *mut u8,
                                       total_out: *mut size_t)
                                       -> c_int;
    pub fn BrotliEncoderIsFinished(s: *mut BrotliEncoderState) -> c_int;
    pub fn BrotliEncoderHasMoreOutput(s: *mut BrotliEncoderState) -> c_int;
}
