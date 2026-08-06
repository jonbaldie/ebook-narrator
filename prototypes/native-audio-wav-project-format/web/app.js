const invoke = window.__TAURI__.core.invoke;
let fixturePath = "";
let currentState = null;

const byId = (id) => document.getElementById(id);

async function refresh(message = "Ready.") {
  currentState = await invoke("snapshot");
  byId("state").textContent = JSON.stringify(currentState, null, 2);
  byId("message").textContent = message;
  byId("message").className = "";
  const segment = currentState.project.segments[currentState.project.currentSegmentIndex];
  byId("teleprompter").textContent = segment ? segment.text : "Import an EPUB.";
  byId("recording-state").textContent = currentState.recording.active
    ? `${currentState.recording.paused ? "Paused" : "Recording"} — ${currentState.recording.elapsedSeconds} seconds — ${currentState.recording.sourceSetting}`
    : currentState.recording.failure || "No Take is recording.";
  fillSelect("input-device", currentState.devices.inputs, currentState.selectedInput);
  fillSelect("output-device", currentState.devices.outputs, currentState.selectedOutput);
  fillTakes();
}

function fillSelect(id, devices, selected) {
  const select = byId(id);
  const previous = selected || select.value;
  const fallback = devices.find((device) => device.isDefault)?.name || devices[0]?.name || "";
  const value = devices.some((device) => device.name === previous) ? previous : fallback;
  select.replaceChildren(...devices.map((device) => {
    const option = document.createElement("option");
    option.value = device.name;
    option.textContent = `${device.name}${device.isDefault ? " (default)" : ""}`;
    return option;
  }));
  if (value) select.value = value;
}

function fillTakes() {
  const select = byId("take");
  const value = currentState.project.selectedTakeId || select.value;
  select.replaceChildren(...currentState.project.takes.map((take) => {
    const option = document.createElement("option");
    option.value = take.id;
    option.textContent = `${take.id.slice(0, 8)} — ${(take.frameCount / take.wavSampleRate).toFixed(1)} seconds`;
    return option;
  }));
  if (value) select.value = value;
  const take = currentState.project.takes.find((item) => item.id === value);
  if (take) {
    byId("trim-start").value = take.trim.startFrame;
    byId("trim-end").value = take.trim.endFrame;
  }
}

async function action(fn) {
  try {
    const result = await fn();
    await refresh("Action complete.");
    return result;
  } catch (error) {
    byId("message").textContent = String(error);
    byId("message").className = "error";
    try { await refresh(String(error)); byId("message").className = "error"; } catch (_) {}
    throw error;
  }
}

byId("create-fixture").onclick = () => action(async () => {
  fixturePath = await invoke("create_fixture_epub");
  byId("epub-path").value = fixturePath;
});
byId("import-fixture").onclick = () => action(() => invoke("import_epub", { path: fixturePath || byId("epub-path").value }));
byId("import-path").onclick = () => action(() => invoke("import_epub", { path: byId("epub-path").value }));
byId("previous").onclick = () => action(() => invoke("move_segment", { offset: -1 }));
byId("next").onclick = () => action(() => invoke("move_segment", { offset: 1 }));
byId("select-devices").onclick = () => action(() => invoke("select_devices", { inputName: byId("input-device").value, outputName: byId("output-device").value }));
byId("start").onclick = () => action(() => invoke("start_recording"));
byId("pause").onclick = () => action(() => invoke("pause_recording"));
byId("resume").onclick = () => action(() => invoke("resume_recording"));
byId("stop").onclick = () => action(() => invoke("stop_recording"));
byId("three-second").onclick = () => action(async () => {
  await invoke("start_recording");
  await new Promise((resolve) => setTimeout(resolve, 3000));
  await invoke("stop_recording");
  await invoke("play_selected_take");
});
byId("select-take").onclick = () => action(() => invoke("select_take", { takeId: byId("take").value }));
byId("take").onchange = () => action(() => invoke("select_take", { takeId: byId("take").value }));
byId("apply-trim").onclick = () => action(async () => {
  await invoke("select_take", { takeId: byId("take").value });
  await invoke("apply_trim", { startFrame: Number(byId("trim-start").value), endFrame: Number(byId("trim-end").value) });
});
byId("play").onclick = () => action(async () => {
  await invoke("select_take", { takeId: byId("take").value });
  await invoke("play_selected_take");
});
byId("export").onclick = () => action(async () => {
  const result = await invoke("export_selected_take");
  byId("export-result").textContent = JSON.stringify(result, null, 2);
});
byId("record-observation").onclick = () => action(() => invoke("record_observation", {
  test: byId("observation-test").value,
  result: byId("observation-result").value,
  note: byId("observation-note").value,
}));

refresh().catch((error) => {
  byId("message").textContent = String(error);
  byId("message").className = "error";
});
setInterval(() => { if (currentState?.recording?.active) refresh("Recording state updated."); }, 1000);
