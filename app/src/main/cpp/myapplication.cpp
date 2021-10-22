// Write C++ code here.
//
// Do not forget to dynamically load the C++ library into your application.
//
// For instance,
//
// In MainActivity.java:
//    static {
//       System.loadLibrary("myapplication");
//    }
//
// Or, in MainActivity.kt:
//    companion object {
//      init {
//         System.loadLibrary("myapplication")
//      }
//    }

#include "VideoRenderer.h"
#include <android/log.h>


extern "C"
JNIEXPORT void JNICALL
Java_com_example_myapplication_MyGL_00024Companion_init(JNIEnv *env, jobject thiz, jobject config) {
    // TODO: implement init()
    auto clazz = env->GetObjectClass(config);
    GLConfig glConfig = {};
    glConfig.width = env->GetIntField(config,env->GetFieldID(clazz,"width","I"));
    glConfig.height = env->GetIntField(config,env->GetFieldID(clazz,"height", "I"));
    glConfig.vsync = env->GetBooleanField(config,env->GetFieldID(clazz,"vsync", "Z"));
    glConfig.msaaCount = env->GetIntField(config,env->GetFieldID(clazz,"msaaCount", "I"));
    glConfig.msaaQuality = env->GetIntField(config,env->GetFieldID(clazz,"msaaQuality", "I"));
    __android_log_print(ANDROID_LOG_INFO,"postfx","GLConfig={width:%d,height:%d,vsync:%d,}",glConfig.width,glConfig.height,glConfig.vsync);
}