use boa_engine::property::PropertyKey;
use boa_engine::{Context, JsObject, JsString, JsValue};

pub fn get_property(object: &JsObject, name: &str, context: &mut Context) -> String {
    let property = object
        .get(PropertyKey::String(JsString::from(name)), context)
        .map_err(|e| format!("Failed to get property: {}", e))
        .unwrap();
    let value = if property.is_string() {
        property
    } else {
        JsValue::from(JsString::from(""))
    };
    value.to_string(context).unwrap().to_std_string_escaped()
}

pub fn has_property(object: &JsObject, name: &str, context: &mut Context) -> bool {
    let property = object
        .get(PropertyKey::String(JsString::from(name)), context)
        .map_err(|e| format!("Failed to get property: {}", e))
        .unwrap();
    !property.is_null_or_undefined()
}
