function showFunctionCall(funcName, params, result) {
  let paramshtml = ''
  for (var key in params) {
      paramshtml += `<span class="text-yellow-500">${key}> `;
      paramshtml += `<span class="text-cyan-500">${params[key]}>  `;
  }

  let html = ` 
    <div class="border-2 border-blue-500 p-4 m-2 rounded-lg bg-gray-800"  >
      ⚡  <span class="text-orange-500">${funcName}</span> ${paramshtml} 
      <details class="block">
        <summary>Result</summary>
        <pre><code class="font-mono whitespace-pre-wrap" >${result}</code></pre>
      </details>
    </div>

  `
  return html
}
