extern crate jni;
extern crate ndk;
#[macro_use] extern crate log;
extern crate android_logger;

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use android_logger::{Config, FilterBuilder};

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use log::Level;
use ndk::trace;
use ndk::event::{InputEvent, Keycode};
use ndk::looper::{FdEvent, Poll, ThreadLooper};
use std::os::unix::prelude::RawFd;
use std::time::Duration;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
    let s = CString::new("Hello from Rust").unwrap();
    unsafe { callback(s.as_ptr()); }
}

const FD:i32 = 2;
const U32_SIZE:usize = std::mem::size_of::<u32>();

#[no_mangle]
#[allow(non_snake_case)]
#[link(name = "log")]
pub extern "C" fn Java_com_example_myapplication_MainActivity_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    android_logger::init_once(
        Config::default().with_min_level(Level::Debug).with_tag("myrust")
    );
    info!("i am invokeCallbackViaJNI");
    let s = String::from("Hello from Rust");
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");
    env.call_method(callback, "callback", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("myrust").unwrap();
    }
    //looper_test();
}

fn looper_test(){
    let looper =
        ThreadLooper::for_thread().expect("no looper for this thread");
    let mut fds = MaybeUninit::<[RawFd; 2]>::uninit();
    unsafe {
        let res = libc::pipe(fds.as_mut_ptr().cast());
        if res != 0 {
            error!("can not create pipes");
            panic!("can not create pipes");
        }
        fds.assume_init();
        let fds = fds.as_mut_ptr().as_mut().unwrap();
        looper.as_foreign().add_fd_with_callback(fds[0], FD, FdEvent::INPUT, Box::new(|fd| {
            unsafe {
                let mut recv = !0u32;
                let res = libc::read(fd, &mut recv as *mut _ as *mut _, U32_SIZE);
                if res < 0 {
                    error!("write error");
                    panic!("write error");
                }
            }
            true
        }))
            .expect("fail to add read fd");
    }
    std::thread::spawn(move || {
        unsafe{
            let fds = fds.as_mut_ptr().as_mut().unwrap();
            // Send a "custom event" to the looper every second
            for i in 0..5 {
                let i_addr = &i as *const _ as *const _;
                std::thread::sleep(Duration::from_secs(1));
                let res = libc::write(fds[1], i_addr, U32_SIZE);
                if res < 0 {
                    error!("write error");
                    panic!("write error");
                }
            }
        }
    });
    let mut can_exit = false;
    while !can_exit{
        match looper.poll_all().unwrap(){
            Poll::Wake=>{

            }
            Poll::Timeout=>{
                unreachable!()
            }
            Poll::Callback=>{
                unreachable!()
            }
            Poll::Event{ ident, fd, events, data }=> {
                match ident{
                    FD=> unsafe {
                        let mut recv = !0u32;
                        let res = libc::read(fd, &mut recv as *mut _ as *mut _, U32_SIZE);
                        if res < 0 {
                            error!("write error");
                            panic!("write error");
                        }
                    }
                    i => {
                        panic!("Unexpected event identifier {}", i);
                    }
                }
            }
        }
    }
}