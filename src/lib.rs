
use neon::prelude::*;

fn flex<'a, C: Context<'a>>(cx: &mut C, len: u32) -> JsResult<'a, JsArray> {
    let a = cx.empty_array();

    for i in 0..len {
        let v = cx.number(i);
        a.set(cx, i, v)?;
    }

    Ok(a)
}

fn fixed<'a, C: Context<'a>>(cx: &mut C, len: u32) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, len);

    for i in 0..len {
        let v = cx.number(i);
        a.set(cx, i, v)?;
    }

    Ok(a)
}

fn make_flex_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let size: Handle<JsNumber> = cx.argument(0)?;
    let size = size.value(&mut cx);
    flex(&mut cx, size as u32)
}

fn make_fixed_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let size: Handle<JsNumber> = cx.argument(0)?;
    let size = size.value(&mut cx);
    fixed(&mut cx, size as u32)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("makeFlexArray", make_flex_array)?;
    cx.export_function("makeFixedArray", make_fixed_array)?;
    Ok(())
}
