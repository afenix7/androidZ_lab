//
// Created by aphx1 on 2021/10/18.
//

#ifndef MY_APPLICATION_VIDEORENDERER_H
#define MY_APPLICATION_VIDEORENDERER_H

#include <jni.h>
#include <EGL/egl.h>
#if __ANDROID_API__ >= 24
#include <GLES3/gl32.h>
#elif __ANDROID_API__ >= 21
#include <GLES3/gl31.h>
#else
#include <GLES3/gl3.h>
#endif
#include <math.h>

struct GLConfig{
    int width;
    int height;
    bool vsync;
    int msaaCount;
    int msaaQuality;
};


class VideoRenderer {

};



#endif //MY_APPLICATION_VIDEORENDERER_H
