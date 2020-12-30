const {generateRandomText} = require('../../lib/random')
const {runSql} = require('../../sqlite3')
const {clientId: permitClientId, redirectUri: permitRedirectUri} = require('../../lib/definitions')

const permitScope = 'foo'

const approve = (req, res, next) => {
  const data = req.body
  const badRequest = (message) => {
    res.status(400).send(`ERROR: ${message}`)
  }

  if (data.response_type !== 'code') {
    badRequest('Not supported response_type')
    return
  }

  if (data.client_id !== permitClientId) {
    badRequest(`Unknown client_id: ${data.client_id}`)
    return
  }

  if (data.scope !== permitScope) {
    badRequest('Not supported scope')
    return
  }

  if (data.redirect_uri !== permitRedirectUri) {
    badRequest(`Bad redirect_uri: ${data.redirect_uri}`)
    return
  }

  if (data.state == null) {
    badRequest('"state" not found')
    return
  }

  const state = data.state
  const code = generateRandomText(40)
  // !!!CAUTION!!!: SQLインジェクション対策はしていません
  runSql(`INSERT INTO authorizations(code, state) VALUES('${code}', '${state}')`)

  res.redirect(`${data.redirect_uri}?code=${code}&state=${state}`)
  next()
}

module.exports = {approve}
