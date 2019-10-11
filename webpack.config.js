const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

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
  plugins: [
    new MonacoWebpackPlugin({
      output: 'static/js',
      languages: [
        'csharp', 
        'cpp',
        'html',
        'java',
        'javascript',
        'markdown',
        'objective-c',
        'perl',
        'php',
        'python',
        'rust',
        'shell',
        'sql',
        'typescript',
      ],
    }),
  ],
}
