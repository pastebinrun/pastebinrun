const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin")
const { StatsWriterPlugin } = require("webpack-stats-plugin")

module.exports = {
  entry: './js/index',
  output: {
    filename: 'static/js/[chunkhash].js',
    path: __dirname,
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
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
    new StatsWriterPlugin({
      filename: 'entry',
      transform(data) {
        return data.assetsByChunkName.main
      },
    }),
  ],
}
