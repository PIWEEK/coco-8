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

async function handleSource(file) {
  const contents = await file.text();
  document.querySelector("#coco-source").innerHTML = contents;
}

async function fetchFile(path) {
  try {
    const response = await fetch(path);
    return response;
  } catch (err) {
    console.error(err);
  }
  return null;
}

function setupRomSelector(defaultRom, { selectEl, ...others }) {
  const defaultOption = selectEl.querySelector(`option[value="${defaultRom}"]`);
  defaultOption.selected = true;

  selectEl.addEventListener("change", async (event) => {
    const filename = event.target.value;
    await fetchBytecodeAndSource(filename, { ...others });
  });
}

function setupControls({
  showBytecodeCheckbox,
  showSourceCheckbox,
  bytecodeEl,
  sourceEl,
}) {
  showBytecodeCheckbox.addEventListener("change", (event) => {
    bytecodeEl.style.display = event.target.checked ? "block" : "none";
  });

  showSourceCheckbox.addEventListener("change", (event) => {
    sourceEl.style.display = event.target.checked ? "block" : "none";
  });
}

async function fetchBytecodeAndSource(
  filename,
  { showSourceCheckbox, showBytecodeCheckbox, bytecodeEl, sourceEl }
) {
  showSourceCheckbox.disabled = true;
  showBytecodeCheckbox.disabled = true;

  bytecodeEl.innerHTML = "Loading…";
  sourceEl.innerHTML = "Loading…";

  fetchFile(`roms/${filename}.rom`).then(async (rom) => {
    await handleFile(rom);
    showBytecodeCheckbox.disabled = false;
  });

  fetchFile(`roms/${filename}.tal`).then(async (source) => {
    await handleSource(source);
    showSourceCheckbox.disabled = false;
  });
}

async function main() {
  const _ = await initWasm("./vendor/coco_ui_bg.wasm");

  const romSelector = document.querySelector("#coco-rom-selector");
  const showBytecodeCheckbox = document.querySelector("#coco-show-bytecode");
  const bytecodeEl = document.querySelector("#coco-bytecode");
  let showSourceCheckbox = document.querySelector("#coco-show-source");
  let sourceEl = document.querySelector("#coco-source");

  let debugControls = {
    sourceEl,
    showBytecodeCheckbox,
    bytecodeEl,
    showSourceCheckbox,
  };

  const defaultRom = "sprite";
  setupRomSelector(`${defaultRom}`, {
    selectEl: romSelector,
    ...debugControls,
  });
  setupControls(debugControls);
  await fetchBytecodeAndSource(defaultRom, debugControls);
}

main();
