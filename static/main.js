    await anonymousLogin()
    var sse = openEventSource('chat');
    var user_id;
    var currParagraph;
    var rawMarkdown = '';
    function message(data, sender) {
        var msgElement = document.createElement('div');
        var avatarElement = document.createElement('img');
        avatarElement.src = sender != 'You' ? '/user.webp' : '/agent.webp';
        avatarElement.classList.add('av');
        var nameElement = document.createElement('span');
        nameElement.textContent = sender;
        nameElement.classList.add('name');
        msgElement.appendChild(avatarElement);
        msgElement.appendChild(nameElement);
        currParagraph = document.createElement('p');
        currParagraph.innerHTML += data;
        msgElement.appendChild(currParagraph);
        rawMarkdown = '';
        chat.appendChild(msgElement);
    }
    sse.onopen = function() {
        chat.innerHTML = "<p><em>Connected!</em></p>";
    }
    sse.onerror = function(e) {
        console.error(e)
        console.warn("Lost server connection, will reconnect in 5 seconds.")
        sse.close()
        //setTimeout( () => {
        //  sse = openEventSource('chat')
        //}, 5000)
    }

    function addBlankMessage() {
      currParagraph = document.createElement('p');
      chat.appendChild(currParagraph);
      rawMarkdown = '';
    }
    sse.addEventListener("user", function(msg) {
        user_id = msg.data;
    });
    sse.addEventListener("fragment", function(frag) {
      if (rawMarkdown == '__WAITING__') {
        message('', 'Agent')
        rawMarkdown = ''
      }
      let text = frag.data.substr(1, frag.data.length-2);
      rawMarkdown += text;
      let html = markdownit().render(rawMarkdown);
      currParagraph.innerHTML = html;
      chat.scrollTop = chat.scrollHeight;
    });
    sse.addEventListener("functionCall", function(fn) {
      console.log({functionCall: fn.data});
      let {name, params, result} = JSON.parse(fn.data);
      params = JSON.parse(params);
      let html = showFunctionCall(name, params, result);
      console.log(html);
      message(html, 'Agent');
      rawMarkdown = '__WAITING__';
    });
    sse.onmessage = function(msg) {
        //message(msg.data);
    };
    var input = document.getElementById("text");
    input.addEventListener("keyup", function(event) {
        if (event.keyCode === 13) {
            event.preventDefault();
            document.getElementById("send").click();
        }
    });
    var xhr;
    send.onclick = function() {
        if (xhr) {
          try {
            xhr.abort();
          } catch (e) {
          }
        }
        var msg = text.value;
        xhr = new XMLHttpRequest();
        let paramstr = `session_id=${window.session_id}&msg=${msg}`
        xhr.open("GET", window.location.host + '/send' + paramstr, true);
        xhr.send(msg);
        text.value = '';
        message(msg, 'You');
        rawMarkdown = '__WAITING__';
    };
