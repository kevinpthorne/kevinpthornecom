const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { experiments } = require('webpack');

module.exports = {
  entry: './src/index.js',
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'Kevin P. Thorne',
    }),
  ],
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist'),
    clean: true,
  },
};
