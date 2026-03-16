//! Android 前台服务：通过 JNI 从 Rust 启停，息屏/后台保持 FTP 运行

#[cfg(target_os = "android")]
use std::sync::Mutex;

#[cfg(target_os = "android")]
use jni::objects::{GlobalRef, JObject, JValue};
#[cfg(target_os = "android")]
use jni::JNIEnv;

#[cfg(target_os = "android")]
struct SendGlobalRef(GlobalRef);
#[cfg(target_os = "android")]
unsafe impl Send for SendGlobalRef {}

#[cfg(target_os = "android")]
static JAVA_VM: Mutex<Option<jni::JavaVM>> = Mutex::new(None);
#[cfg(target_os = "android")]
static APP_CONTEXT: Mutex<Option<SendGlobalRef>> = Mutex::new(None);

/// 由 MainActivity.onCreate 调用，注册 ApplicationContext 供后续启停前台服务
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_ferrisy_ferry_MainActivity_registerAppContext(
    env: JNIEnv,
    _class: jni::sys::jclass,
    context: jni::sys::jobject,
) {
    let vm = match env.get_java_vm() {
        Ok(v) => v,
        Err(e) => {
            log::error!("get_java_vm failed: {:?}", e);
            return;
        }
    };
    let ctx_obj = unsafe { JObject::from_raw(context) };
    let global_ctx = match env.new_global_ref(ctx_obj) {
        Ok(g) => g,
        Err(e) => {
            log::error!("new_global_ref failed: {:?}", e);
            return;
        }
    };
    if let Ok(mut v) = JAVA_VM.lock() {
        *v = Some(vm);
    }
    if let Ok(mut c) = APP_CONTEXT.lock() {
        *c = Some(SendGlobalRef(global_ctx));
    }
    log::debug!("AppContext registered for foreground service");
}

#[cfg(target_os = "android")]
fn with_context<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut JNIEnv, JObject) -> R,
{
    let binding = JAVA_VM.lock().ok()?;
    let vm = binding.as_ref()?;
    let ctx_guard = APP_CONTEXT.lock().ok()?;
    let send_ref = ctx_guard.as_ref()?;
    let mut env = vm.attach_current_thread_as_daemon().ok()?;
    let ctx = env.new_local_ref(send_ref.0.as_obj()).ok()?;
    Some(f(&mut env, ctx))
}

#[cfg(target_os = "android")]
fn start_foreground_service_impl(env: &mut JNIEnv, context: JObject) -> jni::errors::Result<()> {
    let intent_class = env.find_class("android/content/Intent")?;
    let service_class = env.find_class("com/ferrisy/ferry/FtpForegroundService")?;
    let intent = env.new_object(
        intent_class,
        "(Landroid/content/Context;Ljava/lang/Class;)V",
        &[JValue::Object(&context), JValue::Object(&service_class)],
    )?;
    env.call_method(
        &context,
        "startForegroundService",
        "(Landroid/content/Intent;)Landroid/content/ComponentName;",
        &[JValue::Object(&intent)],
    )?;
    Ok(())
}

#[cfg(target_os = "android")]
fn stop_foreground_service_impl(env: &mut JNIEnv, context: JObject) -> jni::errors::Result<()> {
    let intent_class = env.find_class("android/content/Intent")?;
    let service_class = env.find_class("com/ferrisy/ferry/FtpForegroundService")?;
    let intent = env.new_object(
        intent_class,
        "(Landroid/content/Context;Ljava/lang/Class;)V",
        &[JValue::Object(&context), JValue::Object(&service_class)],
    )?;
    env.call_method(
        &context,
        "stopService",
        "(Landroid/content/Intent;)Z",
        &[JValue::Object(&intent)],
    )?;
    Ok(())
}

/// 启动 FTP 前台服务（仅 Android，Rust 在 start_ftp 成功后调用）
#[cfg(target_os = "android")]
pub fn start_foreground_service() {
    if let Some(Some(()))= with_context(|env, ctx| start_foreground_service_impl(env, ctx).ok()) {
        log::info!("FTP foreground service started");
    } else {
        log::warn!("Could not start FTP foreground service (context not registered?)");
    }
}

/// 停止 FTP 前台服务（仅 Android，Rust 在 stop_ftp 中调用）
#[cfg(target_os = "android")]
pub fn stop_foreground_service() {
     if let Some(Some(())) = with_context(|env, ctx| stop_foreground_service_impl(env, ctx).ok()) {
        log::info!("FTP foreground service stopped");
    } else {
        log::warn!("Could not stop FTP foreground service");
    }
}

#[cfg(not(target_os = "android"))]
pub fn start_foreground_service() {}

#[cfg(not(target_os = "android"))]
pub fn stop_foreground_service() {}
