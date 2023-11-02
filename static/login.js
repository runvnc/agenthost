window.baseURL = window.baseURL || window.location.href.replace(/\/$/, '') + '/'
console.log({baseURL: window.baseURL)

async function login(username, password) {
  if (!window.baseURL) {
    throw new Error("No baseURL")
  }
  fetch(window.baseURL + 'login', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify({ username, password }),
  })
  .then(response => response.json())
  .then(data => {
      localStorage.setItem('token', data.token);
  })
}

async function Request(relurl, params) {
  if (!window.baseURL) {
    throw new Error("No baseURL")
  }
  const token = localStorage.getItem('token');
  if (!token) throw new Error("No access token")

  if (!params.headers) params.headers = {}
  Object.assign(params.headers, {'Authorization': `Bearer ${token}`}
  const resp = await fetch( window.baseURL + relurl, params )
  const obj = await resp.json()
  return obj
}
