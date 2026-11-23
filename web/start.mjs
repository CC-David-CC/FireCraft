try {
  const bindings = await import('./firecraft-web.js');
  await bindings.default();
  await bindings.start();
  if (bindings.workspace_ready()) {
    window.wasmBindings = bindings;
  }
} catch (error) {
  const status = document.getElementById('firecraft-start-error');
  status.textContent = 'FireCraft could not start. ' + (error?.message ?? String(error));
  status.setAttribute('data-start-error', '');
  status.hidden = false;
}
