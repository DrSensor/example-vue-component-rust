declare module '*.rs' {
  global {
    namespace WebAssembly {
      interface Instance {
        readonly exports: { [key: string]: any }
      }
    }
  }

  const loadWasm: () => Promise<{
    instance: WebAssembly.Instance
  }>
  export default loadWasm
}
