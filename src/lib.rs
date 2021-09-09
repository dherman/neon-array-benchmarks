use std::cell::RefCell;
use std::ops::Range;

use neon::prelude::*;

struct Iterable(RefCell<Range<u32>>);

impl Iterable {
    fn new(max: u32) -> Self {
        Self(RefCell::new(0..max))
    }

    fn next(&self) -> Option<u32> {
        self.0.borrow_mut().next()
    }
}

impl Finalize for Iterable {}

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

fn make_iterable(mut cx: FunctionContext) -> JsResult<JsBox<Iterable>> {
    let size = cx.argument::<JsNumber>(0)?.value(&mut cx);

    Ok(cx.boxed(Iterable::new(size as u32)))
}

fn iterable_next(mut cx: FunctionContext) -> JsResult<JsValue> {
    let iterable = cx.argument::<JsBox<Iterable>>(0)?;

    if let Some(n) = iterable.next() {
        Ok(cx.number(n).upcast())
    } else {
        Ok(cx.undefined().upcast())
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("makeFlexArray", make_flex_array)?;
    cx.export_function("makeFixedArray", make_fixed_array)?;
    cx.export_function("makeIterable", make_iterable)?;
    cx.export_function("iterableNext", iterable_next)?;
    Ok(())
}
