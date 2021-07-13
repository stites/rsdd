#[macro_use] mod util;
pub mod manager;
pub mod repr;
mod backing_store;

extern crate rand;
extern crate fnv;
extern crate twox_hash;
extern crate quickersort;
extern crate dimacs;
extern crate pretty;
extern crate num;
extern crate maplit;
extern crate libc;

use manager::*;
use manager::bdd_manager::BddManager;
use repr::bdd::BddPtr;

#[no_mangle]
pub extern "C" fn rsdd_mk_bdd_manager_default_order(numvars: usize) -> *mut libc::c_void {
    let r = Box::new(bdd_manager::BddManager::new_default_order(numvars));
    Box::into_raw(r) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn rsdd_new_var(mgr: *mut BddManager) -> *mut libc::c_void {
    let mgr = unsafe { &mut *mgr };
    let r = Box::new(bdd_manager::BddManager::new_var(mgr));
    Box::into_raw(r) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn rsdd_var(mgr: *mut BddManager) -> *mut libc::c_void {
    let mgr = unsafe { &mut *mgr };
    let r = Box::new(BddManager::new_var(mgr));
    Box::into_raw(r) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn rsdd_and(mgr: *mut BddManager, a: u64, b: u64) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.and(BddPtr::from_raw(a), BddPtr::from_raw(b)).raw()
}

#[no_mangle]
pub extern "C" fn rsdd_or(mgr: *mut BddManager, a: u64, b: u64) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.or(BddPtr::from_raw(a), BddPtr::from_raw(b)).raw()
}

#[no_mangle]
pub extern "C" fn rsdd_iff(mgr: *mut BddManager, a: u64, b: u64) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.iff(BddPtr::from_raw(a), BddPtr::from_raw(b)).raw()
}

#[no_mangle]
pub extern "C" fn rsdd_true(mgr: *mut BddManager) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.true_ptr().raw()
}

#[no_mangle]
pub extern "C" fn rsdd_exists(mgr: *mut BddManager, bdd: u64, lbl: u64) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.exists(BddPtr::from_raw(bdd), repr::var_label::VarLabel::new(lbl)).raw()
}

#[no_mangle]
pub extern "C" fn rsdd_condition(mgr: *mut BddManager, bdd: u64, lbl: u64, value: bool) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.condition(BddPtr::from_raw(bdd), repr::var_label::VarLabel::new(lbl), value).raw()
}

#[no_mangle]
pub extern "C" fn rsdd_size(mgr: *mut BddManager, bdd: u64) -> usize {
    let mgr = unsafe { &mut *mgr };
    mgr.count_nodes(BddPtr::from_raw(bdd))
}

#[no_mangle]
pub extern "C" fn rsdd_false(mgr: *mut BddManager) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.false_ptr().raw()
}

#[no_mangle]
pub extern "C" fn rsdd_is_false(mgr: *mut BddManager, ptr: u64) -> bool {
    let mgr = unsafe { &mut *mgr };
    mgr.is_false(BddPtr::from_raw(ptr))
}

#[no_mangle]
pub extern "C" fn rsdd_is_true(mgr: *mut BddManager, ptr: u64) -> bool {
    let mgr = unsafe { &mut *mgr };
    mgr.is_true(BddPtr::from_raw(ptr))
}

#[no_mangle]
pub extern "C" fn rsdd_negate(mgr: *mut BddManager, ptr: u64) -> u64 {
    let mgr = unsafe { &mut *mgr };
    mgr.negate(BddPtr::from_raw(ptr)).raw()
}