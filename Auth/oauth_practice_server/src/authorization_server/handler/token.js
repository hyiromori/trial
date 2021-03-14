const {generateRandomText} = require('../../lib/random')
const {runSelectRow} = require('../../sqlite3')
const {decodeBasicAuth} = require('../../lib/basic_auth')
const {clientId: permitClientId, clientSecret: permitClientSecret, redirectUri: permitRedirectUri} = require('../../lib/definitions')

const token = (req, res, next) => {
  const {authorization} = req.headers

  const badRequest = (message) => {
    res.status(400).send(`ERROR: ${message}`)
  }
  if (typeof authorization !== 'string' || !authorization.startsWith('Basic ')) {
    badRequest('Basic Authorization Error')
    return
  }
  const {clientId, clientSecret} = decodeBasicAuth(authorization.substr(6))
  if (clientId !== permitClientId || clientSecret !== permitClientSecret) {
    badRequest('Invalid Authorization')
    return
  }

  const {grant_type, redirect_uri, code} = req.body

  if (grant_type !== 'authorization_code') {
    badRequest('Not supported grant_type')
    return
  }

  if (redirect_uri !== permitRedirectUri) {
    badRequest(`Bad redirect_uri: ${data.redirect_uri}`)
    return
  }

  // !!!CAUTION!!!: SQLインジェクション対策はしていません
  runSelectRow(`SELECT state FROM authorizations WHERE code = '${code}'`).then(data => {
    if (data == null) {
      badRequest('Invalid code')
      return
    }

    const accessToken = generateRandomText(40)
    res.header('Content-Type', 'application/json')
      .send(JSON.stringify({access_token: accessToken, token_type: 'Bearer'}))
    next()
  })
}

module.exports = {token}
