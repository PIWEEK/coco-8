import initWasm, { runRom } from "./vendor/coco_ui.js";

async function handleFile(file) {
  const buffer = await file.arrayBuffer();
  const rom = new Uint8Array(buffer);

  const bytecode = Array.from(rom)
    .map((x) => x.toString(16).padStart(2, "0"))
    .join(" ");
  document.querySelector("#coco-bytecode").innerHTML = bytecode;

  const output = runRom(rom);
  if (output.debug) {
    console.log(output.debug);
  }
}

async function fetchRom(path) {
  try {
    const response = await fetch(path);
    return response;
  } catch (err) {
    console.error(err);
  }
  return null;
}

function setupRomSelector(selectEl, defaultRom) {
  const defaultOption = selectEl.querySelector(`option[value="${defaultRom}"]`);
  defaultOption.selected = true;

  selectEl.addEventListener("change", async (event) => {
    const romUrl = `/roms/${event.target.value}`;
    const rom = await fetchRom(romUrl);
    if (rom) {
      await handleFile(rom);
    }
  });
}

function setupControls() {
  let showBytecodeCheckbox = document.querySelector("#coco-show-bytecode");
  let bytecodeEl = document.querySelector("#coco-bytecode");

  showBytecodeCheckbox.addEventListener("change", (event) => {
    bytecodeEl.style.display = event.target.checked ? "block" : "none";
  });
}

async function main() {
  const _ = await initWasm("./vendor/coco_ui_bg.wasm");
  const romSelector = document.querySelector("#coco-rom-selector");

  const defaultRom = "put_pixel.rom";
  setupRomSelector(romSelector, defaultRom);
  setupControls();

  const rom = await fetchRom(`/roms/${defaultRom}`);
  if (rom) {
    await handleFile(rom);
  }
}

main();
