const UrlSafeCharacters =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567890";

const generateRandomText = (size) =>
  Array.from({length: size})
    .map(() =>
      UrlSafeCharacters.charAt(
        Math.floor(Math.random() * UrlSafeCharacters.length)
      )
    )
    .join("");

module.exports = {generateRandomText}
