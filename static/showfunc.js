function showFunctionCall(funcName, params, result) {
    // Create a new div element
    var newElement = document.createElement("div");
    newElement.className = "border-2 border-blue-500 p-4 m-2 rounded-lg bg-gray-800";

    // Format the function call name and parameters
    var funcCall = document.createElement("p");

    var funcNameSpan = document.createElement("span");
    funcNameSpan.className = "text-orange-500";
    funcNameSpan.textContent = '⚡ ' + funcName + "(";
    funcCall.appendChild(funcNameSpan);

    for (var key in params) {
        var paramSpan = document.createElement("span");
        paramSpan.className = "text-yellow-500";
        paramSpan.textContent = key + ": ";
        funcCall.appendChild(paramSpan);

        var valueSpan = document.createElement("span");
        valueSpan.className = "text-cyan-500";
        valueSpan.textContent = params[key] + ", ";
        funcCall.appendChild(valueSpan);
    }

    funcCall.textContent = funcCall.textContent.slice(0, -2) + ")";
    newElement.appendChild(funcCall);

    // Add an arrow (summary/details) that will show the full result when clicked
    var summary = document.createElement("summary");
    summary.textContent = "Result";
    var details = document.createElement("details");
    var resultSpan = document.createElement("span");
    resultSpan.className = "text-white";
    resultSpan.textContent = result;
    details.appendChild(resultSpan);
    details.appendChild(summary);
    newElement.appendChild(details);
    return newElement.outerHTML;
}
