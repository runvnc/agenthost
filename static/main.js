    await anonymousLogin();
    openEventSource('/chat')

    loadSessions();

    var user_id;
    var currParagraph;
    var rawMarkdown = '';
    window.sessionLoaded = false;

    async function loadSession(session) {
      window.session_id = session
      window.history.pushState({session_id: session}, "", "?session_id=" + session);
      try { 
        window.sse.close()
      } catch (e) { }
      openEventSource('chat')
      setTimeout( () => {
        let uri = encodeURIComponent("//history")
        console.log({uri})
        sendMsg(uri, false)
      }, 10)
    }

    function removeUSERPrefix(lines) {
      let out = []
      for (let line of lines) {
        if (line.startsWith('USER: '))
          line = line.replace('USER: ', '')
        out.push(line)
      }
      return out
    }

    function removeSysLines(msg) {
      let lines = msg.split('\n')
      let out = []
      for (let line of lines)
        if (!(line.startsWith('SYSTEM: ')))
          out.push(line)
      return removeUSERPrefix(out).join('\n')
    }
  
    function message(data, sender) {
        console.log("Message handler")
        var msgElement = document.createElement('div');
        var avatarElement = document.createElement('img');
        avatarElement.src = sender == 'You' ? '/user.webp' : '/agent.webp';
        avatarElement.classList.add('av');
        var nameElement = document.createElement('span');
        nameElement.textContent = sender;
        nameElement.classList.add('name');
        msgElement.appendChild(avatarElement);
        msgElement.appendChild(nameElement);
        currParagraph = document.createElement('p');
        //currParagraph.innerHTML += data;
        msgElement.appendChild(currParagraph);
        console.log('message()')
        rawMarkdown = '';
        let html = markdownit().render(data);
        currParagraph.innerHTML = html
        chat.appendChild(msgElement)
        chat.scrollTop = chat.scrollHeight;
    }
    async function loadSessions() {
        const sessions = await Request(`/sessions?session_id=${window.session_id}&token=${window.token}`, { method: 'GET' });
        const sessionList = document.getElementById('session-list');
        sessionList.innerHTML = '';
        sessions.forEach(session => {
            const li = document.createElement('li');
            li.textContent = session;
            li.addEventListener('click', () => {
                chat.innerHTML = "<p><em>Loading session...</em></p>";
                loadSession(session);
            });
            sessionList.appendChild(li);
        });
    }

  function openEventSource(relurl) {
    try {
    window.sse.close()
    } catch (e) {

    }
    console.log('openEventSource(',relurl,')')
    const token = localStorage.getItem('token')
    const queryParams = new URLSearchParams(window.location.search);
    window.session_id = queryParams.get('session_id') || '10';
    window.token = token;
    const url = relurl + `?token=${encodeURIComponent(token)}&session_id=${session_id}`
    window.sse = new EventSource(url)
  
    sse.onopen = function() {
        chat.innerHTML = "<p><em>Connected!</em></p>";
    }
    sse.onerror = function(e) {
        console.error(e)
        //console.warn("Lost server connection, will reconnect in 5 seconds.")
        //window.sse.close()
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
        console.log({user: msg.data})
        user_id = msg.data;
    });
    sse.addEventListener("msg", function(msg) {
      setTimeout( () => {
        msg = JSON.parse(msg.data)
        let content = removeSysLines(msg.content)
        console.log(msg.role)
        console.log({msg, name: msg.name})
        if (msg.name == 'SYSTEM') {
          // only show history messages, others are handled as fragments
          message(content, msg.role == 'user' ? 'You' : 'Agent')
        }
        console.log('MESSAGE!', msg)
      }, 1)
    }); 
    sse.addEventListener("fragment", function(frag) {
      console.log("fragment", frag.data);
      setTimeout(()=> {
        if (rawMarkdown == '__WAITING__') {
          message('', 'Agent')
          rawMarkdown = ''
        }
        let text = frag.data.substr(1, frag.data.length-2);
        rawMarkdown += text;
        let html = markdownit().render(rawMarkdown);
        currParagraph.innerHTML = html;
        console.log("updated innerHTML", html);
        chat.scrollTop = chat.scrollHeight;
      }, 1);
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
        console.log("MESSAGE COMPLETE:", msg);
        //message(msg.data);
    };
}


    var input = document.getElementById("text");
    input.addEventListener("keyup", function(event) {
        if (event.keyCode === 13) {
            event.preventDefault();
            document.getElementById("send").click();
        }
    });
    var xhr;

    send.onclick = function() {
      sendMsg(text.value);
    }

    function sendMsg(msg, show=true) {
        if (xhr) {
          try {
            xhr.abort();
          } catch (e) {
          }
        }
        xhr = new XMLHttpRequest();
        let paramstr = `?session_id=${window.session_id}&token=${window.token}&msg=${msg}`
        xhr.open("GET", '/send' + paramstr, true);
        xhr.send(msg);
        if (show) {
          text.value = '';
          message(msg, 'You');
        }
        rawMarkdown = '__WAITING__';
    };
