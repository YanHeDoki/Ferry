package com.ferrisy.ferry

import android.Manifest
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.os.Environment
import android.provider.Settings
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AlertDialog
import androidx.core.content.ContextCompat

class MainActivity : TauriActivity() {

  private val legacyStorageLauncher = registerForActivityResult(
    ActivityResultContracts.RequestMultiplePermissions()
  ) { results ->
    val allGranted = results.values.all { it }
    if (!allGranted) {
      showPermissionRationale()
    }
  }

  private val manageStorageLauncher = registerForActivityResult(
    ActivityResultContracts.StartActivityForResult()
  ) {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && !Environment.isExternalStorageManager()) {
      showPermissionRationale()
    }
  }

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    registerAppContext(applicationContext)
    requestStoragePermission()
  }

  private fun requestStoragePermission() {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      if (!Environment.isExternalStorageManager()) {
        AlertDialog.Builder(this)
          .setTitle("需要存储权限")
          .setMessage("Ferry FTP 服务器需要「所有文件访问」权限，才能上传、删除和移动文件。请在接下来的设置页面中开启。")
          .setPositiveButton("前往设置") { _, _ ->
            val intent = Intent(Settings.ACTION_MANAGE_APP_ALL_FILES_ACCESS_PERMISSION).apply {
              data = Uri.parse("package:$packageName")
            }
            manageStorageLauncher.launch(intent)
          }
          .setNegativeButton("暂不") { d, _ -> d.dismiss() }
          .setCancelable(false)
          .show()
      }
    } else {
      val read = ContextCompat.checkSelfPermission(this, Manifest.permission.READ_EXTERNAL_STORAGE)
      val write = ContextCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE)
      if (read != PackageManager.PERMISSION_GRANTED || write != PackageManager.PERMISSION_GRANTED) {
        legacyStorageLauncher.launch(
          arrayOf(
            Manifest.permission.READ_EXTERNAL_STORAGE,
            Manifest.permission.WRITE_EXTERNAL_STORAGE
          )
        )
      }
    }
  }

  private fun showPermissionRationale() {
    AlertDialog.Builder(this)
      .setTitle("权限未授予")
      .setMessage("未获得存储权限，FTP 文件上传/删除/移动功能将不可用。你可以稍后在系统设置中手动授予。")
      .setPositiveButton("确定") { d, _ -> d.dismiss() }
      .show()
  }

  companion object {
    @JvmStatic
    external fun registerAppContext(context: Context)
  }
}
