#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use wasmtime::*;

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
  inner: Module,
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
  inner: Store<()>,
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
  inner: Linker<Engine>,
}

#[napi]
impl JSLinker {
  #[napi(constructor)]
  pub fn new(engine: &JSEngine) -> Self {
    Self {
      inner: Linker::new(&engine.inner),
    }
  }

  pub fn instantiate(&self, store: &mut JSStore, module: &JSModule) {
    // self.inner.instantiate(&mut store.inner, &module.inner).unwrap();
  }
}

#[napi(js_name = "WasmFunctionTy")]
pub struct WasmFunctionTy {
  inner: FuncType,
}

#[napi(js_name = "WasmFunction")]
pub struct WasmFunction {
  inner: Func,
}

#[napi]
impl WasmFunction {
  #[napi]
  pub fn ty(&self, store: &mut JSStore) -> WasmFunctionTy {
    WasmFunctionTy {
      inner: self.inner.ty(&mut store.inner),
    }
  }

  #[napi]
  pub fn call0(&self, store: &mut JSStore) {
    let mut results = [];
    let result = self.inner.call(&mut store.inner, &[], &mut results);
  }
}

#[napi(js_name = "Instance")]
pub struct JSInstance {
  inner: Instance,
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
  pub fn get_func(&self, store: &mut JSStore, fn_name: String) -> Option<WasmFunction> {
    if let Some(fun) = self.inner.get_func(&mut store.inner, &fn_name) {
      Some(WasmFunction { inner: fun })
    } else {
      None
    }
  }
}
