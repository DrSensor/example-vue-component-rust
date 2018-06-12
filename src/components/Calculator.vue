<template>
  <div>{{a}} <slot/> {{b}} = {{result}}</div>
</template>

<script lang="ts">
import { Component, Prop, Watch, Vue } from 'vue-property-decorator'

@Component
export default class Led extends Vue {
  @Prop() a!: number
  @Prop() b!: number
  @Prop() operation!: string
  @Prop() module!: string

  private wasm!: { instance: WebAssembly.Instance }

  @Watch('operation')
  changeOperation(op: string) {
    this.calculate = this.wasm.instance.exports[op]
  }

  @Watch('module')
  async changeModule(mod: string) {
    let loadWasm
    if (mod === 'algebra') {
      loadWasm = await import(/*webpackChunkName: 'calculator.algebra'*/'@/libs/algebra-matrix2x2/calculator.rs')
    } else if (mod === 'arithmatic') {
      loadWasm = await import(/*webpackChunkName: 'calculator.arithmatic'*/'@/libs/arithmatic/calculator.rs')
    } else {
      loadWasm = await import(/*webpackChunkName: 'calculator.empty'*/'@/libs/empty/calculator.rs')
    }
    this.wasm = await loadWasm.default()
  }

  async mounted() {
    await this.changeModule(this.module)
    await this.changeOperation(this.operation)
  }

  private calculate = (a: number, b: number) => Number()

  get result() {
    return this.calculate(this.a, this.b)
  }
}
</script>

<style scoped>
</style>
