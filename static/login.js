window.baseURL = window.baseURL || window.location.href.replace(/\/$/, '') + '/'
console.log({baseURL: window.baseURL})

async function anonymousLogin() {
  console.log('anonymouseLogin()')
  let user = localStorage.getItem('username')
  let pass = ''
  if (!user) {
    user = generateAnonUsername()
    localStorage.setItem('username', user)
  }
  console.log({user, pass})
  return await login(user, pass)
}

async function login(username, password) {
  if (!window.baseURL) {
    throw new Error("No baseURL")
  }
  console.log('logging in',window.baseURL);
  fetch(window.baseURL + 'login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
  })
  .then(response => response.json())
  .then(data => {
      localStorage.setItem('token', data.token)
      console.log('token is ',data.token)
  })
}

function openEventSource(relurl) {
  
  console.log('openEventSource(',relurl,')')
  const token = localStorage.getItem('token')
  const queryParams = new URLSearchParams(window.location.search);
  const session_id = queryParams.get('session_id') || '10';
  const url = relurl + `?token=${encodeURIComponent(token)}&session_id=${session_id}`
  return new EventSource(url)
}

async function Request(relurl, params) {
  if (!window.baseURL) {
    throw new Error("No baseURL")
  }
  const token = localStorage.getItem('token')
  if (!token) throw new Error("No access token")

  if (!params.headers) params.headers = {}
  Object.assign(params.headers, {'Authorization': `Bearer ${token}`})
  const resp = await fetch( window.baseURL + relurl, params )
  return await resp.json()
}
