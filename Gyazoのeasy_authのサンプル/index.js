const fs = require('fs').promises
const fetch = require('node-fetch')
const FormData = require("form-data");

const GyazoUrl = 'https://upload.gyazo.com/api/upload/easy_auth'
const GyazoClientId = '47d033a70e1778052a106e881c83d669122021aa9f9c454172cccae2b797331b'
const RefererUrl = 'https://hyiromori.com/'

const main = async (imageFilePath) => {
  console.info(imageFilePath)
  const imageBufferData = await fs.readFile(imageFilePath)
  const imageBase64 = imageBufferData.toString('base64')
  const imageDataUrl = `data:image/png;base64,${imageBase64}`

  const formData = new FormData()
  formData.append('client_id', GyazoClientId)
  formData.append('image_url', imageDataUrl)
  formData.append('referer_url', RefererUrl)

  const response = await fetch(GyazoUrl, {
    method: 'POST',
    body: formData,
  })
  console.info("Response Code:", response.status)
  const data = await response.json()
  console.info(data)
}

main(process.argv[2]).catch(console.error)
