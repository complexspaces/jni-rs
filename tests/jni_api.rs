#![cfg(feature = "invocation")]

extern crate error_chain;
extern crate jni;

use jni::objects::{AutoLocal, JObject};

mod util;
use util::{attach_current_thread, unwrap};

static ARRAYLIST_CLASS: &str = "java/util/ArrayList";
static EXCEPTION_CLASS: &str = "java/lang/Exception";
static ARITHMETIC_EXCEPTION_CLASS: &str = "java/lang/ArithmeticException";

#[test]
pub fn is_instance_of_same_class() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(EXCEPTION_CLASS, "()V", &[])));
    assert!(unwrap(&env, env.is_instance_of(obj.as_obj(), EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_superclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[])));
    assert!(unwrap(&env, env.is_instance_of(obj.as_obj(), EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_subclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(EXCEPTION_CLASS, "()V", &[])));
    assert!(!unwrap(&env, env.is_instance_of(obj.as_obj(), ARITHMETIC_EXCEPTION_CLASS)));
}

#[test]
pub fn is_instance_of_not_superclass() {
    let env = attach_current_thread();
    let obj = AutoLocal::new(&env, unwrap(&env, env.new_object(ARITHMETIC_EXCEPTION_CLASS, "()V", &[])));
    assert!(!unwrap(&env, env.is_instance_of(obj.as_obj(), ARRAYLIST_CLASS)));
}

#[test]
pub fn is_instance_of_null() {
    let env = attach_current_thread();
    let obj = JObject::null();
    assert!(unwrap(&env, env.is_instance_of(obj, ARRAYLIST_CLASS)));
    assert!(unwrap(&env, env.is_instance_of(obj, EXCEPTION_CLASS)));
    assert!(unwrap(&env, env.is_instance_of(obj, ARITHMETIC_EXCEPTION_CLASS)));
}