use std::collections::LinkedList;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::vec::Vec;

type GReference = Arc<GReferenceValue>;

struct GObject {
    // psi element type
    dynamic_ivars: LinkedList<i64>, // change to pointer type later
    ivars_count: u8,
    ivars: [i64], // change to pointer type later
}

struct GUnmanagedUnsafe {
    pointer: *mut u8,
    deallocator: fn(*mut u8) -> (), 
}

enum GReferenceValue {
    Object(GObject),
    UnmanagedUnsafe(GUnmanagedUnsafe),
    Integer64(i64),
    Integer64Unsigned(u64),
}
