use robusta_jni::bridge;

#[bridge]
mod jni {
    use robusta_jni::convert::{FromJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::JNIEnv;
    use robusta_jni::jni::objects::{JObject, JValue};
    use jni::objects::JByteArray;

    #[derive(Signature)]
    #[package(de.jpx3.classloader)]
    pub struct ClassLoader {}

    impl<'env> ClassLoader {
        pub extern "jni" fn classLoaded0(
            env: &JNIEnv<'env>,
            name: JObject<'env>
        ) -> bool {
            let this_class = env.find_class("de/jpx3/classloader/ClassLoader")
                .expect("Intave ClassLoader class not found");

            if name.is_null() {
                env.throw_new("java/lang/NullPointerException", "Specified class name is null").unwrap();
                return false;
            }

            let class_loader = env.call_method(
                this_class,
                "getClassLoader",
                "()Ljava/lang/ClassLoader;",
                &[]
            ).expect("Failed to get java class loader from class").l().unwrap();

            let result = env.call_method(
                class_loader,
                "findLoadedClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::from(name)]
            ).expect("findLoadedClass call failed");

            !result.l().expect("Class loader returned unknown object").is_null()
        }

        pub extern "jni" fn classLoad0(
            env: &JNIEnv<'env>,
            byte_array: JObject<'env>,
        ) -> JObject<'env> {
            if byte_array.is_null() {
                env.throw_new("java/lang/NullPointerException", "Specified byte array is null").unwrap();
                return JObject::null();
            }

            let this_class = env.find_class("de/jpx3/classloader/ClassLoader")
                .expect("Intave ClassLoader class not found");

            let class_loader = env.call_method(
                this_class,
                "getClassLoader",
                "()Ljava/lang/ClassLoader;",
                &[]
            ).expect("Failed to get java class loader").l().unwrap();

            let array_length = env.call_static_method(
                "java/lang/reflect/Array",
                "getLength",
                "(Ljava/lang/Object;)I",
                &[JValue::Object(byte_array)]
            ).expect("Failed to get array length").i().unwrap();

            let result = env.call_method(
                class_loader,
                "defineClass",
                "([BII)Ljava/lang/Class;",
                &[
                    JValue::Object(byte_array),
                    JValue::Int(0),
                    JValue::Int(array_length),
                ]
            ).expect("defineClass call failed");

            result.l().expect("defineClass returned unknown object")
        }
    }
}