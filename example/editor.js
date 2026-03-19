function initEditor() {
  const textarea = document.getElementById('editor');
  const errorBox = document.getElementById('error-box');

  textarea.value = window.defaultShader;

  function compile() {
    const err = window.setShader(textarea.value);
    if (err) {
      errorBox.textContent = String(err);
      errorBox.style.display = 'block';
    } else {
      errorBox.style.display = 'none';
    }
  }

  document.getElementById('compile-btn').addEventListener('click', compile);
  textarea.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') compile();
  });
}

function waitForWasm() {
  if (window.defaultShader) {
    initEditor();
  } else {
    setTimeout(waitForWasm, 50);
  }
}
waitForWasm();
