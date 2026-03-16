package com.ferrisy.ferry

import android.content.Context
import android.content.Intent
import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    registerAppContext(applicationContext)
  }

  /** 由 Rust 调用来注册 Context，供启动/停止前台服务使用 */
  companion object {
    @JvmStatic
    external fun registerAppContext(context: Context)
  }
}
