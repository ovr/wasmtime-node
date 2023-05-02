#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::JsNumber;
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
  ty: FuncType,
}

#[napi]
impl WasmFunction {
  #[napi]
  pub fn ty(&self, store: &mut JSStore) -> WasmFunctionTy {
    WasmFunctionTy {
      inner: self.ty.clone(),
    }
  }

  #[napi]
  pub fn call(&self, env: Env, store: &mut JSStore, args: Array) -> napi::Result<JsNumber> {
    let result_types = self.ty.results();
    let mut results = Vec::with_capacity(result_types.len());

    for return_ty in result_types {
      let return_val = match return_ty {
        ValType::F64 => Val::F64(0),
        ValType::I64 => Val::I64(0),
        ValType::I32 => Val::I32(0),
        other_ty => {
          return Err(napi::Error::from_reason(format!(
            "Unsupported return type: {:?}",
            other_ty
          )))
        }
      };
      results.push(return_val);
    }

    let params_types = self.ty.params();
    let mut params = Vec::with_capacity(params_types.len());

    for (idx, param_ty) in self.ty.params().enumerate() {
      let param_val = match param_ty {
        ValType::I32 => {
          let argument_val: JsNumber = args.get(idx as u32)?.unwrap();

          Val::I32(argument_val.get_int32()?)
        }
        ValType::I64 => {
          let argument_val: JsNumber = args.get(idx as u32)?.unwrap();

          Val::I64(argument_val.get_int64()?)
        }
        ValType::F64 => {
          let argument_val: JsNumber = args.get(idx as u32)?.unwrap();

          Val::F64(argument_val.get_double()?.to_bits())
        }
        other_ty => {
          return Err(napi::Error::from_reason(format!(
            "Unsupported parameter type: {:?}",
            other_ty
          )))
        }
      };
      params.push(param_val);
    }

    self
      .inner
      .call(&mut store.inner, &params, &mut results)
      .map_err(|err| napi::Error::from_reason(err.to_string()))?;

    if results.len() > 1 {
      return Err(napi::Error::from_reason(
        "Function returned multiple arguments, please use call1",
      ));
    }

    let result_val = results.pop().unwrap();
    match result_val {
      Val::I32(v) => env.create_int32(v),
      Val::I64(v) => env.create_int64(v),
      Val::F32(v) => env.create_double(f32::from_bits(v) as f64),
      Val::F64(v) => env.create_double(f64::from_bits(v)),
      other_ty => {
        return Err(napi::Error::from_reason(format!(
          "Unsupported return type: {:?}",
          other_ty
        )))
      }
    }
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
      Some(WasmFunction {
        inner: fun,
        ty: fun.ty(&mut store.inner),
      })
    } else {
      None
    }
  }
}
