import test from "ava";
import path from 'path';

import { Config, Engine, Module, Store, Instance } from "../index.js";

test("simple wasm script", (t) => {
  const config = new Config();
  const engine = new Engine(config);

  const module = Module.fromFile(engine, path.join(process.cwd(), '__test__', 'hello.wat'));
  console.log(module);

  const store = new Store(engine);
  console.log(store);

  const instance = new Instance(store, module);

  {
    const test_arg1_i32_r32 = instance.getFunc(store, "test_arg1_i32_ri32");
    t.is(
        test_arg1_i32_r32.call(store, [
          128
        ]),
        128
    );

    t.is(
        test_arg1_i32_r32.call(store, [
          256
        ]),
        256
    );
  }

  {
    const test_arg1_i64_ri64 = instance.getFunc(store, "test_arg1_i64_ri64");
    t.is(
        test_arg1_i64_ri64.call(store, [
          128
        ]),
        128
    );

    t.is(
        test_arg1_i64_ri64.call(store, [
          256
        ]),
        256
    );
  }

  {
    const test_arg1_f64_rf64 = instance.getFunc(store, "test_arg1_f64_rf64");
    t.is(
        test_arg1_f64_rf64.call(store, [
            Math.PI
        ]),
        Math.PI
    );

    t.is(
        test_arg1_f64_rf64.call(store, [
            0
        ]),
        0
    );
  }

  t.assert(true);
});
