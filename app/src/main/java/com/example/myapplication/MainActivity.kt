package com.example.myapplication

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.example.myapplication.ui.theme.MyApplicationTheme

class MainActivity : ComponentActivity() {
    interface JNICallback{
        fun callback(string:String);
    }

    companion object{
        init {
            System.loadLibrary("myrust")
        }
    }

    external fun invokeCallbackViaJNI(callback:JNICallback)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        MyGL.init(GLConfig(720,1280,true,4,1))
        invokeCallbackViaJNI(object : JNICallback{
            override fun callback(string: String) {
                Log.d("MainActivity","now rust says:"+string);
            }
        })
        setContent {
            MyApplicationTheme {
                // A surface container using the 'background' color from the theme
                Surface(color = MaterialTheme.colors.background) {
                    Column() {
                        Greeting("Android")
                    }

                }
            }
        }
    }
}

@Composable
fun Greeting(name: String) {
    Row() {
        Text(text = "Hello $name!")
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    MyApplicationTheme {
        Greeting("Android")
    }
}