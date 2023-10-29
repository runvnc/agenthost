function showFunctionCall(funcName, params, result) {
  let paramshtml = ''
  for (var key in params) {
      paramshtml += `<span class="param_name">${key}</span> `;
      paramshtml += `<span class="param_value">${params[key]}</span>  `;
  }

  let html = ` 
    <div class="block border-2 border-blue-500 p-4 m-2 rounded-lg bg-gray-800"  >
      ⚡  <span class="fn_name">${funcName}</span> ${paramshtml} 
      <details class="block">
        <summary>Result</summary>
        <pre><code class="font-mono whitespace-pre-wrap" >${result}</code></pre>
      </details>
    </div>

  `
  return html
}
