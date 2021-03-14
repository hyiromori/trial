const encodeBasicAuth = (clientId, clientSecret) =>
  Buffer.from(`${clientId}:${clientSecret}`).toString('base64')

const decodeBasicAuth = (token) => {
  const [clientId, clientSecret] = Buffer.from(token, 'base64').toString().split(':')
  return {clientId, clientSecret}
}

module.exports = {encodeBasicAuth, decodeBasicAuth}
