const queryString = require('querystring');
const {generateRandomText} = require('../../lib/random')
const {clientId} = require('../../lib/definitions')

const requestAuthorize = (req, res, next) => {
  const state = generateRandomText(20)
  const query = queryString.encode({
    response_type: 'code',
    scope: 'foo',
    client_id: clientId,
    redirect_uri: '/client/callback',
    state
  })
  res.redirect(302, `/authorization_server/authorize.html?${query}`)
  next()
}

module.exports = {requestAuthorize}
