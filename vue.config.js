module.exports = {
  configureWebpack: {
    module: {
      rules: [{
        test: /\.rs$/,
        use: [{
          loader: 'wasm-loader'
        }, {
          loader: 'rust-native-wasm-loader',
          options: {
            release: process.env.NODE_ENV === 'production',
            gc: process.env.NODE_ENV === 'production'
          }
        }]
      }]
    }
  }
}