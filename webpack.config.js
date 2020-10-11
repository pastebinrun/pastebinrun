const { StatsWriterPlugin } = require("webpack-stats-plugin")
const glob = require('glob')
const path = require('path')

module.exports = [
  {
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
      new StatsWriterPlugin({
        filename: 'entry',
        transform(data) {
          return data.assetsByChunkName.main[0]
        },
      }),
    ],
  },
  {
    entry: Object.fromEntries(glob.sync('./node_modules/prismjs/components/*.min.js')
      .map(p => [path.basename(p), p])),
    output: {
      filename: 'static/js/[name]',
      path: __dirname,
    }
  },
]
