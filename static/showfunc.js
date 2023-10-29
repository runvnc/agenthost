function showFunctionCall(funcName, params, result) {
    // Create a new div element
    var newElement = document.createElement("div");
    newElement.className = "border-2 border-blue-500 p-4 m-2 rounded-lg bg-gray-800 text-white";

    // Format the function call name and parameters
    var funcCall = document.createElement("p");
    funcCall.textContent = '⚡ ' + funcName + "(";
    for (var key in params) {
        funcCall.textContent += key + ": " + params[key] + ", ";
    }
    funcCall.textContent = funcCall.textContent.slice(0, -2) + ")";
    newElement.appendChild(funcCall);

    // Add an arrow (summary/details) that will show the full result when clicked
    var summary = document.createElement("summary");
    summary.textContent = "Result";
    var details = document.createElement("details");
    details.textContent = result;
    details.appendChild(summary);
    newElement.appendChild(details);
    return newElement.outerHTML;
}
