import initWasm, { message } from "./vendor/coco_vm.js";

const _ = await initWasm("./vendor/coco_vm_bg.wasm");

const el = document.querySelector("#output");
el.textContent = message();
