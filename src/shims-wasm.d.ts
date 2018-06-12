declare module '*.rs' {
  global {
    namespace WebAssembly {
      class Instance {
        readonly exports: { [key: string]: any }
        constructor()
      }
    }
  }

  const loadWasm: () => Promise<{
    instance: WebAssembly.Instance
  }>
  export default loadWasm
}

declare module '*.asc' {
  const loadWasm: () => Promise<WebAssembly.Instance>
  export default loadWasm
}
