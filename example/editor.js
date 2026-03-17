import {EditorView, basicSetup} from "https://esm.sh/@codemirror/basic-setup@0.20.0";
import {cpp} from "https://esm.sh/@codemirror/lang-cpp@6.0.2";
import {oneDark} from "https://esm.sh/@codemirror/theme-one-dark@6.1.2";
import {keymap} from "https://esm.sh/@codemirror/view@6.36.5";
import {HighlightStyle, syntaxHighlighting} from "https://esm.sh/@codemirror/language@6.10.8";
import {tags} from "https://esm.sh/@lezer/highlight@1.2.1";

const errorBox = document.getElementById('error-box');

// Custom highlight style for GLSL keywords and types
const glslHighlight = HighlightStyle.define([
  {tag: tags.keyword, color: "#c678dd", fontWeight: "bold"},
  {tag: tags.typeName, color: "#e5c07b"},
  {tag: tags.number, color: "#d19a66"},
  {tag: tags.string, color: "#98c379"},
  {tag: tags.comment, color: "#7f848e", fontStyle: "italic"},
  {tag: tags.function(tags.variableName), color: "#61afef"},
  {tag: tags.operator, color: "#56b6c2"},
  {tag: tags.definition(tags.variableName), color: "#e06c75"},
  {tag: tags.propertyName, color: "#61afef"},
]);

function compile(view) {
  if (typeof window.setShader !== 'function') return true;
  const code = view.state.doc.toString();
  const err = window.setShader(code);
  if (err && err !== null) {
    errorBox.textContent = String(err);
    errorBox.style.display = 'block';
  } else {
    errorBox.style.display = 'none';
  }
  return true;
}

function initEditor() {
  const shaderCode = window.defaultShader ||
    "void mainImage(out vec4 c, in vec2 f) { c = vec4(0.0); }";

  const editor = new EditorView({
    doc: shaderCode,
    extensions: [
      basicSetup,
      cpp(),
      oneDark,
      syntaxHighlighting(glslHighlight),
      keymap.of([{
        key: "Ctrl-Enter",
        mac: "Cmd-Enter",
        run: compile,
      }]),
      EditorView.theme({
        "&": {minHeight: "300px", maxHeight: "500px"},
        ".cm-scroller": {overflow: "auto"},
      }),
    ],
    parent: document.getElementById('editor'),
  });

  document.getElementById('compile-btn').addEventListener('click', () => compile(editor));
}

// Wait for WASM to finish loading before creating the editor.
// Trunk dispatches TrunkApplicationStarted after the WASM module is initialized,
// so we listen for that event instead of using a top-level await (which would
// block the WASM init script from running, causing a deadlock).
if (window.defaultShader) {
  // WASM already loaded (unlikely but handle it)
  initEditor();
} else {
  window.addEventListener("TrunkApplicationStarted", () => initEditor(), {once: true});
}
