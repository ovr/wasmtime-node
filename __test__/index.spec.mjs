import test from "ava";

import { Engine, Module, Linker, Store, Instance } from "../index.js";

test("simple wasm script", (t) => {
  const engine = new Engine();
  console.log(engine);

  const module = Module.fromFile(engine, '/Users/ovr/projects/cube/wasmtime-node/wasmtime-node/__test__/hello.wat');
  console.log(module);

  // const linker = new Linker(engine);
  // console.log(linker);

  const store = new Store(engine);
  console.log(store);

  const instance = new Instance(store, module);
  const answer_fn = instance.getFunc(store, "answer");

  console.log(answer_fn);

  const res = answer_fn.call0(store);
  console.log(res);

  t.assert(true);
});
