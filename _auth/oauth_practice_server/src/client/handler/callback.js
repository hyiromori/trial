const fetch = require('node-fetch')
const {encodeBasicAuth} = require('../../lib/basic_auth')
const {clientId, clientSecret} = require('../../lib/definitions')

const callback = (req, res, next) => {
  const data = req.query
  const body = JSON.stringify({
    grant_type: 'authorization_code',
    redirect_uri: '/client/callback',
    code: data.code
  })

  fetch('http://localhost:45678/authorization_server/token', {
    method: 'POST',
    headers: {
      'Authorization': `Basic ${encodeBasicAuth(clientId, clientSecret)}`,
      'Content-Type': 'application/json'
    },
    body,
  }).then((response) => {
    return response.json()
  }).then((json) => {
    const accessToken = json.access_token
    res.redirect(`/client/success.html?access_token=${accessToken}`)
  }).catch((err) => {
    console.error(err)
  }).finally(next)
}

module.exports = {callback}
