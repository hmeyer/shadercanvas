import {EditorView, basicSetup} from "https://esm.sh/@codemirror/basic-setup@0.20.0";
import {cpp} from "https://esm.sh/@codemirror/lang-cpp@6.0.2";
import {oneDark} from "https://esm.sh/@codemirror/theme-one-dark@6.1.2";
import {keymap} from "https://esm.sh/@codemirror/view@6.36.5";

const defaultShader = `void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}`;

const errorBox = document.getElementById('error-box');

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

const editor = new EditorView({
  doc: defaultShader,
  extensions: [
    basicSetup,
    cpp(),
    oneDark,
    keymap.of([{
      key: "Ctrl-Enter",
      mac: "Cmd-Enter",
      run: compile,
    }]),
    EditorView.theme({
      "&": {maxHeight: "300px"},
      ".cm-scroller": {overflow: "auto"},
    }),
  ],
  parent: document.getElementById('editor'),
});

document.getElementById('compile-btn').addEventListener('click', () => compile(editor));
