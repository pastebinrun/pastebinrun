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
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],
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
