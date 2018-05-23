<template>
  <div>{{a}} <slot/> {{b}} = {{result}}</div>
</template>

<script lang="ts">
import { Component, Prop, Watch, Vue } from 'vue-property-decorator'
import loadWasm from '@/libs/calculator.rs'

@Component
export default class Led extends Vue {
  @Prop() a!: number
  @Prop() b!: number
  @Prop() operation!: string

  @Watch('operation')
  changeOperation(op: string) {
    loadWasm().then(result => {
      this.calculate = result.instance.exports[op]
    })
  }

  beforeCreate() {
    loadWasm().then(result => {
      this.calculate = result.instance.exports.add
    })
  }

  private calculate = (a: number, b: number) => Number()

  get result() {
    return this.calculate(this.a, this.b)
  }
}
</script>

<style scoped>
</style>
