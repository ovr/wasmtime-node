#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use std::cell::RefCell;
use wasmtime::*;
mod test;

#[napi(js_name = "Engine")]
pub struct JSEngine {
  inner: Engine,
}

#[napi]
impl JSEngine {
  #[napi(constructor)]
  pub fn new() -> Self {
    JSEngine {
      inner: Engine::default(),
    }
  }
}

#[napi(js_name = "Module")]
pub struct JSModule {
  inner: Module
}

#[napi]
impl JSModule {
  #[napi(factory)]
  pub fn from_file(js_engine: &JSEngine, path: String) -> Self {
    JSModule {
      inner: Module::from_file(&js_engine.inner, &path).unwrap(),
    }
  }
}

#[napi(js_name = "Store")]
pub struct JSStore {
  inner: Store<()>
}

#[napi]
impl JSStore {
  #[napi(constructor)]
  pub fn new(engine: &JSEngine) -> Self {
    Self {
      inner: Store::new(&engine.inner, ()),
    }
  }
}

#[napi(js_name = "Linker")]
pub struct JSLinker {
  inner: Linker<Engine>
}

#[napi]
impl JSLinker {
  #[napi(constructor)]
  pub fn new(engine: &JSEngine) -> Self {
    Self {
      inner: Linker::new(&engine.inner),
    }
  }

  pub fn instantiate(&self, store: &JSStore, module: &JSModule) {
    // self.inner.instantiate(store.inner, &module.inner);
  }
}

#[napi(js_name = "WasmFunction")]
pub struct WasmFunction {
  inner: Func
}

#[napi(js_name = "Instance")]
pub struct JSInstance {
  inner: Instance
}

#[napi]
impl JSInstance {
  #[napi(constructor)]
  pub fn new(store: &mut JSStore, module: &JSModule) -> Self {
    Self {
      inner: Instance::new(&mut store.inner, &module.inner, &[]).unwrap(),
    }
  }

  #[napi]
  pub fn get_func(&self, store: &mut JSStore, fn_name: String) -> WasmFunction {
    let fun = self.inner.get_func(&mut store.inner, &fn_name).expect("function not defined");

    WasmFunction {
      inner: fun
    }
  }
}
