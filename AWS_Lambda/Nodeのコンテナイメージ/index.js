const {v4: uuid} = require('uuid')

exports.lambdaHandler = (event, context, callback) => callback(null, {
  version: 'v1',
  uuid: uuid()
})
