module.exports = {
  entry: './js/index',
  output: {
    filename: 'static/js/index.js',
    path: __dirname,
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
}
