package com.example.myapplication

class MyGL {
    companion object{
        init {
            System.loadLibrary("postfx")
        }

        external fun init(config:GLConfig)
    }

}